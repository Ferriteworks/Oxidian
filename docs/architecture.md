# Architecture

A high-level look at how Oxidian fits together. You don't need to read this to use the library, but it's helpful if you want to understand why things are structured the way they are or contribute to the project.

## Crate layout

The repository is a Cargo workspace with six crates. The dependency direction is strictly inward: outer crates depend on inner ones, never the reverse.

```
oxidian          ← the crate you import in your bot
├── oxidian-core       ← error types, models, Snowflake, Intents
├── oxidian-gateway    ← WebSocket connection, heartbeat, event dispatch
├── oxidian-http       ← REST client, rate limiting, command sync
├── oxidian-interactions  ← ApplicationCommand, SlashCommandBuilder, components
└── oxidian-voice      ← voice gateway + DAVE E2EE protocol
```

`oxidian` is a thin re-export crate. It pulls everything together and provides the `Bot`, `BotBuilder`, `Context`, `EventHandler`, `Module`, and `Intents` types that most bots will interact with directly.

## Gateway

The gateway crate owns the WebSocket connection to Discord. Internally it uses three concurrent Tokio tasks:

1. **Read task**: drives the stream from `ws.split()`, parsing incoming JSON payloads and routing them by opcode.
2. **Write task**: owns the sink exclusively. Both the read task and the heartbeat task send messages through an `mpsc` channel to this task, which forwards them to the WebSocket. This avoids the need to share a mutex around the sink.
3. **Heartbeat task**: fires every `heartbeat_interval` milliseconds, sends opcode 1, and watches for acknowledgements. If an ack isn't received before the next beat, the connection is considered dead and the task stops so the shard can reconnect.

Parsed events come out as a `DispatchEvent` enum over an `mpsc::Sender<DispatchEvent>` channel. The `Bot` holds the receiver side and fans events out to user handlers.

## Bot and event dispatch

`Bot::start()` does three things:

1. Creates an `HttpClient` and wraps it in a `Context`.
2. Spawns a `Shard` task that connects to the gateway and sends `DispatchEvent` values into the channel.
3. Drives a loop that receives events from the channel and spawns a short-lived task for each one.

Each event task gets a cloned `Context` (just an `Arc` clone, cheap), a cloned `Arc<dyn EventHandler>`, a cloned `Arc<CommandRegistry>`, and a cloned `Arc<Vec<Arc<dyn Module>>>`. There's no global state.

## Module system

`Module` is an `#[async_trait]` trait with three default methods:

- `commands(&self) -> Vec<Command>`: prefix commands to load into the registry at startup.
- `slash_commands(&self) -> Vec<ApplicationCommand>`: slash command *declarations* (definitions, not registration).
- `async handle_interaction(&self, ctx, interaction)`: called at runtime when an `INTERACTION_CREATE` event matches one of this module's declared slash command names.

Modules are `Send + Sync + 'static` so they can be stored as `Arc<dyn Module>` and shared cheaply between event tasks. `BotBuilder::module()` wraps the module in an `Arc`, extracts its prefix commands into the `CommandRegistry`, and stores it for interaction routing.

When an `InteractionCreate` event arrives, `dispatch()` searches the module list for the first module whose `slash_commands()` names match the interaction's command name. If found, that module's `handle_interaction` is called; otherwise the global `EventHandler::interaction` fallback is used.

## Intents

`Intents` is a `bitflags!`-generated type in `oxidian-core`. Each Discord gateway intent is a named constant. The type implements `From<u64>` / `Into<u64>` for interop with the raw gateway payload, and provides `NON_PRIVILEGED` and `ALL` convenience constants.

## HTTP and rate limiting

`HttpClient` sits in front of a `reqwest::Client` and a `RateLimiter`. Before every request, `RateLimiter::acquire()` checks the per-route bucket and the global rate limit, sleeping if necessary. After every response, `RateLimiter::update()` reads Discord's `X-RateLimit-*` headers and updates the bucket.

Discord uses _major parameters_ (channel ID, guild ID) to group routes into buckets, so `/channels/123/messages` and `/channels/456/messages` are independent buckets. The bucket key is derived from the `Route` enum.

Slash command sync is exposed through `bulk_overwrite_global_commands` and `bulk_overwrite_guild_commands`: both call Discord's `PUT /applications/{id}/commands` (or the guild-scoped variant) with the full list of commands to register.

## Error handling

All public APIs return `oxidian::Result<T>`, which is aliased to `Result<T, oxidian::Error>`. The error type is a `#[non_exhaustive]` enum so we can add variants without breaking existing `match` arms. Sub-enums (`HttpError`, `GatewayError`, `VoiceError`) give callers enough detail to handle errors programmatically if they need to.

## Models

Models live in `oxidian-core::models`. They're plain structs that deserialize from Discord's JSON. None of them implement any business logic: methods that require calling Discord (like "delete this message") live on `Context` or `HttpClient`, not on the model itself.

All snowflake IDs are typed as `Snowflake(u64)` rather than raw integers or strings. `Snowflake` deserializes from either format (Discord sends IDs as JSON strings in most contexts), implements `Display` as the raw integer, and provides `created_at()` to extract the timestamp encoded in the ID.

## Voice (scaffolded)

The voice crate handles the Discord voice gateway WebSocket and the DAVE E2EE protocol for encrypted audio. The connection flow mirrors the text gateway: the bot sends a `VoiceStateUpdate` gateway payload, waits for both a `VOICE_STATE_UPDATE` and `VOICE_SERVER_UPDATE` dispatch event, then connects `VoiceConnection` to the voice-specific WebSocket endpoint. The DAVE session key exchange is implemented in `crates/voice/src/dave/`; actual audio send/receive is not yet wired up.

