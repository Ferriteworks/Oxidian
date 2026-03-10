use serde::{Deserialize, Serialize};

use crate::snowflake::Snowflake;

/// A Discord guild role.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Role {
    /// The role's unique snowflake ID.
    pub id: Snowflake,
    /// The role's display name.
    pub name: String,
    /// The role's display colour as a packed RGB integer (`0xRRGGBB`).
    #[serde(default)]
    pub color: u32,
    /// Whether members with this role are shown separately in the member list.
    #[serde(default)]
    pub hoist: bool,
    /// The role's position in the hierarchy (higher = more powerful).
    #[serde(default)]
    pub position: i32,
    /// The permissions bit-string for this role.
    pub permissions: String,
    /// Whether this role is managed by an integration and cannot be manually assigned.
    #[serde(default)]
    pub managed: bool,
    /// Whether anyone can mention this role with `@role`.
    #[serde(default)]
    pub mentionable: bool,
}
