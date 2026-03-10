use std::{future::Future, sync::Arc};

use tokio::sync::mpsc;
use tracing::{debug, error};

use oxidian_core::{
    error::Result,
    models::message::Message,
};
use oxidian_gateway::{events::DispatchEvent, Shard};
use oxidian_http::HttpClient;

use crate::{
    command::CommandRegistry,
    context::Context,
    handler::{DefaultHandler, EventHandler},
};

// ── Bot ───────────────────────────────────────────────────────────────────────

/// The top-level Discord bot, combining the gateway shard, HTTP client,
/// prefix command dispatcher, and event handler.
///
/// Create one with [`Bot::builder`] and call [`Bot::start`] to connect.
///
/// # Example
///
/// ```rust,ignore
/// Bot::builder(token)
///     .intents(INTENTS)
///     .prefix("!")
///     .handler(MyHandler)
///     .command("ping", |ctx, msg, _args| async move {
///         ctx.reply(&msg, "pong!").await?;
///         Ok(())
///     })
///     .build()
///     .start()
///     .await?;
/// ```
pub struct Bot {
    token: String,
    intents: u64,
    prefix: Option<String>,
    handler: Arc<dyn EventHandler>,
    commands: Arc<CommandRegistry>,
}

impl Bot {
    /// Begin building a [`Bot`].
    pub fn builder(token: impl Into<String>) -> BotBuilder {
        BotBuilder::new(token)
    }

    /// Connect to the Discord gateway and process events until the connection
    /// closes or the retry limit is exceeded.
    pub async fn start(self) -> Result<()> {
        let http = Arc::new(HttpClient::new(&self.token)?);
        let ctx = Context::new(Arc::clone(&http));

        let (event_tx, mut event_rx) = mpsc::channel::<DispatchEvent>(256);
        let shard = Shard::new(self.token.clone(), self.intents, event_tx);

        // Drive the gateway on a separate task.
        tokio::spawn(async move {
            if let Err(e) = shard.start().await {
                error!(error = %e, "gateway shard exited with an error");
            }
        });

        // Dispatch events until the shard drops its sender side.
        while let Some(event) = event_rx.recv().await {
            let ctx = ctx.clone();
            let handler = Arc::clone(&self.handler);
            let commands = Arc::clone(&self.commands);
            let prefix = self.prefix.clone();

            tokio::spawn(async move {
                if let Err(e) = dispatch(event, ctx, handler, commands, prefix).await {
                    error!(error = %e, "event dispatch error");
                }
            });
        }

        Ok(())
    }
}

// ── BotBuilder ────────────────────────────────────────────────────────────────

/// Builder for [`Bot`].  Obtain one via [`Bot::builder`].
pub struct BotBuilder {
    token: String,
    intents: u64,
    prefix: Option<String>,
    handler: Option<Arc<dyn EventHandler>>,
    commands: CommandRegistry,
}

impl BotBuilder {
    /// Create a builder for the given bot token.
    pub fn new(token: impl Into<String>) -> Self {
        Self {
            token: token.into(),
            intents: 0,
            prefix: None,
            handler: None,
            commands: CommandRegistry::new(),
        }
    }

    /// Set the [Gateway Intents](https://discord.com/developers/docs/topics/gateway#gateway-intents) bitmask.
    pub fn intents(mut self, intents: u64) -> Self {
        self.intents = intents;
        self
    }

    /// Set the prefix that triggers registered commands (e.g. `"!"`).
    pub fn prefix(mut self, prefix: impl Into<String>) -> Self {
        self.prefix = Some(prefix.into());
        self
    }

    /// Set the event handler that receives gateway events.
    pub fn handler(mut self, handler: impl EventHandler) -> Self {
        self.handler = Some(Arc::new(handler));
        self
    }

    /// Register a prefix command.
    ///
    /// The `name` should **not** include the prefix character (e.g. `"ping"`,
    /// not `"!ping"`).
    pub fn command<F, Fut>(mut self, name: impl Into<String>, f: F) -> Self
    where
        F: Fn(Context, Message, Vec<String>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.commands.register(name, f);
        self
    }

    /// Consume the builder and produce a [`Bot`].
    pub fn build(self) -> Bot {
        Bot {
            token: self.token,
            intents: self.intents,
            prefix: self.prefix,
            handler: self
                .handler
                .unwrap_or_else(|| Arc::new(DefaultHandler)),
            commands: Arc::new(self.commands),
        }
    }
}

// ── Dispatch ──────────────────────────────────────────────────────────────────

async fn dispatch(
    event: DispatchEvent,
    ctx: Context,
    handler: Arc<dyn EventHandler>,
    commands: Arc<CommandRegistry>,
    prefix: Option<String>,
) -> Result<()> {
    match event {
        DispatchEvent::Ready(data) => {
            handler.ready(ctx, data).await;
        }
        DispatchEvent::MessageCreate(msg) => {
            // Check prefix commands before the generic message handler.
            if let Some(ref pfx) = prefix {
                if msg.content.starts_with(pfx.as_str()) {
                    let after = msg.content[pfx.len()..].trim_start();
                    let mut iter = after.splitn(2, char::is_whitespace);
                    let cmd_name = iter.next().unwrap_or("");
                    let args: Vec<String> = iter
                        .next()
                        .unwrap_or("")
                        .split_whitespace()
                        .map(str::to_owned)
                        .collect();

                    if let Some(cmd_fn) = commands.get(cmd_name) {
                        debug!(command = cmd_name, "dispatching prefix command");
                        cmd_fn(ctx, msg, args).await?;
                        return Ok(());
                    }
                }
            }

            handler.message(ctx, msg).await;
        }
        DispatchEvent::MessageDelete(data) => {
            handler.message_delete(ctx, data).await;
        }
        DispatchEvent::GuildCreate(guild) => {
            handler.guild_create(ctx, guild).await;
        }
        DispatchEvent::GuildUpdate(guild) => {
            handler.guild_update(ctx, guild).await;
        }
        other => {
            handler.raw_event(ctx, other).await;
        }
    }
    Ok(())
}
