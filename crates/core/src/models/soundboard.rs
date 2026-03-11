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

/// A Discord soundboard sound.
///
/// Soundboard sounds can be guild-specific or one of Discord's built-in
/// default sounds (which have no `guild_id`).
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SoundboardSound {
    /// The name of the sound.
    pub name: String,
    /// The sound's unique ID.
    pub sound_id: Snowflake,
    /// The playback volume (0.0–1.0).
    pub volume: f64,
    /// Custom emoji ID associated with the sound, if any.
    pub emoji_id: Option<Snowflake>,
    /// Standard emoji name associated with the sound, if any.
    pub emoji_name: Option<String>,
    /// The guild the sound belongs to.  `None` for default sounds.
    pub guild_id: Option<Snowflake>,
    /// Whether the sound can be used.
    pub available: bool,
    /// The user who created the sound, if included in the payload.
    pub user: Option<User>,
}
