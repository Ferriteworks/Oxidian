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

//! Discord embed model and builder.
//!
//! Embeds can be attached to messages and interaction responses to display rich
//! formatted content with titles, descriptions, images, and fields.
//!
//! # Example
//!
//! ```rust,ignore
//! use oxidian_core::models::embed::EmbedBuilder;
//!
//! let embed = EmbedBuilder::new()
//!     .title("Status Report")
//!     .description("Everything is operational.")
//!     .color(0x00FF00)
//!     .field("Uptime", "99.9%", true)
//!     .field("Region", "US East", true)
//!     .footer("Last checked just now")
//!     .build();
//! ```

use serde::{Deserialize, Serialize};

/// A rich embed object attached to a message.
///
/// Use [`EmbedBuilder`] for fluent construction.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Embed {
    /// The embed title (max 256 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// The embed description (max 4096 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// URL that the title hyperlinks to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// ISO 8601 timestamp shown in the footer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    /// Color code as an RGB integer (e.g. `0xFF0000` for red).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<u32>,
    /// Footer text and optional icon.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer: Option<EmbedFooter>,
    /// Large image displayed below the description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<EmbedMedia>,
    /// Small image displayed in the top-right corner.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<EmbedMedia>,
    /// Video data (receive-only; set by Discord for link-preview embeds).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<EmbedMedia>,
    /// Provider information (receive-only; set by Discord for link-preview embeds).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<EmbedProvider>,
    /// Author name, URL, and icon.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<EmbedAuthor>,
    /// Up to 25 name/value field pairs.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub fields: Vec<EmbedField>,
}

/// Footer section of an embed.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbedFooter {
    /// Footer text (max 2048 characters).
    pub text: String,
    /// URL of the footer icon.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
}

/// Media reference used for embed image, thumbnail, and video fields.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbedMedia {
    /// Source URL of the media.
    pub url: String,
    /// Height in pixels (receive-only for video/image previews).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u32>,
    /// Width in pixels (receive-only for video/image previews).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,
}

/// Provider information on a link-preview embed (receive-only).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbedProvider {
    /// Provider name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Provider URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// Author section displayed at the top of an embed.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbedAuthor {
    /// Author name (max 256 characters).
    pub name: String,
    /// URL that the author name hyperlinks to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// URL of the author icon.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
}

/// A name/value field pair displayed inside an embed.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbedField {
    /// Field name (max 256 characters).
    pub name: String,
    /// Field value (max 1024 characters).
    pub value: String,
    /// Whether this field should be displayed inline with adjacent fields.
    #[serde(default)]
    pub inline: bool,
}

// ---------------------------------------------------------------------------
// Builder
// ---------------------------------------------------------------------------

/// Fluent builder for [`Embed`].
///
/// All methods consume and return `self` for chaining.
pub struct EmbedBuilder(Embed);

impl EmbedBuilder {
    /// Create an empty embed builder.
    pub fn new() -> Self {
        Self(Embed::default())
    }

    /// Set the embed title.
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.0.title = Some(title.into());
        self
    }

    /// Set the embed description.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.0.description = Some(description.into());
        self
    }

    /// Set the title hyperlink URL.
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.0.url = Some(url.into());
        self
    }

    /// Set an ISO 8601 timestamp shown in the footer area.
    pub fn timestamp(mut self, timestamp: impl Into<String>) -> Self {
        self.0.timestamp = Some(timestamp.into());
        self
    }

    /// Set the sidebar color as an RGB integer.
    pub fn color(mut self, color: u32) -> Self {
        self.0.color = Some(color);
        self
    }

    /// Set the footer text.
    pub fn footer(mut self, text: impl Into<String>) -> Self {
        self.0.footer = Some(EmbedFooter {
            text: text.into(),
            icon_url: None,
        });
        self
    }

    /// Set the footer text and icon URL.
    pub fn footer_with_icon(
        mut self,
        text: impl Into<String>,
        icon_url: impl Into<String>,
    ) -> Self {
        self.0.footer = Some(EmbedFooter {
            text: text.into(),
            icon_url: Some(icon_url.into()),
        });
        self
    }

    /// Set the large image URL.
    pub fn image(mut self, url: impl Into<String>) -> Self {
        self.0.image = Some(EmbedMedia {
            url: url.into(),
            height: None,
            width: None,
        });
        self
    }

    /// Set the thumbnail image URL (displayed top-right).
    pub fn thumbnail(mut self, url: impl Into<String>) -> Self {
        self.0.thumbnail = Some(EmbedMedia {
            url: url.into(),
            height: None,
            width: None,
        });
        self
    }

    /// Set the author name.
    pub fn author(mut self, name: impl Into<String>) -> Self {
        self.0.author = Some(EmbedAuthor {
            name: name.into(),
            url: None,
            icon_url: None,
        });
        self
    }

    /// Set the author name and hyperlink URL.
    pub fn author_with_url(
        mut self,
        name: impl Into<String>,
        url: impl Into<String>,
    ) -> Self {
        self.0.author = Some(EmbedAuthor {
            name: name.into(),
            url: Some(url.into()),
            icon_url: None,
        });
        self
    }

    /// Append a field to the embed.
    pub fn field(
        mut self,
        name: impl Into<String>,
        value: impl Into<String>,
        inline: bool,
    ) -> Self {
        self.0.fields.push(EmbedField {
            name: name.into(),
            value: value.into(),
            inline,
        });
        self
    }

    /// Consume the builder and return the finished [`Embed`].
    pub fn build(self) -> Embed {
        self.0
    }
}

impl Default for EmbedBuilder {
    fn default() -> Self {
        Self::new()
    }
}
