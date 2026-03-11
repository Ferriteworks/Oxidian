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
