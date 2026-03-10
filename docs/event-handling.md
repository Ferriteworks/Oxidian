# Event Handling

When the bot connects to Discord over the gateway, Discord sends a stream of events — messages, guild updates, members joining, and so on. Oxidian delivers these to your code through the `EventHandler` trait.

## Implementing EventHandler

Create a struct and implement the trait:

```rust
use async_trait::async_trait;
use oxidian::{Context, EventHandler};
use oxidian::core::models::{guild::Guild, message::Message};
use oxidian::gateway::events::{MessageDeleteData, ReadyData};

struct MyHandler;

#[async_trait]
impl EventHandler for MyHandler {
    async fn ready(&self, _ctx: Context, ready: ReadyData) {
        println!("online as {}!", ready.user.username);
    }

    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot {
            return; // ignore other bots
        }
        println!("{}: {}", msg.author.username, msg.content);
    }

    async fn guild_create(&self, _ctx: Context, guild: Guild) {
        println!("joined guild: {} ({})", guild.name, guild.id);
    }
}
```

Pass it to the builder:

```rust
Bot::builder(token)
    .handler(MyHandler)
    .build()
    .start()
    .await?;
```

## Available methods

All methods have empty default implementations, so you only need to override the ones you care about.

| Method | Fired when |
|--------|-----------|
| `ready` | The bot has connected and the gateway session is established |
| `message` | A message is created in any channel the bot can see |
| `message_delete` | A message is deleted |
| `guild_create` | The bot joins a guild or a previously unavailable guild comes back online |
| `guild_update` | A guild's settings change (name, icon, etc.) |
| `raw_event` | Any event that doesn't have a dedicated method above |

## raw_event

`raw_event` receives the full `DispatchEvent` enum including the `Unknown` variant which catches anything Discord sends that Oxidian doesn't handle explicitly yet:

```rust
async fn raw_event(&self, _ctx: Context, event: DispatchEvent) {
    if let DispatchEvent::Unknown { name, data } = event {
        println!("unhandled event: {name}");
    }
}
```

This is also handy for logging all events during development.

## Relation to prefix commands

`EventHandler::message` is called for **every** incoming message that doesn't match a registered prefix command. If a command does match, the command handler runs instead and `message` is not called. If you need to intercept all messages regardless of prefix, use `raw_event` with `DispatchEvent::MessageCreate`.

## Concurrency

Each event is dispatched on its own Tokio task, so handlers for different events can run concurrently. Your `EventHandler` implementation must be `Send + Sync + 'static` — the `Arc<dyn EventHandler>` is shared across tasks.

If you need shared mutable state, wrap it in an `Arc<Mutex<T>>` or `Arc<RwLock<T>>` inside your handler struct:

```rust
use std::sync::{Arc, Mutex};

struct MyHandler {
    message_count: Arc<Mutex<u64>>,
}

#[async_trait]
impl EventHandler for MyHandler {
    async fn message(&self, _ctx: Context, _msg: Message) {
        let mut count = self.message_count.lock().unwrap();
        *count += 1;
        println!("total messages seen: {count}");
    }
}
```

## Intents

Discord only sends events for intents you've declared. If an event handler isn't firing, the most likely culprit is a missing intent. Check the [Gateway Intents](https://discord.com/developers/docs/topics/gateway#gateway-intents) docs and add the relevant bit to your `.intents()` call.

`MESSAGE_CONTENT` (bit 15) is a privileged intent. You must enable it in the Developer Portal for the bot to receive message content in `MESSAGE_CREATE` events.
