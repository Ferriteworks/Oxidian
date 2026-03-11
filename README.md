# Oxidian

A Discord bot library for Rust. Async, modular, and built on [Tokio](https://tokio.rs).

> **This is early-stage software.** The API will change, things will break, and there are features that aren't implemented yet. That said, the gateway connects, events fire, prefix commands work, and slash commands route through the module system.

## What it does

- Connects to the Discord gateway over WebSocket (with heartbeating, reconnects, and exponential backoff)
- Delivers typed gateway events to your handler (`READY`, `MESSAGE_CREATE`, `GUILD_CREATE`, `INTERACTION_CREATE`, and more)
- Handles Discord's REST rate limits automatically — per-route buckets, global limits, and 429 retries
- Ships a **module system** where a single struct groups prefix commands, slash command definitions, and an interaction handler together

## Quick example

```rust
mod commands;

use async_trait::async_trait;
use oxidian::{Bot, Context, EventHandler, Intents};
use oxidian::gateway::events::ReadyData;

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
        .intents(Intents::GUILDS | Intents::GUILD_MESSAGES | Intents::MESSAGE_CONTENT)
        .prefix("!")
        .handler(Handler)
        .module(commands::FunModule)
        .build()
        .start()
        .await
        .unwrap();
}
```

`FunModule` is a struct that implements [`Module`](crates/oxidian/src/command.rs) — it can define prefix commands, declare slash commands, and handle slash command interactions all in one place:

```rust
use async_trait::async_trait;
use oxidian::{command::{Command, Module}, ApplicationCommand, Context};
use oxidian::core::models::interaction::{Interaction, InteractionResponse};
use oxidian::interactions::command::SlashCommandBuilder;

pub struct FunModule;

#[async_trait]
impl Module for FunModule {
    fn commands(&self) -> Vec<Command> {
        vec![
            Command::new("ping", |ctx, msg, _args| async move {
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


## Documentation

- [Getting started](docs/getting-started.md) — step-by-step setup from scratch
- [Commands & modules](docs/commands.md) — prefix commands, slash commands, and the module pattern
- [Event handling](docs/event-handling.md) — implementing `EventHandler` and working with intents
- [Architecture](docs/architecture.md) — how the crates fit together internally

## Workspace layout

```
crates/
  core/          ← error types, models (User, Guild, Message, Interaction, …), Snowflake, Intents
  gateway/       ← WebSocket connection, heartbeat, typed events
  http/          ← REST client with rate limiting + bulk command sync helpers
  interactions/  ← ApplicationCommand, SlashCommandBuilder, components
  voice/         ← voice gateway + DAVE E2EE protocol
  oxidian/       ← re-exports everything, Bot/EventHandler/Context/Module live here
testBot/         ← example bot used for manual testing
```

## Status

| Feature | Status |
|---------|--------|
| Gateway connection + heartbeat | ✅ working |
| Typed dispatch events | ✅ working |
| Prefix commands + module pattern | ✅ working |
| Slash commands (define, sync, route, respond) | ✅ working |
| HTTP client + rate limiting | ✅ working |
| Bulk-overwrite global / guild commands | ✅ working |
| Core models (User, Guild, Channel, Message, Member, Role, …) | ✅ working |
| `Intents` bitflags type | ✅ working |
| Voice gateway scaffolding | 🔧 scaffolded, not functional |
| DAVE E2EE voice protocol | 🔧 scaffolded, not functional |
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