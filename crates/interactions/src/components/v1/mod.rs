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

//! Discord message components (v1: buttons, select menus, text inputs).
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
    /// Opens an SKU purchase flow (requires `sku_id`, no `custom_id`/`url`).
    Premium = 6,
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
            6 => Ok(Self::Premium),
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
    /// Developer-defined identifier (not used for Link/Premium buttons).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_id: Option<String>,
    /// URL to open (Link buttons only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// SKU ID for Premium buttons.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku_id: Option<Snowflake>,
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
            sku_id: None,
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
            sku_id: None,
            disabled: false,
        }
    }

    /// Create a Premium button that opens an SKU purchase flow.
    pub fn premium(sku_id: Snowflake) -> Self {
        Self {
            kind: ComponentType::Button,
            style: ButtonStyle::Premium,
            label: None,
            emoji: None,
            custom_id: None,
            url: None,
            sku_id: Some(sku_id),
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

    /// Add a text input to this row (for modals).
    pub fn text_input(mut self, input: TextInput) -> Self {
        self.components.push(input.into_value());
        self
    }

    /// Add a user select menu to this row.
    pub fn user_select(mut self, menu: UserSelect) -> Self {
        self.components.push(menu.into_value());
        self
    }

    /// Add a role select menu to this row.
    pub fn role_select(mut self, menu: RoleSelect) -> Self {
        self.components.push(menu.into_value());
        self
    }

    /// Add a mentionable select menu to this row.
    pub fn mentionable_select(mut self, menu: MentionableSelect) -> Self {
        self.components.push(menu.into_value());
        self
    }

    /// Add a channel select menu to this row.
    pub fn channel_select(mut self, menu: ChannelSelect) -> Self {
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

// ---------------------------------------------------------------------------
// Text input (modals)
// ---------------------------------------------------------------------------

/// Visual style for a [`TextInput`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "u8", into = "u8")]
pub enum TextInputStyle {
    /// Single-line input.
    Short = 1,
    /// Multi-line input.
    Paragraph = 2,
}

impl TryFrom<u8> for TextInputStyle {
    type Error = String;
    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            1 => Ok(Self::Short),
            2 => Ok(Self::Paragraph),
            _ => Err(format!("unknown text input style: {v}")),
        }
    }
}

impl From<TextInputStyle> for u8 {
    fn from(s: TextInputStyle) -> u8 {
        s as u8
    }
}

/// A text input component for use inside modal dialogs.
///
/// Text inputs must be placed inside an [`ActionRow`].
///
/// ```rust,ignore
/// let row = ActionRow::new().text_input(
///     TextInput::short("name", "Your name")
///         .placeholder("John Doe")
///         .required(),
/// );
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextInput {
    /// Always [`ComponentType::TextInput`].
    #[serde(rename = "type")]
    pub kind: ComponentType,
    pub custom_id: String,
    pub style: TextInputStyle,
    pub label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_length: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_length: Option<u16>,
    #[serde(default)]
    pub required: bool,
    /// Pre-filled value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,
}

impl TextInput {
    /// Create a single-line text input.
    pub fn short(custom_id: impl Into<String>, label: impl Into<String>) -> Self {
        Self::new(TextInputStyle::Short, custom_id, label)
    }

    /// Create a multi-line text input.
    pub fn paragraph(custom_id: impl Into<String>, label: impl Into<String>) -> Self {
        Self::new(TextInputStyle::Paragraph, custom_id, label)
    }

    fn new(
        style: TextInputStyle,
        custom_id: impl Into<String>,
        label: impl Into<String>,
    ) -> Self {
        Self {
            kind: ComponentType::TextInput,
            custom_id: custom_id.into(),
            style,
            label: label.into(),
            min_length: None,
            max_length: None,
            required: false,
            value: None,
            placeholder: None,
        }
    }

    pub fn min_length(mut self, n: u16) -> Self {
        self.min_length = Some(n);
        self
    }

    pub fn max_length(mut self, n: u16) -> Self {
        self.max_length = Some(n);
        self
    }

    pub fn required(mut self) -> Self {
        self.required = true;
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn placeholder(mut self, text: impl Into<String>) -> Self {
        self.placeholder = Some(text.into());
        self
    }

    /// Serialize to a JSON [`Value`].
    pub fn into_value(self) -> Value {
        serde_json::to_value(self).expect("TextInput serialize never fails")
    }
}

// ---------------------------------------------------------------------------
// Auto-populated select menus
// ---------------------------------------------------------------------------

/// A default value for auto-populated select menus.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectDefaultValue {
    /// The entity snowflake ID.
    pub id: Snowflake,
    /// The type of entity: `"user"`, `"role"`, or `"channel"`.
    #[serde(rename = "type")]
    pub kind: String,
}

impl SelectDefaultValue {
    pub fn user(id: Snowflake) -> Self {
        Self {
            id,
            kind: "user".into(),
        }
    }
    pub fn role(id: Snowflake) -> Self {
        Self {
            id,
            kind: "role".into(),
        }
    }
    pub fn channel(id: Snowflake) -> Self {
        Self {
            id,
            kind: "channel".into(),
        }
    }
}

/// A user select menu: Discord auto-populates it with guild members.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSelect {
    /// Always [`ComponentType::UserSelect`].
    #[serde(rename = "type")]
    pub kind: ComponentType,
    pub custom_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_values: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_values: Option<u8>,
    /// Pre-selected default values.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub default_values: Vec<SelectDefaultValue>,
    #[serde(default)]
    pub disabled: bool,
}

impl UserSelect {
    pub fn new(custom_id: impl Into<String>) -> Self {
        Self {
            kind: ComponentType::UserSelect,
            custom_id: custom_id.into(),
            placeholder: None,
            min_values: None,
            max_values: None,
            default_values: Vec::new(),
            disabled: false,
        }
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
        serde_json::to_value(self).expect("UserSelect serialize never fails")
    }
}

/// A role select menu: Discord auto-populates it with guild roles.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleSelect {
    /// Always [`ComponentType::RoleSelect`].
    #[serde(rename = "type")]
    pub kind: ComponentType,
    pub custom_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_values: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_values: Option<u8>,
    /// Pre-selected default values.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub default_values: Vec<SelectDefaultValue>,
    #[serde(default)]
    pub disabled: bool,
}

impl RoleSelect {
    pub fn new(custom_id: impl Into<String>) -> Self {
        Self {
            kind: ComponentType::RoleSelect,
            custom_id: custom_id.into(),
            placeholder: None,
            min_values: None,
            max_values: None,
            default_values: Vec::new(),
            disabled: false,
        }
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
        serde_json::to_value(self).expect("RoleSelect serialize never fails")
    }
}

/// A mentionable select menu: shows both users and roles.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MentionableSelect {
    /// Always [`ComponentType::MentionableSelect`].
    #[serde(rename = "type")]
    pub kind: ComponentType,
    pub custom_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_values: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_values: Option<u8>,
    /// Pre-selected default values.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub default_values: Vec<SelectDefaultValue>,
    #[serde(default)]
    pub disabled: bool,
}

impl MentionableSelect {
    pub fn new(custom_id: impl Into<String>) -> Self {
        Self {
            kind: ComponentType::MentionableSelect,
            custom_id: custom_id.into(),
            placeholder: None,
            min_values: None,
            max_values: None,
            default_values: Vec::new(),
            disabled: false,
        }
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
        serde_json::to_value(self).expect("MentionableSelect serialize never fails")
    }
}

/// A channel select menu: Discord auto-populates it with guild channels.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelSelect {
    /// Always [`ComponentType::ChannelSelect`].
    #[serde(rename = "type")]
    pub kind: ComponentType,
    pub custom_id: String,
    /// Restrict to specific channel types (e.g. `[0]` for text channels only).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub channel_types: Vec<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_values: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_values: Option<u8>,
    /// Pre-selected default values.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub default_values: Vec<SelectDefaultValue>,
    #[serde(default)]
    pub disabled: bool,
}

impl ChannelSelect {
    pub fn new(custom_id: impl Into<String>) -> Self {
        Self {
            kind: ComponentType::ChannelSelect,
            custom_id: custom_id.into(),
            channel_types: Vec::new(),
            placeholder: None,
            min_values: None,
            max_values: None,
            default_values: Vec::new(),
            disabled: false,
        }
    }

    /// Restrict the menu to specific channel types.
    pub fn channel_types(mut self, types: impl Into<Vec<u8>>) -> Self {
        self.channel_types = types.into();
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
        serde_json::to_value(self).expect("ChannelSelect serialize never fails")
    }
}
