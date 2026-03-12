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

/// A Discord poll attached to a message.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Poll {
    /// The question of the poll.  Only `text` is supported.
    pub question: PollMedia,
    /// Each available answer.
    pub answers: Vec<PollAnswer>,
    /// Expiry timestamp (ISO 8601), or `None` if it never expires.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry: Option<String>,
    /// Whether users can select multiple answers.
    pub allow_multiselect: bool,
    /// Layout type.  1 = DEFAULT.
    pub layout_type: u8,
    /// The results of the poll, if finalised.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<PollResults>,
}

/// Text (and optional emoji) associated with a poll question or answer.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PollMedia {
    /// The text content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// Optional emoji.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<PollEmoji>,
}

/// Partial emoji object used inside polls.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PollEmoji {
    /// For custom emoji.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Snowflake>,
    /// For standard emoji.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// A single answer option in a poll.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PollAnswer {
    /// Stable ID for this answer (assigned by Discord).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub answer_id: Option<u32>,
    /// The displayed label and optional emoji.
    pub poll_media: PollMedia,
}

/// Aggregated vote results for a poll.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PollResults {
    /// Whether the poll has finished and results are final.
    pub is_finalized: bool,
    /// Vote counts per answer.
    pub answer_counts: Vec<PollAnswerCount>,
}

/// Vote count for one answer.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PollAnswerCount {
    /// The answer ID this count belongs to.
    pub id: u32,
    /// Number of votes.
    pub count: u32,
    /// Whether the current bot user voted for this answer.
    pub me_voted: bool,
}
