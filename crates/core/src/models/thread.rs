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

/// Thread-specific metadata attached to a thread channel.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ThreadMetadata {
    /// Whether the thread is archived.
    pub archived: bool,
    /// Duration in minutes before the thread is auto-archived.  One of 60, 1440, 4320, 10080.
    pub auto_archive_duration: u32,
    /// ISO 8601 timestamp of when the thread was archived (or its last activity).
    pub archive_timestamp: String,
    /// Whether the thread is locked; only moderators can un-archive a locked thread.
    pub locked: bool,
    /// Whether non-moderators can invite other members to a private thread.
    pub invitable: Option<bool>,
    /// ISO 8601 creation timestamp (only present for threads created after 2022-01-09).
    pub create_timestamp: Option<String>,
}

/// A member's participation record in a thread.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ThreadMember {
    /// The ID of the thread.  Absent when retrieved via `GET /users/@me/threads/archived/private`.
    pub id: Option<Snowflake>,
    /// The ID of the user.  Absent in the same scenario.
    pub user_id: Option<Snowflake>,
    /// ISO 8601 timestamp of when the user joined the thread.
    pub join_timestamp: String,
    /// Any user-thread settings bitfield (currently unused by Discord).
    pub flags: u64,
}

/// A Discord thread channel (news thread, public thread, or private thread).
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Thread {
    /// The thread's snowflake ID.
    pub id: Snowflake,
    /// The guild the thread belongs to.
    pub guild_id: Option<Snowflake>,
    /// The parent channel the thread was created in.
    pub parent_id: Option<Snowflake>,
    /// The user who created the thread.
    pub owner_id: Option<Snowflake>,
    /// The thread name.
    pub name: String,
    /// Channel type: 10 = NEWS_THREAD, 11 = PUBLIC_THREAD, 12 = PRIVATE_THREAD.
    #[serde(rename = "type")]
    pub kind: u8,
    /// Approximate member count, capped at 50.
    pub member_count: Option<u32>,
    /// Approximate message count, capped at 50.
    pub message_count: Option<u32>,
    /// Total number of messages ever sent in the thread (not decremented by deletes).
    pub total_message_sent: Option<u32>,
    /// Thread-specific metadata.
    pub thread_metadata: Option<ThreadMetadata>,
    /// IDs of tags applied to a forum/media thread (max 5).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub applied_tags: Vec<Snowflake>,
    /// Channel flags bitfield.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<u64>,
    /// Slowmode: seconds a user must wait between messages.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limit_per_user: Option<u32>,
    /// The ID of the last message sent in this thread.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_message_id: Option<Snowflake>,
}

/// Channel type for new thread creation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(into = "u8", try_from = "u8")]
pub enum ThreadKind {
    /// Public thread (type 11).
    Public,
    /// Private thread (type 12).
    Private,
    /// News thread (type 10).
    News,
}

impl From<ThreadKind> for u8 {
    fn from(kind: ThreadKind) -> u8 {
        match kind {
            ThreadKind::News => 10,
            ThreadKind::Public => 11,
            ThreadKind::Private => 12,
        }
    }
}

impl TryFrom<u8> for ThreadKind {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            10 => Ok(Self::News),
            11 => Ok(Self::Public),
            12 => Ok(Self::Private),
            _ => Err("unknown thread type"),
        }
    }
}

/// Request body for `POST /channels/{channel.id}/threads`.
///
/// Use [`ThreadCreateOptions::new`] to start with required fields and chain
/// setters to fill in the optional ones.
#[derive(Debug, Clone, Serialize)]
pub struct ThreadCreateOptions {
    /// 1-100 character thread name.
    pub name: String,
    /// Thread type to create.
    #[serde(rename = "type")]
    pub kind: ThreadKind,
    /// Auto-archive duration in minutes (60, 1440, 4320, 10080).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_archive_duration: Option<u32>,
    /// Slowmode in seconds (0-21600).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limit_per_user: Option<u32>,
    /// Whether non-moderators can add other non-moderators. Private threads only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invitable: Option<bool>,
}

impl ThreadCreateOptions {
    /// Create a new builder with the required name and thread type.
    pub fn new(name: impl Into<String>, kind: ThreadKind) -> Self {
        Self {
            name: name.into(),
            kind,
            auto_archive_duration: None,
            rate_limit_per_user: None,
            invitable: None,
        }
    }

    /// Set the auto-archive duration. One of 60, 1440, 4320, 10080.
    pub fn auto_archive_duration(mut self, minutes: u32) -> Self {
        self.auto_archive_duration = Some(minutes);
        self
    }

    /// Set slowmode in seconds (0-21600).
    pub fn rate_limit_per_user(mut self, seconds: u32) -> Self {
        self.rate_limit_per_user = Some(seconds);
        self
    }

    /// Allow non-moderators to invite others. Only meaningful for private threads.
    pub fn invitable(mut self, invitable: bool) -> Self {
        self.invitable = Some(invitable);
        self
    }
}
