use serde::{Deserialize, Serialize};

use crate::snowflake::Snowflake;

/// A Discord channel.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Channel {
    /// The channel's unique snowflake ID.
    pub id: Snowflake,
    /// The channel type integer. See the
    /// [Discord docs](https://discord.com/developers/docs/resources/channel#channel-object-channel-types)
    /// for the full list of values.
    #[serde(rename = "type")]
    pub kind: u8,
    /// The guild this channel belongs to, if any.
    pub guild_id: Option<Snowflake>,
    /// The channel's name.
    pub name: Option<String>,
    /// The channel's topic, if set.
    pub topic: Option<String>,
    /// Whether the channel is age-restricted (NSFW).
    #[serde(default)]
    pub nsfw: bool,
    /// The ID of the last message sent in this channel, if any.
    pub last_message_id: Option<Snowflake>,
    /// The ID of the parent category or, for threads, the parent channel.
    pub parent_id: Option<Snowflake>,
}
