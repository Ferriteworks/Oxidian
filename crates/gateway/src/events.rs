use oxidian_core::{
    models::{
        channel::Channel, guild::Guild, interaction::Interaction, member::Member,
        message::Message, role::Role, user::User,
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
