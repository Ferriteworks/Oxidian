use serde::{Deserialize, Serialize};

use crate::snowflake::Snowflake;

use super::user::User;

/// A Discord sticker that can be sent in messages.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Sticker {
    /// The sticker's snowflake ID.
    pub id: Snowflake,
    /// The name of the sticker.
    pub name: String,
    /// Description of the sticker.
    pub description: Option<String>,
    /// Autocomplete/suggestion tags for the sticker (max 200 characters).
    pub tags: String,
    /// Sticker type: 1 = STANDARD (Discord-owned), 2 = GUILD (uploaded).
    #[serde(rename = "type")]
    pub kind: u8,
    /// Format type: 1 = PNG, 2 = APNG, 3 = LOTTIE, 4 = GIF.
    pub format_type: u8,
    /// Whether the sticker is available for use.
    pub available: Option<bool>,
    /// The guild that owns this sticker.  `None` for standard stickers.
    pub guild_id: Option<Snowflake>,
    /// The user who uploaded the sticker, if included.
    pub user: Option<User>,
    /// The sticker's sort order within its pack.
    pub sort_value: Option<u32>,
}
