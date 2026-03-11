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
    Activity, ApplicationCommandPermission, ApplicationCommandPermissionsUpdateData,
    AuditLogEntry, AutoModerationAction, AutoModerationActionExecutionData,
    AutoModerationActionMetadata, AutoModerationRule, AutoModerationTriggerMetadata,
    ChannelPinsUpdateData, ClientStatus, DispatchEvent, GatewayPayload, GuildBanData,
    GuildEmojisUpdateData, GuildIntegrationsUpdateData, GuildMemberAddData,
    GuildMemberRemoveData, GuildMemberUpdateData, GuildMembersChunkData, GuildRoleData,
    GuildRoleDeleteData, GuildSoundboardSoundsUpdateData, GuildStickersUpdateData,
    HelloData, Integration, IntegrationDeleteData, InviteCreateData, InviteDeleteData,
    MessageDeleteBulkData, MessageDeleteData, PollVoteData, PresenceUpdateData,
    PresenceUser, ReactionData, ReactionEmoji, ReactionRemoveAllData,
    ReactionRemoveEmojiData, ReadyData, ScheduledEventUserData,
    SoundboardSoundDeleteData, Subscription, ThreadDeleteData, ThreadListSyncData,
    ThreadMemberUpdateData, ThreadMembersUpdateData, TypingStartData, UnavailableGuild,
    VoiceServerUpdateData, VoiceStateUpdateData, WebhooksUpdateData,
};
pub use shard::Shard;
