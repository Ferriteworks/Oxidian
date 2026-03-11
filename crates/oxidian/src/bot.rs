use std::{future::Future, sync::Arc};

use tokio::sync::mpsc;
use tracing::{debug, error};

use oxidian_core::{error::Result, intents::Intents, models::message::Message};
use oxidian_gateway::{events::DispatchEvent, Shard};
use oxidian_http::HttpClient;

use crate::{
    command::{Command, CommandRegistry, Module},
    context::{Context, GatewayHandle},
    handler::{DefaultHandler, EventHandler},
};

/// The top-level Discord bot.
pub struct Bot {
    token: String,
    intents: Intents,
    prefix: Option<String>,
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
    pub async fn start(self) -> Result<()> {
        let http = Arc::new(HttpClient::new(&self.token)?);

        let (event_tx, mut event_rx) = mpsc::channel::<DispatchEvent>(256);
        let shard = Shard::new(self.token.clone(), self.intents, event_tx);

        let gateway_handle = GatewayHandle::new(shard.gateway_sender());
        let ctx = Context::new(Arc::clone(&http), gateway_handle);

        // Drive the gateway on a separate task.
        tokio::spawn(async move {
            if let Err(e) = shard.start().await {
                error!(error = %e, "gateway shard exited with an error");
            }
        });

        // Dispatch events until the shard drops its sender side.
        while let Some(event) = event_rx.recv().await {
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

    /// Register a [`Module`] — a struct that groups prefix commands, slash
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
            handler: self.handler.unwrap_or_else(|| Arc::new(DefaultHandler)),
            commands: Arc::new(self.commands),
            modules: Arc::new(self.modules),
        }
    }
}

async fn dispatch(
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
            // Route to the first module whose slash_commands() names match
            // this interaction's command name; fall back to the global handler.
            let routed: Option<Arc<dyn Module>> = {
                let cmd_name = interaction.command_data().map(|d| d.name.clone());
                cmd_name.and_then(|name| {
                    modules
                        .iter()
                        .find(|m| m.slash_commands().iter().any(|c| c.name == name))
                        .map(Arc::clone)
                })
            };
            match routed {
                Some(module) => module.handle_interaction(ctx, interaction).await,
                None => handler.interaction(ctx, interaction).await,
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
