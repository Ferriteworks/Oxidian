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
