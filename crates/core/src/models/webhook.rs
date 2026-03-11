use serde::{Deserialize, Serialize};

use crate::snowflake::Snowflake;

use super::user::User;

/// A Discord webhook.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Webhook {
    /// The webhook's snowflake ID.
    pub id: Snowflake,
    /// 1 = INCOMING, 2 = CHANNEL_FOLLOWER, 3 = APPLICATION.
    #[serde(rename = "type")]
    pub kind: u8,
    /// The guild the webhook is in, if applicable.
    pub guild_id: Option<Snowflake>,
    /// The channel the webhook posts to.
    pub channel_id: Option<Snowflake>,
    /// The user who created the webhook.
    pub user: Option<User>,
    /// The default name of the webhook.
    pub name: Option<String>,
    /// The default avatar hash.
    pub avatar: Option<String>,
    /// The secret token used to sign webhook payloads.
    pub token: Option<String>,
    /// The bot/OAuth2 application that created the webhook.
    pub application_id: Option<Snowflake>,
    /// URL for Slack-compatible / GitHub-compatible webhooks.
    pub url: Option<String>,
}
