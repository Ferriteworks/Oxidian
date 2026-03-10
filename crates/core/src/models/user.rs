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
