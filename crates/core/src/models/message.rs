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

use serde::{Deserialize, Serialize};

use crate::snowflake::Snowflake;

use super::{embed::Embed, user::User};

/// A Discord message.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Message {
    /// The message's unique snowflake ID.
    pub id: Snowflake,
    /// The channel this message was sent in.
    pub channel_id: Snowflake,
    /// The guild this message was sent in, if any.
    pub guild_id: Option<Snowflake>,
    /// The user who authored this message.
    pub author: User,
    /// The message content as a plain string.
    pub content: String,
    /// ISO 8601 timestamp of when the message was sent.
    pub timestamp: String,
    /// ISO 8601 timestamp of when the message was last edited, if ever.
    pub edited_timestamp: Option<String>,
    /// Whether this was a text-to-speech message.
    #[serde(default)]
    pub tts: bool,
    /// Whether this message mentioned `@everyone` or `@here`.
    #[serde(default)]
    pub mention_everyone: bool,
    /// Users explicitly mentioned in this message.
    #[serde(default)]
    pub mentions: Vec<User>,
    /// Whether this message is pinned in its channel.
    #[serde(default)]
    pub pinned: bool,
    /// The message type integer.
    #[serde(rename = "type", default)]
    pub kind: u8,
}

// ---------------------------------------------------------------------------
// CreateMessage — outgoing payload for REST message creation
// ---------------------------------------------------------------------------

/// Payload for creating a new message via the Discord REST API.
///
/// Build one fluently and pass it to [`Context::send_message`]:
///
/// ```rust,ignore
/// use oxidian_core::models::message::CreateMessage;
/// use oxidian_core::models::embed::EmbedBuilder;
///
/// let msg = CreateMessage::new()
///     .content("Check this out!")
///     .embed(EmbedBuilder::new().title("Hello").color(0x5865F2).build());
/// ctx.send_message(channel_id, msg).await?;
/// ```
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateMessage {
    /// Text content of the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// Up to 10 embeds.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub embeds: Vec<Embed>,
    /// Top-level components (action rows for v1, or v2 layout primitives).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub components: Vec<serde_json::Value>,
    /// Message flags (e.g. `64` for ephemeral, `32768` for components v2).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<u64>,
}

impl CreateMessage {
    /// Start building a new message payload.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the text content.
    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.content = Some(content.into());
        self
    }

    /// Append an embed.
    pub fn embed(mut self, embed: Embed) -> Self {
        self.embeds.push(embed);
        self
    }

    /// Append a component (serialized to JSON).
    ///
    /// Accepts any type that implements [`serde::Serialize`] — for example
    /// `ActionRow`, `Container`, etc.
    pub fn component(mut self, component: impl serde::Serialize) -> Self {
        let val = serde_json::to_value(component)
            .expect("component serialization should not fail");
        self.components.push(val);
        self
    }

    /// Set raw message flags.
    pub fn flags(mut self, flags: u64) -> Self {
        self.flags = Some(flags);
        self
    }

    /// Enable components v2 layout mode (sets the `IS_COMPONENTS_V2` flag).
    pub fn components_v2(mut self) -> Self {
        let f = self.flags.unwrap_or(0);
        self.flags = Some(f | (1 << 15));
        self
    }
}
