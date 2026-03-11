//! Slash command definition types and builder.
//!
//! Use [`SlashCommandBuilder`] to construct an [`ApplicationCommand`] that can
//! be registered with Discord via the HTTP client.

use serde::{Deserialize, Serialize};

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
}

impl CommandChoice {
    /// A choice with a string value.
    pub fn string(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            name:  name.into(),
            value: serde_json::Value::String(value.into()),
        }
    }

    /// A choice with an integer value.
    pub fn integer(name: impl Into<String>, value: i64) -> Self {
        Self {
            name:  name.into(),
            value: serde_json::Value::from(value),
        }
    }

    /// A choice with a floating-point number value.
    pub fn number(name: impl Into<String>, value: f64) -> Self {
        Self {
            name:  name.into(),
            value: serde_json::Value::from(value),
        }
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

    /// Finalise and return a [`CommandOption`].
    pub fn build(self) -> CommandOption {
        CommandOption {
            kind: self.kind,
            name: self.name,
            description: self.description,
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
    /// Short description (1–100 chars, chat-input only).
    pub description: String,
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
    /// The command ID (only set on responses from Discord).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Snowflake>,
    /// The application ID (only set on responses from Discord).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<Snowflake>,
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
    description: String,
    options: Vec<CommandOption>,
    default_member_permissions: Option<String>,
    dm_permission: Option<bool>,
}

impl SlashCommandBuilder {
    /// Start building a new chat-input slash command.
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            options: Vec::new(),
            default_member_permissions: None,
            dm_permission: None,
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

    /// Build the final [`ApplicationCommand`].
    pub fn build(self) -> ApplicationCommand {
        ApplicationCommand {
            name: self.name,
            description: self.description,
            kind: ApplicationCommandType::ChatInput,
            options: self.options,
            default_member_permissions: self.default_member_permissions,
            dm_permission: self.dm_permission,
            id: None,
            application_id: None,
        }
    }
}
