# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

```bash
cargo build              # Build all workspace crates
cargo check              # Fast type/borrow check
cargo test               # Run tests
cargo clippy             # Lint
cargo fmt                # Auto-format (max_width=88, tab_spaces=4)
cargo fmt --all -- --check  # CI format check (runs on every push/PR)

# Run examples (require env vars)
DISCORD_TOKEN=<token> cargo run --example ping_bot
DISCORD_TOKEN=<token> APPLICATION_ID=<id> cargo run --example slash_commands
DISCORD_TOKEN=<token> cargo run --example multi_shard
```

## Architecture

Oxidian is a Discord library for Rust, structured as a Cargo workspace with a strict inward dependency hierarchy:

```
oxidian  (Bot, EventHandler, Context, Module)
├── gateway   (WebSocket shards, heartbeat, typed events)
├── http      (REST client, per-bucket rate limiting)
├── interactions  (slash command/component builders)
├── cache     (optional, feature = "cache", DashMap-backed)
├── voice     (optional, DAVE E2EE, scaffolded)
└── core      (models, Snowflake, Intents, Permissions, Error)
```

### Event dispatch

Gateway events flow through three concurrent tasks per shard:
1. **Read task**: parses opcodes, sends heartbeat ticks, forwards `DispatchEvent` over mpsc
2. **Heartbeat task**: owns the ack state, detects zombie connections
3. **Write task**: sole owner of the WS sink, receives from read + heartbeat

`Bot::start()` drives the event loop: each incoming `DispatchEvent` spawns a new Tokio task that clones `Arc<Context>`, `Arc<EventHandler>`, `Arc<CommandRegistry>`, and `Vec<Arc<Module>>`.

### Models (`oxidian-core`)

Plain structs, no business logic. All Discord IDs are `Snowflake(u64)`: deserializes from either JSON string or number. Error type is a `#[non_exhaustive]` enum with variants for HTTP, Gateway, Voice, serde, and Discord API errors. `Permissions` and `Intents` use `bitflags!`.

### HTTP (`oxidian-http`)

60+ endpoint helpers on `HttpClient`. Routes track Discord's major-parameter bucketing (channel_id, guild_id, webhook_id) for rate limiting. Rate-limit state is shared across shards via `Arc<Mutex<RateLimiter>>`.

### Interactions (`oxidian-interactions`)

Fluent builders for slash commands and components:
- `SlashCommandBuilder` / `CommandOptionBuilder` → serializes to Discord's command JSON
- Components: `ActionRow`, `Button`, `StringSelect`, `TextInput`
- Components v2 layout types: `Container`, `Section`, `Separator`, `TextDisplay`

### Module system

Modules are `Arc<dyn Module>` registered on `BotBuilder`. At startup, prefix commands are extracted into `CommandRegistry`; slash commands are dispatched by matching `interaction.data.name` against each module's `slash_commands()` list, then calling `module.handle_interaction()`. Unmatched interactions fall through to the global `EventHandler::interaction`.

### Cache (`oxidian-cache`, feature-gated)

DashMap maps for guilds, channels, users, members, roles, voice states, presences, emojis, stickers. Call `cache.update(&dispatch_event)` before firing user handlers so the cache is always warm when handlers run.

### Voice (`oxidian-voice`)

Voice gateway + DAVE E2EE key-exchange scaffolded in `crates/voice/src/dave/`. Audio send/receive is not yet wired up. Voice connect flow: send `VoiceStateUpdate` → wait for both `VOICE_STATE_UPDATE` and `VOICE_SERVER_UPDATE` gateway events → open `VoiceConnection`.

## Contributing

- PRs target the `dev/` branch, not `main`.
- No AI-generated code (see `.github/CONTRIBUTING.md`: licensing/attribution concerns).
- Code style is enforced by CI via `cargo fmt --all -- --check`.
- Conventions (module structure, naming) are documented in `.github/CONVENTIONS.md`.
