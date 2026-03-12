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

//! Discord timestamp utilities.
//!
//! Covers two things:
//! 1. Extracting the UTC creation time from a Discord snowflake ID.
//! 2. Formatting Unix timestamps as Discord's markdown `<t:unix:style>` strings.

/// Discord's epoch offset in milliseconds (2015-01-01T00:00:00.000Z).
pub const DISCORD_EPOCH_MS: u64 = 1_420_070_400_000;

/// Extract the UTC creation time in **milliseconds** from a Discord snowflake ID.
///
/// All Discord snowflake IDs encode their creation time in the top 42 bits.
pub fn creation_ms(snowflake: u64) -> u64 {
    (snowflake >> 22) + DISCORD_EPOCH_MS
}

/// Extract the UTC creation time in **seconds** from a Discord snowflake ID.
pub fn creation_secs(snowflake: u64) -> u64 {
    creation_ms(snowflake) / 1000
}

/// Discord timestamp display styles.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TimestampStyle {
    /// Short time, e.g. `16:20`
    ShortTime,
    /// Long time, e.g. `16:20:30`
    LongTime,
    /// Short date, e.g. `20/04/2021`
    ShortDate,
    /// Long date, e.g. `20 April 2021`
    LongDate,
    /// Short date/time, e.g. `20 April 2021 16:20`
    #[default]
    ShortDateTime,
    /// Long date/time, e.g. `Tuesday, 20 April 2021 16:20`
    LongDateTime,
    /// Relative, e.g. `2 months ago`
    Relative,
}

impl TimestampStyle {
    fn flag(self) -> char {
        match self {
            Self::ShortTime => 't',
            Self::LongTime => 'T',
            Self::ShortDate => 'd',
            Self::LongDate => 'D',
            Self::ShortDateTime => 'f',
            Self::LongDateTime => 'F',
            Self::Relative => 'R',
        }
    }
}

/// Format a Unix timestamp (seconds) as a Discord markdown timestamp string.
///
/// ```
/// use oxidian_utils::timestamp::{format, TimestampStyle};
///
/// assert_eq!(format(1618953600, TimestampStyle::Relative), "<t:1618953600:R>");
/// ```
pub fn format(unix_secs: u64, style: TimestampStyle) -> String {
    format!("<t:{unix_secs}:{}>", style.flag())
}

/// Format the creation timestamp of a Discord snowflake ID as a markdown
/// timestamp string.
pub fn snowflake_timestamp(snowflake: u64, style: TimestampStyle) -> String {
    format(creation_secs(snowflake), style)
}
