//! Gateway intents as a type-safe bitflag set.
//!
//! ```rust
//! use oxidian_core::intents::Intents;
//!
//! let intents = Intents::GUILDS | Intents::GUILD_MESSAGES | Intents::MESSAGE_CONTENT;
//! assert!(intents.contains(Intents::MESSAGE_CONTENT));
//! ```

use bitflags::bitflags;

bitflags! {
    /// Discord [Gateway Intents](https://discord.com/developers/docs/events/gateway#gateway-intents)
    /// control which events the gateway sends to your bot.
    ///
    /// Combine intents with the `|` operator:
    ///
    /// ```rust
    /// # use oxidian_core::intents::Intents;
    /// let intents = Intents::GUILDS | Intents::GUILD_MESSAGES;
    /// ```
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Intents: u64 {
        /// Guild create/update/delete, role/channel events, thread list sync.
        const GUILDS                          = 1 << 0;
        /// Guild member add/update/remove events.
        const GUILD_MEMBERS                   = 1 << 1;
        /// Guild ban add/remove events.
        const GUILD_MODERATION                = 1 << 2;
        /// Guild emoji/sticker update events.
        const GUILD_EMOJIS_AND_STICKERS       = 1 << 3;
        /// Guild integration update events.
        const GUILD_INTEGRATIONS              = 1 << 4;
        /// Webhook update events.
        const GUILD_WEBHOOKS                  = 1 << 5;
        /// Invite create/delete events.
        const GUILD_INVITES                   = 1 << 6;
        /// Voice state update events.
        const GUILD_VOICE_STATES              = 1 << 7;
        /// Presence update events.
        const GUILD_PRESENCES                 = 1 << 8;
        /// Message create/update/delete in guild channels.
        const GUILD_MESSAGES                  = 1 << 9;
        /// Message reaction add/remove/clear in guild channels.
        const GUILD_MESSAGE_REACTIONS         = 1 << 10;
        /// Typing start events in guild channels.
        const GUILD_MESSAGE_TYPING            = 1 << 11;
        /// Message create/update/delete in DMs.
        const DIRECT_MESSAGES                 = 1 << 12;
        /// Reaction add/remove/clear in DMs.
        const DIRECT_MESSAGE_REACTIONS        = 1 << 13;
        /// Typing start events in DMs.
        const DIRECT_MESSAGE_TYPING           = 1 << 14;
        /// Access to message content (privileged).
        const MESSAGE_CONTENT                 = 1 << 15;
        /// Scheduled event create/update/delete and user add/remove.
        const GUILD_SCHEDULED_EVENTS          = 1 << 16;
        /// Auto moderation configuration events.
        const AUTO_MODERATION_CONFIGURATION   = 1 << 20;
        /// Auto moderation action execution events.
        const AUTO_MODERATION_EXECUTION       = 1 << 21;
        /// Guild message poll vote add/remove.
        const GUILD_MESSAGE_POLLS             = 1 << 24;
        /// DM message poll vote add/remove.
        const DIRECT_MESSAGE_POLLS            = 1 << 25;

        /// All non-privileged intents.
        const NON_PRIVILEGED = Self::GUILDS.bits()
            | Self::GUILD_MODERATION.bits()
            | Self::GUILD_EMOJIS_AND_STICKERS.bits()
            | Self::GUILD_INTEGRATIONS.bits()
            | Self::GUILD_WEBHOOKS.bits()
            | Self::GUILD_INVITES.bits()
            | Self::GUILD_VOICE_STATES.bits()
            | Self::GUILD_MESSAGES.bits()
            | Self::GUILD_MESSAGE_REACTIONS.bits()
            | Self::GUILD_MESSAGE_TYPING.bits()
            | Self::DIRECT_MESSAGES.bits()
            | Self::DIRECT_MESSAGE_REACTIONS.bits()
            | Self::DIRECT_MESSAGE_TYPING.bits()
            | Self::GUILD_SCHEDULED_EVENTS.bits()
            | Self::AUTO_MODERATION_CONFIGURATION.bits()
            | Self::AUTO_MODERATION_EXECUTION.bits()
            | Self::GUILD_MESSAGE_POLLS.bits()
            | Self::DIRECT_MESSAGE_POLLS.bits();

        /// All intents, including privileged ones (GUILD_MEMBERS, GUILD_PRESENCES, MESSAGE_CONTENT).
        const ALL = Self::NON_PRIVILEGED.bits()
            | Self::GUILD_MEMBERS.bits()
            | Self::GUILD_PRESENCES.bits()
            | Self::MESSAGE_CONTENT.bits();
    }
}

impl Intents {
    /// Convert to raw `u64` for the gateway Identify payload.
    #[inline]
    pub fn to_u64(self) -> u64 {
        self.bits()
    }
}

impl From<u64> for Intents {
    fn from(bits: u64) -> Self {
        Self::from_bits_truncate(bits)
    }
}

impl From<Intents> for u64 {
    fn from(intents: Intents) -> Self {
        intents.bits()
    }
}
