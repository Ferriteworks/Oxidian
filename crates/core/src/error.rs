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

use std::fmt;

/// The top-level error type for the entire Oxidian library.
///
/// Every sub-crate converts its own domain error into a variant here so that
/// callers only ever need to handle one error type at the boundary.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    /// An error from the HTTP client or Discord REST API layer.
    #[error("HTTP layer error: {0}")]
    Http(#[from] HttpError),

    /// An error from the WebSocket gateway layer (connection, dispatch, heartbeat).
    #[error("gateway layer error: {0}")]
    Gateway(#[from] GatewayError),

    /// An error from the voice layer (WebSocket signalling, UDP media, DAVE E2EE).
    #[error("voice layer error - {0}")]
    Voice(#[from] VoiceError),

    /// A JSON (de)serialisation failure while processing a Discord payload.
    #[error("failed to (de)serialize Discord payload: {0}")]
    Serialization(#[from] serde_json::Error),

    /// Discord returned a non-2xx response with a structured JSON error body.
    #[error("Discord API returned error {code}: {message}")]
    Api {
        /// Discord JSON error code (<https://discord.com/developers/docs/topics/opcodes-and-status-codes#json>).
        code: u32,
        /// Human-readable message returned by Discord.
        message: String,
    },

    /// The requested resource does not exist on Discord or could not be found locally.
    #[error("resource not found: {0}")]
    NotFound(String),

    /// The supplied snowflake or identifier was missing, malformed, or out of range.
    #[error("invalid or malformed ID: {0}")]
    InvalidId(String),

    /// The bot or user lacks the Discord permissions required to perform this action.
    #[error("missing required permissions to perform this action: {0}")]
    MissingPermissions(String),

    /// The bot token is absent, has been revoked, or failed Discord's verification.
    #[error(
        "authentication failed; check that the bot token is correct and has not been revoked: {0}"
    )]
    Auth(String),

    /// A Discord rate-limit was hit.  The request was not retried.
    #[error(
        "rate limited by Discord on bucket '{bucket}'; retry after {retry_after_ms}ms",
        bucket = bucket.as_deref().unwrap_or("<unknown>")
    )]
    RateLimited {
        /// How long the caller must wait before retrying, in milliseconds.
        retry_after_ms: u64,
        /// The rate-limit bucket that was exhausted, if reported by Discord.
        bucket: Option<String>,
    },

    /// An unexpected internal error that does not fit any other category.
    #[error("unexpected internal error: {0}")]
    Internal(String),

    /// An unexpected error that does not fit any other category (e.g. a dependency failure).
    #[error("unexpected error: {0}")]
    Unexpected(String),
}

impl Error {
    /// Convenience constructor for [`Error::Internal`].
    pub fn internal(msg: impl fmt::Display) -> Self {
        Self::Internal(msg.to_string())
    }

    /// Returns `true` if the error is a rate-limit response.
    pub fn is_rate_limited(&self) -> bool {
        matches!(self, Self::RateLimited { .. })
    }

    /// Returns `true` if the error is an authentication failure.
    pub fn is_auth(&self) -> bool {
        matches!(self, Self::Auth(_))
    }
}

/// Errors specific to the HTTP / REST layer.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum HttpError {
    /// The underlying HTTP transport layer raised an error (e.g. DNS failure, TLS error, connection refused).
    #[error("HTTP transport error: {0}")]
    Request(String),

    /// Discord returned a status code that was not expected for this endpoint.
    #[error("Discord responded with unexpected HTTP {status}: body: {body}")]
    UnexpectedStatus { status: u16, body: String },

    /// The response body could not be decoded into the expected type.
    #[error("failed to decode response body: {0}")]
    Decode(String),

    /// The HTTP request did not receive a response within the allowed time.
    #[error("HTTP request timed out waiting for a response from Discord")]
    Timeout,
}

/// Errors specific to the WebSocket gateway layer.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum GatewayError {
    /// The WebSocket connection to the Discord gateway could not be established or was unexpectedly dropped.
    #[error("gateway WebSocket connection error: {0}")]
    Connection(String),

    /// The gateway sent an opcode that is unknown or not valid in the current state.
    #[error("received unrecognised or invalid gateway opcode {0}")]
    InvalidOpcode(u8),

    /// The gateway did not acknowledge a heartbeat within the expected interval; the connection is considered lost.
    #[error("gateway heartbeat was not acknowledged within the expected interval: connection presumed lost")]
    HeartbeatTimeout,

    /// Discord invalidated the current session.  If `resumable` is `true` a RESUME is possible; otherwise a fresh IDENTIFY is required.
    #[error("gateway session was invalidated by Discord (resumable: {resumable})")]
    SessionInvalidated { resumable: bool },

    /// The gateway is rate-limiting identify / send operations for this shard.
    #[error("gateway rate-limited this shard's identify or send operation")]
    RateLimited,

    /// A DISPATCH event arrived with a `t` field that this library does not recognise.
    #[error("received unrecognised gateway dispatch event type '{0}'")]
    UnknownEvent(String),
}

/// Errors specific to the voice layer (including DAVE E2EE).
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum VoiceError {
    /// The WebSocket signalling connection to Discord's voice server failed.
    #[error("voice WebSocket signalling connection error: {0}")]
    Connection(String),

    /// The UDP media transport encountered an error (e.g. socket bind failure, send/recv error).
    #[error("voice UDP media transport error: {0}")]
    Udp(String),

    /// An error in the DAVE end-to-end encryption protocol (key exchange, ratchet, or decryption failure).
    #[error("DAVE E2EE protocol error: {0}")]
    Dave(String),

    /// The voice session was terminated or expired; a new session must be negotiated.
    #[error("voice session was terminated by Discord or expired; re-connect required")]
    SessionEnded,
}

/// A [`Result`] type that defaults to [`Error`] as the error variant.
pub type Result<T, E = Error> = std::result::Result<T, E>;
