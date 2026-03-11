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

/// A Discord guild (server).
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Guild {
    /// The guild's unique snowflake ID.
    pub id: Snowflake,
    /// The guild's name.
    pub name: String,
    /// The guild's icon hash.
    pub icon: Option<String>,
    /// The snowflake ID of the guild owner.
    pub owner_id: Snowflake,
    /// Number of members in the guild, if provided.
    pub member_count: Option<u32>,
    /// Approximate member count, populated on GUILD_CREATE.
    pub approximate_member_count: Option<u32>,
    /// Whether this guild is currently unavailable due to a Discord outage.
    #[serde(default)]
    pub unavailable: bool,
}
