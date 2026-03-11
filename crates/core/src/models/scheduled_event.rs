use serde::{Deserialize, Serialize};

use crate::snowflake::Snowflake;

/// Location metadata for an EXTERNAL scheduled event.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ScheduledEventEntityMetadata {
    /// The external location string (required for EXTERNAL events).
    pub location: Option<String>,
}

/// A guild scheduled event.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ScheduledEvent {
    /// The event's snowflake ID.
    pub id: Snowflake,
    /// The guild the event belongs to.
    pub guild_id: Snowflake,
    /// The Stage or voice channel hosting the event, if applicable.
    pub channel_id: Option<Snowflake>,
    /// The user that created the event.
    pub creator_id: Option<Snowflake>,
    /// The name of the event (1–100 characters).
    pub name: String,
    /// Optional description (1–1000 characters).
    pub description: Option<String>,
    /// ISO 8601 timestamp for when the event is scheduled to start.
    pub scheduled_start_time: String,
    /// ISO 8601 timestamp for when the event is scheduled to end, if set.
    pub scheduled_end_time: Option<String>,
    /// 2 = GUILD_ONLY (the only non-deprecated privacy level).
    pub privacy_level: u8,
    /// 1 = SCHEDULED, 2 = ACTIVE, 3 = COMPLETED, 4 = CANCELLED.
    pub status: u8,
    /// 1 = STAGE_INSTANCE, 2 = VOICE, 3 = EXTERNAL.
    pub entity_type: u8,
    /// The ID of the entity associated with the event.
    pub entity_id: Option<Snowflake>,
    /// Additional entity metadata (required when `entity_type` is EXTERNAL).
    pub entity_metadata: Option<ScheduledEventEntityMetadata>,
    /// Number of users subscribed to the event.
    pub user_count: Option<u32>,
}
