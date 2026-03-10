# Commands

Oxidian uses a prefix command system. When a message arrives that starts with the bot's configured prefix, the first word after the prefix is matched against registered commands and the matching handler is called with any remaining words as arguments.

## Defining a command

Each command is a `Command` value, created with `Command::new`:

```rust
use oxidian::{command::Command, Context};
use oxidian::core::models::message::Message;

pub fn command() -> Command {
    Command::new("greet", |ctx: Context, msg: Message, args: Vec<String>| async move {
        let name = if args.is_empty() {
            "stranger".to_owned()
        } else {
            args.join(" ")
        };
        ctx.reply(&msg, format!("hey, {name}!")).await?;
        Ok(())
    })
}
```

Sending `!greet Alice` will make the bot reply `hey, Alice!`.

## The module pattern

The recommended layout is one file per command under a `commands/` directory:

```
src/
  main.rs
  commands/
    mod.rs      ← pub mod ping; pub mod greet;
    ping.rs
    greet.rs
```

Each file exposes a single `pub fn command() -> Command` function. In `main.rs` you register them:

```rust
mod commands;

Bot::builder(token)
    .prefix("!")
    .register_module(commands::ping::command())
    .register_module(commands::greet::command())
    .build()
    .start()
    .await?;
```

This keeps every command self-contained and easy to add or remove without touching anything else.

## Inline commands

For quick experiments you can register a command inline without creating a separate file:

```rust
Bot::builder(token)
    .prefix("!")
    .command("ping", |ctx, msg, _args| async move {
        ctx.reply(&msg, "pong!").await?;
        Ok(())
    })
    .build()
    .start()
    .await?;
```

`.command()` and `.register_module()` are interchangeable — they both add entries to the same registry.

## Arguments

The third parameter is `Vec<String>` — the space-separated words that come after the command name. If the user sends `!ban @Levin spamming`, `args` will be `["@Levin", "spamming"]`.

There's no argument parsing built in yet. That's intentional — once we know what the most common patterns are we'll add typed argument extraction without locking anyone into a particular convention.

## Command names

- Names are case-sensitive. `!Ping` won't match a command named `ping`.
- Names cannot contain spaces (which is why they're the first word after the prefix).
- Registering the same name twice silently overwrites the previous handler, so be careful with that.

## Context

Every command receives a `Context` as its first argument. Right now the most useful thing on it is the HTTP client:

```rust
ctx.reply(&msg, "message").await?;          // reply to the same channel
ctx.send(channel_id, "message").await?;     // send to any channel by ID
ctx.http.get_current_user().await?;         // raw HTTP access
```

See [Context](./context.md) for the full surface area.
