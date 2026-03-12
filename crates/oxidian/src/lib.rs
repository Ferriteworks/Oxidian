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

pub use oxidian_core as core;
pub use oxidian_gateway as gateway;
pub use oxidian_http as http;
pub use oxidian_interactions as interactions;
pub use oxidian_voice as voice;

/// The in-memory cache crate, available when the `cache` feature is enabled.
#[cfg(feature = "cache")]
pub use oxidian_cache as cache;

/// Re-export [`Cache`] at crate root for convenience.
#[cfg(feature = "cache")]
pub use oxidian_cache::Cache;

/// Re-export the central error type for convenience.
pub use oxidian_core::error::{Error, Result};

/// Re-export [`Shard`] at crate root for users who want direct gateway access.
pub use oxidian_gateway::Shard;

/// Re-export [`ShardInfo`] for multi-shard deployments.
pub use oxidian_gateway::ShardInfo;

/// Re-export [`DispatchEvent`] for low-level event loop usage.
pub use oxidian_gateway::events::DispatchEvent;

/// Re-export [`HttpClient`] at crate root.
pub use oxidian_http::HttpClient;

/// Re-export [`Snowflake`] at crate root.
pub use oxidian_core::snowflake::Snowflake;

/// Re-export [`Intents`] at crate root.
pub use oxidian_core::intents::Intents;

/// Re-export [`VoiceConnection`] at crate root.
pub use oxidian_voice::VoiceConnection;

/// High-level [`Bot`] entry point.
pub mod bot;
/// Prefix command registry.
pub mod command;
/// Request context passed to event handlers and command callbacks.
pub mod context;
/// [`EventHandler`] trait for receiving gateway events.
pub mod handler;

pub use bot::{dispatch, Bot, BotBuilder, ShardCount};
pub use command::{Command, CommandRegistry, Module};
pub use context::{Context, GatewayHandle};
pub use handler::EventHandler;

/// Re-export [`ApplicationCommand`] at crate root for use in [`Module::slash_commands`].
pub use oxidian_interactions::command::ApplicationCommand;
/// Re-export context-menu command types for [`Module::user_commands`] / [`Module::message_commands`].
pub use oxidian_interactions::context_menu::{MessageCommand, UserCommand};

/// Re-export [`Embed`] and [`EmbedBuilder`] at crate root.
pub use oxidian_core::models::embed::{Embed, EmbedBuilder};
/// Re-export [`CreateMessage`] at crate root.
pub use oxidian_core::models::message::CreateMessage;

/// Initialise Oxidian: installs the rustls crypto provider (ring) and sets up
/// the `tracing` subscriber from the `RUST_LOG` environment variable.
///
/// Call this **once** at the very start of `main`, before creating a [`Shard`].
pub fn init_logging() {
    // Install the ring crypto provider for rustls so TLS connections work.
    // `.ok()` ignores the error if it was already installed (e.g. in tests).
    rustls::crypto::ring::default_provider()
        .install_default()
        .ok();

    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();
}
