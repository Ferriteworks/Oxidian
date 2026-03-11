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

use oxidian_core::{
    models::{
        channel::Channel,
        emoji::Emoji,
        guild::Guild,
        interaction::Interaction,
        member::Member,
        message::Message,
        role::Role,
        scheduled_event::ScheduledEvent,
        soundboard::SoundboardSound,
        stage::StageInstance,
        sticker::Sticker,
        thread::{Thread, ThreadMember},
        user::User,
        voice::VoiceChannelEffect,
    },
    snowflake::Snowflake,
};
use serde::Deserialize;

/// A raw, untyped Discord gateway payload as received over the WebSocket.
///
/// All fields except `op` are optional because their presence depends on the
/// opcode; callers should match on `op` before inspecting the others.
#[derive(Debug, Deserialize)]
pub struct GatewayPayload {
    pub op: u8,
    pub d: Option<serde_json::Value>,
    pub s: Option<u64>,
    pub t: Option<String>,
}

/// Data payload for opcode 10 (`Hello`).
#[derive(Debug, Deserialize)]
pub struct HelloData {
    pub heartbeat_interval: u64,
}

/// Data from the `READY` dispatch event.
#[derive(Debug, Clone, Deserialize)]
pub struct ReadyData {
    pub v: u8,
    pub user: User,
    pub session_id: String,
    pub resume_gateway_url: String,
    pub application: ReadyApplication,
}

/// The minimal application object returned inside `READY`.
#[derive(Debug, Clone, Deserialize)]
pub struct ReadyApplication {
    /// The application (bot) ID.
    pub id: Snowflake,
}

/// Data from a `MESSAGE_DELETE` event.
#[derive(Debug, Clone, Deserialize)]
pub struct MessageDeleteData {
    pub id: Snowflake,
    pub channel_id: Snowflake,
    pub guild_id: Option<Snowflake>,
}

/// Data from a `MESSAGE_DELETE_BULK` event.
#[derive(Debug, Clone, Deserialize)]
pub struct MessageDeleteBulkData {
    /// The IDs of the deleted messages.
    pub ids: Vec<Snowflake>,
    /// The channel the messages were deleted from.
    pub channel_id: Snowflake,
    /// The guild, if applicable.
    pub guild_id: Option<Snowflake>,
}

/// A guild that is unavailable (outage) or has been deleted.
#[derive(Debug, Clone, Deserialize)]
pub struct UnavailableGuild {
    pub id: Snowflake,
    #[serde(default)]
    pub unavailable: bool,
}

/// Data from a `GUILD_MEMBER_ADD` event.
///
/// Discord sends a full `Member` object with an extra `guild_id` field
/// injected at the top level.
#[derive(Debug, Clone, Deserialize)]
pub struct GuildMemberAddData {
    /// The guild the user joined.
    pub guild_id: Snowflake,
    /// The new member.
    #[serde(flatten)]
    pub member: Member,
}

/// Data from a `GUILD_MEMBER_REMOVE` event.
#[derive(Debug, Clone, Deserialize)]
pub struct GuildMemberRemoveData {
    /// The guild the user left or was removed from.
    pub guild_id: Snowflake,
    /// The user who left or was removed.
    pub user: User,
}

/// Data from `GUILD_BAN_ADD` or `GUILD_BAN_REMOVE`.
#[derive(Debug, Clone, Deserialize)]
pub struct GuildBanData {
    /// The guild the ban applies to.
    pub guild_id: Snowflake,
    /// The user who was banned or unbanned.
    pub user: User,
}

/// Data from `GUILD_ROLE_CREATE` or `GUILD_ROLE_UPDATE`.
#[derive(Debug, Clone, Deserialize)]
pub struct GuildRoleData {
    /// The guild the role belongs to.
    pub guild_id: Snowflake,
    /// The created or updated role.
    pub role: Role,
}

/// Data from `GUILD_ROLE_DELETE`.
#[derive(Debug, Clone, Deserialize)]
pub struct GuildRoleDeleteData {
    /// The guild the role was deleted from.
    pub guild_id: Snowflake,
    /// The ID of the deleted role.
    pub role_id: Snowflake,
}

/// The emoji component of a message reaction event.
#[derive(Debug, Clone, Deserialize)]
pub struct ReactionEmoji {
    /// Snowflake ID for custom emoji; `None` for standard Unicode emoji.
    pub id: Option<Snowflake>,
    /// The emoji name, or the Unicode character for standard emoji.
    pub name: Option<String>,
}

/// Data from `MESSAGE_REACTION_ADD` or `MESSAGE_REACTION_REMOVE`.
#[derive(Debug, Clone, Deserialize)]
pub struct ReactionData {
    /// The user who added or removed the reaction.
    pub user_id: Snowflake,
    /// The channel the reacted message is in.
    pub channel_id: Snowflake,
    /// The message that was reacted to.
    pub message_id: Snowflake,
    /// The guild, if the message is in a guild channel.
    pub guild_id: Option<Snowflake>,
    /// The emoji used in the reaction.
    pub emoji: ReactionEmoji,
}

/// Data from `MESSAGE_REACTION_REMOVE_ALL` — all reactions removed from a message.
#[derive(Debug, Clone, Deserialize)]
pub struct ReactionRemoveAllData {
    pub channel_id: Snowflake,
    pub message_id: Snowflake,
    pub guild_id: Option<Snowflake>,
}

/// Data from `MESSAGE_REACTION_REMOVE_EMOJI` — all reactions for one emoji removed.
#[derive(Debug, Clone, Deserialize)]
pub struct ReactionRemoveEmojiData {
    pub channel_id: Snowflake,
    pub message_id: Snowflake,
    pub guild_id: Option<Snowflake>,
    pub emoji: ReactionEmoji,
}

/// Data from `TYPING_START`.
#[derive(Debug, Clone, Deserialize)]
pub struct TypingStartData {
    /// The channel where typing started.
    pub channel_id: Snowflake,
    /// The guild, if applicable.
    pub guild_id: Option<Snowflake>,
    /// The user who started typing.
    pub user_id: Snowflake,
    /// Unix timestamp (seconds) of when typing started.
    pub timestamp: u64,
}

/// Data from a `VOICE_STATE_UPDATE` dispatch event.
#[derive(Debug, Clone, Deserialize)]
pub struct VoiceStateUpdateData {
    pub guild_id: Option<Snowflake>,
    pub channel_id: Option<Snowflake>,
    pub user_id: Snowflake,
    pub session_id: String,
    pub deaf: bool,
    pub mute: bool,
    pub self_deaf: bool,
    pub self_mute: bool,
    #[serde(default)]
    pub self_stream: Option<bool>,
    pub self_video: bool,
    pub suppress: bool,
}

/// Data from a `VOICE_SERVER_UPDATE` dispatch event.
#[derive(Debug, Clone, Deserialize)]
pub struct VoiceServerUpdateData {
    pub token: String,
    pub guild_id: Snowflake,
    pub endpoint: Option<String>,
}

/// Data from a `GUILD_MEMBER_UPDATE` event.
///
/// Sent when guild member properties change (roles, nick, avatar, timeout, etc.).
#[derive(Debug, Clone, Deserialize)]
pub struct GuildMemberUpdateData {
    pub guild_id: Snowflake,
    #[serde(default)]
    pub roles: Vec<Snowflake>,
    pub user: User,
    pub nick: Option<String>,
    pub avatar: Option<String>,
    pub joined_at: Option<String>,
    pub premium_since: Option<String>,
    pub deaf: Option<bool>,
    pub mute: Option<bool>,
    pub pending: Option<bool>,
    /// ISO 8601 timestamp until which the member is timed out (`None` if not).
    pub communication_disabled_until: Option<String>,
}

/// Partial user present in `PRESENCE_UPDATE`.  Only `id` is guaranteed.
#[derive(Debug, Clone, Deserialize)]
pub struct PresenceUser {
    pub id: Snowflake,
}

/// An activity in a user's presence (game, music, streaming, custom, etc.).
#[derive(Debug, Clone, Deserialize)]
pub struct Activity {
    pub name: String,
    /// Activity type: 0=Game, 1=Streaming, 2=Listening, 3=Watching, 4=Custom, 5=Competing.
    #[serde(rename = "type")]
    pub kind: u8,
    pub url: Option<String>,
    pub state: Option<String>,
    pub details: Option<String>,
}

/// Platform-specific online status for a user.
#[derive(Debug, Clone, Deserialize)]
pub struct ClientStatus {
    pub desktop: Option<String>,
    pub mobile: Option<String>,
    pub web: Option<String>,
}

/// Data from a `PRESENCE_UPDATE` event.
#[derive(Debug, Clone, Deserialize)]
pub struct PresenceUpdateData {
    pub user: PresenceUser,
    pub guild_id: Snowflake,
    /// "online" | "idle" | "dnd" | "offline"
    pub status: String,
    #[serde(default)]
    pub activities: Vec<Activity>,
    pub client_status: ClientStatus,
}

/// Data from a `THREAD_DELETE` event (partial thread object).
#[derive(Debug, Clone, Deserialize)]
pub struct ThreadDeleteData {
    pub id: Snowflake,
    pub guild_id: Option<Snowflake>,
    pub parent_id: Option<Snowflake>,
    #[serde(rename = "type")]
    pub kind: u8,
}

/// Data from a `THREAD_LIST_SYNC` event.
///
/// Sent when the bot gains access to a channel, containing all active threads
/// within the synced channels.
#[derive(Debug, Clone, Deserialize)]
pub struct ThreadListSyncData {
    pub guild_id: Snowflake,
    /// Channels whose threads are being synced.  Absent means the entire guild.
    #[serde(default)]
    pub channel_ids: Vec<Snowflake>,
    pub threads: Vec<Thread>,
    pub members: Vec<ThreadMember>,
}

/// Data from a `THREAD_MEMBERS_UPDATE` event.
#[derive(Debug, Clone, Deserialize)]
pub struct ThreadMembersUpdateData {
    /// The thread that was updated.
    pub id: Snowflake,
    pub guild_id: Snowflake,
    /// Approximate member count, capped at 50.
    pub member_count: u32,
    #[serde(default)]
    pub added_members: Vec<ThreadMember>,
    #[serde(default)]
    pub removed_member_ids: Vec<Snowflake>,
}

/// Data from a `CHANNEL_PINS_UPDATE` event.
#[derive(Debug, Clone, Deserialize)]
pub struct ChannelPinsUpdateData {
    pub guild_id: Option<Snowflake>,
    pub channel_id: Snowflake,
    /// ISO 8601 timestamp of the most recent pin, or `None` if all pins were removed.
    pub last_pin_timestamp: Option<String>,
}

/// Data from `GUILD_SCHEDULED_EVENT_USER_ADD` or `GUILD_SCHEDULED_EVENT_USER_REMOVE`.
#[derive(Debug, Clone, Deserialize)]
pub struct ScheduledEventUserData {
    pub guild_scheduled_event_id: Snowflake,
    pub user_id: Snowflake,
    pub guild_id: Snowflake,
}

/// Trigger metadata for an auto-moderation rule.
#[derive(Debug, Clone, Deserialize)]
pub struct AutoModerationTriggerMetadata {
    #[serde(default)]
    pub keyword_filter: Vec<String>,
    #[serde(default)]
    pub regex_patterns: Vec<String>,
    /// Preset keyword list IDs: 1=PROFANITY, 2=SEXUAL_CONTENT, 3=SLURS.
    #[serde(default)]
    pub presets: Vec<u8>,
    #[serde(default)]
    pub allow_list: Vec<String>,
    pub mention_total_limit: Option<u32>,
    pub mention_raid_protection_enabled: Option<bool>,
}

/// Metadata for an auto-moderation action.
#[derive(Debug, Clone, Deserialize)]
pub struct AutoModerationActionMetadata {
    /// Channel to send an alert message to (SEND_ALERT_MESSAGE actions).
    pub channel_id: Option<Snowflake>,
    /// Timeout duration in seconds (TIMEOUT actions; max 2419200 = 4 weeks).
    pub duration_seconds: Option<u32>,
    /// Custom message shown to the user whose message was blocked.
    pub custom_message: Option<String>,
}

/// An auto-moderation action within a rule or execution payload.
#[derive(Debug, Clone, Deserialize)]
pub struct AutoModerationAction {
    /// 1 = BLOCK_MESSAGE, 2 = SEND_ALERT_MESSAGE, 3 = TIMEOUT, 4 = BLOCK_MEMBER_INTERACTION.
    #[serde(rename = "type")]
    pub kind: u8,
    pub metadata: Option<AutoModerationActionMetadata>,
}

/// A full auto-moderation rule (sent in CREATE / UPDATE / DELETE events).
#[derive(Debug, Clone, Deserialize)]
pub struct AutoModerationRule {
    pub id: Snowflake,
    pub guild_id: Snowflake,
    pub name: String,
    pub creator_id: Snowflake,
    /// 1 = MESSAGE_SEND, 2 = MEMBER_UPDATE.
    pub event_type: u8,
    /// 1 = KEYWORD, 3 = SPAM, 4 = KEYWORD_PRESET, 5 = MENTION_SPAM, 6 = MEMBER_PROFILE.
    pub trigger_type: u8,
    pub trigger_metadata: AutoModerationTriggerMetadata,
    pub actions: Vec<AutoModerationAction>,
    pub enabled: bool,
    #[serde(default)]
    pub exempt_roles: Vec<Snowflake>,
    #[serde(default)]
    pub exempt_channels: Vec<Snowflake>,
}

/// Data from an `AUTO_MODERATION_ACTION_EXECUTION` event.
#[derive(Debug, Clone, Deserialize)]
pub struct AutoModerationActionExecutionData {
    pub guild_id: Snowflake,
    pub action: AutoModerationAction,
    pub rule_id: Snowflake,
    pub rule_trigger_type: u8,
    pub user_id: Snowflake,
    pub channel_id: Option<Snowflake>,
    pub message_id: Option<Snowflake>,
    pub alert_system_message_id: Option<Snowflake>,
    /// The content of the blocked message (requires MESSAGE_CONTENT intent).
    pub content: String,
    pub matched_keyword: Option<String>,
    pub matched_content: Option<String>,
}

/// Data from `POLL_VOTE_ADD` or `POLL_VOTE_REMOVE`.
#[derive(Debug, Clone, Deserialize)]
pub struct PollVoteData {
    pub user_id: Snowflake,
    pub channel_id: Snowflake,
    pub message_id: Snowflake,
    pub guild_id: Option<Snowflake>,
    /// 1-based answer index as defined in the poll's `answers` array.
    pub answer_id: u32,
}

/// Data from `GUILD_SOUNDBOARD_SOUND_DELETE`.
///
/// Only the IDs are sent on delete; use [`SoundboardSound`] for create/update.
#[derive(Debug, Clone, Deserialize)]
pub struct SoundboardSoundDeleteData {
    pub sound_id: Snowflake,
    pub guild_id: Snowflake,
}

/// Data from `GUILD_EMOJIS_UPDATE`.
#[derive(Debug, Clone, Deserialize)]
pub struct GuildEmojisUpdateData {
    pub guild_id: Snowflake,
    /// The full updated list of emojis for the guild.
    pub emojis: Vec<Emoji>,
}

/// Data from `GUILD_STICKERS_UPDATE`.
#[derive(Debug, Clone, Deserialize)]
pub struct GuildStickersUpdateData {
    pub guild_id: Snowflake,
    /// The full updated list of stickers for the guild.
    pub stickers: Vec<Sticker>,
}

/// Data from a `GUILD_INTEGRATIONS_UPDATE` event (thin payload — just the guild).
#[derive(Debug, Clone, Deserialize)]
pub struct GuildIntegrationsUpdateData {
    pub guild_id: Snowflake,
}

/// A bot/OAuth2 integration attached to a guild.
#[derive(Debug, Clone, Deserialize)]
pub struct Integration {
    pub id: Snowflake,
    pub name: String,
    /// "twitch" | "youtube" | "discord" | "guild_subscription"
    #[serde(rename = "type")]
    pub kind: String,
    pub enabled: bool,
    pub syncing: Option<bool>,
    pub role_id: Option<Snowflake>,
    pub enable_emoticons: Option<bool>,
    /// 1 = REMOVE_ROLE, 2 = KICK
    pub expire_behavior: Option<u8>,
    pub expire_grace_period: Option<u32>,
    pub user: Option<User>,
    pub synced_at: Option<String>,
    pub subscriber_count: Option<u32>,
    pub revoked: Option<bool>,
    /// The guild this integration belongs to (injected on gateway events).
    pub guild_id: Option<Snowflake>,
    pub application_id: Option<Snowflake>,
}

/// Data from `INTEGRATION_DELETE`.
#[derive(Debug, Clone, Deserialize)]
pub struct IntegrationDeleteData {
    pub id: Snowflake,
    pub guild_id: Snowflake,
    /// ID of the bot/OAuth2 application this integration was for.
    pub application_id: Option<Snowflake>,
}

/// A single permission override entry inside an application command permissions object.
#[derive(Debug, Clone, Deserialize)]
pub struct ApplicationCommandPermission {
    /// Snowflake ID of the role, user, or channel this entry targets.
    pub id: Snowflake,
    /// 1 = ROLE, 2 = USER, 3 = CHANNEL
    #[serde(rename = "type")]
    pub kind: u8,
    pub permission: bool,
}

/// Data from `APPLICATION_COMMAND_PERMISSIONS_UPDATE`.
#[derive(Debug, Clone, Deserialize)]
pub struct ApplicationCommandPermissionsUpdateData {
    /// The command whose permissions changed.
    pub id: Snowflake,
    pub application_id: Snowflake,
    pub guild_id: Snowflake,
    pub permissions: Vec<ApplicationCommandPermission>,
}

/// An entry in the guild audit log.
#[derive(Debug, Clone, Deserialize)]
pub struct AuditLogEntry {
    pub id: Snowflake,
    /// The user who performed the action, if available.
    pub user_id: Option<Snowflake>,
    /// The action type (see Discord docs for the full table of ~70 values).
    pub action_type: u32,
    /// The entity that was targeted by the action.
    pub target_id: Option<String>,
    /// Optional reason attached to the action.
    pub reason: Option<String>,
}

/// A single member object inside a `GUILD_MEMBERS_CHUNK` payload.
#[derive(Debug, Clone, Deserialize)]
pub struct ChunkMember {
    #[serde(flatten)]
    pub member: Member,
}

/// Data from a `GUILD_MEMBERS_CHUNK` event.
///
/// Sent in response to an op 8 Request Guild Members.
#[derive(Debug, Clone, Deserialize)]
pub struct GuildMembersChunkData {
    pub guild_id: Snowflake,
    pub members: Vec<Member>,
    /// The index of this chunk (0-based).
    pub chunk_index: u32,
    /// Total number of chunks Discord will send for this request.
    pub chunk_count: u32,
    /// Presences for the returned members, if requested.
    #[serde(default)]
    pub presences: Vec<PresenceUpdateData>,
    /// The nonce passed in the original request, if any.
    pub nonce: Option<String>,
}

/// Data from `INVITE_CREATE`.
#[derive(Debug, Clone, Deserialize)]
pub struct InviteCreateData {
    pub channel_id: Snowflake,
    pub code: String,
    /// ISO 8601 creation timestamp.
    pub created_at: String,
    pub guild_id: Option<Snowflake>,
    pub inviter: Option<User>,
    /// Max age in seconds; 0 = never expires.
    pub max_age: u32,
    /// Max uses; 0 = unlimited.
    pub max_uses: u32,
    pub target_type: Option<u8>,
    pub target_user: Option<User>,
    pub temporary: bool,
    pub uses: u32,
}

/// Data from `INVITE_DELETE`.
#[derive(Debug, Clone, Deserialize)]
pub struct InviteDeleteData {
    pub channel_id: Snowflake,
    pub guild_id: Option<Snowflake>,
    pub code: String,
}

/// A premium app subscription (Discord Monetization).
#[derive(Debug, Clone, Deserialize)]
pub struct Subscription {
    pub id: Snowflake,
    pub user_id: Snowflake,
    pub sku_ids: Vec<Snowflake>,
    pub entitlement_ids: Vec<Snowflake>,
    /// ISO 8601 timestamp when the subscription started.
    pub current_period_start: String,
    /// ISO 8601 timestamp when the current subscription period ends.
    pub current_period_end: String,
    /// 0 = ACTIVE, 1 = ENDING, 2 = INACTIVE
    pub status: u8,
    pub canceled_at: Option<String>,
    pub country: Option<String>,
}

/// Data from `THREAD_MEMBER_UPDATE`.
///
/// Sent when the *current user's* (bot's) thread member object changes.
#[derive(Debug, Clone, Deserialize)]
pub struct ThreadMemberUpdateData {
    #[serde(flatten)]
    pub member: ThreadMember,
    /// The guild the thread is in (added by Discord to the event payload).
    pub guild_id: Snowflake,
}

/// Data from `WEBHOOKS_UPDATE`.
#[derive(Debug, Clone, Deserialize)]
pub struct WebhooksUpdateData {
    pub guild_id: Snowflake,
    pub channel_id: Snowflake,
}

/// Data from `GUILD_SOUNDBOARD_SOUNDS_UPDATE` (batch update of all sounds).
#[derive(Debug, Clone, Deserialize)]
pub struct GuildSoundboardSoundsUpdateData {
    pub guild_id: Snowflake,
    pub soundboard_sounds: Vec<SoundboardSound>,
}

/// A fully parsed Discord dispatch event (opcode 0).
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum DispatchEvent {
    // ── Session ───────────────────────────────────────────────────────────────
    /// The bot connected and the gateway session is ready.
    Ready(ReadyData),
    /// A previously disconnected session was successfully resumed.
    Resumed,

    // ── Messages ──────────────────────────────────────────────────────────────
    MessageCreate(Message),
    MessageUpdate(Message),
    MessageDelete(MessageDeleteData),
    MessageDeleteBulk(MessageDeleteBulkData),
    MessageReactionAdd(ReactionData),
    MessageReactionRemove(ReactionData),
    MessageReactionRemoveAll(ReactionRemoveAllData),
    MessageReactionRemoveEmoji(ReactionRemoveEmojiData),

    // ── Guilds ────────────────────────────────────────────────────────────────
    GuildCreate(Guild),
    GuildUpdate(Guild),
    GuildDelete(UnavailableGuild),

    // ── Members ───────────────────────────────────────────────────────────────
    GuildMemberAdd(GuildMemberAddData),
    GuildMemberRemove(GuildMemberRemoveData),
    /// Role/nick/avatar or other member properties changed.
    GuildMemberUpdate(GuildMemberUpdateData),

    // ── Presence ──────────────────────────────────────────────────────────────
    /// A user's presence (status, activities) changed.
    PresenceUpdate(PresenceUpdateData),

    // ── Bans ─────────────────────────────────────────────────────────────────
    GuildBanAdd(GuildBanData),
    GuildBanRemove(GuildBanData),

    // ── Roles ─────────────────────────────────────────────────────────────────
    GuildRoleCreate(GuildRoleData),
    GuildRoleUpdate(GuildRoleData),
    GuildRoleDelete(GuildRoleDeleteData),

    // ── Channels ──────────────────────────────────────────────────────────────
    ChannelCreate(Channel),
    ChannelUpdate(Channel),
    ChannelDelete(Channel),
    /// The last-pinned message in a channel changed.
    ChannelPinsUpdate(ChannelPinsUpdateData),

    // ── Threads ───────────────────────────────────────────────────────────────
    ThreadCreate(Thread),
    ThreadUpdate(Thread),
    ThreadDelete(ThreadDeleteData),
    /// Sync of all active threads for channels the bot can access.
    ThreadListSync(ThreadListSyncData),
    /// Members were added to or removed from a thread.
    ThreadMembersUpdate(ThreadMembersUpdateData),

    // ── Stage instances ───────────────────────────────────────────────────────
    StageInstanceCreate(StageInstance),
    StageInstanceUpdate(StageInstance),
    StageInstanceDelete(StageInstance),

    // ── Scheduled events ──────────────────────────────────────────────────────
    GuildScheduledEventCreate(ScheduledEvent),
    GuildScheduledEventUpdate(ScheduledEvent),
    GuildScheduledEventDelete(ScheduledEvent),
    GuildScheduledEventUserAdd(ScheduledEventUserData),
    GuildScheduledEventUserRemove(ScheduledEventUserData),

    // ── Auto moderation ───────────────────────────────────────────────────────
    AutoModerationRuleCreate(AutoModerationRule),
    AutoModerationRuleUpdate(AutoModerationRule),
    AutoModerationRuleDelete(AutoModerationRule),
    AutoModerationActionExecution(AutoModerationActionExecutionData),

    // ── Polls ─────────────────────────────────────────────────────────────────
    PollVoteAdd(PollVoteData),
    PollVoteRemove(PollVoteData),

    // ── Soundboard ────────────────────────────────────────────────────────────
    GuildSoundboardSoundCreate(SoundboardSound),
    GuildSoundboardSoundUpdate(SoundboardSound),
    GuildSoundboardSoundDelete(SoundboardSoundDeleteData),

    // ── Emojis / Stickers ─────────────────────────────────────────────────────
    /// The guild's full emoji list was updated.
    GuildEmojisUpdate(GuildEmojisUpdateData),
    /// The guild's full sticker list was updated.
    GuildStickersUpdate(GuildStickersUpdateData),

    // ── Audit log ─────────────────────────────────────────────────────────────
    /// A moderator action was logged in the guild audit log.
    GuildAuditLogEntryCreate(AuditLogEntry),

    // ── Integrations ──────────────────────────────────────────────────────────
    /// The guild's integration list changed (thin event — no detail).
    GuildIntegrationsUpdate(GuildIntegrationsUpdateData),
    IntegrationCreate(Integration),
    IntegrationUpdate(Integration),
    IntegrationDelete(IntegrationDeleteData),

    // ── Invites ───────────────────────────────────────────────────────────────
    InviteCreate(InviteCreateData),
    InviteDelete(InviteDeleteData),

    // ── Members (bulk) ────────────────────────────────────────────────────────
    /// Response to an op 8 Request Guild Members; may arrive in multiple chunks.
    GuildMembersChunk(GuildMembersChunkData),

    // ── User ──────────────────────────────────────────────────────────────────
    /// The current user (bot) updated their own profile.
    UserUpdate(User),

    // ── Threads (bot's own membership) ────────────────────────────────────────
    /// The bot's own thread-member record was updated.
    ThreadMemberUpdate(ThreadMemberUpdateData),

    // ── Webhooks ──────────────────────────────────────────────────────────────
    /// A webhook in a channel was created, updated, or deleted.
    WebhooksUpdate(WebhooksUpdateData),

    // ── Voice effects ─────────────────────────────────────────────────────────
    /// A user sent a soundboard sound or emoji reaction in a voice channel.
    VoiceChannelEffectSend(VoiceChannelEffect),

    // ── Application commands ──────────────────────────────────────────────────
    /// Permission overrides for a specific application command changed.
    ApplicationCommandPermissionsUpdate(ApplicationCommandPermissionsUpdateData),

    // ── Soundboard (batch) ────────────────────────────────────────────────────
    /// Bulk update of all soundboard sounds in a guild.
    GuildSoundboardSoundsUpdate(GuildSoundboardSoundsUpdateData),

    // ── Monetization ──────────────────────────────────────────────────────────
    SubscriptionCreate(Subscription),
    SubscriptionUpdate(Subscription),
    SubscriptionDelete(Subscription),

    // ── Interactions ──────────────────────────────────────────────────────────
    InteractionCreate(Interaction),

    // ── Typing ────────────────────────────────────────────────────────────────
    TypingStart(TypingStartData),

    // ── Voice ─────────────────────────────────────────────────────────────────
    VoiceStateUpdate(VoiceStateUpdateData),
    VoiceServerUpdate(VoiceServerUpdateData),

    /// An event that Oxidian doesn't yet parse into a typed variant.
    Unknown {
        name: String,
        data: serde_json::Value,
    },
}
