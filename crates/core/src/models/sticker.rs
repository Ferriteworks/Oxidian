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

/// A Discord sticker.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Sticker {
    /// The sticker's unique ID.
    pub id: Snowflake,
    /// ID of the pack the sticker is from (standard stickers only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pack_id: Option<Snowflake>,
    /// Name of the sticker.
    pub name: String,
    /// Description of the sticker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Autocomplete/suggestion tags for the sticker (max 200 characters, comma-separated).
    #[serde(default)]
    pub tags: String,
    /// Type of sticker: 1 = STANDARD, 2 = GUILD.
    #[serde(rename = "type")]
    pub kind: u8,
    /// Format type: 1 = PNG, 2 = APNG, 3 = LOTTIE, 4 = GIF.
    pub format_type: u8,
    /// Whether this guild sticker can be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available: Option<bool>,
    /// ID of the guild that owns this sticker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<Snowflake>,
    /// The user that uploaded the guild sticker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<super::user::User>,
    /// The standard sticker's sort order within its pack.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_value: Option<u32>,
}

/// A partial sticker item included in messages.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StickerItem {
    /// The sticker's unique ID.
    pub id: Snowflake,
    /// Name of the sticker.
    pub name: String,
    /// Format type: 1 = PNG, 2 = APNG, 3 = LOTTIE, 4 = GIF.
    pub format_type: u8,
}
