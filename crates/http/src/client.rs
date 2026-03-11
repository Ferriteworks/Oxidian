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

//! The Oxidian HTTP client for the Discord REST API.
//!
//! [`HttpClient`] wraps [`reqwest::Client`] and adds:
//! - Automatic `Authorization` and `User-Agent` headers on every request.
//! - Per-bucket and global rate-limit tracking via [`RateLimiter`].
//! - Structured error conversion for 4xx / 5xx responses.
//! - A single generic [`HttpClient::request`] method that all route helpers
//!   delegate to, making it trivial to add new endpoints.

use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE, USER_AGENT},
    StatusCode,
};
use serde::de::DeserializeOwned;
use serde_json::Value;
use tracing::{debug, warn};

use oxidian_core::error::{Error as OxidianError, HttpError};

use crate::{
    ratelimit::{RateLimitHeaders, RateLimiter},
    routes::{Method, Route},
};

/// Discord REST API base URL (v10).
pub const BASE_URL: &str = "https://discord.com/api/v10";

/// User-Agent sent on every request.  Discord requires a meaningful UA.
const USER_AGENT_VALUE: &str = concat!(
    "DiscordBot (https://github.com/Ferriteworks/Oxidian, ",
    env!("CARGO_PKG_VERSION"),
    ")"
);

/// JSON structure returned by Discord on 4xx errors.
#[derive(Debug, serde::Deserialize)]
struct DiscordApiError {
    code: u32,
    message: String,
}

/// Async HTTP client for the Discord REST API.
///
/// Cheap to clone — the inner `reqwest::Client` and `RateLimiter` both use
/// `Arc` internally.
#[derive(Debug, Clone)]
pub struct HttpClient {
    inner: reqwest::Client,
    rate_limiter: RateLimiter,
}

impl HttpClient {
    /// Create a new [`HttpClient`] authenticated with the given bot token.
    ///
    /// The token must be the raw value **without** the `"Bot "` prefix — this
    /// method prepends it.
    pub fn new(token: &str) -> Result<Self, OxidianError> {
        let auth_value = format!("Bot {token}");
        let mut default_headers = HeaderMap::new();

        default_headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&auth_value).map_err(|e| {
                HttpError::Request(format!("invalid token characters: {e}"))
            })?,
        );
        default_headers.insert(USER_AGENT, HeaderValue::from_static(USER_AGENT_VALUE));

        let inner = reqwest::Client::builder()
            .default_headers(default_headers)
            .build()
            .map_err(|e| HttpError::Request(e.to_string()))?;

        Ok(Self {
            inner,
            rate_limiter: RateLimiter::new(),
        })
    }

    /// Send a request for the given [`Route`], optionally with a JSON body,
    /// and deserialize the response into `T`.
    ///
    /// This method:
    /// 1. Waits for the relevant rate-limit bucket to clear.
    /// 2. Dispatches the request.
    /// 3. Updates bucket state from response headers.
    /// 4. Handles `429 Too Many Requests` by sleeping and retrying once.
    /// 5. Returns a structured error for any other non-2xx status.
    pub async fn request<T: DeserializeOwned>(
        &self,
        route: Route,
        body: Option<Value>,
    ) -> Result<T, OxidianError> {
        let bucket_hint = route.bucket();
        // Retry once on 429 before giving up.
        for attempt in 0u8..2 {
            self.rate_limiter.acquire(Some(&bucket_hint)).await;

            let req = self.build_request(&route, body.clone())?;

            debug!(method = ?route.method(), url = %route.url(), attempt, "sending HTTP request");

            let resp = self
                .inner
                .execute(req)
                .await
                .map_err(|e| HttpError::Request(e.to_string()))?;

            let rl_headers = RateLimitHeaders::from_response(&resp);
            self.rate_limiter.update(&rl_headers).await;

            // If Discord gave us a real bucket ID, track it against this route.
            let status = resp.status();

            if status == StatusCode::TOO_MANY_REQUESTS {
                // Parse the retry_after from the JSON body.
                let body_text = resp.text().await.unwrap_or_default();
                let retry_after: f64 = serde_json::from_str::<Value>(&body_text)
                    .ok()
                    .and_then(|v| v["retry_after"].as_f64())
                    .unwrap_or(1.0);

                if rl_headers.global {
                    self.rate_limiter.set_global_retry_after(retry_after).await;
                } else {
                    warn!(
                        bucket = %bucket_hint,
                        retry_after_secs = retry_after,
                        "429 on bucket — sleeping before retry"
                    );
                    tokio::time::sleep(std::time::Duration::from_secs_f64(retry_after))
                        .await;
                }

                if attempt == 0 {
                    continue; // retry
                }

                return Err(OxidianError::RateLimited {
                    retry_after_ms: (retry_after * 1000.0) as u64,
                    bucket: Some(bucket_hint),
                });
            }

            if !status.is_success() {
                let body_text = resp.text().await.unwrap_or_default();
                // Try to parse Discord's JSON error body.
                if let Ok(api_err) = serde_json::from_str::<DiscordApiError>(&body_text)
                {
                    return Err(OxidianError::Api {
                        code: api_err.code,
                        message: api_err.message,
                    });
                }
                return Err(HttpError::UnexpectedStatus {
                    status: status.as_u16(),
                    body: body_text,
                }
                .into());
            }

            let response_text = resp
                .text()
                .await
                .map_err(|e| HttpError::Decode(e.to_string()))?;

            // 204 No Content and similar empty-body successes — try deserialising
            // from JSON `null` so that `Result<()>` callers succeed.
            let effective = if response_text.is_empty() {
                "null"
            } else {
                &response_text
            };

            return serde_json::from_str::<T>(effective).map_err(|e| {
                HttpError::Decode(format!("{e} — body: {response_text}")).into()
            });
        }

        unreachable!()
    }

    /// Fetch the current bot user (`GET /users/@me`).
    pub async fn get_current_user(&self) -> Result<Value, OxidianError> {
        self.request(Route::GetCurrentUser, None).await
    }

    /// Fetch a user by ID (`GET /users/{user.id}`).
    pub async fn get_user(
        &self,
        user_id: oxidian_core::snowflake::Snowflake,
    ) -> Result<Value, OxidianError> {
        self.request(Route::GetUser { user_id }, None).await
    }

    /// Fetch a channel by ID (`GET /channels/{channel.id}`).
    pub async fn get_channel(
        &self,
        channel_id: oxidian_core::snowflake::Snowflake,
    ) -> Result<Value, OxidianError> {
        self.request(Route::GetChannel { channel_id }, None).await
    }

    /// Send a message to a channel (`POST /channels/{channel.id}/messages`).
    ///
    /// `body` is the raw JSON payload.  Use [`serde_json::json!`] to construct
    /// it: `json!({ "content": "hello" })`.
    pub async fn create_message(
        &self,
        channel_id: oxidian_core::snowflake::Snowflake,
        body: Value,
    ) -> Result<Value, OxidianError> {
        self.request(Route::CreateMessage { channel_id }, Some(body))
            .await
    }

    /// Delete a message (`DELETE /channels/{channel.id}/messages/{message.id}`).
    pub async fn delete_message(
        &self,
        channel_id: oxidian_core::snowflake::Snowflake,
        message_id: oxidian_core::snowflake::Snowflake,
    ) -> Result<Value, OxidianError> {
        self.request(
            Route::DeleteMessage {
                channel_id,
                message_id,
            },
            None,
        )
        .await
    }

    /// Fetch a guild by ID (`GET /guilds/{guild.id}`).
    pub async fn get_guild(
        &self,
        guild_id: oxidian_core::snowflake::Snowflake,
    ) -> Result<Value, OxidianError> {
        self.request(Route::GetGuild { guild_id }, None).await
    }

    /// Fetch the recommended gateway URL and shard count (`GET /gateway/bot`).
    pub async fn get_gateway_bot(&self) -> Result<Value, OxidianError> {
        self.request(Route::GetGatewayBot, None).await
    }

    /// Fetch all global commands for the application.
    pub async fn get_global_commands(
        &self,
        application_id: oxidian_core::snowflake::Snowflake,
    ) -> Result<Value, OxidianError> {
        self.request(Route::GetGlobalCommands { application_id }, None)
            .await
    }

    /// Register (or overwrite) a global command.
    ///
    /// `body` is a serialised [`ApplicationCommand`](oxidian_interactions::command::ApplicationCommand).
    pub async fn create_global_command(
        &self,
        application_id: oxidian_core::snowflake::Snowflake,
        body: Value,
    ) -> Result<Value, OxidianError> {
        self.request(Route::CreateGlobalCommand { application_id }, Some(body))
            .await
    }

    /// Delete a global command by ID.
    pub async fn delete_global_command(
        &self,
        application_id: oxidian_core::snowflake::Snowflake,
        command_id: oxidian_core::snowflake::Snowflake,
    ) -> Result<(), OxidianError> {
        self.request(
            Route::DeleteGlobalCommand {
                application_id,
                command_id,
            },
            None,
        )
        .await
    }

    /// Fetch all guild-scoped commands for the application.
    pub async fn get_guild_commands(
        &self,
        application_id: oxidian_core::snowflake::Snowflake,
        guild_id: oxidian_core::snowflake::Snowflake,
    ) -> Result<Value, OxidianError> {
        self.request(
            Route::GetGuildCommands {
                application_id,
                guild_id,
            },
            None,
        )
        .await
    }

    /// Register (or overwrite) a guild-scoped command.
    pub async fn create_guild_command(
        &self,
        application_id: oxidian_core::snowflake::Snowflake,
        guild_id: oxidian_core::snowflake::Snowflake,
        body: Value,
    ) -> Result<Value, OxidianError> {
        self.request(
            Route::CreateGuildCommand {
                application_id,
                guild_id,
            },
            Some(body),
        )
        .await
    }

    /// Bulk overwrite **all** global commands.
    ///
    /// Replaces the full global command list atomically. Commands not in
    /// `commands` are deleted; commands in `commands` are created or updated.
    ///
    /// This is the recommended way to sync your command definitions with
    /// Discord. Equivalent to `PUT /applications/{app}/commands`.
    pub async fn bulk_overwrite_global_commands(
        &self,
        application_id: oxidian_core::snowflake::Snowflake,
        commands: &[oxidian_interactions::command::ApplicationCommand],
    ) -> Result<Value, OxidianError> {
        let body =
            serde_json::to_value(commands).map_err(OxidianError::Serialization)?;
        self.request(
            Route::BulkOverwriteGlobalCommands { application_id },
            Some(body),
        )
        .await
    }

    /// Bulk overwrite **all** guild-scoped commands.
    ///
    /// Replaces the full guild command list atomically. Commands not in
    /// `commands` are deleted; commands in `commands` are created or updated.
    ///
    /// Equivalent to `PUT /applications/{app}/guilds/{guild}/commands`.
    pub async fn bulk_overwrite_guild_commands(
        &self,
        application_id: oxidian_core::snowflake::Snowflake,
        guild_id: oxidian_core::snowflake::Snowflake,
        commands: &[oxidian_interactions::command::ApplicationCommand],
    ) -> Result<Value, OxidianError> {
        let body =
            serde_json::to_value(commands).map_err(OxidianError::Serialization)?;
        self.request(
            Route::BulkOverwriteGuildCommands {
                application_id,
                guild_id,
            },
            Some(body),
        )
        .await
    }

    /// Delete a guild-scoped command by ID.
    pub async fn delete_guild_command(
        &self,
        application_id: oxidian_core::snowflake::Snowflake,
        guild_id: oxidian_core::snowflake::Snowflake,
        command_id: oxidian_core::snowflake::Snowflake,
    ) -> Result<(), OxidianError> {
        self.request(
            Route::DeleteGuildCommand {
                application_id,
                guild_id,
                command_id,
            },
            None,
        )
        .await
    }

    /// Respond to a Discord interaction.
    ///
    /// `body` must be a serialised
    /// [`InteractionResponse`](oxidian_core::models::interaction::InteractionResponse).
    /// Returns `()` because Discord replies with `204 No Content`.
    pub async fn create_interaction_response(
        &self,
        interaction_id: oxidian_core::snowflake::Snowflake,
        interaction_token: &str,
        body: Value,
    ) -> Result<(), OxidianError> {
        self.request(
            Route::CreateInteractionResponse {
                interaction_id,
                interaction_token: interaction_token.to_owned(),
            },
            Some(body),
        )
        .await
    }

    /// Edit a message (`PATCH /channels/{channel.id}/messages/{message.id}`).
    pub async fn edit_message(
        &self,
        channel_id: oxidian_core::snowflake::Snowflake,
        message_id: oxidian_core::snowflake::Snowflake,
        body: Value,
    ) -> Result<Value, OxidianError> {
        self.request(
            Route::EditMessage {
                channel_id,
                message_id,
            },
            Some(body),
        )
        .await
    }

    /// Edit the original response to a deferred interaction.
    ///
    /// `PATCH /webhooks/{application_id}/{interaction_token}/messages/@original`
    pub async fn edit_original_interaction_response(
        &self,
        application_id: oxidian_core::snowflake::Snowflake,
        interaction_token: &str,
        body: Value,
    ) -> Result<Value, OxidianError> {
        self.request(
            Route::EditOriginalInteractionResponse {
                application_id,
                interaction_token: interaction_token.to_owned(),
            },
            Some(body),
        )
        .await
    }

    fn build_request(
        &self,
        route: &Route,
        body: Option<Value>,
    ) -> Result<reqwest::Request, OxidianError> {
        let url = route.url();
        let method = match route.method() {
            Method::Get => reqwest::Method::GET,
            Method::Post => reqwest::Method::POST,
            Method::Patch => reqwest::Method::PATCH,
            Method::Put => reqwest::Method::PUT,
            Method::Delete => reqwest::Method::DELETE,
        };

        let mut builder = self.inner.request(method, &url);

        if let Some(json) = body {
            let text =
                serde_json::to_string(&json).map_err(OxidianError::Serialization)?;
            builder = builder.header(CONTENT_TYPE, "application/json").body(text);
        }

        builder
            .build()
            .map_err(|e| HttpError::Request(e.to_string()).into())
    }
}
