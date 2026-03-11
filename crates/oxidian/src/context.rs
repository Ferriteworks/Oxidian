use std::sync::Arc;

use oxidian_core::{
    error::{Error as OxidianError, HttpError, Result},
    models::{
        interaction::{Interaction, InteractionResponse},
        message::Message,
    },
    snowflake::Snowflake,
};
use oxidian_http::HttpClient;

/// A handle for sending outbound messages to the Discord gateway WebSocket.
///
/// Obtained via [`Context::gateway`].  Cheap to clone.
#[derive(Clone)]
pub struct GatewayHandle {
    tx: Arc<tokio::sync::broadcast::Sender<serde_json::Value>>,
}

impl GatewayHandle {
    pub(crate) fn new(
        tx: Arc<tokio::sync::broadcast::Sender<serde_json::Value>>,
    ) -> Self {
        Self { tx }
    }

    /// Join or leave a voice channel.
    ///
    /// Pass `channel_id = None` to disconnect from voice.
    ///
    /// After calling this, wait for both a [`VoiceStateUpdateData`] and a
    /// [`VoiceServerUpdateData`] event (both scoped to the same `guild_id`)
    /// before connecting to the voice WebSocket with
    /// [`VoiceConnection::connect`](oxidian_voice::connection::VoiceConnection::connect).
    pub fn join_voice(
        &self,
        guild_id: Snowflake,
        channel_id: Option<Snowflake>,
        self_mute: bool,
        self_deaf: bool,
    ) -> Result<()> {
        let payload = serde_json::json!({
            "op": 4u8,
            "d": {
                "guild_id":   guild_id.to_string(),
                "channel_id": channel_id.map(|id| id.to_string()),
                "self_mute":  self_mute,
                "self_deaf":  self_deaf,
            }
        });
        self.tx
            .send(payload)
            .map(|_| ())
            .map_err(|_| OxidianError::internal("gateway not connected"))
    }

    /// Send a raw JSON gateway payload for advanced use cases.
    pub fn send_raw(&self, payload: serde_json::Value) -> Result<()> {
        self.tx
            .send(payload)
            .map(|_| ())
            .map_err(|_| OxidianError::internal("gateway not connected"))
    }
}

/// A request context passed to every event handler and command callback.
///
/// Provides ergonomic access to the HTTP client and the gateway so handlers
/// can respond to events without holding global references.
///
/// `Context` is cheap to clone — both inner fields are `Arc`-backed.
#[derive(Clone)]
pub struct Context {
    /// The HTTP client for Discord REST API calls.
    pub http: Arc<HttpClient>,
    /// A handle for sending outbound gateway commands (e.g. voice state update).
    pub gateway: GatewayHandle,
}

impl Context {
    pub(crate) fn new(http: Arc<HttpClient>, gateway: GatewayHandle) -> Self {
        Self { http, gateway }
    }

    /// Send a message to a channel.
    ///
    /// ```rust,ignore
    /// ctx.send(channel_id, "Hello, world!").await?;
    /// ```
    pub async fn send(
        &self,
        channel_id: Snowflake,
        content: impl Into<String>,
    ) -> Result<Message> {
        let body = serde_json::json!({ "content": content.into() });
        let value = self.http.create_message(channel_id, body).await?;
        serde_json::from_value(value).map_err(|e| {
            HttpError::Decode(format!("failed to deserialize sent message: {e}")).into()
        })
    }

    /// Reply to an existing message by sending to the same channel.
    ///
    /// ```rust,ignore
    /// ctx.reply(&msg, "pong!").await?;
    /// ```
    pub async fn reply(
        &self,
        msg: &Message,
        content: impl Into<String>,
    ) -> Result<Message> {
        self.send(msg.channel_id, content).await
    }

    /// Respond to a Discord interaction.
    ///
    /// Calls `POST /interactions/{id}/{token}/callback`.
    /// Discord returns `204 No Content` on success.
    ///
    /// ```rust,ignore
    /// ctx.respond(&interaction, InteractionResponse::message("Pong!")).await?;
    /// ```
    pub async fn respond(
        &self,
        interaction: &Interaction,
        response: InteractionResponse,
    ) -> Result<()> {
        let body =
            serde_json::to_value(&response).map_err(OxidianError::Serialization)?;
        self.http
            .create_interaction_response(interaction.id, &interaction.token, body)
            .await
    }
}
