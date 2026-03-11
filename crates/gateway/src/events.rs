use oxidian_core::{models::{guild::Guild, interaction::Interaction, message::Message}, snowflake::Snowflake};
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

/// Data from the READY dispatch event.
#[derive(Debug, Clone, Deserialize)]
pub struct ReadyData {
    pub v: u8,
    pub user: oxidian_core::models::user::User,
    pub session_id: String,
    pub resume_gateway_url: String,
}


/// Data from a MESSAGE_DELETE event.
#[derive(Debug, Clone, Deserialize)]
pub struct MessageDeleteData {
    pub id: Snowflake,
    pub channel_id: Snowflake,
    pub guild_id: Option<Snowflake>,
}


#[derive(Debug, Clone, Deserialize)]
pub struct UnavailableGuild {
    pub id: Snowflake,
    #[serde(default)]
    pub unavailable: bool,
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
    Ready(ReadyData),
    MessageCreate(Message),
    MessageDelete(MessageDeleteData),
    GuildCreate(Guild),
    GuildUpdate(Guild),
    GuildDelete(UnavailableGuild),
    InteractionCreate(Interaction),
    VoiceStateUpdate(VoiceStateUpdateData),
    VoiceServerUpdate(VoiceServerUpdateData),
    Unknown {
        name: String,
        data: serde_json::Value,
    },
}
