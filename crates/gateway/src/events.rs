use oxidian_core::{models::{guild::Guild, message::Message}, snowflake::Snowflake};
use serde::Deserialize;

// ── Raw gateway payload ───────────────────────────────────────────────────────

/// A raw, untyped Discord gateway payload as received over the WebSocket.
///
/// All fields except `op` are optional because their presence depends on the
/// opcode; callers should match on `op` before inspecting the others.
#[derive(Debug, Deserialize)]
pub struct GatewayPayload {
    /// The opcode identifying the type of payload.
    pub op: u8,
    /// The inner data object; present for most opcodes.
    pub d: Option<serde_json::Value>,
    /// Sequence number; only present for `DISPATCH` (op 0) events.
    pub s: Option<u64>,
    /// Event name; only present for `DISPATCH` (op 0) events.
    pub t: Option<String>,
}

// ── Hello ─────────────────────────────────────────────────────────────────────

/// Data payload for opcode 10 (`Hello`).
#[derive(Debug, Deserialize)]
pub struct HelloData {
    /// Interval in milliseconds at which the client must send heartbeats.
    pub heartbeat_interval: u64,
}

// ── Ready ─────────────────────────────────────────────────────────────────────

/// Data from the READY dispatch event.
#[derive(Debug, Clone, Deserialize)]
pub struct ReadyData {
    /// Discord gateway protocol version negotiated for this session.
    pub v: u8,
    /// The bot user object.
    pub user: oxidian_core::models::user::User,
    /// Opaque session ID required for RESUME.
    pub session_id: String,
    /// The URL to reconnect/resume on.
    pub resume_gateway_url: String,
}

// ── Message delete ────────────────────────────────────────────────────────────

/// Data from a MESSAGE_DELETE event.
#[derive(Debug, Clone, Deserialize)]
pub struct MessageDeleteData {
    pub id: Snowflake,
    pub channel_id: Snowflake,
    pub guild_id: Option<Snowflake>,
}

// ── Unavailable guild ─────────────────────────────────────────────────────────

/// A guild that is unavailable due to an outage, or whose ID appeared in a
/// GUILD_DELETE event.
#[derive(Debug, Clone, Deserialize)]
pub struct UnavailableGuild {
    pub id: Snowflake,
    #[serde(default)]
    pub unavailable: bool,
}

// ── DispatchEvent ─────────────────────────────────────────────────────────────

/// A fully parsed Discord dispatch event (opcode 0).
///
/// Each variant corresponds to a `t` value sent by the gateway.
/// `Unknown` catches any event type that isn't explicitly handled.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum DispatchEvent {
    /// The bot has successfully identified and is ready.
    Ready(ReadyData),
    /// A new message was created in a channel.
    MessageCreate(Message),
    /// A message was deleted.
    MessageDelete(MessageDeleteData),
    /// The bot joined a guild or a guild became available.
    GuildCreate(Guild),
    /// A guild was updated.
    GuildUpdate(Guild),
    /// The bot was removed from a guild, or the guild became unavailable.
    GuildDelete(UnavailableGuild),
    /// An event type that isn't explicitly handled above.
    Unknown {
        /// The event name (`t` field from the payload).
        name: String,
        /// The raw event data.
        data: serde_json::Value,
    },
}
