use async_trait::async_trait;

use oxidian_core::models::{
    channel::Channel, guild::Guild, interaction::Interaction, message::Message,
};
use oxidian_gateway::events::{
    DispatchEvent, GuildBanData, GuildMemberAddData, GuildMemberRemoveData,
    GuildRoleData, GuildRoleDeleteData, MessageDeleteBulkData, MessageDeleteData,
    ReactionData, ReactionRemoveAllData, ReactionRemoveEmojiData, ReadyData,
    TypingStartData, VoiceServerUpdateData, VoiceStateUpdateData,
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

    /// Called for every event that doesn't have a dedicated handler method.
    /// Useful for logging or handling less-common event types.
    async fn raw_event(&self, _ctx: Context, _event: DispatchEvent) {}
}

/// No-op handler used as the default when none is provided to [`BotBuilder`].
pub(crate) struct DefaultHandler;

#[async_trait]
impl EventHandler for DefaultHandler {}
