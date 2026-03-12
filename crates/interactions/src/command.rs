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

//! Slash command definition types and builder.
//!
//! Use [`SlashCommandBuilder`] to construct an [`ApplicationCommand`] that can
//! be registered with Discord via the HTTP client.

use serde::{Deserialize, Serialize};

use std::collections::HashMap;

use oxidian_core::{
    models::interaction::{ApplicationCommandType, CommandOptionType},
    snowflake::Snowflake,
};

/// A predefined choice for a string, integer, or number option.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandChoice {
    /// Display name shown to the user.
    pub name: String,
    /// Value submitted when the user picks this choice.
    pub value: serde_json::Value,
    /// Localized display names, keyed by Discord locale code (e.g. `"de"`).
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub name_localizations: HashMap<String, String>,
}

impl CommandChoice {
    /// A choice with a string value.
    pub fn string(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value: serde_json::Value::String(value.into()),
            name_localizations: HashMap::new(),
        }
    }

    /// A choice with an integer value.
    pub fn integer(name: impl Into<String>, value: i64) -> Self {
        Self {
            name: name.into(),
            value: serde_json::Value::from(value),
            name_localizations: HashMap::new(),
        }
    }

    /// A choice with a floating-point number value.
    pub fn number(name: impl Into<String>, value: f64) -> Self {
        Self {
            name: name.into(),
            value: serde_json::Value::from(value),
            name_localizations: HashMap::new(),
        }
    }

    /// Set localized display names for this choice.
    pub fn name_localizations(mut self, map: HashMap<String, String>) -> Self {
        self.name_localizations = map;
        self
    }
}

/// A parameter definition for a slash command.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandOption {
    /// The option's type.
    #[serde(rename = "type")]
    pub kind: CommandOptionType,
    /// The option name (1–32 chars, lowercase, no spaces).
    pub name: String,
    /// A short description (1–100 chars).
    pub description: String,
    /// Localized option names keyed by Discord locale code.
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub name_localizations: HashMap<String, String>,
    /// Localized option descriptions keyed by Discord locale code.
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub description_localizations: HashMap<String, String>,
    /// Whether this option must be provided.
    #[serde(default)]
    pub required: bool,
    /// Predefined choices; if non-empty the user must pick one.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub choices: Vec<CommandChoice>,
    /// Nested options (sub-commands / sub-command groups).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub options: Vec<CommandOption>,
    /// Minimum numeric value (integer / number options).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_value: Option<f64>,
    /// Maximum numeric value (integer / number options).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_value: Option<f64>,
    /// Minimum string length (string options).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_length: Option<u32>,
    /// Maximum string length (string options).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_length: Option<u32>,
    /// Whether this option enables autocomplete.
    #[serde(default)]
    pub autocomplete: bool,
}

/// Builder for [`CommandOption`].
pub struct CommandOptionBuilder {
    kind: CommandOptionType,
    name: String,
    description: String,
    name_localizations: HashMap<String, String>,
    description_localizations: HashMap<String, String>,
    required: bool,
    choices: Vec<CommandChoice>,
    options: Vec<CommandOption>,
    min_value: Option<f64>,
    max_value: Option<f64>,
    min_length: Option<u32>,
    max_length: Option<u32>,
    autocomplete: bool,
}

impl CommandOptionBuilder {
    /// Create a new option builder.
    pub fn new(
        kind: CommandOptionType,
        name: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            name: name.into(),
            description: description.into(),
            name_localizations: HashMap::new(),
            description_localizations: HashMap::new(),
            required: false,
            choices: Vec::new(),
            options: Vec::new(),
            min_value: None,
            max_value: None,
            min_length: None,
            max_length: None,
            autocomplete: false,
        }
    }

    /// Mark this option as required.
    pub fn required(mut self) -> Self {
        self.required = true;
        self
    }

    /// Add a predefined choice.
    pub fn choice(mut self, c: CommandChoice) -> Self {
        self.choices.push(c);
        self
    }

    /// Add a nested option (sub-command / sub-command group).
    pub fn option(mut self, o: CommandOption) -> Self {
        self.options.push(o);
        self
    }

    /// Set the minimum accepted value for integer / number options.
    pub fn min_value(mut self, v: f64) -> Self {
        self.min_value = Some(v);
        self
    }

    /// Set the maximum accepted value for integer / number options.
    pub fn max_value(mut self, v: f64) -> Self {
        self.max_value = Some(v);
        self
    }

    /// Set the minimum accepted string length.
    pub fn min_length(mut self, n: u32) -> Self {
        self.min_length = Some(n);
        self
    }

    /// Set the maximum accepted string length.
    pub fn max_length(mut self, n: u32) -> Self {
        self.max_length = Some(n);
        self
    }

    /// Enable autocomplete for this option.
    pub fn autocomplete(mut self) -> Self {
        self.autocomplete = true;
        self
    }

    /// Set localized option names.
    pub fn name_localizations(mut self, map: HashMap<String, String>) -> Self {
        self.name_localizations = map;
        self
    }

    /// Set localized option descriptions.
    pub fn description_localizations(mut self, map: HashMap<String, String>) -> Self {
        self.description_localizations = map;
        self
    }

    /// Finalise and return a [`CommandOption`].
    pub fn build(self) -> CommandOption {
        CommandOption {
            kind: self.kind,
            name: self.name,
            description: self.description,
            name_localizations: self.name_localizations,
            description_localizations: self.description_localizations,
            required: self.required,
            choices: self.choices,
            options: self.options,
            min_value: self.min_value,
            max_value: self.max_value,
            min_length: self.min_length,
            max_length: self.max_length,
            autocomplete: self.autocomplete,
        }
    }
}

/// The definition of a Discord application command (used when registering
/// with the Discord REST API).
///
/// Build one with [`SlashCommandBuilder`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationCommand {
    /// The command name (1–32 chars).
    pub name: String,
    /// Localized command names keyed by Discord locale code.
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub name_localizations: HashMap<String, String>,
    /// Short description (1–100 chars, chat-input only).
    pub description: String,
    /// Localized command descriptions keyed by Discord locale code.
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub description_localizations: HashMap<String, String>,
    /// The kind of command.
    #[serde(rename = "type")]
    pub kind: ApplicationCommandType,
    /// The parameter list.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub options: Vec<CommandOption>,
    /// Required member permissions as a bitfield string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_member_permissions: Option<String>,
    /// Whether this command is available in DMs (default `true`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dm_permission: Option<bool>,
    /// Whether the command is age-restricted (NSFW).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsfw: Option<bool>,
    /// Installation contexts where the command is available.
    /// `0` = GUILD_INSTALL, `1` = USER_INSTALL.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub integration_types: Vec<u8>,
    /// Interaction contexts where the command can be used.
    /// `0` = GUILD, `1` = BOT_DM, `2` = PRIVATE_CHANNEL.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contexts: Vec<u8>,
    /// The command ID (only set on responses from Discord).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Snowflake>,
    /// The application ID (only set on responses from Discord).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<Snowflake>,
    /// Guild ID if this is a guild-scoped command (only on responses).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<Snowflake>,
    /// Auto-incrementing version (only set on responses from Discord).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<Snowflake>,
}

/// Fluent builder for chat-input slash commands.
///
/// # Example
///
/// ```rust,ignore
/// let cmd = SlashCommandBuilder::new("greet", "Greet a user")
///     .option(
///         CommandOptionBuilder::new(CommandOptionType::User, "user", "Who to greet")
///             .required()
///             .build(),
///     )
///     .build();
/// ```
pub struct SlashCommandBuilder {
    name: String,
    name_localizations: HashMap<String, String>,
    description: String,
    description_localizations: HashMap<String, String>,
    options: Vec<CommandOption>,
    default_member_permissions: Option<String>,
    dm_permission: Option<bool>,
    nsfw: Option<bool>,
    integration_types: Vec<u8>,
    contexts: Vec<u8>,
}

impl SlashCommandBuilder {
    /// Start building a new chat-input slash command.
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            name_localizations: HashMap::new(),
            description: description.into(),
            description_localizations: HashMap::new(),
            options: Vec::new(),
            default_member_permissions: None,
            dm_permission: None,
            nsfw: None,
            integration_types: Vec::new(),
            contexts: Vec::new(),
        }
    }

    /// Add an option (parameter) to the command.
    pub fn option(mut self, option: CommandOption) -> Self {
        self.options.push(option);
        self
    }

    /// Restrict this command to members with the given permission bitfield.
    pub fn default_member_permissions(mut self, perms: impl Into<String>) -> Self {
        self.default_member_permissions = Some(perms.into());
        self
    }

    /// Control whether this command is available in DMs (default: `true`).
    pub fn dm_permission(mut self, allowed: bool) -> Self {
        self.dm_permission = Some(allowed);
        self
    }

    /// Set localized command names.
    pub fn name_localizations(mut self, map: HashMap<String, String>) -> Self {
        self.name_localizations = map;
        self
    }

    /// Set localized command descriptions.
    pub fn description_localizations(mut self, map: HashMap<String, String>) -> Self {
        self.description_localizations = map;
        self
    }

    /// Mark this command as age-restricted (NSFW).
    pub fn nsfw(mut self, nsfw: bool) -> Self {
        self.nsfw = Some(nsfw);
        self
    }

    /// Set the installation contexts where this command is available.
    ///
    /// Values: `0` = GUILD_INSTALL, `1` = USER_INSTALL.
    pub fn integration_types(mut self, types: Vec<u8>) -> Self {
        self.integration_types = types;
        self
    }

    /// Set the interaction contexts where this command can be used.
    ///
    /// Values: `0` = GUILD, `1` = BOT_DM, `2` = PRIVATE_CHANNEL.
    pub fn contexts(mut self, ctx: Vec<u8>) -> Self {
        self.contexts = ctx;
        self
    }

    /// Build the final [`ApplicationCommand`].
    pub fn build(self) -> ApplicationCommand {
        ApplicationCommand {
            name: self.name,
            name_localizations: self.name_localizations,
            description: self.description,
            description_localizations: self.description_localizations,
            kind: ApplicationCommandType::ChatInput,
            options: self.options,
            default_member_permissions: self.default_member_permissions,
            dm_permission: self.dm_permission,
            nsfw: self.nsfw,
            integration_types: self.integration_types,
            contexts: self.contexts,
            id: None,
            application_id: None,
            guild_id: None,
            version: None,
        }
    }
}
