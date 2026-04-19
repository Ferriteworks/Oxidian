# Oxidian

A Discord bot library for Rust. Async, modular, and built on [Tokio](https://tokio.rs).

> **Early-stage software.** The API will change and features are still being filled in. The gateway connects, events fire, commands work, and a large portion of the Discord v10 REST API is covered.

## What it does

- Connects to the Discord gateway over WebSocket, with heartbeating, reconnect, and exponential backoff.
- Delivers typed gateway events to your handler (`READY`, `MESSAGE_CREATE`, `GUILD_CREATE`, `INTERACTION_CREATE`, and the rest).
- Respects Discord's REST rate limits: per-route buckets, global limits, and 429 retries.
- Ships a module system where a single struct groups prefix commands, slash command definitions, and an interaction handler together.
- `Permissions` is a full 49-flag bitflags type with Discord's string-encoded u64 serde.
- Voice gateway connects, sends Identify with `max_dave_protocol_version`, and follows the DAVE opcode state machine.
- Optional in-memory cache (feature flag `cache`) backed by `DashMap`: guilds, channels, members, roles, voice states, presences, emojis, stickers.

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

`FunModule` is a struct that implements [`Module`](crates/oxidian/src/command.rs). It can define prefix commands, declare slash commands, and handle slash command interactions all in one place:

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
        ctx.respond(&interaction, InteractionResponse::message("pong!"))
            .await
            .ok();
    }
}
```

## High-level vs low-level

Oxidian ships two ways to use it:

- **High-level**: depend on the `oxidian` facade crate. You get `Bot::builder`, the event handler trait, modules, and everything re-exported from one place. This is the recommended path for most bots.
- **Low-level**: depend on the individual crates (`oxidian-core`, `oxidian-gateway`, `oxidian-http`, `oxidian-interactions`, `oxidian-voice`, `oxidian-cache`) directly and wire your own event loop. There is no forced runtime: the crates only depend on `tokio` for async primitives, not on the facade. See [`examples/low_level.rs`](examples/low_level.rs).

## Documentation

- [Getting started](docs/getting-started.md): step-by-step setup from scratch.
- [Commands and modules](docs/commands.md): prefix commands, slash commands, and the module pattern.
- [Event handling](docs/event-handling.md): implementing `EventHandler` and working with intents.
- [Architecture](docs/architecture.md): how the crates fit together internally.

> Docs lag the codebase. The source and examples are the most complete reference for now.

## Workspace layout

```
crates/
  core/          error types, models (User, Guild, Message, Interaction, Poll, Sticker, ...), Snowflake, Intents, Permissions
  gateway/       WebSocket connection, heartbeat, typed events, resume/reconnect
  http/          REST client with rate limiting, multipart upload, endpoint helpers
  interactions/  ApplicationCommand, SlashCommandBuilder, components v1 + v2
  utils/         CDN URL builders, mention formatters, timestamp helpers
  voice/         voice gateway, RTP send path, DAVE opcode state machine
  cache/         optional DashMap-backed in-memory cache (feature = "cache")
  oxidian/       facade crate: re-exports everything, hosts Bot/EventHandler/Context/Module
testBot/         example bot used for manual testing
```

## Status

| Feature | Status |
|---------|--------|
| Gateway connection + heartbeat | done |
| Typed dispatch events | done |
| Prefix commands + module pattern | done |
| Slash commands (define, sync, route, respond) | done |
| Buttons, select menus (all 5 types) | done |
| Context menus (User + Message commands) | done |
| Autocomplete | done |
| Modals (TextInput + response) | done |
| Components v2 layout types | done |
| HTTP client + rate limiting | done |
| Bulk-overwrite global / guild commands | done |
| Application command permissions v2 | done |
| Webhooks REST (get, modify, delete, execute) | done |
| Core models (User, Guild, Channel, Message, Member, Role, ...) | done |
| `Intents` + `Permissions` bitflags | done |
| Threads REST (create, archive, members, list) | done |
| Member timeouts (`communication_disabled_until`) | done |
| Scheduled events REST (CRUD) | done |
| Stickers REST with multipart upload | done |
| Sticker packs (Nitro) REST | done |
| Auto moderation REST (CRUD) | done |
| Stage instances REST (CRUD) | done |
| Polls REST (voters, end poll) | done |
| Soundboard REST (defaults, guild CRUD, send) | done |
| Audit logs REST | done |
| Monetization (entitlements, SKUs) REST | done |
| Application emoji REST | done |
| Localization (`name_localizations`, `description_localizations`) | done |
| Optional in-memory cache (`cache` feature) | done |
| Multi-sharding | done |
| Resume / session recovery | done |
| Low-level gateway access | done |
| Voice gateway (Identify, session, heartbeat, RTP) | done |
| Voice RTP send path with AES-256-GCM | done |
| DAVE gateway state machine (opcodes 20-28) | done |
| DAVE MLS group state, per-sender AEAD keys, E2EE send path | partial (scaffold; needs live testing) |

The DAVE E2EE row is called out honestly: the voice gateway handles every DAVE opcode Discord sends, stores the MLS payloads, and tracks transitions, but the MLS group processing and the E2EE frame cipher are scaffolds marked with TODOs in the source. Full DAVE requires live integration tests against Discord's voice server and is the active work item. Do not rely on it for production end-to-end encryption yet.

## License

MIT. See [LICENSE](LICENSE).

## Contributing

Contributions are welcome. Open an issue or PR. Please target the `dev/` branch rather than `main`: `main` is reserved for stable release candidates.
