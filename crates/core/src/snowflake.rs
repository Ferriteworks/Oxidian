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

use std::fmt;
use std::num::ParseIntError;
use std::str::FromStr;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Discord epoch: 2015-01-01T00:00:00.000Z as milliseconds since Unix epoch.
const DISCORD_EPOCH_MS: u64 = 1_420_070_400_000;

/// A Discord Snowflake ID.
///
/// Snowflakes are 64-bit integers that encode a timestamp, worker ID, process
/// ID, and sequence number.  They are transmitted by Discord as JSON strings
/// to avoid JavaScript lossy integer handling.
///
/// # Bit layout (MSB = 63)
/// ```text
/// 63..22  timestamp  : ms since Discord epoch (2015-01-01)
/// 21..17  worker ID
/// 16..12  process ID
/// 11..0   sequence number
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Snowflake(u64);

impl Snowflake {
    /// Create a [`Snowflake`] from a raw `u64`.
    pub const fn new(id: u64) -> Self {
        Self(id)
    }

    /// Return the raw inner `u64`.
    pub const fn get(self) -> u64 {
        self.0
    }

    /// Extract the creation timestamp encoded in the snowflake.
    ///
    /// Returns `None` if the encoded millisecond value would overflow [`SystemTime`].
    pub fn created_at(self) -> Option<SystemTime> {
        let ms = (self.0 >> 22).checked_add(DISCORD_EPOCH_MS)?;
        UNIX_EPOCH.checked_add(Duration::from_millis(ms))
    }

    /// Return the creation timestamp as milliseconds since the Unix epoch.
    pub fn created_at_ms(self) -> u64 {
        (self.0 >> 22) + DISCORD_EPOCH_MS
    }

    /// Return `true` if this snowflake is the zero value.
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }
}

impl fmt::Display for Snowflake {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Snowflake {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}

impl From<u64> for Snowflake {
    fn from(id: u64) -> Self {
        Self(id)
    }
}

impl From<Snowflake> for u64 {
    fn from(s: Snowflake) -> u64 {
        s.0
    }
}

// Discord transmits snowflakes as JSON strings to avoid JS 53-bit integer loss.
impl Serialize for Snowflake {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.0.to_string())
    }
}

impl<'de> Deserialize<'de> for Snowflake {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct SnowflakeVisitor;

        impl serde::de::Visitor<'_> for SnowflakeVisitor {
            type Value = Snowflake;

            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "a Discord snowflake as a JSON string or integer")
            }

            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                v.parse::<u64>().map(Snowflake).map_err(E::custom)
            }

            fn visit_u64<E: serde::de::Error>(self, v: u64) -> Result<Self::Value, E> {
                Ok(Snowflake(v))
            }

            fn visit_i64<E: serde::de::Error>(self, v: i64) -> Result<Self::Value, E> {
                Ok(Snowflake(v as u64))
            }
        }

        deserializer.deserialize_any(SnowflakeVisitor)
    }
}
