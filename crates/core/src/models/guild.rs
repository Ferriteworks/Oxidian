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

use serde::{Deserialize, Serialize};

use crate::snowflake::Snowflake;

use super::{
    channel::Channel, emoji::Emoji, member::Member, role::Role, sticker::Sticker,
    thread::Thread,
};

// ---------------------------------------------------------------------------
// Guild-specific sub-types
// ---------------------------------------------------------------------------

/// Guild verification level: controls who can send messages.
///
/// `0` = None, `1` = Low, `2` = Medium, `3` = High, `4` = VeryHigh.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "u8", into = "u8")]
pub enum VerificationLevel {
    /// No requirements.
    None = 0,
    /// Must have a verified email.
    Low = 1,
    /// Must be registered on Discord for longer than 5 minutes.
    Medium = 2,
    /// Must be a member of the guild for longer than 10 minutes.
    High = 3,
    /// Must have a verified phone number.
    VeryHigh = 4,
}

impl TryFrom<u8> for VerificationLevel {
    type Error = String;
    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(Self::None),
            1 => Ok(Self::Low),
            2 => Ok(Self::Medium),
            3 => Ok(Self::High),
            4 => Ok(Self::VeryHigh),
            _ => Err(format!("unknown verification level: {v}")),
        }
    }
}
impl From<VerificationLevel> for u8 {
    fn from(v: VerificationLevel) -> u8 {
        v as u8
    }
}

/// Default message notification level.
///
/// `0` = AllMessages, `1` = OnlyMentions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "u8", into = "u8")]
pub enum DefaultMessageNotifications {
    /// Members receive notifications for all messages.
    AllMessages = 0,
    /// Members only receive notifications for direct mentions.
    OnlyMentions = 1,
}

impl TryFrom<u8> for DefaultMessageNotifications {
    type Error = String;
    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(Self::AllMessages),
            1 => Ok(Self::OnlyMentions),
            _ => Err(format!("unknown default message notifications level: {v}")),
        }
    }
}
impl From<DefaultMessageNotifications> for u8 {
    fn from(v: DefaultMessageNotifications) -> u8 {
        v as u8
    }
}

/// Explicit content filter level for media / links.
///
/// `0` = Disabled, `1` = MembersWithoutRoles, `2` = AllMembers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "u8", into = "u8")]
pub enum ExplicitContentFilter {
    /// No scanning.
    Disabled = 0,
    /// Scan media from members without a role.
    MembersWithoutRoles = 1,
    /// Scan media from all members.
    AllMembers = 2,
}

impl TryFrom<u8> for ExplicitContentFilter {
    type Error = String;
    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(Self::Disabled),
            1 => Ok(Self::MembersWithoutRoles),
            2 => Ok(Self::AllMembers),
            _ => Err(format!("unknown explicit content filter: {v}")),
        }
    }
}
impl From<ExplicitContentFilter> for u8 {
    fn from(v: ExplicitContentFilter) -> u8 {
        v as u8
    }
}

/// MFA (2FA) requirement for admin actions.
///
/// `0` = None (2FA not required), `1` = Elevated (2FA required).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "u8", into = "u8")]
pub enum MfaLevel {
    /// No MFA requirement.
    None = 0,
    /// MFA required for admin operations.
    Elevated = 1,
}

impl TryFrom<u8> for MfaLevel {
    type Error = String;
    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(Self::None),
            1 => Ok(Self::Elevated),
            _ => Err(format!("unknown MFA level: {v}")),
        }
    }
}
impl From<MfaLevel> for u8 {
    fn from(v: MfaLevel) -> u8 {
        v as u8
    }
}

/// Server Boost tier.
///
/// `0` = None, `1` = Tier1, `2` = Tier2, `3` = Tier3.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "u8", into = "u8")]
pub enum PremiumTier {
    /// No Server Boost tier.
    None = 0,
    /// Tier 1 boosts.
    Tier1 = 1,
    /// Tier 2 boosts.
    Tier2 = 2,
    /// Tier 3 boosts.
    Tier3 = 3,
}

impl TryFrom<u8> for PremiumTier {
    type Error = String;
    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(Self::None),
            1 => Ok(Self::Tier1),
            2 => Ok(Self::Tier2),
            3 => Ok(Self::Tier3),
            _ => Err(format!("unknown premium tier: {v}")),
        }
    }
}
impl From<PremiumTier> for u8 {
    fn from(v: PremiumTier) -> u8 {
        v as u8
    }
}

/// Guild NSFW content level.
///
/// `0` = Default, `1` = Explicit, `2` = Safe, `3` = AgeRestricted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "u8", into = "u8")]
pub enum NsfwLevel {
    /// Default level: not explicitly classified.
    Default = 0,
    /// Explicitly NSFW content allowed.
    Explicit = 1,
    /// Safe for all audiences.
    Safe = 2,
    /// Age-restricted (18+).
    AgeRestricted = 3,
}

impl TryFrom<u8> for NsfwLevel {
    type Error = String;
    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(Self::Default),
            1 => Ok(Self::Explicit),
            2 => Ok(Self::Safe),
            3 => Ok(Self::AgeRestricted),
            _ => Err(format!("unknown NSFW level: {v}")),
        }
    }
}
impl From<NsfwLevel> for u8 {
    fn from(v: NsfwLevel) -> u8 {
        v as u8
    }
}

/// A channel shown in the guild's Welcome Screen.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WelcomeScreenChannel {
    /// The channel's snowflake ID.
    pub channel_id: Snowflake,
    /// Description of the channel shown on the welcome screen.
    pub description: String,
    /// Custom emoji ID for the channel, if any.
    pub emoji_id: Option<Snowflake>,
    /// Custom emoji name or Unicode emoji for the channel, if any.
    pub emoji_name: Option<String>,
}

/// The guild's Welcome Screen configuration (shown to new members).
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WelcomeScreen {
    /// Description shown at the top of the welcome screen.
    pub description: Option<String>,
    /// Up to 5 channels shown on the welcome screen.
    #[serde(default)]
    pub welcome_channels: Vec<WelcomeScreenChannel>,
}

// ---------------------------------------------------------------------------
// Main Guild struct
// ---------------------------------------------------------------------------

/// A Discord guild (server), as returned by GUILD_CREATE, GUILD_UPDATE,
/// `GET /guilds/:id`, and related endpoints.
///
/// Many fields are only present in GUILD_CREATE (e.g. `channels`, `members`,
/// `presences`) and are marked `Option` / `#[serde(default)]` accordingly.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Guild {
    // ── Core identity ─────────────────────────────────────────────────────
    /// The guild's unique snowflake ID.
    pub id: Snowflake,
    /// The guild's display name.
    pub name: String,
    /// Icon image hash.
    pub icon: Option<String>,
    /// Icon hash (returned when in the template object).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_hash: Option<String>,
    /// Splash image hash (shown on invite pages).
    pub splash: Option<String>,
    /// Discovery splash image hash (shown on the discovery page).
    pub discovery_splash: Option<String>,

    // ── Ownership ─────────────────────────────────────────────────────────
    /// Whether the current user is the guild owner.
    /// Only present in `GET /users/@me/guilds`.
    #[serde(default)]
    pub owner: bool,
    /// Snowflake ID of the guild owner.
    pub owner_id: Snowflake,
    /// Total permissions for the current user in the guild (excluding overwrites).
    /// Only present in `GET /users/@me/guilds`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<String>,

    // ── AFK ───────────────────────────────────────────────────────────────
    /// The channel to move members to after `afk_timeout` seconds of inactivity.
    pub afk_channel_id: Option<Snowflake>,
    /// AFK timeout in seconds (60 / 300 / 900 / 1800 / 3600).
    #[serde(default)]
    pub afk_timeout: u32,

    // ── Widget ────────────────────────────────────────────────────────────
    /// Whether the widget is enabled.
    #[serde(default)]
    pub widget_enabled: bool,
    /// The channel to show in the widget.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub widget_channel_id: Option<Snowflake>,

    // ── Moderation ────────────────────────────────────────────────────────
    /// Verification level required to send messages.
    #[serde(default = "VerificationLevel::none")]
    pub verification_level: VerificationLevel,
    /// Default message notification level.
    #[serde(default = "DefaultMessageNotifications::all_messages")]
    pub default_message_notifications: DefaultMessageNotifications,
    /// Explicit content filter level.
    #[serde(default = "ExplicitContentFilter::disabled")]
    pub explicit_content_filter: ExplicitContentFilter,
    /// MFA (2FA) requirement for admin actions.
    #[serde(default = "MfaLevel::none")]
    pub mfa_level: MfaLevel,

    // ── Roles / Emojis / Stickers / Features ─────────────────────────────
    /// All roles in the guild, ordered by position.
    #[serde(default)]
    pub roles: Vec<Role>,
    /// Custom emoji available in the guild.
    #[serde(default)]
    pub emojis: Vec<Emoji>,
    /// Guild feature flags (strings like `"COMMUNITY"`, `"DISCOVERABLE"`, …).
    #[serde(default)]
    pub features: Vec<String>,
    /// Custom stickers available in the guild.
    #[serde(default)]
    pub stickers: Vec<Sticker>,

    // ── Application / System channels ─────────────────────────────────────
    /// The application ID of the bot that created the guild, if bot-created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<Snowflake>,
    /// The channel for system messages (joins, boosts, etc.).
    pub system_channel_id: Option<Snowflake>,
    /// System channel feature flags bitfield.
    #[serde(default)]
    pub system_channel_flags: u32,
    /// The channel where Community guilds post rules and guidelines.
    pub rules_channel_id: Option<Snowflake>,
    /// The channel where admins and moderators receive Discord safety alerts.
    pub safety_alerts_channel_id: Option<Snowflake>,
    /// The channel for public updates from Discord to Community guild admins.
    pub public_updates_channel_id: Option<Snowflake>,

    // ── Discovery / Vanity ────────────────────────────────────────────────
    /// Max number of presences (online members). `null` for large guilds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_presences: Option<u32>,
    /// Maximum number of members (hard limit).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_members: Option<u32>,
    /// Vanity URL code (e.g. `"discord"` for `discord.gg/discord`).
    pub vanity_url_code: Option<String>,
    /// Short description of the guild (Community guilds only).
    pub description: Option<String>,
    /// Banner image hash.
    pub banner: Option<String>,

    // ── Boosts ────────────────────────────────────────────────────────────
    /// Current Server Boost tier.
    #[serde(default = "PremiumTier::none")]
    pub premium_tier: PremiumTier,
    /// Number of boost subscriptions the guild currently has.
    #[serde(default)]
    pub premium_subscription_count: u32,
    /// Whether the Server Boost progress bar is shown in the guild.
    #[serde(default)]
    pub premium_progress_bar_enabled: bool,

    // ── Locale / Welcome Screen / NSFW ────────────────────────────────────
    /// The guild's preferred locale (used in discovery and system messages).
    #[serde(default = "default_locale")]
    pub preferred_locale: String,
    /// Welcome screen shown to new members (Community guilds only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub welcome_screen: Option<WelcomeScreen>,
    /// Guild NSFW content level.
    #[serde(default = "NsfwLevel::default_level")]
    pub nsfw_level: NsfwLevel,

    // ── Video / Media ─────────────────────────────────────────────────────
    /// Maximum number of users in a video channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_video_channel_users: Option<u32>,
    /// Maximum number of users in a stage video channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_stage_video_channel_users: Option<u32>,

    // ── Counts (approximate / online) ─────────────────────────────────────
    /// Total number of members in the guild.
    /// Populated in GUILD_CREATE and `GET /guilds/:id?with_counts=true`.
    pub member_count: Option<u32>,
    /// Approximate member count (populated with the `with_counts` query param).
    pub approximate_member_count: Option<u32>,
    /// Approximate presence (online) count.
    pub approximate_presence_count: Option<u32>,

    // ── GUILD_CREATE-only fields ───────────────────────────────────────────
    /// Whether this guild is currently unavailable due to a Discord outage.
    #[serde(default)]
    pub unavailable: bool,
    /// Text, voice, and category channels in the guild.
    /// Only populated in GUILD_CREATE.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub channels: Vec<Channel>,
    /// Active threads the current user has permission to view.
    /// Only populated in GUILD_CREATE.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub threads: Vec<Thread>,
    /// Guild members (only included for small guilds in GUILD_CREATE).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub members: Vec<Member>,
    /// Partial presences of members in the guild.
    /// Only populated in GUILD_CREATE.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub presences: Vec<serde_json::Value>,
    /// Voice states of members in voice channels.
    /// Only populated in GUILD_CREATE.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub voice_states: Vec<serde_json::Value>,
    /// Stage instances active in the guild's stage channels.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub stage_instances: Vec<serde_json::Value>,
    /// Scheduled events in the guild.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub guild_scheduled_events: Vec<serde_json::Value>,
}

// Serde default helpers for enum fields.

impl VerificationLevel {
    fn none() -> Self {
        Self::None
    }
}
impl DefaultMessageNotifications {
    fn all_messages() -> Self {
        Self::AllMessages
    }
}
impl ExplicitContentFilter {
    fn disabled() -> Self {
        Self::Disabled
    }
}
impl MfaLevel {
    fn none() -> Self {
        Self::None
    }
}
impl PremiumTier {
    fn none() -> Self {
        Self::None
    }
}
impl NsfwLevel {
    fn default_level() -> Self {
        Self::Default
    }
}

fn default_locale() -> String {
    "en-US".to_owned()
}
