# Oxidian

A Discord bot library for Rust. Async, modular, and built on [Tokio](https://tokio.rs).

> **This is early-stage software.** The API will change and there are still features being filled in. That said, the gateway connects, events fire, commands work, and a large portion of the Discord v10 REST API is covered.

## What it does

- Connects to the Discord gateway over WebSocket (with heartbeating, reconnects, and exponential backoff)
- Delivers typed gateway events to your handler (`READY`, `MESSAGE_CREATE`, `GUILD_CREATE`, `INTERACTION_CREATE`, and more)
- Handles Discord's REST rate limits automatically — per-route buckets, global limits, and 429 retries
- Ships a **module system** where a single struct groups prefix commands, slash command definitions, and an interaction handler together
- **Permissions v2** — full 49-flag `Permissions` bitflags type with Discord's string-encoded u64 serde
- **Voice** gateway scaffolding with DAVE E2EE protocol support
- **Optional in-memory cache** (feature flag `cache`) backed by `DashMap` — guilds, channels, members, roles, voice states, presences, emojis, stickers

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

`FunModule` is a struct that implements [`Module`](crates/oxidian/src/command.rs) -— it can define prefix commands, declare slash commands, and handle slash command interactions all in one place:

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

> Docs are still catching up with the codebase. The source and examples are the most complete reference for now.

## Workspace layout

```
crates/
  core/          ← error types, models (User, Guild, Message, Interaction, Poll, Sticker, …), Snowflake, Intents, Permissions
  gateway/       ← WebSocket connection, heartbeat, typed events, resume/reconnect
  http/          ← REST client with rate limiting, multipart upload, 60+ endpoint helpers
  interactions/  ← ApplicationCommand, SlashCommandBuilder, components v1 + v2
  voice/         ← voice gateway + DAVE E2EE protocol (scaffolded)
  cache/         ← optional DashMap-backed in-memory cache (feature = "cache")
  oxidian/       ← re-exports everything; Bot/EventHandler/Context/Module live here
testBot/         ← example bot used for manual testing
```

## Status

| Feature | Status |
|---------|--------|
| Gateway connection + heartbeat | ✅ |
| Typed dispatch events | ✅ |
| Prefix commands + module pattern | ✅ |
| Slash commands (define, sync, route, respond) | ✅ |
| Select menus (all 5 types) | ✅ |
| Context menus (User + Message commands) | ✅ |
| Autocomplete | ✅ |
| Modals (TextInput + response) | ✅ |
| Components v2 layout types | ✅ |
| HTTP client + rate limiting | ✅ |
| Bulk-overwrite global / guild commands | ✅ |
| Core models (User, Guild, Channel, Message, Member, Role, …) | ✅ |
| `Intents` + `Permissions` bitflags | ✅ |
| Threads REST (create, archive, members, list) | ✅ |
| Member timeouts (`communication_disabled_until`) | ✅ |
| Scheduled events REST (CRUD) | ✅ |
| Stickers REST — guild stickers with real multipart upload | ✅ |
| Sticker packs (Nitro) REST | ✅ |
| Auto moderation REST (CRUD) | ✅ |
| Stage instances REST (CRUD) | ✅ |
| Polls REST (voters, end poll) | ✅ |
| Soundboard REST (defaults, guild CRUD, send) | ✅ |
| Audit logs REST | ✅ |
| Monetization (entitlements, SKUs) REST | ✅ |
| Optional in-memory cache (`cache` feature) | ✅ |
| Multi-sharding | ✅ |
| Resume / session recovery | ✅ |
| Low-level gateway access | ✅ |
| Voice gateway | ✅ |
| DAVE E2EE voice protocol | ✅ |

## License

MIT — see [LICENSE](LICENSE).

## Contributing

Contributions are welcome. Open an issue or PR — please target the `dev/` branch rather than `main`. The `main` branch is reserved for stable release candidates.