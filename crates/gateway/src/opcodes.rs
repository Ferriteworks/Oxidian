/// All Discord gateway opcodes as defined in the official documentation.
///
/// <https://discord.com/developers/docs/topics/opcodes-and-status-codes#gateway-gateway-opcodes>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Opcode {
    /// Receive — an event was dispatched.
    Dispatch = 0,
    /// Send / Receive — keep the WebSocket connection alive.
    Heartbeat = 1,
    /// Send — start a new session during the initial handshake.
    Identify = 2,
    /// Send — update the client's presence / status.
    PresenceUpdate = 3,
    /// Send — used to join, move, or disconnect from a voice channel.
    VoiceStateUpdate = 4,
    /// Send — resume a previously disconnected session.
    Resume = 6,
    /// Receive — Discord requests the client to reconnect and resume.
    Reconnect = 7,
    /// Send — request members for a guild.
    RequestGuildMembers = 8,
    /// Receive — the session has been invalidated; may or may not be resumable.
    InvalidSession = 9,
    /// Receive — sent on connect; contains the heartbeat interval.
    Hello = 10,
    /// Receive — acknowledgement of a heartbeat sent by the client.
    HeartbeatAck = 11,
}

impl Opcode {
    /// Convert a raw integer received from Discord into an [`Opcode`].
    ///
    /// Returns `None` for any value that is not a recognised opcode so the
    /// caller can decide how to handle unknown/future opcodes gracefully.
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::Dispatch),
            1 => Some(Self::Heartbeat),
            2 => Some(Self::Identify),
            3 => Some(Self::PresenceUpdate),
            4 => Some(Self::VoiceStateUpdate),
            6 => Some(Self::Resume),
            7 => Some(Self::Reconnect),
            8 => Some(Self::RequestGuildMembers),
            9 => Some(Self::InvalidSession),
            10 => Some(Self::Hello),
            11 => Some(Self::HeartbeatAck),
            _ => None,
        }
    }
}
