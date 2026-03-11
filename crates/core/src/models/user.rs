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

/// A Discord user account.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct User {
    /// The user's unique snowflake ID.
    pub id: Snowflake,
    /// The user's username (not unique across the platform).
    pub username: String,
    /// The user's four-digit tag. `"0"` for migrated accounts without a tag.
    #[serde(default)]
    pub discriminator: String,
    /// The user's display name, if one has been set.
    pub global_name: Option<String>,
    /// The user's avatar hash, or `None` if they have no custom avatar.
    pub avatar: Option<String>,
    /// Whether this user is a bot account.
    #[serde(default)]
    pub bot: bool,
    /// Whether this user is an official Discord system user.
    #[serde(default)]
    pub system: bool,
}
