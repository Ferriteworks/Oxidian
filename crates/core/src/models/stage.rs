use serde::{Deserialize, Serialize};

use crate::snowflake::Snowflake;

/// A live stage instance in a Stage channel.
///
/// Stage instances are created when a moderator starts a Stage and destroyed
/// when the Stage ends.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StageInstance {
    /// The stage instance's snowflake ID.
    pub id: Snowflake,
    /// The guild the Stage lives in.
    pub guild_id: Snowflake,
    /// The Stage channel this instance is associated with.
    pub channel_id: Snowflake,
    /// The topic of the Stage instance (1–120 characters).
    pub topic: String,
    /// Privacy level.  2 = GUILD_ONLY (the only non-deprecated value).
    pub privacy_level: u8,
    /// The ID of the scheduled event that created this Stage, if any.
    pub guild_scheduled_event_id: Option<Snowflake>,
}
