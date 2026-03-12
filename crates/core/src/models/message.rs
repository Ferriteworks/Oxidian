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

use super::{
    allowed_mentions::AllowedMentions, embed::Embed, emoji::Emoji, user::User,
};

/// A file attached to a Discord message.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Attachment {
    pub id: Snowflake,
    pub filename: String,
    pub size: u64,
    pub url: String,
    pub proxy_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// A reaction entry on a Discord message.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Reaction {
    pub count: u32,
    #[serde(default)]
    pub me: bool,
    pub emoji: Emoji,
}

// ---------------------------------------------------------------------------
// MessageReference — for replies and forwards
// ---------------------------------------------------------------------------

/// Controls what type of reference a [`MessageReference`] is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "u8", into = "u8")]
pub enum MessageReferenceType {
    /// A standard reply (default).
    Default = 0,
    /// A forwarded message.
    Forward = 1,
}

impl TryFrom<u8> for MessageReferenceType {
    type Error = String;
    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(Self::Default),
            1 => Ok(Self::Forward),
            _ => Err(format!("unknown message reference type: {v}")),
        }
    }
}

impl From<MessageReferenceType> for u8 {
    fn from(t: MessageReferenceType) -> u8 {
        t as u8
    }
}

impl Default for MessageReferenceType {
    fn default() -> Self {
        Self::Default
    }
}

/// A reference to another message (for replies and forwards).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageReference {
    /// The type of reference (reply or forward).
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<MessageReferenceType>,
    /// The ID of the message being referenced.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<Snowflake>,
    /// The channel the referenced message exists in.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<Snowflake>,
    /// The guild the referenced message exists in.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<Snowflake>,
    /// Whether to error if the referenced message doesn't exist (default `true`).
    /// Only applies to replies.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fail_if_not_exists: Option<bool>,
}

/// A snapshot of a forwarded message.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MessageSnapshot {
    /// Partial message object — contains content, embeds, attachments, etc.
    pub message: MessageSnapshotData,
}

/// Partial message data included in a forwarded message snapshot.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MessageSnapshotData {
    /// The message content.
    #[serde(default)]
    pub content: String,
    /// Embeds from the original message.
    #[serde(default)]
    pub embeds: Vec<Embed>,
    /// Attachments from the original message.
    #[serde(default)]
    pub attachments: Vec<Attachment>,
    /// Timestamp of the original message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    /// Timestamp when the original message was edited.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edited_timestamp: Option<String>,
    /// Message flags from the original message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<u64>,
    /// Sticker items from the original message.
    #[serde(default)]
    pub sticker_items: Vec<serde_json::Value>,
    /// Components from the original message.
    #[serde(default)]
    pub components: Vec<serde_json::Value>,
}

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
    /// Message flags bitfield.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<u64>,
    /// Reference data for replies / forwards.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_reference: Option<MessageReference>,
    /// The message this one is replying to (partial, only for replies).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referenced_message: Option<Box<Message>>,
    /// Snapshots of forwarded messages.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub message_snapshots: Vec<MessageSnapshot>,
    /// Embeds attached to this message.
    #[serde(default)]
    pub embeds: Vec<Embed>,
    /// File attachments.
    #[serde(default)]
    pub attachments: Vec<Attachment>,
    /// Components attached to this message.
    #[serde(default)]
    pub components: Vec<serde_json::Value>,
    /// Sticker items sent with the message.
    #[serde(default)]
    pub sticker_items: Vec<serde_json::Value>,
    /// Reactions on the message.
    #[serde(default)]
    pub reactions: Vec<Reaction>,
    /// The thread that was started from this message, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread: Option<serde_json::Value>,
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
    /// Reference to another message (for replies and forwards).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_reference: Option<MessageReference>,
    /// Sticker IDs to include (max 3).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sticker_ids: Vec<Snowflake>,
    /// Allowed mentions configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_mentions: Option<AllowedMentions>,
    /// Whether this is a TTS message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tts: Option<bool>,
    /// A poll attached to this message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poll: Option<serde_json::Value>,
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

    /// Reply to a message. Sets a DEFAULT message reference.
    pub fn reply(mut self, message_id: Snowflake, fail_if_not_exists: bool) -> Self {
        self.message_reference = Some(MessageReference {
            kind: Some(MessageReferenceType::Default),
            message_id: Some(message_id),
            channel_id: None,
            guild_id: None,
            fail_if_not_exists: Some(fail_if_not_exists),
        });
        self
    }

    /// Forward a message. Sets a FORWARD message reference.
    pub fn forward(mut self, message_id: Snowflake, channel_id: Snowflake) -> Self {
        self.message_reference = Some(MessageReference {
            kind: Some(MessageReferenceType::Forward),
            message_id: Some(message_id),
            channel_id: Some(channel_id),
            guild_id: None,
            fail_if_not_exists: None,
        });
        self
    }

    /// Set a raw message reference.
    pub fn message_reference(mut self, reference: MessageReference) -> Self {
        self.message_reference = Some(reference);
        self
    }

    /// Set allowed mentions.
    pub fn allowed_mentions(mut self, mentions: AllowedMentions) -> Self {
        self.allowed_mentions = Some(mentions);
        self
    }
}
