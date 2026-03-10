# Getting Started

This guide walks you through setting up a basic Discord bot with Oxidian from scratch. By the end you'll have a bot online that responds to a `!ping` command.

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
oxidian    = { git = "https://github.com/KilledInAction/Oxidian" }
tokio      = { version = "1", features = ["full"] }
async-trait = "0.1"
tracing    = "0.1"
```

## Write the bot

Replace `src/main.rs` with:

```rust
mod commands;

use async_trait::async_trait;
use tracing::info;

use oxidian::{Bot, Context, EventHandler};
use oxidian::core::models::message::Message;
use oxidian::gateway::events::ReadyData;

// GUILDS + GUILD_MESSAGES + MESSAGE_CONTENT
const INTENTS: u64 = (1 << 0) | (1 << 9) | (1 << 15);

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
        .intents(INTENTS)
        .prefix("!")
        .handler(MyHandler)
        .register_module(commands::ping::command())
        .build()
        .start()
        .await
        .expect("bot crashed");
}
```

Then create `src/commands/mod.rs`:

```rust
pub mod ping;
```

And `src/commands/ping.rs`:

```rust
use oxidian::{command::Command, Context};
use oxidian::core::models::message::Message;

pub fn command() -> Command {
    Command::new("ping", |ctx: Context, msg: Message, _args| async move {
        ctx.reply(&msg, "pong!").await?;
        Ok(())
    })
}
```

## Run it

```sh
DISCORD_TOKEN=your_token_here cargo run
```

You should see `logged in as YourBot#0000!` in the console. Send `!ping` in any channel the bot can see and it will reply with `pong!`.

## What's happening

- `Bot::builder` sets up the bot.
- `.intents()` tells Discord which events to send. `MESSAGE_CONTENT` (bit 15) is a privileged intent — you need to enable it in the Developer Portal under your bot's settings.
- `.prefix("!")` means any message starting with `!` gets checked against registered commands.
- `.register_module()` loads a command from its own file — see [Commands](./commands.md) for more detail.
- `EventHandler` is the trait you implement to react to gateway events like `READY`, `MESSAGE_CREATE`, or `GUILD_CREATE`.
- `init_logging()` reads the `RUST_LOG` environment variable. Set `RUST_LOG=debug` if you want to see every gateway payload.
