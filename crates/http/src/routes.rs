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

    /// `GET /users/@me`
    GetCurrentUser,
    /// `GET /users/{user.id}`
    GetUser {
        /// The ID of the user to fetch.
        user_id: Snowflake,
    },

    /// `GET /gateway/bot` — returns the recommended shard count and WSS URL.
    GetGatewayBot,

    /// `GET /applications/{application_id}/commands`
    GetGlobalCommands { application_id: Snowflake },
    /// `POST /applications/{application_id}/commands`
    CreateGlobalCommand { application_id: Snowflake },
    /// `PUT /applications/{application_id}/commands` — bulk overwrite all global commands.
    BulkOverwriteGlobalCommands { application_id: Snowflake },
    /// `DELETE /applications/{application_id}/commands/{command_id}`
    DeleteGlobalCommand {
        application_id: Snowflake,
        command_id: Snowflake,
    },
    /// `GET /applications/{application_id}/guilds/{guild_id}/commands`
    GetGuildCommands {
        application_id: Snowflake,
        guild_id: Snowflake,
    },
    /// `POST /applications/{application_id}/guilds/{guild_id}/commands`
    CreateGuildCommand {
        application_id: Snowflake,
        guild_id: Snowflake,
    },
    /// `PUT /applications/{application_id}/guilds/{guild_id}/commands` — bulk overwrite all guild commands.
    BulkOverwriteGuildCommands {
        application_id: Snowflake,
        guild_id: Snowflake,
    },
    /// `DELETE /applications/{application_id}/guilds/{guild_id}/commands/{command_id}`
    DeleteGuildCommand {
        application_id: Snowflake,
        guild_id: Snowflake,
        command_id: Snowflake,
    },
    /// `POST /interactions/{interaction_id}/{interaction_token}/callback`
    CreateInteractionResponse {
        interaction_id: Snowflake,
        interaction_token: String,
    },
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
            | Self::GetGatewayBot
            | Self::GetGlobalCommands { .. }
            | Self::GetGuildCommands { .. } => Method::Get,

            Self::CreateMessage { .. }
            | Self::CreateGlobalCommand { .. }
            | Self::CreateGuildCommand { .. }
            | Self::CreateInteractionResponse { .. } => Method::Post,

            Self::BulkOverwriteGlobalCommands { .. }
            | Self::BulkOverwriteGuildCommands { .. } => Method::Put,

            Self::DeleteMessage { .. }
            | Self::DeleteGlobalCommand { .. }
            | Self::DeleteGuildCommand { .. } => Method::Delete,
        }
    }

    /// The fully-formed URL for this route.
    pub fn url(&self) -> String {
        let base = BASE_URL;
        match self {
            Self::GetChannel { channel_id } => format!("{base}/channels/{channel_id}"),
            Self::CreateMessage { channel_id } => format!("{base}/channels/{channel_id}/messages"),
            Self::GetMessage {
                channel_id,
                message_id,
            } => format!("{base}/channels/{channel_id}/messages/{message_id}"),
            Self::DeleteMessage {
                channel_id,
                message_id,
            } => format!("{base}/channels/{channel_id}/messages/{message_id}"),
            Self::GetGuild { guild_id } => format!("{base}/guilds/{guild_id}"),
            Self::GetGuildMember { guild_id, user_id } => {
                format!("{base}/guilds/{guild_id}/members/{user_id}")
            }
            Self::GetCurrentUser => format!("{base}/users/@me"),
            Self::GetUser { user_id } => format!("{base}/users/{user_id}"),
            Self::GetGatewayBot => format!("{base}/gateway/bot"),
            Self::GetGlobalCommands { application_id } => {
                format!("{base}/applications/{application_id}/commands")
            }
            Self::CreateGlobalCommand { application_id } => {
                format!("{base}/applications/{application_id}/commands")
            }
            Self::BulkOverwriteGlobalCommands { application_id } => {
                format!("{base}/applications/{application_id}/commands")
            }
            Self::DeleteGlobalCommand {
                application_id,
                command_id,
            } => format!("{base}/applications/{application_id}/commands/{command_id}"),
            Self::GetGuildCommands {
                application_id,
                guild_id,
            } => format!("{base}/applications/{application_id}/guilds/{guild_id}/commands"),
            Self::CreateGuildCommand {
                application_id,
                guild_id,
            } => format!("{base}/applications/{application_id}/guilds/{guild_id}/commands"),
            Self::BulkOverwriteGuildCommands {
                application_id,
                guild_id,
            } => format!("{base}/applications/{application_id}/guilds/{guild_id}/commands"),
            Self::DeleteGuildCommand {
                application_id,
                guild_id,
                command_id,
            } => format!(
                "{base}/applications/{application_id}/guilds/{guild_id}/commands/{command_id}"
            ),
            Self::CreateInteractionResponse {
                interaction_id,
                interaction_token,
            } => format!("{base}/interactions/{interaction_id}/{interaction_token}/callback"),
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
            | Self::DeleteMessage { channel_id, .. } => format!("channel:{channel_id}"),

            Self::GetGuild { guild_id } | Self::GetGuildMember { guild_id, .. } => {
                format!("guild:{guild_id}")
            }

            Self::GetCurrentUser | Self::GetUser { .. } | Self::GetGatewayBot => {
                "global".to_owned()
            }

            Self::GetGlobalCommands { application_id }
            | Self::CreateGlobalCommand { application_id }
            | Self::BulkOverwriteGlobalCommands { application_id }
            | Self::DeleteGlobalCommand { application_id, .. } => {
                format!("application:{application_id}:commands")
            }

            Self::GetGuildCommands {
                application_id,
                guild_id,
            }
            | Self::CreateGuildCommand {
                application_id,
                guild_id,
            }
            | Self::BulkOverwriteGuildCommands {
                application_id,
                guild_id,
            }
            | Self::DeleteGuildCommand {
                application_id,
                guild_id,
                ..
            } => format!("application:{application_id}:guild:{guild_id}:commands"),

            Self::CreateInteractionResponse { .. } => "interaction".to_owned(),
        }
    }
}
