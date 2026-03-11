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

//! Multi-shard example using the high-level `Bot` builder.
//!
//! This demonstrates both `.shards(n)` (fixed count) and `.auto_shards()`
//! (let Discord recommend the shard count based on guild count).
//!
//! # Running
//!
//! ```sh
//! DISCORD_TOKEN=<token> cargo run --example multi_shard
//! ```

use async_trait::async_trait;
use oxidian::core::models::message::Message;
use oxidian::gateway::events::ReadyData;
use oxidian::{Bot, Context, EventHandler, Intents};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, ready: ReadyData) {
        println!("Shard is ready! Logged in as {}", ready.user.username);
    }

    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot {
            return;
        }
        if msg.content == "!ping" {
            ctx.reply(&msg, "Pong! 🏓 (multi-shard)").await.ok();
        }
    }
}

#[tokio::main]
async fn main() {
    oxidian::init_logging();

    let token = std::env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN not set");

    // Option A: Fixed shard count.
    // Bot::builder(&token)
    //     .intents(Intents::GUILDS | Intents::GUILD_MESSAGES | Intents::MESSAGE_CONTENT)
    //     .shards(2)
    //     .handler(Handler)
    //     .build()
    //     .start()
    //     .await
    //     .expect("bot error");

    // Option B: Let Discord pick the shard count based on guild count.
    Bot::builder(token)
        .intents(Intents::GUILDS | Intents::GUILD_MESSAGES | Intents::MESSAGE_CONTENT)
        .auto_shards()
        .handler(Handler)
        .build()
        .start()
        .await
        .expect("bot error");
}
