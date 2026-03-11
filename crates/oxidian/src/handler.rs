use async_trait::async_trait;

use oxidian_core::models::{
    channel::Channel, guild::Guild, interaction::Interaction, message::Message,
    scheduled_event::ScheduledEvent, soundboard::SoundboardSound, stage::StageInstance,
    thread::Thread, user::User, voice::VoiceChannelEffect,
};
use oxidian_gateway::events::{
    ApplicationCommandPermissionsUpdateData, AuditLogEntry,
    AutoModerationActionExecutionData, AutoModerationRule, ChannelPinsUpdateData,
    DispatchEvent, GuildBanData, GuildEmojisUpdateData, GuildIntegrationsUpdateData,
    GuildMemberAddData, GuildMemberRemoveData, GuildMemberUpdateData,
    GuildMembersChunkData, GuildRoleData, GuildRoleDeleteData,
    GuildSoundboardSoundsUpdateData, GuildStickersUpdateData, Integration,
    IntegrationDeleteData, InviteCreateData, InviteDeleteData, MessageDeleteBulkData,
    MessageDeleteData, PollVoteData, PresenceUpdateData, ReactionData,
    ReactionRemoveAllData, ReactionRemoveEmojiData, ReadyData, ScheduledEventUserData,
    SoundboardSoundDeleteData, Subscription, ThreadDeleteData, ThreadListSyncData,
    ThreadMemberUpdateData, ThreadMembersUpdateData, TypingStartData,
    VoiceServerUpdateData, VoiceStateUpdateData, WebhooksUpdateData,
};

use crate::context::Context;

/// Trait for receiving and responding to Discord gateway events.
///
/// All methods have empty default implementations so you only need to override
/// the events you care about.
///
/// # Example
///
/// ```rust,ignore
/// use oxidian::{EventHandler, Context};
/// use oxidian::core::models::message::Message;
/// use oxidian::gateway::events::ReadyData;
///
/// struct MyHandler;
///
/// #[async_trait::async_trait]
/// impl EventHandler for MyHandler {
///     async fn ready(&self, _ctx: Context, ready: ReadyData) {
///         println!("Logged in as {}!", ready.user.username);
///     }
///
///     async fn message(&self, ctx: Context, msg: Message) {
///         if msg.content == "!ping" {
///             ctx.reply(&msg, "pong!").await.ok();
///         }
///     }
/// }
/// ```
#[async_trait]
pub trait EventHandler: Send + Sync + 'static {
    /// Called when the bot has connected and is ready to receive events.
    async fn ready(&self, _ctx: Context, _ready: ReadyData) {}

    /// Called when a message is created in any channel the bot can see.
    async fn message(&self, _ctx: Context, _msg: Message) {}

    /// Called when a message is deleted.
    async fn message_delete(&self, _ctx: Context, _data: MessageDeleteData) {}

    /// Called when the bot joins a guild or a guild becomes available.
    async fn guild_create(&self, _ctx: Context, _guild: Guild) {}

    /// Called when a guild's settings are updated.
    async fn guild_update(&self, _ctx: Context, _guild: Guild) {}

    /// Called when a user invokes a slash command, clicks a component, or
    /// submits a modal.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// async fn interaction(&self, ctx: Context, interaction: Interaction) {
    ///     if let Some(data) = interaction.command_data() {
    ///         if data.name == "ping" {
    ///             ctx.respond(&interaction, InteractionResponse::message("Pong!"))
    ///                 .await.ok();
    ///         }
    ///     }
    /// }
    /// ```
    async fn interaction(&self, _ctx: Context, _interaction: Interaction) {}

    /// Called when a previously disconnected session has been successfully resumed.
    async fn resumed(&self, _ctx: Context) {}

    /// Called when a message is edited.
    async fn message_update(&self, _ctx: Context, _msg: Message) {}

    /// Called when multiple messages are deleted at once.
    async fn message_delete_bulk(&self, _ctx: Context, _data: MessageDeleteBulkData) {}

    /// Called when a reaction is added to a message.
    async fn message_reaction_add(&self, _ctx: Context, _data: ReactionData) {}

    /// Called when a reaction is removed from a message.
    async fn message_reaction_remove(&self, _ctx: Context, _data: ReactionData) {}

    /// Called when all reactions are removed from a message.
    async fn message_reaction_remove_all(
        &self,
        _ctx: Context,
        _data: ReactionRemoveAllData,
    ) {
    }

    /// Called when all reactions for a single emoji are removed from a message.
    async fn message_reaction_remove_emoji(
        &self,
        _ctx: Context,
        _data: ReactionRemoveEmojiData,
    ) {
    }

    /// Called when a user joins a guild the bot is in.
    async fn guild_member_add(&self, _ctx: Context, _data: GuildMemberAddData) {}

    /// Called when a user leaves or is removed from a guild.
    async fn guild_member_remove(&self, _ctx: Context, _data: GuildMemberRemoveData) {}

    /// Called when a user is banned from a guild.
    async fn guild_ban_add(&self, _ctx: Context, _data: GuildBanData) {}

    /// Called when a user's ban is removed from a guild.
    async fn guild_ban_remove(&self, _ctx: Context, _data: GuildBanData) {}

    /// Called when a role is created in a guild.
    async fn guild_role_create(&self, _ctx: Context, _data: GuildRoleData) {}

    /// Called when a role is updated in a guild.
    async fn guild_role_update(&self, _ctx: Context, _data: GuildRoleData) {}

    /// Called when a role is deleted from a guild.
    async fn guild_role_delete(&self, _ctx: Context, _data: GuildRoleDeleteData) {}

    /// Called when a channel is created.
    async fn channel_create(&self, _ctx: Context, _channel: Channel) {}

    /// Called when a channel is updated.
    async fn channel_update(&self, _ctx: Context, _channel: Channel) {}

    /// Called when a channel is deleted.
    async fn channel_delete(&self, _ctx: Context, _channel: Channel) {}

    /// Called when a user starts typing in a channel.
    async fn typing_start(&self, _ctx: Context, _data: TypingStartData) {}

    /// Called when a user's voice state changes in any guild the bot is in.
    async fn voice_state_update(&self, _ctx: Context, _state: VoiceStateUpdateData) {}

    /// Called when Discord provides voice server connection details.
    ///
    /// Typically triggered after calling [`GatewayHandle::join_voice`]. Use
    /// the `endpoint` and `token` (together with the `session_id` from the
    /// matching [`Self::voice_state_update`] event) to connect via
    /// [`VoiceConnection::connect`](oxidian_voice::connection::VoiceConnection::connect).
    async fn voice_server_update(&self, _ctx: Context, _server: VoiceServerUpdateData) {
    }

    /// Called when a guild member's properties change (roles, nickname, avatar, timeout, etc.).
    async fn guild_member_update(&self, _ctx: Context, _data: GuildMemberUpdateData) {}

    /// Called when a user's presence (status or activities) changes in a guild.
    async fn presence_update(&self, _ctx: Context, _data: PresenceUpdateData) {}

    /// Called when the last-pinned message in a channel changes.
    async fn channel_pins_update(&self, _ctx: Context, _data: ChannelPinsUpdateData) {}

    /// Called when a thread is created or the bot gains access to a thread.
    async fn thread_create(&self, _ctx: Context, _thread: Thread) {}

    /// Called when a thread is updated.
    async fn thread_update(&self, _ctx: Context, _thread: Thread) {}

    /// Called when a thread is deleted.
    async fn thread_delete(&self, _ctx: Context, _data: ThreadDeleteData) {}

    /// Called with the full snapshot of threads for channels the bot gains access to.
    async fn thread_list_sync(&self, _ctx: Context, _data: ThreadListSyncData) {}

    /// Called when members are added or removed from a thread.
    async fn thread_members_update(
        &self,
        _ctx: Context,
        _data: ThreadMembersUpdateData,
    ) {
    }

    /// Called when a Stage instance is created.
    async fn stage_instance_create(&self, _ctx: Context, _stage: StageInstance) {}

    /// Called when a Stage instance is updated.
    async fn stage_instance_update(&self, _ctx: Context, _stage: StageInstance) {}

    /// Called when a Stage instance ends and is deleted.
    async fn stage_instance_delete(&self, _ctx: Context, _stage: StageInstance) {}

    /// Called when a guild scheduled event is created.
    async fn guild_scheduled_event_create(
        &self,
        _ctx: Context,
        _event: ScheduledEvent,
    ) {
    }

    /// Called when a guild scheduled event is updated.
    async fn guild_scheduled_event_update(
        &self,
        _ctx: Context,
        _event: ScheduledEvent,
    ) {
    }

    /// Called when a guild scheduled event is deleted or cancelled.
    async fn guild_scheduled_event_delete(
        &self,
        _ctx: Context,
        _event: ScheduledEvent,
    ) {
    }

    /// Called when a user subscribes to a guild scheduled event.
    async fn guild_scheduled_event_user_add(
        &self,
        _ctx: Context,
        _data: ScheduledEventUserData,
    ) {
    }

    /// Called when a user unsubscribes from a guild scheduled event.
    async fn guild_scheduled_event_user_remove(
        &self,
        _ctx: Context,
        _data: ScheduledEventUserData,
    ) {
    }

    /// Called when an auto-moderation rule is created.
    async fn auto_moderation_rule_create(
        &self,
        _ctx: Context,
        _rule: AutoModerationRule,
    ) {
    }

    /// Called when an auto-moderation rule is updated.
    async fn auto_moderation_rule_update(
        &self,
        _ctx: Context,
        _rule: AutoModerationRule,
    ) {
    }

    /// Called when an auto-moderation rule is deleted.
    async fn auto_moderation_rule_delete(
        &self,
        _ctx: Context,
        _rule: AutoModerationRule,
    ) {
    }

    /// Called when an auto-moderation rule is triggered and an action is executed.
    async fn auto_moderation_action_execution(
        &self,
        _ctx: Context,
        _data: AutoModerationActionExecutionData,
    ) {
    }

    /// Called when a user adds a vote to a poll.
    async fn poll_vote_add(&self, _ctx: Context, _data: PollVoteData) {}

    /// Called when a user removes their vote from a poll.
    async fn poll_vote_remove(&self, _ctx: Context, _data: PollVoteData) {}

    /// Called when a soundboard sound is created in a guild.
    async fn guild_soundboard_sound_create(
        &self,
        _ctx: Context,
        _sound: SoundboardSound,
    ) {
    }

    /// Called when a soundboard sound in a guild is updated.
    async fn guild_soundboard_sound_update(
        &self,
        _ctx: Context,
        _sound: SoundboardSound,
    ) {
    }

    /// Called when a soundboard sound is deleted from a guild.
    async fn guild_soundboard_sound_delete(
        &self,
        _ctx: Context,
        _data: SoundboardSoundDeleteData,
    ) {
    }

    /// Called when the guild's emoji list is updated.
    async fn guild_emojis_update(&self, _ctx: Context, _data: GuildEmojisUpdateData) {}

    /// Called when the guild's sticker list is updated.
    async fn guild_stickers_update(
        &self,
        _ctx: Context,
        _data: GuildStickersUpdateData,
    ) {
    }

    /// Called when a moderator creates an audit log entry in the guild.
    async fn guild_audit_log_entry_create(&self, _ctx: Context, _entry: AuditLogEntry) {
    }

    /// Called when the guild's integration list changes (no detail; use REST to fetch).
    async fn guild_integrations_update(
        &self,
        _ctx: Context,
        _data: GuildIntegrationsUpdateData,
    ) {
    }

    /// Called when a bot/OAuth2 integration is added to a guild.
    async fn integration_create(&self, _ctx: Context, _integration: Integration) {}

    /// Called when a bot/OAuth2 integration is updated in a guild.
    async fn integration_update(&self, _ctx: Context, _integration: Integration) {}

    /// Called when a bot/OAuth2 integration is removed from a guild.
    async fn integration_delete(&self, _ctx: Context, _data: IntegrationDeleteData) {}

    /// Called when a new invite is created.
    async fn invite_create(&self, _ctx: Context, _data: InviteCreateData) {}

    /// Called when an invite is deleted or expires.
    async fn invite_delete(&self, _ctx: Context, _data: InviteDeleteData) {}

    /// Called with a chunk of guild members in response to an op 8 request.
    async fn guild_members_chunk(&self, _ctx: Context, _data: GuildMembersChunkData) {}

    /// Called when the current user (bot) updates their own profile.
    async fn user_update(&self, _ctx: Context, _user: User) {}

    /// Called when the bot's own thread-member record is updated.
    async fn thread_member_update(&self, _ctx: Context, _data: ThreadMemberUpdateData) {
    }

    /// Called when a webhook in a channel is created, updated, or deleted.
    async fn webhooks_update(&self, _ctx: Context, _data: WebhooksUpdateData) {}

    /// Called when a user sends a soundboard sound or emoji effect in a voice channel.
    async fn voice_channel_effect_send(
        &self,
        _ctx: Context,
        _effect: VoiceChannelEffect,
    ) {
    }

    /// Called when permission overrides for an application command change.
    async fn application_command_permissions_update(
        &self,
        _ctx: Context,
        _data: ApplicationCommandPermissionsUpdateData,
    ) {
    }

    /// Called with a batch update of all soundboard sounds in a guild.
    async fn guild_soundboard_sounds_update(
        &self,
        _ctx: Context,
        _data: GuildSoundboardSoundsUpdateData,
    ) {
    }

    /// Called when a user starts a premium app subscription.
    async fn subscription_create(&self, _ctx: Context, _subscription: Subscription) {}

    /// Called when a premium app subscription is updated (e.g. renewal).
    async fn subscription_update(&self, _ctx: Context, _subscription: Subscription) {}

    /// Called when a premium app subscription is cancelled or expires.
    async fn subscription_delete(&self, _ctx: Context, _subscription: Subscription) {}

    /// Called for every event that doesn't have a dedicated handler method.
    /// Useful for logging or handling less-common event types.
    async fn raw_event(&self, _ctx: Context, _event: DispatchEvent) {}
}

/// No-op handler used as the default when none is provided to [`BotBuilder`].
pub(crate) struct DefaultHandler;

#[async_trait]
impl EventHandler for DefaultHandler {}
