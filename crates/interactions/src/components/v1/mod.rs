//! Discord message components (v1 — buttons, select menus, text inputs).
//!
//! Wrap components inside an [`ActionRow`] before including them in a message.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use oxidian_core::snowflake::Snowflake;

/// Type discriminant for a message component.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "u8", into = "u8")]
pub enum ComponentType {
    ActionRow = 1,
    Button = 2,
    StringSelect = 3,
    TextInput = 4,
    UserSelect = 5,
    RoleSelect = 6,
    MentionableSelect = 7,
    ChannelSelect = 8,
}

impl TryFrom<u8> for ComponentType {
    type Error = String;
    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            1 => Ok(Self::ActionRow),
            2 => Ok(Self::Button),
            3 => Ok(Self::StringSelect),
            4 => Ok(Self::TextInput),
            5 => Ok(Self::UserSelect),
            6 => Ok(Self::RoleSelect),
            7 => Ok(Self::MentionableSelect),
            8 => Ok(Self::ChannelSelect),
            _ => Err(format!("unknown component type: {v}")),
        }
    }
}

impl From<ComponentType> for u8 {
    fn from(t: ComponentType) -> u8 {
        t as u8
    }
}

/// Button visual style.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "u8", into = "u8")]
pub enum ButtonStyle {
    /// Blurple.
    Primary = 1,
    /// Grey.
    Secondary = 2,
    /// Green.
    Success = 3,
    /// Red.
    Danger = 4,
    /// Opens a URL (no `custom_id`, must have `url`).
    Link = 5,
}

impl TryFrom<u8> for ButtonStyle {
    type Error = String;
    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            1 => Ok(Self::Primary),
            2 => Ok(Self::Secondary),
            3 => Ok(Self::Success),
            4 => Ok(Self::Danger),
            5 => Ok(Self::Link),
            _ => Err(format!("unknown button style: {v}")),
        }
    }
}

impl From<ButtonStyle> for u8 {
    fn from(s: ButtonStyle) -> u8 {
        s as u8
    }
}

/// A partial emoji used inside components.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartialEmoji {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Snowflake>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    pub animated: bool,
}

/// A clickable button component.
///
/// # Example
///
/// ```rust,ignore
/// let btn = Button::new(ButtonStyle::Primary, "my_button")
///     .label("Click me!")
///     .build();
/// let row = ActionRow::new().button(btn);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Button {
    /// Always [`ComponentType::Button`].
    #[serde(rename = "type")]
    pub kind: ComponentType,
    pub style: ButtonStyle,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<PartialEmoji>,
    /// Developer-defined identifier (not used for Link buttons).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_id: Option<String>,
    /// URL to open (Link buttons only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default)]
    pub disabled: bool,
}

impl Button {
    /// Create a button with a click interaction style.
    pub fn new(style: ButtonStyle, custom_id: impl Into<String>) -> Self {
        Self {
            kind: ComponentType::Button,
            style,
            label: None,
            emoji: None,
            custom_id: Some(custom_id.into()),
            url: None,
            disabled: false,
        }
    }

    /// Create a Link button that opens a URL.
    pub fn link(url: impl Into<String>) -> Self {
        Self {
            kind: ComponentType::Button,
            style: ButtonStyle::Link,
            label: None,
            emoji: None,
            custom_id: None,
            url: Some(url.into()),
            disabled: false,
        }
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn emoji(mut self, emoji: PartialEmoji) -> Self {
        self.emoji = Some(emoji);
        self
    }

    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }

    /// Serialize to a JSON [`Value`] suitable for a message component payload.
    pub fn into_value(self) -> Value {
        serde_json::to_value(self).expect("Button serialize never fails")
    }
}

/// One option in a string select menu.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectOption {
    pub label: String,
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<PartialEmoji>,
    /// Whether this option is pre-selected.
    #[serde(default)]
    pub default: bool,
}

impl SelectOption {
    pub fn new(label: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
            description: None,
            emoji: None,
            default: false,
        }
    }

    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    pub fn default_selected(mut self) -> Self {
        self.default = true;
        self
    }
}

/// A string select menu component.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StringSelect {
    /// Always [`ComponentType::StringSelect`].
    #[serde(rename = "type")]
    pub kind: ComponentType,
    pub custom_id: String,
    pub options: Vec<SelectOption>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_values: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_values: Option<u8>,
    #[serde(default)]
    pub disabled: bool,
}

impl StringSelect {
    pub fn new(custom_id: impl Into<String>) -> Self {
        Self {
            kind: ComponentType::StringSelect,
            custom_id: custom_id.into(),
            options: Vec::new(),
            placeholder: None,
            min_values: None,
            max_values: None,
            disabled: false,
        }
    }

    pub fn option(mut self, opt: SelectOption) -> Self {
        self.options.push(opt);
        self
    }

    pub fn placeholder(mut self, text: impl Into<String>) -> Self {
        self.placeholder = Some(text.into());
        self
    }

    pub fn min_values(mut self, n: u8) -> Self {
        self.min_values = Some(n);
        self
    }

    pub fn max_values(mut self, n: u8) -> Self {
        self.max_values = Some(n);
        self
    }

    /// Serialize to a JSON [`Value`].
    pub fn into_value(self) -> Value {
        serde_json::to_value(self).expect("StringSelect serialize never fails")
    }
}

/// A container that holds up to 5 interactive components in a single row.
///
/// Messages support up to 5 action rows.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionRow {
    /// Always [`ComponentType::ActionRow`].
    #[serde(rename = "type")]
    pub kind: ComponentType,
    pub components: Vec<Value>,
}

impl ActionRow {
    pub fn new() -> Self {
        Self {
            kind: ComponentType::ActionRow,
            components: Vec::new(),
        }
    }

    /// Add a button to this row.
    pub fn button(mut self, btn: Button) -> Self {
        self.components.push(btn.into_value());
        self
    }

    /// Add a string select menu to this row.
    pub fn select(mut self, menu: StringSelect) -> Self {
        self.components.push(menu.into_value());
        self
    }

    /// Serialize to a JSON [`Value`].
    pub fn into_value(self) -> Value {
        serde_json::to_value(self).expect("ActionRow serialize never fails")
    }
}

impl Default for ActionRow {
    fn default() -> Self {
        Self::new()
    }
}
