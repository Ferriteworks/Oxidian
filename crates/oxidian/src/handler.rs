use async_trait::async_trait;

use oxidian_core::models::{guild::Guild, interaction::Interaction, message::Message};
use oxidian_gateway::events::{DispatchEvent, MessageDeleteData, ReadyData, VoiceServerUpdateData, VoiceStateUpdateData};

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

    /// Called when a user's voice state changes in any guild the bot is in.
    async fn voice_state_update(&self, _ctx: Context, _state: VoiceStateUpdateData) {}

    /// Called when Discord provides voice server connection details.
    ///
    /// Typically triggered after calling [`GatewayHandle::join_voice`]. Use
    /// the `endpoint` and `token` (together with the `session_id` from the
    /// matching [`Self::voice_state_update`] event) to connect via
    /// [`VoiceConnection::connect`](oxidian_voice::connection::VoiceConnection::connect).
    async fn voice_server_update(&self, _ctx: Context, _server: VoiceServerUpdateData) {}

    /// Called for every event that doesn't have a dedicated handler method.
    /// Useful for logging or handling less-common event types.
    async fn raw_event(&self, _ctx: Context, _event: DispatchEvent) {}
}

/// No-op handler used as the default when none is provided to [`BotBuilder`].
pub(crate) struct DefaultHandler;

#[async_trait]
impl EventHandler for DefaultHandler {}
