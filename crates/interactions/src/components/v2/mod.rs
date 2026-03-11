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
            9  => Ok(Self::Section),
            10 => Ok(Self::TextDisplay),
            11 => Ok(Self::Thumbnail),
            12 => Ok(Self::MediaGallery),
            13 => Ok(Self::File),
            14 => Ok(Self::Separator),
            17 => Ok(Self::Container),
            _  => Err(format!("unknown v2 component type: {v}")),
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
            media: UnfurlMedia { url: url.into() },
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
