# Commands and Modules

Oxidian organises commands through the **module** pattern. A module is a struct that implements the `Module` trait, which lets it declare prefix commands, slash command definitions, and an interaction handler: all in one place.

## The Module trait

```rust
use async_trait::async_trait;
use oxidian::{command::{Command, Module}, ApplicationCommand, Context};
use oxidian::core::models::interaction::{Interaction, InteractionResponse};
use oxidian::interactions::command::SlashCommandBuilder;

pub struct FunModule;

#[async_trait]
impl Module for FunModule {
    // ── Prefix commands ───────────────────────────────────────────────────────

    fn commands(&self) -> Vec<Command> {
        vec![
            Command::new("ping", |ctx: Context, msg, _args| async move {
                ctx.reply(&msg, "pong!").await?;
                Ok(())
            }),
        ]
    }

    // ── Slash command definitions ─────────────────────────────────────────────

    fn slash_commands(&self) -> Vec<ApplicationCommand> {
        vec![
            SlashCommandBuilder::new("ping", "Replies with pong!").build(),
        ]
    }

    // ── Slash command handler ─────────────────────────────────────────────────

    async fn handle_interaction(&self, ctx: Context, interaction: Interaction) {
        if let Err(e) = ctx.respond(&interaction, InteractionResponse::message("pong! 🏓")).await {
            tracing::error!("failed to respond to /ping: {e}");
        }
    }
}
```

All three methods have default implementations that return empty vecs / do nothing, so you only need to override the ones you use.

## Registering a module

Pass it to `.module()` on the builder:

```rust
Bot::builder(token)
    .intents(Intents::GUILDS | Intents::GUILD_MESSAGES | Intents::MESSAGE_CONTENT)
    .prefix("!")
    .handler(MyHandler)
    .module(FunModule)
    .module(ModerationModule)
    .build()
    .start()
    .await?;
```

`BotBuilder::module` does two things:

1. Extracts prefix commands from `Module::commands()` and loads them into the registry.
2. Stores the module `Arc`-wrapped for slash interaction routing at runtime.

## Recommended layout

```
src/
  main.rs
  commands/
    mod.rs        ← one Module impl per group
    fun.rs
    moderation.rs
```

Each file defines one struct and its `Module` impl. `commands/mod.rs` just re-exports them:

```rust
pub mod fun;
pub mod moderation;

pub use fun::FunModule;
pub use moderation::ModerationModule;
```

## Defining prefix commands

`Command::new(name, handler)` takes a command name and an async callback:

```rust
Command::new("greet", |ctx: Context, msg: Message, args: Vec<String>| async move {
    let name = args.first().map(String::as_str).unwrap_or("stranger");
    ctx.reply(&msg, format!("hey, {name}!")).await?;
    Ok(())
})
```

Sending `!greet Alice` will reply `hey, Alice!`.

**Command name rules:**
- Case-sensitive: `!Ping` won't match a command named `ping`.
- No spaces (first word after the prefix).
- Registering the same name twice silently overwrites the first handler.

## Defining slash commands

Use `SlashCommandBuilder` to declare commands. You can add typed options with `CommandOptionBuilder`:

```rust
use oxidian::interactions::command::{SlashCommandBuilder, CommandOptionBuilder};
use oxidian::core::models::interaction::CommandOptionType;

fn slash_commands(&self) -> Vec<ApplicationCommand> {
    vec![
        SlashCommandBuilder::new("greet", "Greet someone")
            .option(
                CommandOptionBuilder::new(CommandOptionType::User, "user", "Who to greet")
                    .required()
                    .build(),
            )
            .build(),
    ]
}
```

Declarations produced by `slash_commands()` are *definitions* only: Discord doesn't know about them until you sync them via the HTTP client. See [Syncing slash commands](#syncing-slash-commands) below.

## Syncing slash commands

After building the bot collect the commands you want to register and call the HTTP helper:

```rust
let bot = Bot::builder(token) /* ... */ .build();

// Collect all slash command definitions from all modules
let commands: Vec<ApplicationCommand> = vec![
    SlashCommandBuilder::new("ping", "Pong!").build(),
];

// Register globally (takes up to 1 hour to propagate)
bot.http.bulk_overwrite_global_commands(&commands).await?;

// Or register to a specific guild instantly (use during development)
bot.http.bulk_overwrite_guild_commands(guild_id, &commands).await?;

bot.start().await?;
```

## Handling slash command interactions

When Discord sends an `INTERACTION_CREATE` event, Oxidian checks each registered module's `slash_commands()` list for a name match and calls that module's `handle_interaction`. If no module claims it, the global `EventHandler::interaction` fallback is called.

Use `interaction.command_data()` to get the interaction's command name and options, and `ctx.respond()` to reply:

```rust
async fn handle_interaction(&self, ctx: Context, interaction: Interaction) {
    if let Some(data) = interaction.command_data() {
        match data.name.as_str() {
            "ping" => {
                ctx.respond(&interaction, InteractionResponse::message("pong! 🏓"))
                    .await
                    .ok();
            }
            "greet" => {
                let name = data.options.iter()
                    .find(|o| o.name == "user")
                    .and_then(|o| o.value.as_str())
                    .unwrap_or("stranger");
                ctx.respond(&interaction, InteractionResponse::message(format!("hey, {name}!")))
                    .await
                    .ok();
            }
            _ => {}
        }
    }
}
```

`InteractionResponse::ephemeral(content)` sends a reply visible only to the invoking user. `InteractionResponse::defer()` acknowledges the interaction without a message so you can follow up later.

## Context

Every command and interaction handler receives a `Context`. The most useful methods:

```rust
ctx.reply(&msg, "message").await?;                          // prefix: reply to channel
ctx.send(channel_id, "message").await?;                     // send to any channel
ctx.respond(&interaction, InteractionResponse::message("…")).await?;  // slash: respond
ctx.http.get_current_user().await?;                         // raw HTTP access
ctx.gateway.join_voice(guild_id, Some(channel_id), false, false)?;   // voice
```

