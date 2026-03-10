use serde::{Deserialize, Serialize};

use crate::snowflake::Snowflake;

use super::user::User;

/// A guild member — the relationship between a [`User`] and a guild.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Member {
    /// The underlying user, if included in the payload.
    pub user: Option<User>,
    /// The member's guild-specific nickname, if set.
    pub nick: Option<String>,
    /// The member's guild-specific avatar hash, if set.
    pub avatar: Option<String>,
    /// IDs of the roles assigned to this member.
    #[serde(default)]
    pub roles: Vec<Snowflake>,
    /// ISO 8601 timestamp of when this user joined the guild.
    pub joined_at: String,
    /// Whether the member has been server-deafened in voice channels.
    #[serde(default)]
    pub deaf: bool,
    /// Whether the member has been server-muted in voice channels.
    #[serde(default)]
    pub mute: bool,
    /// Whether the member has not yet passed the guild's membership screening.
    #[serde(default)]
    pub pending: bool,
}
