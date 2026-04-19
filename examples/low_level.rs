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

//! Low-level example: own the event loop without using `Bot`.
//!
//! This demonstrates that every Oxidian crate is usable directly and that the
//! high-level `Bot` is just convenience sugar on top.
//!
//! # Running
//!
//! ```sh
//! DISCORD_TOKEN=<token> cargo run --example low_level
//! ```

use std::sync::Arc;

use oxidian::core::models::interaction::InteractionResponse;
use oxidian::gateway::events::DispatchEvent;
use oxidian::{Context, GatewayHandle, HttpClient, Intents, Shard};

#[tokio::main]
async fn main() {
    oxidian::init_logging();

    let token = std::env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN not set");

    // 1. Create the HTTP client.
    let http = Arc::new(HttpClient::new(&token).expect("failed to create HTTP client"));

    // 2. Create the event channel and shard.
    let (event_tx, mut event_rx) = tokio::sync::mpsc::channel::<DispatchEvent>(256);
    let shard = Shard::new(&token, Intents::GUILDS | Intents::GUILD_MESSAGES, event_tx);

    // 3. Build a Context (HTTP + gateway handle): same one used in Bot.
    let gateway = GatewayHandle::new(shard.gateway_sender());

    #[cfg(feature = "cache")]
    let cache = std::sync::Arc::new(oxidian::Cache::new());

    #[cfg(feature = "cache")]
    let ctx = Context::new(Arc::clone(&http), gateway, cache);

    #[cfg(not(feature = "cache"))]
    let ctx = Context::new(Arc::clone(&http), gateway);

    // 4. Drive the shard on a background task.
    tokio::spawn(async move {
        if let Err(e) = shard.start().await {
            eprintln!("gateway error: {e}");
        }
    });

    // 5. Your own event loop: pattern match on whatever you care about.
    while let Some(event) = event_rx.recv().await {
        let ctx = ctx.clone();
        tokio::spawn(async move {
            match event {
                DispatchEvent::Ready(ready) => {
                    println!("Logged in as {}", ready.user.username);
                }
                DispatchEvent::MessageCreate(msg) => {
                    if !msg.author.bot && msg.content == "!ping" {
                        ctx.reply(&msg, "Pong! 🏓 (low-level)").await.ok();
                    }
                }
                DispatchEvent::InteractionCreate(interaction) => {
                    if let Some(data) = interaction.command_data() {
                        if data.name == "ping" {
                            ctx.respond(
                                &interaction,
                                InteractionResponse::message("Pong! 🏓 (low-level)"),
                            )
                            .await
                            .ok();
                        }
                    }
                }
                _ => {} // handle other events as needed
            }
        });
    }
}
