//! Per-bucket rate-limit tracking for the Discord REST API.
//!
//! Discord uses a bucket-based rate-limiting system.  Each route (or group of
//! routes sharing a bucket) has its own independent limit.  After every
//! response Discord sends the remaining headers:
//!
//! ```text
//! X-RateLimit-Limit:      the maximum requests allowed in the window
//! X-RateLimit-Remaining:  requests remaining in the current window
//! X-RateLimit-Reset:      Unix timestamp (float) when the window resets
//! X-RateLimit-Reset-After seconds until the window resets
//! X-RateLimit-Bucket:     opaque bucket ID string
//! X-RateLimit-Global:     "true" if this is a global rate limit hit
//! X-RateLimit-Scope:      "user" | "shared" | "global"
//! ```
//!
//! The [`RateLimiter`] stores one [`Bucket`] per bucket ID.  Before every
//! request the caller checks [`RateLimiter::acquire`]; if the bucket is
//! exhausted the method sleeps until the reset instant, then returns.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

use tokio::sync::Mutex;
use tokio::time::sleep;
use tracing::{debug, warn};

// ── Bucket ────────────────────────────────────────────────────────────────────

/// State of a single Discord rate-limit bucket.
#[derive(Debug)]
pub struct Bucket {
    /// Maximum requests allowed per window.
    pub limit: u32,
    /// Requests remaining in the current window.
    pub remaining: u32,
    /// When the window resets (monotonic clock).
    pub reset_at: Instant,
}

impl Bucket {
    /// Returns `true` if the bucket is currently exhausted.
    pub fn is_exhausted(&self) -> bool {
        self.remaining == 0 && Instant::now() < self.reset_at
    }

    /// How long until this bucket resets.  Returns `None` if already reset.
    pub fn time_until_reset(&self) -> Option<Duration> {
        let now = Instant::now();
        if self.reset_at > now {
            Some(self.reset_at - now)
        } else {
            None
        }
    }
}

// ── Rate-limit headers ────────────────────────────────────────────────────────

/// Parsed Discord rate-limit headers from an HTTP response.
#[derive(Debug, Default)]
pub struct RateLimitHeaders {
    /// Opaque bucket identifier.
    pub bucket: Option<String>,
    /// Maximum requests in the window.
    pub limit: Option<u32>,
    /// Remaining requests in the window.
    pub remaining: Option<u32>,
    /// Seconds until the window resets.
    pub reset_after: Option<f64>,
    /// `true` if this is a global rate-limit response.
    pub global: bool,
    /// Scope of the rate limit (`"user"`, `"shared"`, or `"global"`).
    pub scope: Option<String>,
}

impl RateLimitHeaders {
    /// Extract rate-limit headers from a [`reqwest::Response`].
    pub fn from_response(resp: &reqwest::Response) -> Self {
        let headers = resp.headers();
        let get = |key: &str| -> Option<String> {
            headers.get(key)?.to_str().ok().map(str::to_owned)
        };

        Self {
            bucket:      get("x-ratelimit-bucket"),
            limit:       get("x-ratelimit-limit").and_then(|v| v.parse().ok()),
            remaining:   get("x-ratelimit-remaining").and_then(|v| v.parse().ok()),
            reset_after: get("x-ratelimit-reset-after").and_then(|v| v.parse().ok()),
            global:      get("x-ratelimit-global").as_deref() == Some("true"),
            scope:       get("x-ratelimit-scope"),
        }
    }
}

// ── RateLimiter ───────────────────────────────────────────────────────────────

/// Shared, async-safe rate-limit state for all Discord buckets.
///
/// Clone cheaply — the inner state is wrapped in `Arc<Mutex<_>>`.
#[derive(Debug, Clone, Default)]
pub struct RateLimiter {
    /// Map of bucket ID → bucket state.
    buckets: Arc<Mutex<HashMap<String, Bucket>>>,
    /// Global rate-limit reset instant (applies across all requests).
    global_reset: Arc<Mutex<Option<Instant>>>,
}

impl RateLimiter {
    /// Create a new, empty [`RateLimiter`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Wait until the given bucket (and any global limit) allows a request.
    ///
    /// Pass `None` when the bucket is not yet known (first request to a route).
    pub async fn acquire(&self, bucket_id: Option<&str>) {
        // Check global limit first.
        {
            let global = self.global_reset.lock().await;
            if let Some(reset_at) = *global {
                let now = Instant::now();
                if reset_at > now {
                    let wait = reset_at - now;
                    warn!(wait_ms = wait.as_millis(), "global rate limit active — sleeping");
                    drop(global);
                    sleep(wait).await;
                }
            }
        }

        // Check per-bucket limit.
        if let Some(id) = bucket_id {
            let buckets = self.buckets.lock().await;
            if let Some(bucket) = buckets.get(id) {
                if let Some(wait) = bucket.time_until_reset().filter(|_| bucket.is_exhausted()) {
                    warn!(
                        bucket = id,
                        wait_ms = wait.as_millis(),
                        "bucket exhausted — sleeping until reset"
                    );
                    drop(buckets);
                    sleep(wait).await;
                }
            }
        }
    }

    /// Update internal bucket state from the headers of a completed response.
    pub async fn update(&self, headers: &RateLimitHeaders) {
        if headers.global {
            if let Some(after) = headers.reset_after {
                let reset_at = Instant::now() + Duration::from_secs_f64(after);
                *self.global_reset.lock().await = Some(reset_at);
                warn!(reset_after_secs = after, "global rate limit set");
            }
            return;
        }

        let (Some(id), Some(limit), Some(remaining), Some(reset_after)) = (
            headers.bucket.as_deref(),
            headers.limit,
            headers.remaining,
            headers.reset_after,
        ) else {
            return;
        };

        let reset_at = Instant::now() + Duration::from_secs_f64(reset_after);
        debug!(
            bucket = id,
            remaining,
            limit,
            reset_after_secs = reset_after,
            "updated rate-limit bucket"
        );

        self.buckets.lock().await.insert(
            id.to_owned(),
            Bucket { limit, remaining, reset_at },
        );
    }

    /// Register a global rate limit from a `429 Too Many Requests` response.
    ///
    /// `retry_after_secs` comes from the JSON body field `retry_after`.
    pub async fn set_global_retry_after(&self, retry_after_secs: f64) {
        let reset_at = Instant::now() + Duration::from_secs_f64(retry_after_secs);
        *self.global_reset.lock().await = Some(reset_at);
        warn!(
            retry_after_secs,
            "global 429 received — all requests paused"
        );
    }
}
