/// WebSocket connection management and the main gateway event loop.
pub mod connection;
/// Typed Discord gateway events and raw payload types.
pub mod events;
/// Heartbeat loop task.
pub mod heartbeat;
/// Discord gateway opcodes enum.
pub mod opcodes;
/// High-level shard abstraction.
pub mod shard;

pub use events::{
    DispatchEvent, ReadyData,
    MessageDeleteData, MessageDeleteBulkData,
    ReactionData, ReactionEmoji, ReactionRemoveAllData, ReactionRemoveEmojiData,
    GuildMemberAddData, GuildMemberRemoveData,
    GuildBanData,
    GuildRoleData, GuildRoleDeleteData,
    TypingStartData,
    VoiceStateUpdateData, VoiceServerUpdateData,
};
pub use connection::SessionState;
pub use shard::Shard;
