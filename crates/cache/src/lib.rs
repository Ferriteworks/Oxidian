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

//! In-memory cache updated in real time by gateway events.
//!
//! The cache is **never** proactively fetched — it is populated and maintained
//! exclusively by gateway dispatch events. Call [`Cache::update`] with every
//! [`DispatchEvent`] **before** firing user event handlers.
//!
//! All inner maps use [`DashMap`] for lock-free concurrent reads/writes, so
//! the cache can be shared across tasks cheaply via an `Arc<Cache>`.
//!
//! # High-level usage
//!
//! When the `cache` feature is enabled on the `oxidian` crate, the [`Bot`]
//! automatically creates a cache and wires it into [`Context`].
//!
//! # Low-level usage
//!
//! ```rust,ignore
//! use std::sync::Arc;
//! use oxidian_cache::Cache;
//!
//! let cache = Arc::new(Cache::new());
//! // In your event loop:
//! cache.update(&event);
//! // then dispatch to handlers …
//! ```

use dashmap::DashMap;
use oxidian_core::{
    models::{
        channel::Channel,
        emoji::Emoji,
        guild::Guild,
        member::Member,
        role::Role,
        sticker::Sticker,
        user::User,
    },
    snowflake::Snowflake,
};
use oxidian_gateway::events::{
    DispatchEvent, PresenceUpdateData, VoiceStateUpdateData,
};
use tracing::trace;

/// A composite key for entities scoped to a guild (members, voice states, presences).
type GuildUser = (Snowflake, Snowflake);

/// A composite key for entities scoped to a guild (roles, channels, emojis, stickers).
type GuildEntity = (Snowflake, Snowflake);

/// In-memory cache of Discord state, updated by gateway events.
///
/// Every public map is a [`DashMap`] — concurrent reads never block each other
/// and writers only lock the affected shard (bucket), not the entire map.
pub struct Cache {
    /// Current bot user, populated on `READY`.
    current_user: DashMap<(), User>,

    /// Guilds keyed by guild ID.
    guilds: DashMap<Snowflake, Guild>,

    /// Channels keyed by channel ID.
    channels: DashMap<Snowflake, Channel>,

    /// Users keyed by user ID (de-duplicated across guilds).
    users: DashMap<Snowflake, User>,

    /// Guild members keyed by `(guild_id, user_id)`.
    members: DashMap<GuildUser, Member>,

    /// Roles keyed by `(guild_id, role_id)`.
    roles: DashMap<GuildEntity, Role>,

    /// Voice states keyed by `(guild_id, user_id)`.
    voice_states: DashMap<GuildUser, VoiceStateUpdateData>,

    /// Presences keyed by `(guild_id, user_id)`.
    presences: DashMap<GuildUser, PresenceUpdateData>,

    /// Emoji keyed by `(guild_id, emoji_id)`.
    emojis: DashMap<GuildEntity, Emoji>,

    /// Stickers keyed by `(guild_id, sticker_id)`.
    stickers: DashMap<GuildEntity, Sticker>,
}

impl Default for Cache {
    fn default() -> Self {
        Self::new()
    }
}

impl Cache {
    /// Create a new, empty cache.
    pub fn new() -> Self {
        Self {
            current_user: DashMap::new(),
            guilds: DashMap::new(),
            channels: DashMap::new(),
            users: DashMap::new(),
            members: DashMap::new(),
            roles: DashMap::new(),
            voice_states: DashMap::new(),
            presences: DashMap::new(),
            emojis: DashMap::new(),
            stickers: DashMap::new(),
        }
    }

    // ── Reads ─────────────────────────────────────────────────────────────

    /// The current bot user, if READY has been received.
    pub fn current_user(&self) -> Option<User> {
        self.current_user.get(&()).map(|r| r.value().clone())
    }

    /// Look up a guild by ID.
    pub fn guild(&self, id: Snowflake) -> Option<Guild> {
        self.guilds.get(&id).map(|r| r.value().clone())
    }

    /// Number of cached guilds.
    pub fn guild_count(&self) -> usize {
        self.guilds.len()
    }

    /// Look up a channel by ID.
    pub fn channel(&self, id: Snowflake) -> Option<Channel> {
        self.channels.get(&id).map(|r| r.value().clone())
    }

    /// Look up a user by ID.
    pub fn user(&self, id: Snowflake) -> Option<User> {
        self.users.get(&id).map(|r| r.value().clone())
    }

    /// Look up a guild member by guild + user ID.
    pub fn member(&self, guild_id: Snowflake, user_id: Snowflake) -> Option<Member> {
        self.members.get(&(guild_id, user_id)).map(|r| r.value().clone())
    }

    /// Look up a role by guild + role ID.
    pub fn role(&self, guild_id: Snowflake, role_id: Snowflake) -> Option<Role> {
        self.roles.get(&(guild_id, role_id)).map(|r| r.value().clone())
    }

    /// All roles for a guild.
    pub fn guild_roles(&self, guild_id: Snowflake) -> Vec<Role> {
        self.roles
            .iter()
            .filter(|r| r.key().0 == guild_id)
            .map(|r| r.value().clone())
            .collect()
    }

    /// Look up a voice state by guild + user ID.
    pub fn voice_state(
        &self,
        guild_id: Snowflake,
        user_id: Snowflake,
    ) -> Option<VoiceStateUpdateData> {
        self.voice_states.get(&(guild_id, user_id)).map(|r| r.value().clone())
    }

    /// Look up a presence by guild + user ID.
    pub fn presence(
        &self,
        guild_id: Snowflake,
        user_id: Snowflake,
    ) -> Option<PresenceUpdateData> {
        self.presences.get(&(guild_id, user_id)).map(|r| r.value().clone())
    }

    /// All emojis for a guild.
    pub fn guild_emojis(&self, guild_id: Snowflake) -> Vec<Emoji> {
        self.emojis
            .iter()
            .filter(|r| r.key().0 == guild_id)
            .map(|r| r.value().clone())
            .collect()
    }

    /// All channels in a guild.
    pub fn guild_channels(&self, guild_id: Snowflake) -> Vec<Channel> {
        self.channels
            .iter()
            .filter(|r| r.value().guild_id == Some(guild_id))
            .map(|r| r.value().clone())
            .collect()
    }

    /// All members in a guild (from cache — may be incomplete if GUILD_MEMBERS intent is off).
    pub fn guild_members(&self, guild_id: Snowflake) -> Vec<Member> {
        self.members
            .iter()
            .filter(|r| r.key().0 == guild_id)
            .map(|r| r.value().clone())
            .collect()
    }

    // ── Update ────────────────────────────────────────────────────────────

    /// Update the cache from a gateway dispatch event.
    ///
    /// **Call this before dispatching the event to user handlers** so that
    /// handler code always sees up-to-date state.
    pub fn update(&self, event: &DispatchEvent) {
        match event {
            // ── Session ──────────────────────────────────────────────────
            DispatchEvent::Ready(ready) => {
                self.current_user.insert((), ready.user.clone());
                trace!("cache: stored current user {}", ready.user.username);
            }

            // ── Guilds ──────────────────────────────────────────────────
            DispatchEvent::GuildCreate(guild) | DispatchEvent::GuildUpdate(guild) => {
                trace!(guild_id = %guild.id.get(), "cache: upsert guild");
                self.guilds.insert(guild.id, guild.clone());
            }
            DispatchEvent::GuildDelete(unavailable) => {
                if !unavailable.unavailable {
                    // Bot was removed — purge all guild-scoped data.
                    trace!(guild_id = %unavailable.id.get(), "cache: remove guild (left)");
                    self.purge_guild(unavailable.id);
                }
                // If unavailable == true it's a temporary outage; keep cached data.
            }

            // ── Channels ─────────────────────────────────────────────────
            DispatchEvent::ChannelCreate(ch) | DispatchEvent::ChannelUpdate(ch) => {
                trace!(channel_id = %ch.id.get(), "cache: upsert channel");
                self.channels.insert(ch.id, ch.clone());
            }
            DispatchEvent::ChannelDelete(ch) => {
                trace!(channel_id = %ch.id.get(), "cache: remove channel");
                self.channels.remove(&ch.id);
            }

            // ── Roles ────────────────────────────────────────────────────
            DispatchEvent::GuildRoleCreate(data) | DispatchEvent::GuildRoleUpdate(data) => {
                trace!(guild_id = %data.guild_id.get(), role_id = %data.role.id.get(), "cache: upsert role");
                self.roles.insert((data.guild_id, data.role.id), data.role.clone());
            }
            DispatchEvent::GuildRoleDelete(data) => {
                trace!(guild_id = %data.guild_id.get(), role_id = %data.role_id.get(), "cache: remove role");
                self.roles.remove(&(data.guild_id, data.role_id));
            }

            // ── Members ──────────────────────────────────────────────────
            DispatchEvent::GuildMemberAdd(data) => {
                if let Some(ref user) = data.member.user {
                    trace!(guild_id = %data.guild_id.get(), user_id = %user.id.get(), "cache: add member");
                    self.users.insert(user.id, user.clone());
                    self.members.insert((data.guild_id, user.id), data.member.clone());
                }
            }
            DispatchEvent::GuildMemberUpdate(data) => {
                trace!(guild_id = %data.guild_id.get(), user_id = %data.user.id.get(), "cache: update member");
                self.users.insert(data.user.id, data.user.clone());
                // Patch the existing member entry if it exists.
                if let Some(mut entry) = self.members.get_mut(&(data.guild_id, data.user.id)) {
                    let m = entry.value_mut();
                    m.nick = data.nick.clone();
                    m.avatar = data.avatar.clone();
                    m.roles = data.roles.clone();
                    m.user = Some(data.user.clone());
                    if let Some(deaf) = data.deaf {
                        m.deaf = deaf;
                    }
                    if let Some(mute) = data.mute {
                        m.mute = mute;
                    }
                    if let Some(pending) = data.pending {
                        m.pending = pending;
                    }
                }
            }
            DispatchEvent::GuildMemberRemove(data) => {
                trace!(guild_id = %data.guild_id.get(), user_id = %data.user.id.get(), "cache: remove member");
                self.members.remove(&(data.guild_id, data.user.id));
                // Don't remove from users — they may be in other guilds.
            }
            DispatchEvent::GuildMembersChunk(data) => {
                trace!(guild_id = %data.guild_id.get(), count = data.members.len(), "cache: members chunk");
                for member in &data.members {
                    if let Some(ref user) = member.user {
                        self.users.insert(user.id, user.clone());
                        self.members.insert((data.guild_id, user.id), member.clone());
                    }
                }
            }

            // ── Voice states ─────────────────────────────────────────────
            DispatchEvent::VoiceStateUpdate(state) => {
                if let Some(guild_id) = state.guild_id {
                    if state.channel_id.is_some() {
                        trace!(guild_id = %guild_id.get(), user_id = %state.user_id.get(), "cache: upsert voice state");
                        self.voice_states.insert((guild_id, state.user_id), state.clone());
                    } else {
                        // User left voice — remove.
                        trace!(guild_id = %guild_id.get(), user_id = %state.user_id.get(), "cache: remove voice state");
                        self.voice_states.remove(&(guild_id, state.user_id));
                    }
                }
            }

            // ── Presences ────────────────────────────────────────────────
            DispatchEvent::PresenceUpdate(data) => {
                trace!(guild_id = %data.guild_id.get(), user_id = %data.user.id.get(), "cache: upsert presence");
                self.presences.insert((data.guild_id, data.user.id), data.clone());
            }

            // ── Emojis ───────────────────────────────────────────────────
            DispatchEvent::GuildEmojisUpdate(data) => {
                trace!(guild_id = %data.guild_id.get(), count = data.emojis.len(), "cache: replace emojis");
                // Remove old emojis for this guild, then insert new ones.
                self.emojis.retain(|k, _| k.0 != data.guild_id);
                for emoji in &data.emojis {
                    if let Some(id) = emoji.id {
                        self.emojis.insert((data.guild_id, id), emoji.clone());
                    }
                }
            }

            // ── Stickers ─────────────────────────────────────────────────
            DispatchEvent::GuildStickersUpdate(data) => {
                trace!(guild_id = %data.guild_id.get(), count = data.stickers.len(), "cache: replace stickers");
                self.stickers.retain(|k, _| k.0 != data.guild_id);
                for sticker in &data.stickers {
                    self.stickers.insert((data.guild_id, sticker.id), sticker.clone());
                }
            }

            // ── User (self) ──────────────────────────────────────────────
            DispatchEvent::UserUpdate(user) => {
                trace!(user_id = %user.id.get(), "cache: update current user");
                self.current_user.insert((), user.clone());
                self.users.insert(user.id, user.clone());
            }

            // ── Messages carry author info ───────────────────────────────
            DispatchEvent::MessageCreate(msg) => {
                self.users.insert(msg.author.id, msg.author.clone());
            }

            // Everything else — no cache impact.
            _ => {}
        }
    }

    /// Remove all data associated with a guild.
    fn purge_guild(&self, guild_id: Snowflake) {
        self.guilds.remove(&guild_id);
        self.channels.retain(|_, ch| ch.guild_id != Some(guild_id));
        self.members.retain(|k, _| k.0 != guild_id);
        self.roles.retain(|k, _| k.0 != guild_id);
        self.voice_states.retain(|k, _| k.0 != guild_id);
        self.presences.retain(|k, _| k.0 != guild_id);
        self.emojis.retain(|k, _| k.0 != guild_id);
        self.stickers.retain(|k, _| k.0 != guild_id);
    }
}

impl std::fmt::Debug for Cache {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Cache")
            .field("guilds", &self.guilds.len())
            .field("channels", &self.channels.len())
            .field("users", &self.users.len())
            .field("members", &self.members.len())
            .field("roles", &self.roles.len())
            .field("voice_states", &self.voice_states.len())
            .field("presences", &self.presences.len())
            .field("emojis", &self.emojis.len())
            .field("stickers", &self.stickers.len())
            .finish()
    }
}
