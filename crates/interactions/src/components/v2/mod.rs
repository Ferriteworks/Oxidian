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

//! Discord message components v2.
//!
//! Components v2 adds layout primitives such as [`Section`], [`TextDisplay`],
//! [`MediaGallery`], [`Separator`], and [`Container`] that give bots rich
//! formatting capabilities beyond the original button / select menu system.
//!
//! Enable v2 by setting message flag `IS_COMPONENTS_V2` (bit 15, value `32768`)
//! in the response flags.

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Type discriminant for a v2 component.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "u8", into = "u8")]
pub enum ComponentV2Type {
    Section = 9,
    TextDisplay = 10,
    Thumbnail = 11,
    MediaGallery = 12,
    File = 13,
    Separator = 14,
    Container = 17,
}

impl TryFrom<u8> for ComponentV2Type {
    type Error = String;
    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            9 => Ok(Self::Section),
            10 => Ok(Self::TextDisplay),
            11 => Ok(Self::Thumbnail),
            12 => Ok(Self::MediaGallery),
            13 => Ok(Self::File),
            14 => Ok(Self::Separator),
            17 => Ok(Self::Container),
            _ => Err(format!("unknown v2 component type: {v}")),
        }
    }
}

impl From<ComponentV2Type> for u8 {
    fn from(t: ComponentV2Type) -> u8 {
        t as u8
    }
}

/// A media reference (image, gif, or video URL).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnfurlMedia {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
}

/// A text block displaying formatted markdown content.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextDisplay {
    #[serde(rename = "type")]
    pub kind: ComponentV2Type,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u32>,
}

impl TextDisplay {
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            kind: ComponentV2Type::TextDisplay,
            content: content.into(),
            id: None,
        }
    }

    pub fn into_value(self) -> Value {
        serde_json::to_value(self).expect("TextDisplay serialize never fails")
    }
}

/// A horizontal separator / divider line between components.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Separator {
    #[serde(rename = "type")]
    pub kind: ComponentV2Type,
    /// Show a visible divider line.
    #[serde(default)]
    pub divider: bool,
    /// Spacing: `1` = small, `2` = large.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spacing: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u32>,
}

impl Separator {
    pub fn new() -> Self {
        Self {
            kind: ComponentV2Type::Separator,
            divider: true,
            spacing: None,
            id: None,
        }
    }

    pub fn into_value(self) -> Value {
        serde_json::to_value(self).expect("Separator serialize never fails")
    }
}

impl Default for Separator {
    fn default() -> Self {
        Self::new()
    }
}

/// A single item inside a [`MediaGallery`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaGalleryItem {
    pub media: UnfurlMedia,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub spoiler: bool,
}

impl MediaGalleryItem {
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            media: UnfurlMedia {
                url: url.into(),
                proxy_url: None,
                height: None,
                width: None,
                content_type: None,
            },
            description: None,
            spoiler: false,
        }
    }

    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }
}

/// A gallery of up to 10 media items.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaGallery {
    #[serde(rename = "type")]
    pub kind: ComponentV2Type,
    pub items: Vec<MediaGalleryItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u32>,
}

impl MediaGallery {
    pub fn new() -> Self {
        Self {
            kind: ComponentV2Type::MediaGallery,
            items: Vec::new(),
            id: None,
        }
    }

    pub fn item(mut self, item: MediaGalleryItem) -> Self {
        self.items.push(item);
        self
    }

    pub fn into_value(self) -> Value {
        serde_json::to_value(self).expect("MediaGallery serialize never fails")
    }
}

impl Default for MediaGallery {
    fn default() -> Self {
        Self::new()
    }
}

/// A full-width top-level container holding other v2 components.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Container {
    #[serde(rename = "type")]
    pub kind: ComponentV2Type,
    pub components: Vec<Value>,
    /// Accent color as an RGB integer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accent_color: Option<u32>,
    #[serde(default)]
    pub spoiler: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u32>,
}

impl Container {
    pub fn new() -> Self {
        Self {
            kind: ComponentV2Type::Container,
            components: Vec::new(),
            accent_color: None,
            spoiler: false,
            id: None,
        }
    }

    /// Add any serializable component.
    pub fn component(mut self, c: Value) -> Self {
        self.components.push(c);
        self
    }

    pub fn accent_color(mut self, color: u32) -> Self {
        self.accent_color = Some(color);
        self
    }

    pub fn into_value(self) -> Value {
        serde_json::to_value(self).expect("Container serialize never fails")
    }
}

impl Default for Container {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Section
// ---------------------------------------------------------------------------

/// A layout section containing text content and an optional accessory
/// (button or thumbnail) displayed to the right.
///
/// ```rust,ignore
/// let section = Section::new()
///     .text(TextDisplay::new("Some info"))
///     .accessory(Thumbnail::new("https://example.com/icon.png"));
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Section {
    #[serde(rename = "type")]
    pub kind: ComponentV2Type,
    /// Text components inside the section (typically [`TextDisplay`]).
    pub components: Vec<Value>,
    /// An accessory displayed to the right (a [`Button`] or [`Thumbnail`]).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accessory: Option<Box<Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u32>,
}

impl Section {
    pub fn new() -> Self {
        Self {
            kind: ComponentV2Type::Section,
            components: Vec::new(),
            accessory: None,
            id: None,
        }
    }

    /// Add a text display component to this section.
    pub fn text(mut self, text: TextDisplay) -> Self {
        self.components.push(text.into_value());
        self
    }

    /// Set the accessory displayed alongside this section.
    pub fn accessory(mut self, accessory: impl serde::Serialize) -> Self {
        let val = serde_json::to_value(accessory)
            .expect("accessory serialization never fails");
        self.accessory = Some(Box::new(val));
        self
    }

    pub fn into_value(self) -> Value {
        serde_json::to_value(self).expect("Section serialize never fails")
    }
}

impl Default for Section {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Thumbnail
// ---------------------------------------------------------------------------

/// A small image typically used as an accessory inside a [`Section`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Thumbnail {
    #[serde(rename = "type")]
    pub kind: ComponentV2Type,
    pub media: UnfurlMedia,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub spoiler: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u32>,
}

impl Thumbnail {
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            kind: ComponentV2Type::Thumbnail,
            media: UnfurlMedia {
                url: url.into(),
                proxy_url: None,
                height: None,
                width: None,
                content_type: None,
            },
            description: None,
            spoiler: false,
            id: None,
        }
    }

    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    pub fn spoiler(mut self) -> Self {
        self.spoiler = true;
        self
    }

    pub fn into_value(self) -> Value {
        serde_json::to_value(self).expect("Thumbnail serialize never fails")
    }
}

// ---------------------------------------------------------------------------
// File
// ---------------------------------------------------------------------------

/// A file attachment component displayed as a downloadable block.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileComponent {
    #[serde(rename = "type")]
    pub kind: ComponentV2Type,
    pub file: UnfurlMedia,
    #[serde(default)]
    pub spoiler: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u32>,
}

impl FileComponent {
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            kind: ComponentV2Type::File,
            file: UnfurlMedia {
                url: url.into(),
                proxy_url: None,
                height: None,
                width: None,
                content_type: None,
            },
            spoiler: false,
            id: None,
        }
    }

    pub fn spoiler(mut self) -> Self {
        self.spoiler = true;
        self
    }

    pub fn into_value(self) -> Value {
        serde_json::to_value(self).expect("FileComponent serialize never fails")
    }
}
