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

use super::user::User;

/// A guild member — the relationship between a [`User`] and a guild.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Member {
    /// The underlying user, if included in the payload.
    pub user: Option<User>,
    /// The member's guild-specific nickname, if set.
    pub nick: Option<String>,
    /// The member's guild-specific avatar hash, if set.
    pub avatar: Option<String>,
    /// IDs of the roles assigned to this member.
    #[serde(default)]
    pub roles: Vec<Snowflake>,
    /// ISO 8601 timestamp of when this user joined the guild.
    pub joined_at: String,
    /// Whether the member has been server-deafened in voice channels.
    #[serde(default)]
    pub deaf: bool,
    /// Whether the member has been server-muted in voice channels.
    #[serde(default)]
    pub mute: bool,
    /// Whether the member has not yet passed the guild's membership screening.
    #[serde(default)]
    pub pending: bool,
}
