// MIT License
//
// Copyright (c) 2026 Ferriteworks organization and its rightful owners.
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

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

// Re-export commonly used component types at the crate root for convenience.
pub use components::{
    ActionRow, Button, ButtonStyle, StringSelect, TextInput, TextInputStyle,
};
pub use components::{Container, Section, Separator, TextDisplay};
