use serde::{Deserialize, Serialize};

use super::{channel::Channel, guild::Guild, user::User};

/// The type of target for a voice-channel invite.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct InviteTargetType(pub u8);
// 1 = STREAM, 2 = EMBEDDED_APPLICATION

/// A Discord invite, as returned by the REST API or gateway events.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Invite {
    /// The unique invite code (not a Snowflake).
    pub code: String,
    /// The guild the invite is for, if applicable.
    pub guild: Option<Guild>,
    /// The channel the invite leads to.
    pub channel: Option<Channel>,
    /// The user who created the invite.
    pub inviter: Option<User>,
    /// The target user for a stream invite.
    pub target_user: Option<User>,
    /// Target type for a voice-channel invite.
    pub target_type: Option<InviteTargetType>,
    /// Approximate number of online members (only present with `with_counts`).
    pub approximate_presence_count: Option<u32>,
    /// Approximate total member count (only present with `with_counts`).
    pub approximate_member_count: Option<u32>,
    /// ISO 8601 expiry timestamp, if the invite has one.
    pub expires_at: Option<String>,
}
