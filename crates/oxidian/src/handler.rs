use async_trait::async_trait;

use oxidian_core::models::{guild::Guild, message::Message};
use oxidian_gateway::events::{DispatchEvent, MessageDeleteData, ReadyData};

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

    /// Called for every event that doesn't have a dedicated handler method.
    /// Useful for logging or handling less-common event types.
    async fn raw_event(&self, _ctx: Context, _event: DispatchEvent) {}
}

/// No-op handler used as the default when none is provided to [`BotBuilder`].
pub(crate) struct DefaultHandler;

#[async_trait]
impl EventHandler for DefaultHandler {}
