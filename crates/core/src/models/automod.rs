// MIT License
//
// Copyright (c) 2026 Ferriteworks organization and its rightful owners.
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

//! Request bodies for the Auto-Moderation REST endpoints.
//!
//! Event payloads are deserialised in [`oxidian_gateway::events`]. The types
//! in this module are the *outbound* shapes used when creating or modifying a
//! rule via [`oxidian_http::HttpClient::create_auto_moderation_rule`].

use serde::Serialize;

use crate::snowflake::Snowflake;

/// Event type a rule triggers on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(into = "u8")]
pub enum AutoModEventType {
    /// A member sent a message.
    MessageSend,
    /// A member updated their profile.
    MemberUpdate,
}

impl From<AutoModEventType> for u8 {
    fn from(v: AutoModEventType) -> u8 {
        match v {
            AutoModEventType::MessageSend => 1,
            AutoModEventType::MemberUpdate => 2,
        }
    }
}

/// What the rule matches on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(into = "u8")]
pub enum AutoModTriggerType {
    /// Check message content against custom keywords.
    Keyword,
    /// Generic spam detection.
    Spam,
    /// Match against one of Discord's preset keyword lists.
    KeywordPreset,
    /// Limit the number of unique mentions per message.
    MentionSpam,
    /// Check member profile content.
    MemberProfile,
}

impl From<AutoModTriggerType> for u8 {
    fn from(v: AutoModTriggerType) -> u8 {
        match v {
            AutoModTriggerType::Keyword => 1,
            AutoModTriggerType::Spam => 3,
            AutoModTriggerType::KeywordPreset => 4,
            AutoModTriggerType::MentionSpam => 5,
            AutoModTriggerType::MemberProfile => 6,
        }
    }
}

/// Action taken when the rule triggers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(into = "u8")]
pub enum AutoModActionType {
    /// Block the message.
    BlockMessage,
    /// Post an alert to a moderator channel.
    SendAlertMessage,
    /// Time the user out (keyword triggers only).
    Timeout,
    /// Block the member from interacting in the guild.
    BlockMemberInteraction,
}

impl From<AutoModActionType> for u8 {
    fn from(v: AutoModActionType) -> u8 {
        match v {
            AutoModActionType::BlockMessage => 1,
            AutoModActionType::SendAlertMessage => 2,
            AutoModActionType::Timeout => 3,
            AutoModActionType::BlockMemberInteraction => 4,
        }
    }
}

/// Extra fields required by certain action types.
#[derive(Debug, Clone, Default, Serialize)]
pub struct AutoModActionMetadata {
    /// Channel to post alerts to (required for `SendAlertMessage`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<Snowflake>,
    /// Timeout length in seconds, max 2_419_200 (4 weeks). For `Timeout` only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_seconds: Option<u32>,
    /// Optional custom rejection message shown to the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_message: Option<String>,
}

/// A single action entry.
#[derive(Debug, Clone, Serialize)]
pub struct AutoModAction {
    #[serde(rename = "type")]
    pub kind: AutoModActionType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<AutoModActionMetadata>,
}

impl AutoModAction {
    /// Build a plain action with no metadata (e.g. `BlockMessage`).
    pub fn new(kind: AutoModActionType) -> Self {
        Self {
            kind,
            metadata: None,
        }
    }

    /// Build an action with metadata.
    pub fn with_metadata(kind: AutoModActionType, metadata: AutoModActionMetadata) -> Self {
        Self {
            kind,
            metadata: Some(metadata),
        }
    }
}

/// Trigger configuration for keyword / preset / mention-spam rules.
#[derive(Debug, Clone, Default, Serialize)]
pub struct AutoModTriggerMetadata {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub keyword_filter: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub regex_patterns: Vec<String>,
    /// Preset keyword list IDs: 1=PROFANITY, 2=SEXUAL_CONTENT, 3=SLURS.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub presets: Vec<u8>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub allow_list: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mention_total_limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mention_raid_protection_enabled: Option<bool>,
}

/// Request body for `POST /guilds/{guild.id}/auto-moderation/rules`.
///
/// Use [`AutoModRuleOptions::new`] and chain setters to fill in optional
/// fields. The trigger metadata is only sent when non-empty.
#[derive(Debug, Clone, Serialize)]
pub struct AutoModRuleOptions {
    pub name: String,
    pub event_type: AutoModEventType,
    pub trigger_type: AutoModTriggerType,
    #[serde(skip_serializing_if = "trigger_metadata_is_empty")]
    pub trigger_metadata: AutoModTriggerMetadata,
    pub actions: Vec<AutoModAction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub exempt_roles: Vec<Snowflake>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub exempt_channels: Vec<Snowflake>,
}

fn trigger_metadata_is_empty(m: &AutoModTriggerMetadata) -> bool {
    m.keyword_filter.is_empty()
        && m.regex_patterns.is_empty()
        && m.presets.is_empty()
        && m.allow_list.is_empty()
        && m.mention_total_limit.is_none()
        && m.mention_raid_protection_enabled.is_none()
}

impl AutoModRuleOptions {
    /// Start a new rule with the required fields.
    pub fn new(
        name: impl Into<String>,
        event_type: AutoModEventType,
        trigger_type: AutoModTriggerType,
    ) -> Self {
        Self {
            name: name.into(),
            event_type,
            trigger_type,
            trigger_metadata: AutoModTriggerMetadata::default(),
            actions: Vec::new(),
            enabled: None,
            exempt_roles: Vec::new(),
            exempt_channels: Vec::new(),
        }
    }

    /// Replace trigger metadata.
    pub fn trigger_metadata(mut self, meta: AutoModTriggerMetadata) -> Self {
        self.trigger_metadata = meta;
        self
    }

    /// Add an action.
    pub fn action(mut self, action: AutoModAction) -> Self {
        self.actions.push(action);
        self
    }

    /// Replace all actions at once.
    pub fn actions(mut self, actions: Vec<AutoModAction>) -> Self {
        self.actions = actions;
        self
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = Some(enabled);
        self
    }

    pub fn exempt_roles(mut self, roles: Vec<Snowflake>) -> Self {
        self.exempt_roles = roles;
        self
    }

    pub fn exempt_channels(mut self, channels: Vec<Snowflake>) -> Self {
        self.exempt_channels = channels;
        self
    }
}
