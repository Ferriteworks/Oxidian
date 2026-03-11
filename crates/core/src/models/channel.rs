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
