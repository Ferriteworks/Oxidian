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

//! Discord permission flags (v2).
//!
//! Permissions are a `u64` bitfield. Discord sends them as JSON strings, so
//! custom (de)serialization handles that conversion transparently.
//!
//! ```rust
//! use oxidian_core::permissions::Permissions;
//!
//! let perms = Permissions::SEND_MESSAGES | Permissions::EMBED_LINKS;
//! assert!(perms.contains(Permissions::SEND_MESSAGES));
//! ```

use bitflags::bitflags;

bitflags! {
    /// Discord [permission flags](https://discord.com/developers/docs/topics/permissions#permissions-bitwise-permission-flags).
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Permissions: u64 {
        const CREATE_INSTANT_INVITE               = 1 << 0;
        const KICK_MEMBERS                        = 1 << 1;
        const BAN_MEMBERS                         = 1 << 2;
        const ADMINISTRATOR                       = 1 << 3;
        const MANAGE_CHANNELS                     = 1 << 4;
        const MANAGE_GUILD                        = 1 << 5;
        const ADD_REACTIONS                        = 1 << 6;
        const VIEW_AUDIT_LOG                      = 1 << 7;
        const PRIORITY_SPEAKER                    = 1 << 8;
        const STREAM                              = 1 << 9;
        const VIEW_CHANNEL                        = 1 << 10;
        const SEND_MESSAGES                       = 1 << 11;
        const SEND_TTS_MESSAGES                   = 1 << 12;
        const MANAGE_MESSAGES                     = 1 << 13;
        const EMBED_LINKS                         = 1 << 14;
        const ATTACH_FILES                        = 1 << 15;
        const READ_MESSAGE_HISTORY                = 1 << 16;
        const MENTION_EVERYONE                    = 1 << 17;
        const USE_EXTERNAL_EMOJIS                 = 1 << 18;
        const VIEW_GUILD_INSIGHTS                 = 1 << 19;
        const CONNECT                             = 1 << 20;
        const SPEAK                               = 1 << 21;
        const MUTE_MEMBERS                        = 1 << 22;
        const DEAFEN_MEMBERS                      = 1 << 23;
        const MOVE_MEMBERS                        = 1 << 24;
        const USE_VAD                             = 1 << 25;
        const CHANGE_NICKNAME                     = 1 << 26;
        const MANAGE_NICKNAMES                    = 1 << 27;
        const MANAGE_ROLES                        = 1 << 28;
        const MANAGE_WEBHOOKS                     = 1 << 29;
        const MANAGE_GUILD_EXPRESSIONS            = 1 << 30;
        const USE_APPLICATION_COMMANDS            = 1 << 31;
        const REQUEST_TO_SPEAK                    = 1 << 32;
        const MANAGE_EVENTS                       = 1 << 33;
        const MANAGE_THREADS                      = 1 << 34;
        const CREATE_PUBLIC_THREADS               = 1 << 35;
        const CREATE_PRIVATE_THREADS              = 1 << 36;
        const USE_EXTERNAL_STICKERS               = 1 << 37;
        const SEND_MESSAGES_IN_THREADS            = 1 << 38;
        const USE_EMBEDDED_ACTIVITIES             = 1 << 39;
        const MODERATE_MEMBERS                    = 1 << 40;
        const VIEW_CREATOR_MONETIZATION_ANALYTICS = 1 << 41;
        const USE_SOUNDBOARD                      = 1 << 42;
        const CREATE_GUILD_EXPRESSIONS            = 1 << 43;
        const CREATE_EVENTS                       = 1 << 44;
        const USE_EXTERNAL_SOUNDS                 = 1 << 45;
        const SEND_VOICE_MESSAGES                 = 1 << 46;
        const SEND_POLLS                          = 1 << 49;
        const USE_EXTERNAL_APPS                   = 1 << 50;
    }
}

impl Permissions {
    /// Parse a permissions string (as Discord sends them in JSON) into a
    /// `Permissions` bitfield.
    pub fn from_string(s: &str) -> Self {
        let bits = s.parse::<u64>().unwrap_or(0);
        Self::from_bits_truncate(bits)
    }

    /// Serialize this permissions bitfield into the string format Discord
    /// expects.
    pub fn to_string_value(&self) -> String {
        self.bits().to_string()
    }
}

/// Deserialize permissions from a JSON string (Discord sends them as `"123"`).
impl<'de> serde::Deserialize<'de> for Permissions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(Self::from_string(&s))
    }
}

/// Serialize permissions as a JSON string.
impl serde::Serialize for Permissions {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string_value())
    }
}
