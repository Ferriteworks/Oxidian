// MIT License
//
// Copyright (c) 2026 Ferriteworks organization and its rightful owners.
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use std::sync::Arc;

use tokio::sync::{broadcast, mpsc};
use tracing::{error, info, warn};

use oxidian_core::{
    error::{Error as OxidianError, GatewayError},
    intents::Intents,
};

use crate::{
    connection::{self, SessionState},
    events::DispatchEvent,
};

/// Identifies this shard within a multi-shard deployment.
///
/// Discord uses the formula `(guild_id >> 22) % num_shards` to determine
/// which shard receives events for a given guild.
///
/// For single-shard bots, use `ShardInfo::single()` (the default).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShardInfo {
    /// Zero-based shard ID.
    pub shard_id: u32,
    /// Total number of shards in the deployment.
    pub num_shards: u32,
}

impl ShardInfo {
    /// Create a `ShardInfo` for a single-shard setup (`[0, 1]`).
    pub fn single() -> Self {
        Self {
            shard_id: 0,
            num_shards: 1,
        }
    }

    /// Create a `ShardInfo` for shard `id` out of `total` shards.
    pub fn new(shard_id: u32, num_shards: u32) -> Self {
        assert!(shard_id < num_shards, "shard_id must be < num_shards");
        Self {
            shard_id,
            num_shards,
        }
    }
}

impl Default for ShardInfo {
    fn default() -> Self {
        Self::single()
    }
}

/// A single Discord gateway shard.
pub struct Shard {
    token: String,
    intents: Intents,
    shard_info: ShardInfo,
    event_tx: mpsc::Sender<DispatchEvent>,
    outbound_tx: Arc<broadcast::Sender<serde_json::Value>>,
}

impl Shard {
    /// Create a new `Shard` with default shard info (`[0, 1]`).
    pub fn new(
        token: impl Into<String>,
        intents: Intents,
        event_tx: mpsc::Sender<DispatchEvent>,
    ) -> Self {
        Self::with_shard_info(token, intents, ShardInfo::single(), event_tx)
    }

    /// Create a new `Shard` with explicit shard info for multi-sharding.
    ///
    /// ```rust,ignore
    /// use oxidian_gateway::shard::{Shard, ShardInfo};
    ///
    /// let shard_0 = Shard::with_shard_info(token, intents, ShardInfo::new(0, 2), tx_0);
    /// let shard_1 = Shard::with_shard_info(token, intents, ShardInfo::new(1, 2), tx_1);
    /// ```
    pub fn with_shard_info(
        token: impl Into<String>,
        intents: Intents,
        shard_info: ShardInfo,
        event_tx: mpsc::Sender<DispatchEvent>,
    ) -> Self {
        let (outbound_tx, _) = broadcast::channel(64);
        Self {
            token: token.into(),
            intents,
            shard_info,
            event_tx,
            outbound_tx: Arc::new(outbound_tx),
        }
    }

    /// Return the shard info for this shard.
    pub fn shard_info(&self) -> ShardInfo {
        self.shard_info
    }

    /// Return a clone of the broadcast sender used for outbound gateway
    /// payloads.  Messages sent when no connection is active are discarded.
    pub fn gateway_sender(&self) -> Arc<broadcast::Sender<serde_json::Value>> {
        Arc::clone(&self.outbound_tx)
    }

    /// Connect to the Discord gateway and drive the event loop.
    ///
    /// Retries on transient errors (connection drops, op 7 reconnect, op 9
    /// invalid session) up to a fixed limit before giving up.
    pub async fn start(&self) -> Result<(), OxidianError> {
        const MAX_RETRIES: u32 = 5;
        let mut attempts = 0u32;
        let mut session: Option<SessionState> = None;

        loop {
            info!(
                attempt = attempts + 1,
                resuming = session.is_some(),
                "starting gateway session"
            );

            // Subscribe before each connection so the new socket drains
            // outbound messages sent during the lifetime of that session.
            let outbound_rx = self.outbound_tx.subscribe();

            match connection::connect(
                &self.token,
                self.intents,
                Some(self.shard_info),
                self.event_tx.clone(),
                outbound_rx,
                session.take(),
            )
            .await
            {
                Ok(None) => {
                    info!("gateway connection closed cleanly");
                    return Ok(());
                }
                Ok(Some(state)) => {
                    // Gateway sent op 7 or op 9 (resumable) — reconnect and resume.
                    attempts += 1;
                    if attempts >= MAX_RETRIES {
                        error!(attempts, "exceeded maximum gateway reconnect attempts");
                        return Err(GatewayError::Connection(format!(
                            "gave up after {MAX_RETRIES} reconnect attempts"
                        ))
                        .into());
                    }
                    let delay =
                        std::time::Duration::from_millis(500 * u64::from(attempts));
                    warn!(delay_ms = delay.as_millis(), session_id = %state.session_id, "reconnecting to resume session");
                    tokio::time::sleep(delay).await;
                    session = Some(state);
                }
                Err(OxidianError::Gateway(GatewayError::SessionInvalidated {
                    resumable: false,
                }))
                | Err(OxidianError::Gateway(GatewayError::Connection(_))) => {
                    // Non-resumable — start a fresh session after backoff.
                    session = None;
                    attempts += 1;
                    if attempts >= MAX_RETRIES {
                        error!(attempts, "exceeded maximum gateway reconnect attempts");
                        return Err(GatewayError::Connection(format!(
                            "gave up after {MAX_RETRIES} reconnect attempts"
                        ))
                        .into());
                    }
                    let backoff =
                        std::time::Duration::from_secs(2u64.pow(attempts.min(6)));
                    warn!(
                        backoff_secs = backoff.as_secs(),
                        "transient error — reconnecting with fresh Identify after backoff"
                    );
                    tokio::time::sleep(backoff).await;
                }
                Err(e) => return Err(e),
            }
        }
    }
}
