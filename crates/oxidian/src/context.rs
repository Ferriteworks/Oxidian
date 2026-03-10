use std::sync::Arc;

use oxidian_core::{
    error::{HttpError, Result},
    models::message::Message,
    snowflake::Snowflake,
};
use oxidian_http::HttpClient;

/// A request context passed to every event handler and command callback.
///
/// Provides ergonomic access to the HTTP client so handlers can respond to
/// events without holding a global reference to the client.
///
/// `Context` is cheap to clone — the HTTP client is wrapped in an [`Arc`].
#[derive(Clone)]
pub struct Context {
    /// The HTTP client for making Discord REST API calls.
    pub http: Arc<HttpClient>,
}

impl Context {
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
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
    pub async fn reply(&self, msg: &Message, content: impl Into<String>) -> Result<Message> {
        self.send(msg.channel_id, content).await
    }
}
