use std::{collections::HashMap, future::Future, pin::Pin, sync::Arc};

use oxidian_core::{error::Result, models::message::Message};

use crate::context::Context;

/// A pinned, boxed future returned by a command handler.
pub type BoxFuture = Pin<Box<dyn Future<Output = Result<()>> + Send>>;

/// A type-erased, cheaply cloneable command handler function.
pub type CommandFn = Arc<dyn Fn(Context, Message, Vec<String>) -> BoxFuture + Send + Sync>;

// ── Command ───────────────────────────────────────────────────────────────────

/// A self-contained command definition — a name paired with its handler.
///
/// Create one with [`Command::new`] and register it on the bot with
/// [`BotBuilder::register_module`]. The typical pattern is to define one
/// command per file and expose it via a `pub fn command() -> Command` function:
///
/// ```rust,ignore
/// // commands/ping.rs
/// use oxidian::command::Command;
///
/// pub fn command() -> Command {
///     Command::new("ping", |ctx, msg, _args| async move {
///         ctx.reply(&msg, "pong!").await?;
///         Ok(())
///     })
/// }
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

// ── CommandRegistry ───────────────────────────────────────────────────────────

/// Registry mapping command names to their handler functions.
///
/// Populated via [`BotBuilder::command`] / [`BotBuilder::register_module`] and
/// queried by the event dispatch loop when a message matches the configured prefix.
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

    /// Look up a registered command by name. Returns `None` if not found.
    pub fn get(&self, name: &str) -> Option<CommandFn> {
        self.commands.get(name).cloned()
    }
}
