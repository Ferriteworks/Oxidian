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

use super::{channel::Channel, guild::Guild, user::User};

/// The type of target for a voice-channel invite.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct InviteTargetType(pub u8);
// 1 = STREAM, 2 = EMBEDDED_APPLICATION

/// A Discord invite, as returned by the REST API or gateway events.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Invite {
    /// The unique invite code (not a Snowflake).
    pub code: String,
    /// The guild the invite is for, if applicable.
    pub guild: Option<Guild>,
    /// The channel the invite leads to.
    pub channel: Option<Channel>,
    /// The user who created the invite.
    pub inviter: Option<User>,
    /// The target user for a stream invite.
    pub target_user: Option<User>,
    /// Target type for a voice-channel invite.
    pub target_type: Option<InviteTargetType>,
    /// Approximate number of online members (only present with `with_counts`).
    pub approximate_presence_count: Option<u32>,
    /// Approximate total member count (only present with `with_counts`).
    pub approximate_member_count: Option<u32>,
    /// ISO 8601 expiry timestamp, if the invite has one.
    pub expires_at: Option<String>,
}
