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

/// The type of a Discord channel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "u8", into = "u8")]
pub enum ChannelType {
    GuildText = 0,
    Dm = 1,
    GuildVoice = 2,
    GroupDm = 3,
    GuildCategory = 4,
    GuildAnnouncement = 5,
    AnnouncementThread = 10,
    PublicThread = 11,
    PrivateThread = 12,
    GuildStageVoice = 13,
    GuildDirectory = 14,
    GuildForum = 15,
    GuildMedia = 16,
}

impl TryFrom<u8> for ChannelType {
    type Error = String;
    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(Self::GuildText),
            1 => Ok(Self::Dm),
            2 => Ok(Self::GuildVoice),
            3 => Ok(Self::GroupDm),
            4 => Ok(Self::GuildCategory),
            5 => Ok(Self::GuildAnnouncement),
            10 => Ok(Self::AnnouncementThread),
            11 => Ok(Self::PublicThread),
            12 => Ok(Self::PrivateThread),
            13 => Ok(Self::GuildStageVoice),
            14 => Ok(Self::GuildDirectory),
            15 => Ok(Self::GuildForum),
            16 => Ok(Self::GuildMedia),
            _ => Err(format!("unknown channel type: {v}")),
        }
    }
}

impl From<ChannelType> for u8 {
    fn from(v: ChannelType) -> u8 {
        v as u8
    }
}

/// A tag available in a forum or media channel.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ForumTag {
    /// The tag's snowflake ID.
    pub id: Snowflake,
    /// The tag name (0–20 characters).
    pub name: String,
    /// Whether this tag can only be added/removed by moderators.
    #[serde(default)]
    pub moderated: bool,
    /// The ID of the custom emoji for this tag.
    pub emoji_id: Option<Snowflake>,
    /// The Unicode emoji for this tag (if not using a custom emoji).
    pub emoji_name: Option<String>,
}

/// The default reaction emoji shown on forum posts.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DefaultReaction {
    /// The custom emoji ID, if using a custom emoji.
    pub emoji_id: Option<Snowflake>,
    /// The Unicode emoji, if using a standard emoji.
    pub emoji_name: Option<String>,
}

/// A Discord channel.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Channel {
    /// The channel's unique snowflake ID.
    pub id: Snowflake,
    /// The channel type.
    #[serde(rename = "type")]
    pub kind: ChannelType,
    /// The guild this channel belongs to, if any.
    pub guild_id: Option<Snowflake>,
    /// The channel's display position.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<u32>,
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
    /// Slowmode: seconds a user must wait between messages (0–21600).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limit_per_user: Option<u32>,
    /// Channel flags bitfield.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<u64>,
    /// Permission overwrites for this channel.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub permission_overwrites: Vec<serde_json::Value>,
    // ── Forum / Media channel fields ────────────────────────────────
    /// Tags available in a forum or media channel.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub available_tags: Vec<ForumTag>,
    /// The default reaction emoji shown on new forum posts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_reaction_emoji: Option<DefaultReaction>,
    /// Default thread auto-archive duration (minutes) for newly created threads.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_auto_archive_duration: Option<u32>,
    /// Default slowmode for newly created threads.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_thread_rate_limit_per_user: Option<u32>,
    /// The default sort order for a forum channel.
    /// `0` = LATEST_ACTIVITY, `1` = CREATION_DATE.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_sort_order: Option<u8>,
    /// The default forum layout view.
    /// `0` = NOT_SET, `1` = LIST_VIEW, `2` = GALLERY_VIEW.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_forum_layout: Option<u8>,
}
