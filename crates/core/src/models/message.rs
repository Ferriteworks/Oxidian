use serde::{Deserialize, Serialize};

use crate::snowflake::Snowflake;

use super::user::User;

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
