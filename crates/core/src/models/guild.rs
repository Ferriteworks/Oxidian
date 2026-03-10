use serde::{Deserialize, Serialize};

use crate::snowflake::Snowflake;

/// A Discord guild (server).
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Guild {
    /// The guild's unique snowflake ID.
    pub id: Snowflake,
    /// The guild's name.
    pub name: String,
    /// The guild's icon hash.
    pub icon: Option<String>,
    /// The snowflake ID of the guild owner.
    pub owner_id: Snowflake,
    /// Number of members in the guild, if provided.
    pub member_count: Option<u32>,
    /// Approximate member count, populated on GUILD_CREATE.
    pub approximate_member_count: Option<u32>,
    /// Whether this guild is currently unavailable due to a Discord outage.
    #[serde(default)]
    pub unavailable: bool,
}
