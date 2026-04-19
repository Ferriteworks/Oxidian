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

use std::sync::Arc;

use oxidian_core::{
    error::{Error as OxidianError, HttpError, Result},
    models::{
        interaction::{Interaction, InteractionResponse},
        message::{CreateMessage, Message},
    },
    snowflake::Snowflake,
};
use oxidian_http::HttpClient;

#[cfg(feature = "cache")]
use oxidian_cache::Cache;

/// A handle for sending outbound messages to the Discord gateway WebSocket.
///
/// Obtained via [`Context::gateway`].  Cheap to clone.
#[derive(Clone)]
pub struct GatewayHandle {
    tx: Arc<tokio::sync::broadcast::Sender<serde_json::Value>>,
}

impl GatewayHandle {
    /// Create a new `GatewayHandle` from a broadcast sender.
    ///
    /// This is useful when building a custom event loop without `Bot`.
    pub fn new(tx: Arc<tokio::sync::broadcast::Sender<serde_json::Value>>) -> Self {
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
/// `Context` is cheap to clone: both inner fields are `Arc`-backed.
#[derive(Clone)]
pub struct Context {
    /// The HTTP client for Discord REST API calls.
    pub http: Arc<HttpClient>,
    /// A handle for sending outbound gateway commands (e.g. voice state update).
    pub gateway: GatewayHandle,
    /// The in-memory cache, when the `cache` feature is enabled.
    #[cfg(feature = "cache")]
    pub cache: Arc<Cache>,
}

impl Context {
    /// Create a new `Context`.
    ///
    /// This is useful when building a custom event loop without `Bot`.
    #[cfg(feature = "cache")]
    pub fn new(
        http: Arc<HttpClient>,
        gateway: GatewayHandle,
        cache: Arc<Cache>,
    ) -> Self {
        Self {
            http,
            gateway,
            cache,
        }
    }

    /// Create a new `Context`.
    ///
    /// This is useful when building a custom event loop without `Bot`.
    #[cfg(not(feature = "cache"))]
    pub fn new(http: Arc<HttpClient>, gateway: GatewayHandle) -> Self {
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

    /// Send a structured message payload to a channel.
    ///
    /// Use this when you need embeds, components, or flags. For plain text,
    /// [`send`](Self::send) is simpler.
    ///
    /// ```rust,ignore
    /// use oxidian::core::models::message::CreateMessage;
    /// use oxidian::core::models::embed::EmbedBuilder;
    ///
    /// let msg = CreateMessage::new()
    ///     .content("Look at this embed!")
    ///     .embed(EmbedBuilder::new().title("Hi").color(0x00FF00).build());
    /// ctx.send_message(channel_id, msg).await?;
    /// ```
    pub async fn send_message(
        &self,
        channel_id: Snowflake,
        message: CreateMessage,
    ) -> Result<Message> {
        let body =
            serde_json::to_value(&message).map_err(OxidianError::Serialization)?;
        let value = self.http.create_message(channel_id, body).await?;
        serde_json::from_value(value).map_err(|e| {
            HttpError::Decode(format!("failed to deserialize sent message: {e}")).into()
        })
    }

    /// Edit the original response to a deferred interaction.
    ///
    /// ```rust,ignore
    /// // In the handler, first defer:
    /// ctx.respond(&interaction, InteractionResponse::defer()).await?;
    ///
    /// // ... do async work ...
    ///
    /// // Then edit the deferred response:
    /// let msg = CreateMessage::new().content("Done!");
    /// ctx.edit_response(&interaction, msg).await?;
    /// ```
    pub async fn edit_response(
        &self,
        interaction: &Interaction,
        message: CreateMessage,
    ) -> Result<Message> {
        let body =
            serde_json::to_value(&message).map_err(OxidianError::Serialization)?;
        let value = self
            .http
            .edit_original_interaction_response(
                interaction.application_id,
                &interaction.token,
                body,
            )
            .await?;
        serde_json::from_value(value).map_err(|e| {
            HttpError::Decode(format!("failed to deserialize edited response: {e}"))
                .into()
        })
    }
}
