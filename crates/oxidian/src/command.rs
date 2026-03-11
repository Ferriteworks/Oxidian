use std::{collections::HashMap, future::Future, pin::Pin, sync::Arc};

use oxidian_core::{error::Result, models::{interaction::Interaction, message::Message}};
use oxidian_interactions::command::ApplicationCommand;

use crate::context::Context;

/// A pinned, boxed future returned by a command handler.
pub type BoxFuture = Pin<Box<dyn Future<Output = Result<()>> + Send>>;

/// A type-erased, cheaply cloneable command handler function.
pub type CommandFn = Arc<dyn Fn(Context, Message, Vec<String>) -> BoxFuture + Send + Sync>;

/// A self-contained command definition — a name paired with its handler.
///
/// Create one with [`Command::new`] and register it on the bot with
/// [`BotBuilder::register_module`] or inside a [`Module`] implementation.
///
/// ```rust,ignore
/// use oxidian::command::Command;
///
/// let cmd = Command::new("ping", |ctx, msg, _args| async move {
///     ctx.reply(&msg, "pong!").await?;
///     Ok(())
/// });
/// ```
pub struct Command {
    pub(crate) name: String,
    pub(crate) handler: CommandFn,
}

impl Command {
    /// Create a new `Command` with the given name and async handler function.
    ///
    /// `name` should not include the prefix character — just the bare word
    /// (e.g. `"ping"`, not `"!ping"`).
    pub fn new<F, Fut>(name: impl Into<String>, f: F) -> Self
    where
        F: Fn(Context, Message, Vec<String>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        let f = Arc::new(f);
        let handler: CommandFn =
            Arc::new(move |ctx, msg, args| Box::pin(f(ctx, msg, args)) as BoxFuture);
        Self { name: name.into(), handler }
    }
}

/// A collection of related commands and slash commands grouped under a single
/// struct — the Rust equivalent of a Python discord.py Cog.
///
/// Register a module with [`BotBuilder::module`]. The bot will:
/// - Add all prefix commands from [`commands`](Self::commands) to the registry.
/// - Route `INTERACTION_CREATE` events to [`handle_interaction`](Self::handle_interaction)
///   when the command name matches one of the names in [`slash_commands`](Self::slash_commands).
///
/// All methods have default no-op implementations so you only implement what you need.
///
/// # Example
///
/// ```rust,ignore
/// use async_trait::async_trait;
/// use oxidian::{command::{Command, Module}, Context, ApplicationCommand};
/// use oxidian::core::models::interaction::{Interaction, InteractionResponse};
/// use oxidian::interactions::command::SlashCommandBuilder;
///
/// struct FunModule;
///
/// #[async_trait]
/// impl Module for FunModule {
///     fn commands(&self) -> Vec<Command> {
///         vec![
///             Command::new("ping", |ctx, msg, _args| async move {
///                 ctx.reply(&msg, "pong!").await?;
///                 Ok(())
///             }),
///         ]
///     }
///
///     fn slash_commands(&self) -> Vec<ApplicationCommand> {
///         vec![SlashCommandBuilder::new("ping", "Replies with pong!").build()]
///     }
///
///     async fn handle_interaction(&self, ctx: Context, interaction: Interaction) {
///         if let Some(data) = interaction.command_data() {
///             if data.name == "ping" {
///                 ctx.respond(&interaction, InteractionResponse::message("pong! 🏓"))
///                     .await.ok();
///             }
///         }
///     }
/// }
/// ```
#[async_trait::async_trait]
pub trait Module: Send + Sync + 'static {
    /// Prefix commands this module provides.
    fn commands(&self) -> Vec<Command> { vec![] }

    /// Slash command definitions this module provides.
    ///
    /// Returned command names are used to route `INTERACTION_CREATE` events to
    /// [`handle_interaction`](Self::handle_interaction). Use
    /// `http.bulk_overwrite_global_commands()` or
    /// `http.bulk_overwrite_guild_commands()` to sync these with Discord.
    fn slash_commands(&self) -> Vec<ApplicationCommand> { vec![] }

    /// Called when a slash command interaction arrives whose name matches one
    /// of the names returned by [`slash_commands`](Self::slash_commands).
    async fn handle_interaction(&self, _ctx: Context, _interaction: Interaction) {}
}

/// Registry mapping command names to their handler functions.
///
/// Populated via [`BotBuilder::command`], [`BotBuilder::module`], or
/// [`BotBuilder::register_module`] and queried by the event dispatch loop when
/// a message matches the configured prefix.
#[derive(Default, Clone)]
pub struct CommandRegistry {
    commands: HashMap<String, CommandFn>,
}

impl CommandRegistry {
    /// Create an empty registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a command handler under `name` directly.
    pub fn register<F, Fut>(&mut self, name: impl Into<String>, f: F)
    where
        F: Fn(Context, Message, Vec<String>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        let f = Arc::new(f);
        let handler: CommandFn =
            Arc::new(move |ctx, msg, args| Box::pin(f(ctx, msg, args)) as BoxFuture);
        self.commands.insert(name.into(), handler);
    }

    /// Register a pre-built [`Command`] descriptor.
    pub fn add(&mut self, cmd: Command) {
        self.commands.insert(cmd.name, cmd.handler);
    }

    /// Register all **prefix** commands from a [`Module`].
    ///
    /// Note: slash command routing requires registering the module via
    /// [`BotBuilder::module`] instead, which stores the module for
    /// interaction dispatch as well.
    pub fn add_module(&mut self, module: &impl Module) {
        for cmd in module.commands() {
            self.add(cmd);
        }
    }

    /// Look up a registered command by name. Returns `None` if not found.
    pub fn get(&self, name: &str) -> Option<CommandFn> {
        self.commands.get(name).cloned()
    }
}
