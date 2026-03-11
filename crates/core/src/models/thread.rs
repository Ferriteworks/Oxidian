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
    /// Thread-specific metadata.
    pub thread_metadata: Option<ThreadMetadata>,
}
