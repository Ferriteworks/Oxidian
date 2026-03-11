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

/// A Discord emoji — either a custom guild emoji or a standard Unicode emoji.
///
/// When used in reaction or activity payloads, `id` is `None` for standard
/// Unicode emoji and `name` holds the Unicode character.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Emoji {
    /// The emoji's snowflake ID.  `None` for standard Unicode emoji.
    pub id: Option<Snowflake>,
    /// The emoji name, or the Unicode character for standard emoji.
    pub name: Option<String>,
    /// Role IDs that are allowed to use this emoji.
    #[serde(default)]
    pub roles: Vec<Snowflake>,
    /// The user who uploaded this emoji, if included.
    pub user: Option<User>,
    /// Whether this emoji must be wrapped in colons.
    pub require_colons: Option<bool>,
    /// Whether this emoji is managed by an integration.
    pub managed: Option<bool>,
    /// Whether this emoji is animated.
    pub animated: Option<bool>,
    /// Whether this emoji can be used (may be `false` if the guild lost boosts).
    pub available: Option<bool>,
}
