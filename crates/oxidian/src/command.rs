use std::{collections::HashMap, future::Future, pin::Pin, sync::Arc};

use oxidian_core::{error::Result, models::message::Message};

use crate::context::Context;

/// A pinned, boxed future returned by a command handler.
pub type BoxFuture = Pin<Box<dyn Future<Output = Result<()>> + Send>>;

/// A type-erased, cheaply cloneable command handler function.
pub type CommandFn = Arc<dyn Fn(Context, Message, Vec<String>) -> BoxFuture + Send + Sync>;

/// Registry mapping command names to their handler functions.
///
/// Populated via [`BotBuilder::command`] and queried by the event dispatch
/// loop when a message matches the configured prefix.
#[derive(Default, Clone)]
pub struct CommandRegistry {
    commands: HashMap<String, CommandFn>,
}

impl CommandRegistry {
    /// Create an empty registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a command handler under `name`.
    ///
    /// `f` receives a [`Context`], the triggering [`Message`], and the
    /// space-separated arguments that follow the command name.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// registry.register("ping", |ctx, msg, _args| async move {
    ///     ctx.reply(&msg, "pong!").await?;
    ///     Ok(())
    /// });
    /// ```
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

    /// Look up a registered command by name. Returns `None` if not found.
    pub fn get(&self, name: &str) -> Option<CommandFn> {
        self.commands.get(name).cloned()
    }
}
