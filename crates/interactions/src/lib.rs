//! Discord interaction types for slash commands, components, and modals.
//!
//! # Slash commands
//!
//! Build and register commands with [`command::SlashCommandBuilder`]:
//!
//! ```rust,ignore
//! use oxidian_interactions::command::{SlashCommandBuilder, CommandOptionBuilder};
//! use oxidian_core::models::interaction::CommandOptionType;
//!
//! let cmd = SlashCommandBuilder::new("ping", "Reply with pong!")
//!     .option(
//!         CommandOptionBuilder::new(CommandOptionType::String, "message", "Optional message")
//!             .build(),
//!     )
//!     .build();
//! ```

pub mod autocomplete;
pub mod command;
pub mod components;
pub mod context_menu;

pub use autocomplete::AutocompleteResponse;
pub use command::{
    ApplicationCommand, CommandChoice, CommandOption, CommandOptionBuilder,
    SlashCommandBuilder,
};
pub use context_menu::{MessageCommand, UserCommand};
