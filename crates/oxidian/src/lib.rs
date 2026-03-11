pub use oxidian_core as core;
pub use oxidian_gateway as gateway;
pub use oxidian_http as http;
pub use oxidian_interactions as interactions;
pub use oxidian_voice as voice;

/// Re-export the central error type for convenience.
pub use oxidian_core::error::{Error, Result};

/// Re-export [`Shard`] at crate root for users who want direct gateway access.
pub use oxidian_gateway::Shard;

/// Re-export [`HttpClient`] at crate root.
pub use oxidian_http::HttpClient;

/// Re-export [`Snowflake`] at crate root.
pub use oxidian_core::snowflake::Snowflake;

/// Re-export [`Intents`] at crate root.
pub use oxidian_core::intents::Intents;

/// Re-export [`VoiceConnection`] at crate root.
pub use oxidian_voice::VoiceConnection;

/// Request context passed to event handlers and command callbacks.
pub mod context;
/// [`EventHandler`] trait for receiving gateway events.
pub mod handler;
/// Prefix command registry.
pub mod command;
/// High-level [`Bot`] entry point.
pub mod bot;

pub use bot::{Bot, BotBuilder};
pub use command::{Command, Module};
/// Re-export [`ApplicationCommand`] at crate root for use in [`Module::slash_commands`].
pub use oxidian_interactions::command::ApplicationCommand;
pub use context::{Context, GatewayHandle};
pub use handler::EventHandler;

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
