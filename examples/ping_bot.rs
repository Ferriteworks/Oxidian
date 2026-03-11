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

// Example: a simple ping bot that responds to `!ping` with "Pong!".
//!
//! # Running
//!
//! ```sh
//! DISCORD_TOKEN=<token> cargo run --example ping_bot
//! ```

use async_trait::async_trait;
use oxidian::core::models::message::Message;
use oxidian::{Bot, Context, EventHandler, Intents};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, ready: oxidian::gateway::events::ReadyData) {
        println!("Logged in as {}", ready.user.username);
    }

    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot {
            return;
        }
        if msg.content == "!ping" {
            ctx.reply(&msg, "Pong! 🏓").await.ok();
        }
    }
}

#[tokio::main]
async fn main() {
    oxidian::init_logging();

    let token = std::env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN not set");

    Bot::builder(token)
        .intents(
            Intents::GUILDS
                .union(Intents::GUILD_MESSAGES)
                .union(Intents::MESSAGE_CONTENT),
        )
        .handler(Handler)
        .build()
        .start()
        .await
        .expect("bot exited with an error");
}
