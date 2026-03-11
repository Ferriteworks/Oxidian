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
