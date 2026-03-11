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

//! Slash commands example.
//!
//! Registers a `/ping` command globally, then listens for interactions.
//!
//! # Running
//!
//! ```sh
//! DISCORD_TOKEN=<token> APPLICATION_ID=<app_id> cargo run --example slash_commands
//! ```

use async_trait::async_trait;
use oxidian::{Bot, Context, EventHandler, Intents, Snowflake};
use oxidian::core::models::interaction::{Interaction, InteractionResponse};
use oxidian::gateway::events::ReadyData;
use oxidian::interactions::command::SlashCommandBuilder;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, ready: ReadyData) {
        println!("Logged in as {}#{}", ready.user.username, ready.user.discriminator);
    }

    async fn interaction(&self, ctx: Context, interaction: Interaction) {
        if let Some(data) = interaction.command_data() {
            match data.name.as_str() {
                "ping" => {
                    ctx.respond(&interaction, InteractionResponse::message("Pong! 🏓"))
                        .await
                        .ok();
                }
                "hello" => {
                    let who = data
                        .options
                        .iter()
                        .find(|o| o.name == "name")
                        .and_then(|o| o.value.as_ref())
                        .and_then(|v| v.as_str())
                        .unwrap_or("world");
                    ctx.respond(
                        &interaction,
                        InteractionResponse::message(format!("Hello, {who}!")),
                    )
                    .await
                    .ok();
                }
                _ => {}
            }
        }
    }
}

#[tokio::main]
async fn main() {
    oxidian::init_logging();

    let token = std::env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN not set");
    let app_id: u64 = std::env::var("APPLICATION_ID")
        .expect("APPLICATION_ID not set")
        .parse()
        .expect("APPLICATION_ID must be a numeric snowflake");

    let http = oxidian::HttpClient::new(&token).expect("failed to create HTTP client");

    // Register /ping globally.
    let ping_cmd = serde_json::to_value(
        SlashCommandBuilder::new("ping", "Reply with pong!").build(),
    )
    .unwrap();
    http.create_global_command(Snowflake::new(app_id), ping_cmd)
        .await
        .expect("failed to register /ping");

    // Register /hello with a required string option.
    use oxidian::interactions::command::{CommandOptionBuilder, CommandOption};
    use oxidian::core::models::interaction::CommandOptionType;
    let hello_opt: CommandOption = CommandOptionBuilder::new(
        CommandOptionType::String,
        "name",
        "Who to greet",
    )
    .required()
    .build();
    let hello_cmd = serde_json::to_value(
        SlashCommandBuilder::new("hello", "Greet someone").option(hello_opt).build(),
    )
    .unwrap();
    http.create_global_command(Snowflake::new(app_id), hello_cmd)
        .await
        .expect("failed to register /hello");

    println!("Slash commands registered. Starting bot...");

    Bot::builder(token)
        .intents(Intents::empty()) // slash commands do not need any gateway intents
        .handler(Handler)
        .build()
        .start()
        .await
        .expect("bot exited with an error");
}
