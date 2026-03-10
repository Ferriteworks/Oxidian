# Oxidian

A Discord bot library for Rust. Async, modular, and built on [Tokio](https://tokio.rs).

> **This is early-stage software.** The API will change, things will break, and there are features that aren't implemented yet. That said, the gateway connects, events fire, and prefix commands work. If you're building something and want to use Oxidian, expect to track main closely for now.

## What it does

- Connects to the Discord gateway over WebSocket (with heartbeating, reconnects, and exponential backoff)
- Delivers typed gateway events to your handler (`READY`, `MESSAGE_CREATE`, `GUILD_CREATE`, and more)
- Handles Discord's REST rate limits automatically — per-route buckets, global limits, and 429 retries
- Ships a prefix command system so you can organize commands as individual files

## Quick example

```rust
mod commands;

use async_trait::async_trait;
use oxidian::{Bot, Context, EventHandler};
use oxidian::gateway::events::ReadyData;

// GUILDS | GUILD_MESSAGES | MESSAGE_CONTENT
const INTENTS: u64 = (1 << 0) | (1 << 9) | (1 << 15);

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, ready: ReadyData) {
        println!("online as {}!", ready.user.username);
    }
}

#[tokio::main]
async fn main() {
    oxidian::init_logging();
    let token = std::env::var("DISCORD_TOKEN").unwrap();

    Bot::builder(token)
        .intents(INTENTS)
        .prefix("!")
        .handler(Handler)
        .register_module(commands::ping::command())
        .build()
        .start()
        .await
        .unwrap();
}
```

Each command lives in its own file:

```rust
// commands/ping.rs
use oxidian::{command::Command, Context};
use oxidian::core::models::message::Message;

pub fn command() -> Command {
    Command::new("ping", |ctx: Context, msg: Message, _args| async move {
        ctx.reply(&msg, "pong!").await?;
        Ok(())
    })
}
```

## Documentation

- [Getting started](docs/getting-started.md) — step-by-step setup from scratch
- [Commands](docs/commands.md) — the prefix command system and module pattern
- [Event handling](docs/event-handling.md) — implementing `EventHandler` and working with intents
- [Architecture](docs/architecture.md) — how the crates fit together internally

## Workspace layout

```
crates/
  core/          ← error types, models (User, Guild, Message, …), Snowflake
  gateway/       ← WebSocket connection, heartbeat, typed events
  http/          ← REST client with rate limiting
  interactions/  ← slash commands (early)
  voice/         ← voice (early)
  oxidian/       ← re-exports everything, Bot/EventHandler/Context live here
testBot/         ← example bot used for manual testing
```

## Status

| Feature | Status |
|---------|--------|
| Gateway connection + heartbeat | ✅ working |
| Typed dispatch events | ✅ working |
| Prefix commands + module pattern | ✅ working |
| HTTP client + rate limiting | ✅ working |
| Core models (User, Guild, Channel, Message, Member, Role) | ✅ working |
| Slash commands | 🔧 scaffolded, not functional |
| Voice | 🔧 scaffolded, not functional |
| Resume / session recovery | ⏳ not started |
| Sharding | ⏳ not started |

## License

MIT — see [LICENSE](LICENSE).



## Documentation

Documentation is currently in progress and may be incomplete. Please refer to the source code for usage examples and API details. As development continues, more comprehensive documentation will be provided.

You can probably find some documentation in docs/

## How can I contribute?

Contributions are welcome! Please feel free to open issues or submit pull requests. Refer to the CONTRIBUTING.md file for guidelines on how to contribute to the project.

## Branches
Currently, the main branch is the primary development branch. There may be feature branches for specific features or bug fixes, but these are not guaranteed to be stable due to the early stage of development. Always check the branch status and documentation before using or contributing to a specific branch.

After the initial foundation phase, we will work strictly in the dev/ branch, and only merge to main/ when we have a stable release candidate. This will help ensure that the main branch remains stable and production-ready as we continue development.

## License
Oxidian is licensed under the MIT License. See the LICENSE file for more details.