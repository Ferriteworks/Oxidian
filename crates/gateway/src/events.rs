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

/// Minimal `READY` event data — the fields needed to confirm the bot is online.
///
/// The full payload contains much more; additional fields will be added as
/// models are implemented.
#[derive(Debug, Deserialize)]
pub struct ReadyData {
    /// Discord gateway protocol version negotiated for this session.
    pub v: u8,
    /// The bot user object returned by Discord.
    pub user: ReadyUser,
    /// Opaque session ID required for `RESUME`.
    pub session_id: String,
    /// The URL to use when reconnecting / resuming.
    pub resume_gateway_url: String,
}

/// Minimal user object embedded in the `READY` payload.
#[derive(Debug, Deserialize)]
pub struct ReadyUser {
    /// The bot's Discord user ID (snowflake).
    pub id: String,
    /// The bot's username.
    pub username: String,
    /// The bot's discriminator (may be "0" for migrated accounts).
    pub discriminator: String,
}
