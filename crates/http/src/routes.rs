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
    /// `PATCH /channels/{channel.id}/messages/{message.id}`
    EditMessage {
        channel_id: Snowflake,
        message_id: Snowflake,
    },
    /// `PATCH /webhooks/{application_id}/{interaction_token}/messages/@original`
    EditOriginalInteractionResponse {
        application_id: Snowflake,
        interaction_token: String,
    },

    // ── Threads ───────────────────────────────────────────────────────────
    /// `POST /channels/{channel.id}/threads` — start a thread from a message or without one.
    CreateThread { channel_id: Snowflake },
    /// `POST /channels/{channel.id}/messages/{message.id}/threads` — start a thread from a message.
    CreateThreadFromMessage {
        channel_id: Snowflake,
        message_id: Snowflake,
    },
    /// `PUT /channels/{thread.id}/thread-members/@me` — join a thread.
    JoinThread { thread_id: Snowflake },
    /// `DELETE /channels/{thread.id}/thread-members/@me` — leave a thread.
    LeaveThread { thread_id: Snowflake },
    /// `PUT /channels/{thread.id}/thread-members/{user.id}` — add a member to a thread.
    AddThreadMember {
        thread_id: Snowflake,
        user_id: Snowflake,
    },
    /// `DELETE /channels/{thread.id}/thread-members/{user.id}` — remove a member from a thread.
    RemoveThreadMember {
        thread_id: Snowflake,
        user_id: Snowflake,
    },
    /// `GET /channels/{channel.id}/threads/archived/public`
    ListPublicArchivedThreads { channel_id: Snowflake },
    /// `GET /channels/{channel.id}/threads/archived/private`
    ListPrivateArchivedThreads { channel_id: Snowflake },
    /// `PATCH /channels/{channel.id}` — modify a thread (archive, lock, rename, etc.).
    ModifyThread { channel_id: Snowflake },

    // ── Guild Members (modify / timeout) ──────────────────────────────────
    /// `PATCH /guilds/{guild.id}/members/{user.id}`
    ModifyGuildMember {
        guild_id: Snowflake,
        user_id: Snowflake,
    },

    // ── Scheduled Events ──────────────────────────────────────────────────
    /// `GET /guilds/{guild.id}/scheduled-events`
    ListScheduledEvents { guild_id: Snowflake },
    /// `POST /guilds/{guild.id}/scheduled-events`
    CreateScheduledEvent { guild_id: Snowflake },
    /// `GET /guilds/{guild.id}/scheduled-events/{event.id}`
    GetScheduledEvent {
        guild_id: Snowflake,
        event_id: Snowflake,
    },
    /// `PATCH /guilds/{guild.id}/scheduled-events/{event.id}`
    ModifyScheduledEvent {
        guild_id: Snowflake,
        event_id: Snowflake,
    },
    /// `DELETE /guilds/{guild.id}/scheduled-events/{event.id}`
    DeleteScheduledEvent {
        guild_id: Snowflake,
        event_id: Snowflake,
    },

    // ── Stickers ──────────────────────────────────────────────────────────
    /// `GET /guilds/{guild.id}/stickers`
    ListGuildStickers { guild_id: Snowflake },
    /// `GET /guilds/{guild.id}/stickers/{sticker.id}`
    GetGuildSticker {
        guild_id: Snowflake,
        sticker_id: Snowflake,
    },
    /// `POST /guilds/{guild.id}/stickers`
    CreateGuildSticker { guild_id: Snowflake },
    /// `PATCH /guilds/{guild.id}/stickers/{sticker.id}`
    ModifyGuildSticker {
        guild_id: Snowflake,
        sticker_id: Snowflake,
    },
    /// `DELETE /guilds/{guild.id}/stickers/{sticker.id}`
    DeleteGuildSticker {
        guild_id: Snowflake,
        sticker_id: Snowflake,
    },

    // ── Auto Moderation ───────────────────────────────────────────────────
    /// `GET /guilds/{guild.id}/auto-moderation/rules`
    ListAutoModerationRules { guild_id: Snowflake },
    /// `GET /guilds/{guild.id}/auto-moderation/rules/{rule.id}`
    GetAutoModerationRule {
        guild_id: Snowflake,
        rule_id: Snowflake,
    },
    /// `POST /guilds/{guild.id}/auto-moderation/rules`
    CreateAutoModerationRule { guild_id: Snowflake },
    /// `PATCH /guilds/{guild.id}/auto-moderation/rules/{rule.id}`
    ModifyAutoModerationRule {
        guild_id: Snowflake,
        rule_id: Snowflake,
    },
    /// `DELETE /guilds/{guild.id}/auto-moderation/rules/{rule.id}`
    DeleteAutoModerationRule {
        guild_id: Snowflake,
        rule_id: Snowflake,
    },

    // ── Entitlements (Monetization) ───────────────────────────────────────
    /// `GET /applications/{application.id}/entitlements`
    ListEntitlements { application_id: Snowflake },
    /// `POST /applications/{application.id}/entitlements` — create a test entitlement.
    CreateTestEntitlement { application_id: Snowflake },
    /// `DELETE /applications/{application.id}/entitlements/{entitlement.id}` — delete a test entitlement.
    DeleteTestEntitlement {
        application_id: Snowflake,
        entitlement_id: Snowflake,
    },
    /// `GET /applications/{application.id}/skus`
    ListSkus { application_id: Snowflake },
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
            | Self::GetGuildCommands { .. }
            | Self::ListPublicArchivedThreads { .. }
            | Self::ListPrivateArchivedThreads { .. }
            | Self::ListScheduledEvents { .. }
            | Self::GetScheduledEvent { .. }
            | Self::ListGuildStickers { .. }
            | Self::GetGuildSticker { .. }
            | Self::ListAutoModerationRules { .. }
            | Self::GetAutoModerationRule { .. }
            | Self::ListEntitlements { .. }
            | Self::ListSkus { .. } => Method::Get,

            Self::CreateMessage { .. }
            | Self::CreateGlobalCommand { .. }
            | Self::CreateGuildCommand { .. }
            | Self::CreateInteractionResponse { .. }
            | Self::CreateThread { .. }
            | Self::CreateThreadFromMessage { .. }
            | Self::CreateScheduledEvent { .. }
            | Self::CreateGuildSticker { .. }
            | Self::CreateAutoModerationRule { .. }
            | Self::CreateTestEntitlement { .. } => Method::Post,

            Self::BulkOverwriteGlobalCommands { .. }
            | Self::BulkOverwriteGuildCommands { .. }
            | Self::JoinThread { .. }
            | Self::AddThreadMember { .. } => Method::Put,

            Self::EditMessage { .. }
            | Self::EditOriginalInteractionResponse { .. }
            | Self::ModifyThread { .. }
            | Self::ModifyGuildMember { .. }
            | Self::ModifyScheduledEvent { .. }
            | Self::ModifyGuildSticker { .. }
            | Self::ModifyAutoModerationRule { .. } => Method::Patch,

            Self::DeleteMessage { .. }
            | Self::DeleteGlobalCommand { .. }
            | Self::DeleteGuildCommand { .. }
            | Self::LeaveThread { .. }
            | Self::RemoveThreadMember { .. }
            | Self::DeleteScheduledEvent { .. }
            | Self::DeleteGuildSticker { .. }
            | Self::DeleteAutoModerationRule { .. }
            | Self::DeleteTestEntitlement { .. } => Method::Delete,
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
            Self::EditMessage {
                channel_id,
                message_id,
            } => format!("{base}/channels/{channel_id}/messages/{message_id}"),
            Self::EditOriginalInteractionResponse {
                application_id,
                interaction_token,
            } => format!(
                "{base}/webhooks/{application_id}/{interaction_token}/messages/@original"
            ),

            // ── Threads ──────────────────────────────────────────────────
            Self::CreateThread { channel_id } => {
                format!("{base}/channels/{channel_id}/threads")
            }
            Self::CreateThreadFromMessage {
                channel_id,
                message_id,
            } => format!("{base}/channels/{channel_id}/messages/{message_id}/threads"),
            Self::JoinThread { thread_id } => {
                format!("{base}/channels/{thread_id}/thread-members/@me")
            }
            Self::LeaveThread { thread_id } => {
                format!("{base}/channels/{thread_id}/thread-members/@me")
            }
            Self::AddThreadMember {
                thread_id,
                user_id,
            } => format!("{base}/channels/{thread_id}/thread-members/{user_id}"),
            Self::RemoveThreadMember {
                thread_id,
                user_id,
            } => format!("{base}/channels/{thread_id}/thread-members/{user_id}"),
            Self::ListPublicArchivedThreads { channel_id } => {
                format!("{base}/channels/{channel_id}/threads/archived/public")
            }
            Self::ListPrivateArchivedThreads { channel_id } => {
                format!("{base}/channels/{channel_id}/threads/archived/private")
            }
            Self::ModifyThread { channel_id } => {
                format!("{base}/channels/{channel_id}")
            }

            // ── Guild Members ────────────────────────────────────────────
            Self::ModifyGuildMember { guild_id, user_id } => {
                format!("{base}/guilds/{guild_id}/members/{user_id}")
            }

            // ── Scheduled Events ─────────────────────────────────────────
            Self::ListScheduledEvents { guild_id } => {
                format!("{base}/guilds/{guild_id}/scheduled-events")
            }
            Self::CreateScheduledEvent { guild_id } => {
                format!("{base}/guilds/{guild_id}/scheduled-events")
            }
            Self::GetScheduledEvent { guild_id, event_id } => {
                format!("{base}/guilds/{guild_id}/scheduled-events/{event_id}")
            }
            Self::ModifyScheduledEvent { guild_id, event_id } => {
                format!("{base}/guilds/{guild_id}/scheduled-events/{event_id}")
            }
            Self::DeleteScheduledEvent { guild_id, event_id } => {
                format!("{base}/guilds/{guild_id}/scheduled-events/{event_id}")
            }

            // ── Stickers ─────────────────────────────────────────────────
            Self::ListGuildStickers { guild_id } => {
                format!("{base}/guilds/{guild_id}/stickers")
            }
            Self::GetGuildSticker {
                guild_id,
                sticker_id,
            } => format!("{base}/guilds/{guild_id}/stickers/{sticker_id}"),
            Self::CreateGuildSticker { guild_id } => {
                format!("{base}/guilds/{guild_id}/stickers")
            }
            Self::ModifyGuildSticker {
                guild_id,
                sticker_id,
            } => format!("{base}/guilds/{guild_id}/stickers/{sticker_id}"),
            Self::DeleteGuildSticker {
                guild_id,
                sticker_id,
            } => format!("{base}/guilds/{guild_id}/stickers/{sticker_id}"),

            // ── Auto Moderation ──────────────────────────────────────────
            Self::ListAutoModerationRules { guild_id } => {
                format!("{base}/guilds/{guild_id}/auto-moderation/rules")
            }
            Self::GetAutoModerationRule { guild_id, rule_id } => {
                format!("{base}/guilds/{guild_id}/auto-moderation/rules/{rule_id}")
            }
            Self::CreateAutoModerationRule { guild_id } => {
                format!("{base}/guilds/{guild_id}/auto-moderation/rules")
            }
            Self::ModifyAutoModerationRule { guild_id, rule_id } => {
                format!("{base}/guilds/{guild_id}/auto-moderation/rules/{rule_id}")
            }
            Self::DeleteAutoModerationRule { guild_id, rule_id } => {
                format!("{base}/guilds/{guild_id}/auto-moderation/rules/{rule_id}")
            }

            // ── Entitlements / Monetization ──────────────────────────────
            Self::ListEntitlements { application_id } => {
                format!("{base}/applications/{application_id}/entitlements")
            }
            Self::CreateTestEntitlement { application_id } => {
                format!("{base}/applications/{application_id}/entitlements")
            }
            Self::DeleteTestEntitlement {
                application_id,
                entitlement_id,
            } => format!(
                "{base}/applications/{application_id}/entitlements/{entitlement_id}"
            ),
            Self::ListSkus { application_id } => {
                format!("{base}/applications/{application_id}/skus")
            }
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
            | Self::DeleteMessage { channel_id, .. }
            | Self::EditMessage { channel_id, .. }
            | Self::CreateThread { channel_id }
            | Self::CreateThreadFromMessage { channel_id, .. }
            | Self::ListPublicArchivedThreads { channel_id }
            | Self::ListPrivateArchivedThreads { channel_id }
            | Self::ModifyThread { channel_id } => format!("channel:{channel_id}"),

            Self::JoinThread { thread_id }
            | Self::LeaveThread { thread_id }
            | Self::AddThreadMember { thread_id, .. }
            | Self::RemoveThreadMember { thread_id, .. } => {
                format!("channel:{thread_id}")
            }

            Self::GetGuild { guild_id }
            | Self::GetGuildMember { guild_id, .. }
            | Self::ModifyGuildMember { guild_id, .. } => {
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

            Self::EditOriginalInteractionResponse { application_id, .. } => {
                format!("webhook:{application_id}")
            }

            // ── Scheduled Events ─────────────────────────────────────────
            Self::ListScheduledEvents { guild_id }
            | Self::CreateScheduledEvent { guild_id }
            | Self::GetScheduledEvent { guild_id, .. }
            | Self::ModifyScheduledEvent { guild_id, .. }
            | Self::DeleteScheduledEvent { guild_id, .. } => {
                format!("guild:{guild_id}:events")
            }

            // ── Stickers ─────────────────────────────────────────────────
            Self::ListGuildStickers { guild_id }
            | Self::GetGuildSticker { guild_id, .. }
            | Self::CreateGuildSticker { guild_id }
            | Self::ModifyGuildSticker { guild_id, .. }
            | Self::DeleteGuildSticker { guild_id, .. } => {
                format!("guild:{guild_id}:stickers")
            }

            // ── Auto Moderation ──────────────────────────────────────────
            Self::ListAutoModerationRules { guild_id }
            | Self::GetAutoModerationRule { guild_id, .. }
            | Self::CreateAutoModerationRule { guild_id }
            | Self::ModifyAutoModerationRule { guild_id, .. }
            | Self::DeleteAutoModerationRule { guild_id, .. } => {
                format!("guild:{guild_id}:automod")
            }

            // ── Entitlements / Monetization ──────────────────────────────
            Self::ListEntitlements { application_id }
            | Self::CreateTestEntitlement { application_id }
            | Self::DeleteTestEntitlement { application_id, .. }
            | Self::ListSkus { application_id } => {
                format!("application:{application_id}:entitlements")
            }
        }
    }
}
