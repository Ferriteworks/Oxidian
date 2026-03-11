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

// Example: sending a Components V2 message with layout primitives.
//!
//! Registers a `/demo` slash command that responds with a Container holding
//! a TextDisplay, Separator, Section with Thumbnail, and a MediaGallery.
//!
//! # Running
//!
//! ```sh
//! DISCORD_TOKEN=<token> cargo run --example components_v2
//! ```

use async_trait::async_trait;
use oxidian::core::models::interaction::{Interaction, InteractionResponse};
use oxidian::gateway::events::ReadyData;
use oxidian::interactions::command::SlashCommandBuilder;
use oxidian::interactions::components::{
    Container, MediaGallery, MediaGalleryItem, Section, Separator, TextDisplay, Thumbnail,
};
use oxidian::{Bot, Context, CreateMessage, EventHandler, Intents};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: ReadyData) {
        println!("Logged in as {}", ready.user.username);

        let cmds = vec![
            SlashCommandBuilder::new("demo", "Components V2 demo").build(),
        ];
        ctx.http
            .bulk_overwrite_global_commands(ready.application.id, &cmds)
            .await
            .expect("failed to sync commands");
        println!("Slash commands registered.");
    }

    async fn interaction(&self, ctx: Context, interaction: Interaction) {
        if let Some(data) = interaction.command_data() {
            if data.name == "demo" {
                // Defer so we can send with the IS_COMPONENTS_V2 flag.
                ctx.respond(&interaction, InteractionResponse::defer())
                    .await
                    .ok();

                let header = TextDisplay::new("# Components V2 Demo\nBuilt with Oxidian.");

                let section = Section::new()
                    .text(TextDisplay::new("**Oxidian** — a Rust Discord library."))
                    .accessory(Thumbnail::new(
                        "https://cdn.discordapp.com/embed/avatars/0.png",
                    ));

                let gallery = MediaGallery::new()
                    .item(MediaGalleryItem::new(
                        "https://cdn.discordapp.com/embed/avatars/1.png",
                    ))
                    .item(MediaGalleryItem::new(
                        "https://cdn.discordapp.com/embed/avatars/2.png",
                    ));

                let container = Container::new()
                    .accent_color(0x5865F2)
                    .component(header.into_value())
                    .component(Separator::new().into_value())
                    .component(section.into_value())
                    .component(gallery.into_value());

                let msg = CreateMessage::new().components_v2().component(container);

                ctx.edit_response(&interaction, msg).await.ok();
            }
        }
    }
}

#[tokio::main]
async fn main() {
    oxidian::init_logging();

    let token = std::env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN not set");

    Bot::builder(token)
        .intents(Intents::GUILDS)
        .handler(Handler)
        .build()
        .start()
        .await
        .expect("bot exited with an error");
}
