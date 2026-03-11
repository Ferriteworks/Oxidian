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

pub use connection::SessionState;
pub use events::{
    DispatchEvent, GuildBanData, GuildMemberAddData, GuildMemberRemoveData,
    GuildRoleData, GuildRoleDeleteData, MessageDeleteBulkData, MessageDeleteData,
    ReactionData, ReactionEmoji, ReactionRemoveAllData, ReactionRemoveEmojiData,
    ReadyData, TypingStartData, VoiceServerUpdateData, VoiceStateUpdateData,
};
pub use shard::Shard;
