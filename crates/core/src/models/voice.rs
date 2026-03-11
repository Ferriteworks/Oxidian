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

use super::emoji::Emoji;

/// The type of effect sent in a voice channel.
///
/// 1 = SOUNDBOARD, 5 = EMOJI_REACTION
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct VoiceChannelEffectType(pub u8);

/// Payload received when a user triggers a voice-channel effect (emoji or soundboard).
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VoiceChannelEffect {
    /// The channel the effect was sent in.
    pub channel_id: Snowflake,
    /// The guild the channel belongs to.
    pub guild_id: Snowflake,
    /// The user who sent the effect.
    pub user_id: Snowflake,
    /// The emoji used for `EMOJI_REACTION` effects.
    pub emoji: Option<Emoji>,
    /// Animation type: 0 = PREMIUM, 1 = BASIC.
    pub animation_type: Option<u8>,
    /// The ID of the animation used (if any).
    pub animation_id: Option<u32>,
    /// The ID of the soundboard sound used (if effect is SOUNDBOARD).
    pub sound_id: Option<Snowflake>,
    /// The volume of the soundboard sound (if effect is SOUNDBOARD).
    pub sound_volume: Option<f64>,
}
