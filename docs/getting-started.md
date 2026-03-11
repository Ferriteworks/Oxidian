# Getting Started

This guide walks you through setting up a basic Discord bot with Oxidian from scratch. By the end you'll have a bot online that responds to a `!ping` prefix command and a `/ping` slash command.

## Prerequisites

- Rust 1.75 or later — install via [rustup](https://rustup.rs)
- A Discord application with a bot user and a token (create one at the [Discord Developer Portal](https://discord.com/developers/applications))
- The bot must be invited to a server so you can test it

## Create a new project

```sh
cargo new my-bot
cd my-bot
```

Add Oxidian and the other required crates to `Cargo.toml`:

```toml
[dependencies]
oxidian     = { git = "https://github.com/Ferriteworks/Oxidian" }
tokio       = { version = "1", features = ["full"] }
async-trait = "0.1"
tracing     = "0.1"
```

## Write the bot

### `src/main.rs`

```rust
mod commands;

use async_trait::async_trait;
use tracing::info;

use oxidian::{Bot, Context, EventHandler, Intents};
use oxidian::gateway::events::ReadyData;

struct MyHandler;

#[async_trait]
impl EventHandler for MyHandler {
    async fn ready(&self, _ctx: Context, ready: ReadyData) {
        info!("logged in as {}!", ready.user.username);
    }
}

#[tokio::main]
async fn main() {
    oxidian::init_logging();

    let token = std::env::var("DISCORD_TOKEN")
        .expect("DISCORD_TOKEN must be set");

    Bot::builder(token)
        .intents(Intents::GUILDS | Intents::GUILD_MESSAGES | Intents::MESSAGE_CONTENT)
        .prefix("!")
        .handler(MyHandler)
        .module(commands::FunModule)
        .build()
        .start()
        .await
        .expect("bot crashed");
}
```

### `src/commands/mod.rs`

```rust
use async_trait::async_trait;
use oxidian::{command::{Command, Module}, ApplicationCommand, Context};
use oxidian::core::models::message::Message;
use oxidian::core::models::interaction::{Interaction, InteractionResponse};
use oxidian::interactions::command::SlashCommandBuilder;

pub struct FunModule;

#[async_trait]
impl Module for FunModule {
    fn commands(&self) -> Vec<Command> {
        vec![
            Command::new("ping", |ctx: Context, msg: Message, _args| async move {
                ctx.reply(&msg, "pong!").await?;
                Ok(())
            }),
        ]
    }

    fn slash_commands(&self) -> Vec<ApplicationCommand> {
        vec![SlashCommandBuilder::new("ping", "Replies with pong!").build()]
    }

    async fn handle_interaction(&self, ctx: Context, interaction: Interaction) {
        ctx.respond(&interaction, InteractionResponse::message("pong! 🏓"))
            .await
            .ok();
    }
}
```

## Run it

```sh
DISCORD_TOKEN=your_token_here cargo run
```

You should see `logged in as YourBot#0000!` in the console.

- Send `!ping` in any channel the bot can see → it replies `pong!` (prefix command).
- Use `/ping` after you've synced slash commands (see below) → it replies `pong! 🏓`.

## Syncing slash commands

Discord only shows `/ping` in the command list after you register it. Add a sync call before `bot.start()`:

```rust
let bot = Bot::builder(token) /* ... */ .build();

// Sync to a specific guild instantly during development
let guild_id = Snowflake::from(YOUR_GUILD_ID);
bot.http
    .bulk_overwrite_guild_commands(guild_id, &[
        SlashCommandBuilder::new("ping", "Replies with pong!").build(),
    ])
    .await
    .expect("failed to sync commands");

bot.start().await.expect("bot crashed");
```

Use `bulk_overwrite_global_commands` for production (takes up to an hour to propagate everywhere).

## What's happening

- `Intents` is a bitflags type — combine constants with `|`. `MESSAGE_CONTENT` is a privileged intent that must be enabled in the Developer Portal under your bot settings.
- `.module(FunModule)` registers all of `FunModule`'s prefix commands and stores it for slash routing.
- When Discord sends an `INTERACTION_CREATE` event, Oxidian matches the command name against each module's `slash_commands()` list and calls `handle_interaction` on the matching module.
- `init_logging()` reads `RUST_LOG`. Set `RUST_LOG=debug` to see every gateway payload.

