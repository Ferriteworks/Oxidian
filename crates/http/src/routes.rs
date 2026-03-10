//! Typed Discord REST API route definitions.
//!
//! Each variant of [`Route`] maps to a specific Discord endpoint.  The route
//! knows its HTTP method, its fully-formed URL path, and the bucket identifier
//! used for rate-limit tracking.
//!
//! Bucket IDs follow Discord's rule: routes that share major parameters (guild
//! ID, channel ID, webhook ID) share a rate-limit bucket for those resources.

use crate::client::BASE_URL;
use oxidian_core::snowflake::Snowflake;

/// HTTP method for a [`Route`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Method {
    /// HTTP GET.
    Get,
    /// HTTP POST.
    Post,
    /// HTTP PATCH.
    Patch,
    /// HTTP PUT.
    Put,
    /// HTTP DELETE.
    Delete,
}

/// A typed Discord REST API endpoint.
///
/// Call [`Route::method`], [`Route::url`], and [`Route::bucket`] to get the
/// info needed to dispatch a request.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum Route {
    // ── Channels ────────────────────────────────────────────────────────────
    /// `GET /channels/{channel.id}`
    GetChannel {
        /// The ID of the channel to fetch.
        channel_id: Snowflake,
    },
    /// `POST /channels/{channel.id}/messages`
    CreateMessage {
        /// The ID of the channel to send the message in.
        channel_id: Snowflake,
    },
    /// `GET /channels/{channel.id}/messages/{message.id}`
    GetMessage {
        /// The channel that contains the message.
        channel_id: Snowflake,
        /// The ID of the message to fetch.
        message_id: Snowflake,
    },
    /// `DELETE /channels/{channel.id}/messages/{message.id}`
    DeleteMessage {
        /// The channel that contains the message.
        channel_id: Snowflake,
        /// The ID of the message to delete.
        message_id: Snowflake,
    },

    // ── Guilds ──────────────────────────────────────────────────────────────
    /// `GET /guilds/{guild.id}`
    GetGuild {
        /// The ID of the guild to fetch.
        guild_id: Snowflake,
    },
    /// `GET /guilds/{guild.id}/members/{user.id}`
    GetGuildMember {
        /// The guild that contains the member.
        guild_id: Snowflake,
        /// The ID of the user (member) to fetch.
        user_id: Snowflake,
    },

    // ── Users ───────────────────────────────────────────────────────────────
    /// `GET /users/@me`
    GetCurrentUser,
    /// `GET /users/{user.id}`
    GetUser {
        /// The ID of the user to fetch.
        user_id: Snowflake,
    },

    // ── Gateway ─────────────────────────────────────────────────────────────
    /// `GET /gateway/bot` — returns the recommended shard count and WSS URL.
    GetGatewayBot,
}

impl Route {
    /// The HTTP method for this route.
    pub fn method(&self) -> Method {
        match self {
            Self::GetChannel { .. }
            | Self::GetMessage { .. }
            | Self::GetGuild { .. }
            | Self::GetGuildMember { .. }
            | Self::GetCurrentUser
            | Self::GetUser { .. }
            | Self::GetGatewayBot => Method::Get,

            Self::CreateMessage { .. } => Method::Post,
            Self::DeleteMessage { .. } => Method::Delete,
        }
    }

    /// The fully-formed URL for this route.
    pub fn url(&self) -> String {
        let base = BASE_URL;
        match self {
            Self::GetChannel { channel_id }      => format!("{base}/channels/{channel_id}"),
            Self::CreateMessage { channel_id }   => format!("{base}/channels/{channel_id}/messages"),
            Self::GetMessage { channel_id, message_id } =>
                format!("{base}/channels/{channel_id}/messages/{message_id}"),
            Self::DeleteMessage { channel_id, message_id } =>
                format!("{base}/channels/{channel_id}/messages/{message_id}"),
            Self::GetGuild { guild_id }          => format!("{base}/guilds/{guild_id}"),
            Self::GetGuildMember { guild_id, user_id } =>
                format!("{base}/guilds/{guild_id}/members/{user_id}"),
            Self::GetCurrentUser                 => format!("{base}/users/@me"),
            Self::GetUser { user_id }            => format!("{base}/users/{user_id}"),
            Self::GetGatewayBot                  => format!("{base}/gateway/bot"),
        }
    }

    /// The rate-limit bucket identifier for this route.
    ///
    /// Discord buckets routes by their major path parameter (channel, guild,
    /// or webhook ID).  This is used as the lookup key in [`RateLimiter`].
    ///
    /// [`RateLimiter`]: crate::ratelimit::RateLimiter
    pub fn bucket(&self) -> String {
        match self {
            Self::GetChannel { channel_id }
            | Self::CreateMessage { channel_id }
            | Self::GetMessage { channel_id, .. }
            | Self::DeleteMessage { channel_id, .. } =>
                format!("channel:{channel_id}"),

            Self::GetGuild { guild_id }
            | Self::GetGuildMember { guild_id, .. } =>
                format!("guild:{guild_id}"),

            Self::GetCurrentUser
            | Self::GetUser { .. }
            | Self::GetGatewayBot => "global".to_owned(),
        }
    }
}
