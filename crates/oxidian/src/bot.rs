// MIT License
//
// Copyright (c) 2026 Ferriteworks organization and its rightful owners.
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use std::{future::Future, sync::Arc};

use tokio::sync::mpsc;
use tracing::{debug, error, info};

use oxidian_core::{error::Result, intents::Intents, models::message::Message};
use oxidian_gateway::{events::DispatchEvent, Shard, ShardInfo};
use oxidian_http::HttpClient;

#[cfg(feature = "cache")]
use oxidian_cache::Cache;

use crate::{
    command::{Command, CommandRegistry, Module},
    context::{Context, GatewayHandle},
    handler::{DefaultHandler, EventHandler},
};

/// Shard count configuration.
#[derive(Debug, Clone, Copy)]
pub enum ShardCount {
    /// A single shard (the default).
    Single,
    /// A fixed number of shards.
    Fixed(u32),
    /// Query Discord's `GET /gateway/bot` for the recommended shard count.
    Auto,
}

impl Default for ShardCount {
    fn default() -> Self {
        Self::Single
    }
}

/// The top-level Discord bot.
pub struct Bot {
    token: String,
    intents: Intents,
    prefix: Option<String>,
    shard_count: ShardCount,
    handler: Arc<dyn EventHandler>,
    commands: Arc<CommandRegistry>,
    modules: Arc<Vec<Arc<dyn Module>>>,
}

impl Bot {
    /// Begin building a [`Bot`].
    pub fn builder(token: impl Into<String>) -> BotBuilder {
        BotBuilder::new(token)
    }

    /// Connect to the Discord gateway and process events until the connection
    /// closes or the retry limit is exceeded.
    ///
    /// When multiple shards are configured, each shard runs in its own task
    /// and all feed events into a single dispatch loop.
    pub async fn start(self) -> Result<()> {
        let http = Arc::new(HttpClient::new(&self.token)?);

        let num_shards = match self.shard_count {
            ShardCount::Single => 1u32,
            ShardCount::Fixed(n) => n,
            ShardCount::Auto => {
                let resp = http.get_gateway_bot().await?;
                let recommended = resp["shards"].as_u64().unwrap_or(1) as u32;
                info!(
                    recommended,
                    "auto-shard: Discord recommends {recommended} shard(s)"
                );
                recommended.max(1)
            }
        };

        let (event_tx, mut event_rx) =
            mpsc::channel::<DispatchEvent>(256 * num_shards as usize);

        // Spawn all shards. We share a single outbound broadcast per shard
        // (voice state updates etc. are shard-specific) but a single HTTP
        // client and dispatch loop.
        //
        // For single-shard (the common case), this is identical to before.
        let mut gateway_handles = Vec::with_capacity(num_shards as usize);
        for shard_id in 0..num_shards {
            let shard_info = if num_shards == 1 {
                ShardInfo::single()
            } else {
                ShardInfo::new(shard_id, num_shards)
            };
            let shard = Shard::with_shard_info(
                self.token.clone(),
                self.intents,
                shard_info,
                event_tx.clone(),
            );
            gateway_handles.push(shard.gateway_sender());
            let shard_id_log = shard_id;
            tokio::spawn(async move {
                if let Err(e) = shard.start().await {
                    error!(shard = shard_id_log, error = %e, "shard exited with an error");
                }
            });
            // Discord requires a 5-second delay between IDENTIFY requests for
            // multi-shard bots.
            if num_shards > 1 && shard_id + 1 < num_shards {
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            }
        }

        // Use the first shard's gateway handle for Context. For multi-shard
        // bots doing voice, you'll want low-level per-shard contexts instead.
        let gateway_handle = GatewayHandle::new(gateway_handles.remove(0));

        #[cfg(feature = "cache")]
        let cache = Arc::new(Cache::new());

        #[cfg(feature = "cache")]
        let ctx = Context::new(Arc::clone(&http), gateway_handle, Arc::clone(&cache));

        #[cfg(not(feature = "cache"))]
        let ctx = Context::new(Arc::clone(&http), gateway_handle);

        // Dispatch events until the shard drops its sender side.
        while let Some(event) = event_rx.recv().await {
            // Update cache BEFORE firing user handlers so they see fresh state.
            #[cfg(feature = "cache")]
            cache.update(&event);

            let ctx = ctx.clone();
            let handler = Arc::clone(&self.handler);
            let commands = Arc::clone(&self.commands);
            let modules = Arc::clone(&self.modules);
            let prefix = self.prefix.clone();

            tokio::spawn(async move {
                if let Err(e) =
                    dispatch(event, ctx, handler, commands, modules, prefix).await
                {
                    error!(error = %e, "event dispatch error");
                }
            });
        }

        Ok(())
    }
}

/// Builder for [`Bot`].  Obtain one via [`Bot::builder`].
pub struct BotBuilder {
    token: String,
    intents: Intents,
    prefix: Option<String>,
    shard_count: ShardCount,
    handler: Option<Arc<dyn EventHandler>>,
    commands: CommandRegistry,
    modules: Vec<Arc<dyn Module>>,
}

impl BotBuilder {
    /// Create a builder for the given bot token.
    pub fn new(token: impl Into<String>) -> Self {
        Self {
            token: token.into(),
            intents: Intents::empty(),
            prefix: None,
            shard_count: ShardCount::default(),
            handler: None,
            commands: CommandRegistry::new(),
            modules: Vec::new(),
        }
    }

    /// Set the [Gateway Intents](https://discord.com/developers/docs/topics/gateway#gateway-intents).
    ///
    /// ```rust,ignore
    /// use oxidian_core::intents::Intents;
    ///
    /// Bot::builder(token)
    ///     .intents(Intents::GUILDS | Intents::GUILD_MESSAGES | Intents::MESSAGE_CONTENT)
    /// ```
    pub fn intents(mut self, intents: Intents) -> Self {
        self.intents = intents;
        self
    }

    /// Set the prefix that triggers registered commands (e.g. `"!"`).
    pub fn prefix(mut self, prefix: impl Into<String>) -> Self {
        self.prefix = Some(prefix.into());
        self
    }

    /// Run the bot with a fixed number of shards.
    ///
    /// Each shard runs in its own task and identifies with `[shard_id, num_shards]`.
    /// Discord requires at least a 5-second gap between IDENTIFY requests, so
    /// startup will take `(n - 1) * 5` seconds.
    ///
    /// ```rust,ignore
    /// Bot::builder(token)
    ///     .shards(2)
    ///     .build()
    ///     .start()
    ///     .await?;
    /// ```
    pub fn shards(mut self, num_shards: u32) -> Self {
        self.shard_count = if num_shards <= 1 {
            ShardCount::Single
        } else {
            ShardCount::Fixed(num_shards)
        };
        self
    }

    /// Let Discord tell us how many shards to use (`GET /gateway/bot`).
    ///
    /// The recommended shard count is based on the bot's guild count.
    /// Small bots will get 1 shard (identical to the default).
    ///
    /// ```rust,ignore
    /// Bot::builder(token)
    ///     .auto_shards()
    ///     .build()
    ///     .start()
    ///     .await?;
    /// ```
    pub fn auto_shards(mut self) -> Self {
        self.shard_count = ShardCount::Auto;
        self
    }

    /// Set the event handler that receives gateway events.
    pub fn handler(mut self, handler: impl EventHandler) -> Self {
        self.handler = Some(Arc::new(handler));
        self
    }

    /// Register an inline prefix command.
    ///
    /// The `name` should **not** include the prefix character (e.g. `"ping"`,
    /// not `"!ping"`).
    pub fn command<F, Fut>(mut self, name: impl Into<String>, f: F) -> Self
    where
        F: Fn(Context, Message, Vec<String>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.commands.register(name, f);
        self
    }

    /// Register a [`Command`] module (single command).
    ///
    /// ```rust,ignore
    /// Bot::builder(token)
    ///     .register_module(commands::ping::command())
    ///     .build()
    /// ```
    pub fn register_module(mut self, cmd: Command) -> Self {
        self.commands.add(cmd);
        self
    }

    /// Register a [`Module`]: a struct that groups prefix commands, slash
    /// command definitions, and an interaction handler.
    ///
    /// ```rust,ignore
    /// Bot::builder(token)
    ///     .module(FunModule)
    ///     .module(ModerationModule)
    ///     .build()
    /// ```
    pub fn module(mut self, m: impl Module) -> Self {
        let m: Arc<dyn Module> = Arc::new(m);
        // Extract prefix commands into the registry.
        for cmd in m.commands() {
            self.commands.add(cmd);
        }
        // Store the module for slash command routing.
        self.modules.push(m);
        self
    }

    /// Consume the builder and produce a [`Bot`].
    pub fn build(self) -> Bot {
        Bot {
            token: self.token,
            intents: self.intents,
            prefix: self.prefix,
            shard_count: self.shard_count,
            handler: self.handler.unwrap_or_else(|| Arc::new(DefaultHandler)),
            commands: Arc::new(self.commands),
            modules: Arc::new(self.modules),
        }
    }
}

/// Dispatch a single gateway event to the appropriate handler/module.
///
/// This is the same dispatch logic used internally by [`Bot::start`]. Low-level
/// users who run their own event loop can call this directly.
pub async fn dispatch(
    event: DispatchEvent,
    ctx: Context,
    handler: Arc<dyn EventHandler>,
    commands: Arc<CommandRegistry>,
    modules: Arc<Vec<Arc<dyn Module>>>,
    prefix: Option<String>,
) -> Result<()> {
    match event {
        DispatchEvent::Ready(data) => {
            handler.ready(ctx, data).await;
        }
        DispatchEvent::MessageCreate(msg) => {
            // Check prefix commands before the generic message handler.
            if let Some(ref pfx) = prefix {
                if msg.content.starts_with(pfx.as_str()) {
                    let after = msg.content[pfx.len()..].trim_start();
                    let mut iter = after.splitn(2, char::is_whitespace);
                    let cmd_name = iter.next().unwrap_or("");
                    let args: Vec<String> = iter
                        .next()
                        .unwrap_or("")
                        .split_whitespace()
                        .map(str::to_owned)
                        .collect();

                    if let Some(cmd_fn) = commands.get(cmd_name) {
                        debug!(command = cmd_name, "dispatching prefix command");
                        cmd_fn(ctx, msg, args).await?;
                        return Ok(());
                    }
                }
            }

            handler.message(ctx, msg).await;
        }
        DispatchEvent::MessageDelete(data) => {
            handler.message_delete(ctx, data).await;
        }
        DispatchEvent::GuildCreate(guild) => {
            handler.guild_create(ctx, guild).await;
        }
        DispatchEvent::GuildUpdate(guild) => {
            handler.guild_update(ctx, guild).await;
        }
        DispatchEvent::InteractionCreate(interaction) => {
            use oxidian_core::models::interaction::{
                ApplicationCommandType, InteractionType,
            };

            match interaction.kind {
                InteractionType::ApplicationCommand => {
                    if let Some(data) = interaction.command_data() {
                        let module = match data.kind {
                            ApplicationCommandType::ChatInput => {
                                modules.iter().find(|m| {
                                    m.slash_commands()
                                        .iter()
                                        .any(|c| c.name == data.name)
                                })
                            }
                            ApplicationCommandType::User => modules.iter().find(|m| {
                                m.user_commands().iter().any(|c| c.name == data.name)
                            }),
                            ApplicationCommandType::Message => {
                                modules.iter().find(|m| {
                                    m.message_commands()
                                        .iter()
                                        .any(|c| c.name == data.name)
                                })
                            }
                        };

                        if let Some(module) = module.map(Arc::clone) {
                            match data.kind {
                                ApplicationCommandType::ChatInput => {
                                    module.handle_interaction(ctx, interaction).await;
                                }
                                _ => {
                                    module.handle_context_menu(ctx, interaction).await;
                                }
                            }
                        } else {
                            handler.interaction(ctx, interaction).await;
                        }
                    } else {
                        handler.interaction(ctx, interaction).await;
                    }
                }
                InteractionType::ApplicationCommandAutocomplete => {
                    let module = interaction.command_data().and_then(|data| {
                        modules
                            .iter()
                            .find(|m| {
                                m.slash_commands().iter().any(|c| c.name == data.name)
                            })
                            .map(Arc::clone)
                    });
                    match module {
                        Some(m) => {
                            m.handle_autocomplete(ctx, interaction).await;
                        }
                        None => handler.interaction(ctx, interaction).await,
                    }
                }
                InteractionType::MessageComponent | InteractionType::ModalSubmit => {
                    for module in modules.iter() {
                        module
                            .handle_component(ctx.clone(), interaction.clone())
                            .await;
                    }
                }
                // Ping and any future types: fall through to the event handler.
                _ => handler.interaction(ctx, interaction).await,
            }
        }
        DispatchEvent::Resumed => {
            handler.resumed(ctx).await;
        }
        DispatchEvent::MessageUpdate(msg) => {
            handler.message_update(ctx, msg).await;
        }
        DispatchEvent::MessageDeleteBulk(data) => {
            handler.message_delete_bulk(ctx, data).await;
        }
        DispatchEvent::MessageReactionAdd(data) => {
            handler.message_reaction_add(ctx, data).await;
        }
        DispatchEvent::MessageReactionRemove(data) => {
            handler.message_reaction_remove(ctx, data).await;
        }
        DispatchEvent::MessageReactionRemoveAll(data) => {
            handler.message_reaction_remove_all(ctx, data).await;
        }
        DispatchEvent::MessageReactionRemoveEmoji(data) => {
            handler.message_reaction_remove_emoji(ctx, data).await;
        }
        DispatchEvent::GuildMemberAdd(data) => {
            handler.guild_member_add(ctx, data).await;
        }
        DispatchEvent::GuildMemberRemove(data) => {
            handler.guild_member_remove(ctx, data).await;
        }
        DispatchEvent::GuildBanAdd(data) => {
            handler.guild_ban_add(ctx, data).await;
        }
        DispatchEvent::GuildBanRemove(data) => {
            handler.guild_ban_remove(ctx, data).await;
        }
        DispatchEvent::GuildRoleCreate(data) => {
            handler.guild_role_create(ctx, data).await;
        }
        DispatchEvent::GuildRoleUpdate(data) => {
            handler.guild_role_update(ctx, data).await;
        }
        DispatchEvent::GuildRoleDelete(data) => {
            handler.guild_role_delete(ctx, data).await;
        }
        DispatchEvent::ChannelCreate(channel) => {
            handler.channel_create(ctx, channel).await;
        }
        DispatchEvent::ChannelUpdate(channel) => {
            handler.channel_update(ctx, channel).await;
        }
        DispatchEvent::ChannelDelete(channel) => {
            handler.channel_delete(ctx, channel).await;
        }
        DispatchEvent::TypingStart(data) => {
            handler.typing_start(ctx, data).await;
        }
        DispatchEvent::VoiceStateUpdate(state) => {
            handler.voice_state_update(ctx, state).await;
        }
        DispatchEvent::VoiceServerUpdate(server) => {
            handler.voice_server_update(ctx, server).await;
        }
        DispatchEvent::GuildMemberUpdate(data) => {
            handler.guild_member_update(ctx, data).await;
        }
        DispatchEvent::PresenceUpdate(data) => {
            handler.presence_update(ctx, data).await;
        }
        DispatchEvent::ChannelPinsUpdate(data) => {
            handler.channel_pins_update(ctx, data).await;
        }
        DispatchEvent::ThreadCreate(thread) => {
            handler.thread_create(ctx, thread).await;
        }
        DispatchEvent::ThreadUpdate(thread) => {
            handler.thread_update(ctx, thread).await;
        }
        DispatchEvent::ThreadDelete(data) => {
            handler.thread_delete(ctx, data).await;
        }
        DispatchEvent::ThreadListSync(data) => {
            handler.thread_list_sync(ctx, data).await;
        }
        DispatchEvent::ThreadMembersUpdate(data) => {
            handler.thread_members_update(ctx, data).await;
        }
        DispatchEvent::StageInstanceCreate(stage) => {
            handler.stage_instance_create(ctx, stage).await;
        }
        DispatchEvent::StageInstanceUpdate(stage) => {
            handler.stage_instance_update(ctx, stage).await;
        }
        DispatchEvent::StageInstanceDelete(stage) => {
            handler.stage_instance_delete(ctx, stage).await;
        }
        DispatchEvent::GuildScheduledEventCreate(event) => {
            handler.guild_scheduled_event_create(ctx, event).await;
        }
        DispatchEvent::GuildScheduledEventUpdate(event) => {
            handler.guild_scheduled_event_update(ctx, event).await;
        }
        DispatchEvent::GuildScheduledEventDelete(event) => {
            handler.guild_scheduled_event_delete(ctx, event).await;
        }
        DispatchEvent::GuildScheduledEventUserAdd(data) => {
            handler.guild_scheduled_event_user_add(ctx, data).await;
        }
        DispatchEvent::GuildScheduledEventUserRemove(data) => {
            handler.guild_scheduled_event_user_remove(ctx, data).await;
        }
        DispatchEvent::AutoModerationRuleCreate(rule) => {
            handler.auto_moderation_rule_create(ctx, rule).await;
        }
        DispatchEvent::AutoModerationRuleUpdate(rule) => {
            handler.auto_moderation_rule_update(ctx, rule).await;
        }
        DispatchEvent::AutoModerationRuleDelete(rule) => {
            handler.auto_moderation_rule_delete(ctx, rule).await;
        }
        DispatchEvent::AutoModerationActionExecution(data) => {
            handler.auto_moderation_action_execution(ctx, data).await;
        }
        DispatchEvent::PollVoteAdd(data) => {
            handler.poll_vote_add(ctx, data).await;
        }
        DispatchEvent::PollVoteRemove(data) => {
            handler.poll_vote_remove(ctx, data).await;
        }
        DispatchEvent::GuildSoundboardSoundCreate(sound) => {
            handler.guild_soundboard_sound_create(ctx, sound).await;
        }
        DispatchEvent::GuildSoundboardSoundUpdate(sound) => {
            handler.guild_soundboard_sound_update(ctx, sound).await;
        }
        DispatchEvent::GuildSoundboardSoundDelete(data) => {
            handler.guild_soundboard_sound_delete(ctx, data).await;
        }
        DispatchEvent::GuildEmojisUpdate(data) => {
            handler.guild_emojis_update(ctx, data).await;
        }
        DispatchEvent::GuildStickersUpdate(data) => {
            handler.guild_stickers_update(ctx, data).await;
        }
        DispatchEvent::GuildAuditLogEntryCreate(entry) => {
            handler.guild_audit_log_entry_create(ctx, entry).await;
        }
        DispatchEvent::GuildIntegrationsUpdate(data) => {
            handler.guild_integrations_update(ctx, data).await;
        }
        DispatchEvent::IntegrationCreate(integration) => {
            handler.integration_create(ctx, integration).await;
        }
        DispatchEvent::IntegrationUpdate(integration) => {
            handler.integration_update(ctx, integration).await;
        }
        DispatchEvent::IntegrationDelete(data) => {
            handler.integration_delete(ctx, data).await;
        }
        DispatchEvent::InviteCreate(data) => {
            handler.invite_create(ctx, data).await;
        }
        DispatchEvent::InviteDelete(data) => {
            handler.invite_delete(ctx, data).await;
        }
        DispatchEvent::GuildMembersChunk(data) => {
            handler.guild_members_chunk(ctx, data).await;
        }
        DispatchEvent::UserUpdate(user) => {
            handler.user_update(ctx, user).await;
        }
        DispatchEvent::ThreadMemberUpdate(data) => {
            handler.thread_member_update(ctx, data).await;
        }
        DispatchEvent::WebhooksUpdate(data) => {
            handler.webhooks_update(ctx, data).await;
        }
        DispatchEvent::VoiceChannelEffectSend(effect) => {
            handler.voice_channel_effect_send(ctx, effect).await;
        }
        DispatchEvent::ApplicationCommandPermissionsUpdate(data) => {
            handler
                .application_command_permissions_update(ctx, data)
                .await;
        }
        DispatchEvent::GuildSoundboardSoundsUpdate(data) => {
            handler.guild_soundboard_sounds_update(ctx, data).await;
        }
        DispatchEvent::SubscriptionCreate(sub) => {
            handler.subscription_create(ctx, sub).await;
        }
        DispatchEvent::SubscriptionUpdate(sub) => {
            handler.subscription_update(ctx, sub).await;
        }
        DispatchEvent::SubscriptionDelete(sub) => {
            handler.subscription_delete(ctx, sub).await;
        }
        other => {
            handler.raw_event(ctx, other).await;
        }
    }
    Ok(())
}
