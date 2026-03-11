//! Discord interaction model.
//!
//! Interactions arrive via the `INTERACTION_CREATE` gateway dispatch event when
//! a user invokes a slash command, clicks a button, submits a modal, etc.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    models::{member::Member, message::Message, user::User},
    snowflake::Snowflake,
};

/// The type of an incoming interaction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "u8", into = "u8")]
pub enum InteractionType {
    /// Sent by Discord to test webhook connectivity.
    Ping = 1,
    /// A slash command, user command, or message command was invoked.
    ApplicationCommand = 2,
    /// An interactive component (button, select menu, …) was used.
    MessageComponent = 3,
    /// The user typed an option with autocomplete enabled.
    ApplicationCommandAutocomplete = 4,
    /// A modal was submitted.
    ModalSubmit = 5,
}

impl TryFrom<u8> for InteractionType {
    type Error = String;
    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            1 => Ok(Self::Ping),
            2 => Ok(Self::ApplicationCommand),
            3 => Ok(Self::MessageComponent),
            4 => Ok(Self::ApplicationCommandAutocomplete),
            5 => Ok(Self::ModalSubmit),
            _ => Err(format!("unknown interaction type: {v}")),
        }
    }
}

impl From<InteractionType> for u8 {
    fn from(t: InteractionType) -> u8 {
        t as u8
    }
}

/// Discriminates between chat-input (`/command`), user, and message commands.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "u8", into = "u8")]
pub enum ApplicationCommandType {
    /// A `/slash` command.
    ChatInput = 1,
    /// Right-click → Apps on a user.
    User = 2,
    /// Right-click → Apps on a message.
    Message = 3,
}

impl TryFrom<u8> for ApplicationCommandType {
    type Error = String;
    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            1 => Ok(Self::ChatInput),
            2 => Ok(Self::User),
            3 => Ok(Self::Message),
            _ => Err(format!("unknown application command type: {v}")),
        }
    }
}

impl From<ApplicationCommandType> for u8 {
    fn from(t: ApplicationCommandType) -> u8 {
        t as u8
    }
}

impl Default for ApplicationCommandType {
    fn default() -> Self {
        Self::ChatInput
    }
}

/// The type of a command option (parameter).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "u8", into = "u8")]
pub enum CommandOptionType {
    SubCommand = 1,
    SubCommandGroup = 2,
    String = 3,
    Integer = 4,
    Boolean = 5,
    User = 6,
    Channel = 7,
    Role = 8,
    Mentionable = 9,
    Number = 10,
    Attachment = 11,
}

impl TryFrom<u8> for CommandOptionType {
    type Error = String;
    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            1 => Ok(Self::SubCommand),
            2 => Ok(Self::SubCommandGroup),
            3 => Ok(Self::String),
            4 => Ok(Self::Integer),
            5 => Ok(Self::Boolean),
            6 => Ok(Self::User),
            7 => Ok(Self::Channel),
            8 => Ok(Self::Role),
            9 => Ok(Self::Mentionable),
            10 => Ok(Self::Number),
            11 => Ok(Self::Attachment),
            _ => Err(format!("unknown command option type: {v}")),
        }
    }
}

impl From<CommandOptionType> for u8 {
    fn from(t: CommandOptionType) -> u8 {
        t as u8
    }
}

/// Entities resolved from snowflake IDs in an application command invocation.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResolvedData {
    /// Users resolved from option values.
    #[serde(default)]
    pub users: HashMap<Snowflake, User>,
    /// Guild members resolved from option values.
    #[serde(default)]
    pub members: HashMap<Snowflake, Member>,
}

/// A single option value provided to an application command.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CommandInteractionOption {
    /// The option name.
    pub name: String,
    /// The option type.
    #[serde(rename = "type")]
    pub kind: CommandOptionType,
    /// The resolved value (string, integer, boolean, snowflake, …).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
    /// Nested options for sub-commands or sub-command groups.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub options: Vec<CommandInteractionOption>,
    /// `true` when this option triggered autocomplete.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub focused: Option<bool>,
}

/// Interaction data for an application command (slash, user, or message).
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ApplicationCommandData {
    /// The ID of the command that was invoked.
    pub id: Snowflake,
    /// The name of the command.
    pub name: String,
    /// The kind of command.
    #[serde(rename = "type")]
    pub kind: ApplicationCommandType,
    /// Options (arguments) provided by the user.
    #[serde(default)]
    pub options: Vec<CommandInteractionOption>,
    /// Resolved entities referenced by options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolved: Option<ResolvedData>,
    /// Target entity ID for user/message commands.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_id: Option<Snowflake>,
}

/// Interaction data for a message component (button, select menu, …).
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MessageComponentData {
    /// The developer-defined `custom_id` of the component.
    pub custom_id: String,
    /// The component type (2 = button, 3 = string select, …).
    pub component_type: u8,
    /// Selected values (for select menus).
    #[serde(default)]
    pub values: Vec<String>,
}

/// Interaction data for a modal submission.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ModalSubmitData {
    /// The developer-defined `custom_id` of the modal.
    pub custom_id: String,
    /// Action row components containing the submitted text inputs.
    pub components: Vec<serde_json::Value>,
}

/// The data payload of an interaction, discriminated by unique required fields.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InteractionData {
    /// Data from an application command invocation (has `id` + `name` + `type`).
    ApplicationCommand(ApplicationCommandData),
    /// Data from a message component interaction (has `component_type`).
    MessageComponent(MessageComponentData),
    /// Data from a modal submission (has `components` array).
    ModalSubmit(ModalSubmitData),
}

/// An incoming Discord interaction.
///
/// Interactions are delivered as `INTERACTION_CREATE` gateway dispatch events
/// and must be acknowledged within 3 seconds.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Interaction {
    /// Unique ID of this interaction instance.
    pub id: Snowflake,
    /// The application this interaction belongs to.
    pub application_id: Snowflake,
    /// The kind of interaction.
    #[serde(rename = "type")]
    pub kind: InteractionType,
    /// Type-specific data payload.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<InteractionData>,
    /// The guild this interaction was sent from (absent for DMs).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<Snowflake>,
    /// The channel this interaction was sent from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<Snowflake>,
    /// The guild member who triggered the interaction (guild-only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member: Option<Member>,
    /// The user who triggered the interaction (DM-only; use [`Self::author`]).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
    /// Continuation token used for responding to the interaction.
    pub token: String,
    /// Always `1`.
    pub version: u8,
    /// The message the component was attached to (component interactions only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<Message>,
}

impl Interaction {
    /// Return the user who triggered this interaction, whether it came from a
    /// guild (via `member.user`) or a DM (via `user`).
    pub fn author(&self) -> Option<&User> {
        self.user
            .as_ref()
            .or_else(|| self.member.as_ref().and_then(|m| m.user.as_ref()))
    }

    /// Return the application command data, if applicable.
    pub fn command_data(&self) -> Option<&ApplicationCommandData> {
        match &self.data {
            Some(InteractionData::ApplicationCommand(d)) => Some(d),
            _ => None,
        }
    }

    /// Return the message component data, if applicable.
    pub fn component_data(&self) -> Option<&MessageComponentData> {
        match &self.data {
            Some(InteractionData::MessageComponent(d)) => Some(d),
            _ => None,
        }
    }

    /// Return the modal submit data, if applicable.
    pub fn modal_data(&self) -> Option<&ModalSubmitData> {
        match &self.data {
            Some(InteractionData::ModalSubmit(d)) => Some(d),
            _ => None,
        }
    }
}

/// How the bot responds to an interaction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "u8", into = "u8")]
pub enum InteractionResponseType {
    /// ACK a `Ping` (webhook endpoint verification).
    Pong = 1,
    /// Send a message as the response, shown immediately.
    ChannelMessageWithSource = 4,
    /// Acknowledge the interaction with a loading state; edit the response later.
    DeferredChannelMessageWithSource = 5,
    /// For component interactions: acknowledge without sending a message.
    DeferredUpdateMessage = 6,
    /// For component interactions: edit the message the component is on.
    UpdateMessage = 7,
    /// Respond to an autocomplete request with choices.
    ApplicationCommandAutocompleteResult = 8,
    /// Respond with a modal dialog.
    Modal = 9,
}

impl TryFrom<u8> for InteractionResponseType {
    type Error = String;
    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            1 => Ok(Self::Pong),
            4 => Ok(Self::ChannelMessageWithSource),
            5 => Ok(Self::DeferredChannelMessageWithSource),
            6 => Ok(Self::DeferredUpdateMessage),
            7 => Ok(Self::UpdateMessage),
            8 => Ok(Self::ApplicationCommandAutocompleteResult),
            9 => Ok(Self::Modal),
            _ => Err(format!("unknown interaction response type: {v}")),
        }
    }
}

impl From<InteractionResponseType> for u8 {
    fn from(t: InteractionResponseType) -> u8 {
        t as u8
    }
}

/// Message data for an interaction response.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InteractionCallbackData {
    /// Text content of the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// Message flags. Set bit 6 (`64`) for ephemeral messages.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<u64>,
    /// Embeds to include.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub embeds: Vec<serde_json::Value>,
    /// Components to include.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub components: Vec<serde_json::Value>,
}

impl InteractionCallbackData {
    /// A simple text response.
    pub fn message(content: impl Into<String>) -> Self {
        Self {
            content: Some(content.into()),
            ..Default::default()
        }
    }

    /// An ephemeral text response visible only to the invoking user.
    pub fn ephemeral(content: impl Into<String>) -> Self {
        Self {
            content: Some(content.into()),
            flags: Some(64),
            ..Default::default()
        }
    }
}

/// Body sent to `POST /interactions/{id}/{token}/callback`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractionResponse {
    /// How to respond.
    #[serde(rename = "type")]
    pub kind: InteractionResponseType,
    /// Optional data payload.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<InteractionCallbackData>,
}

impl InteractionResponse {
    /// Reply with a visible message.
    pub fn message(content: impl Into<String>) -> Self {
        Self {
            kind: InteractionResponseType::ChannelMessageWithSource,
            data: Some(InteractionCallbackData::message(content)),
        }
    }

    /// Reply with an ephemeral message visible only to the invoking user.
    pub fn ephemeral(content: impl Into<String>) -> Self {
        Self {
            kind: InteractionResponseType::ChannelMessageWithSource,
            data: Some(InteractionCallbackData::ephemeral(content)),
        }
    }

    /// Acknowledge with a loading state; edit the response later.
    pub fn defer() -> Self {
        Self {
            kind: InteractionResponseType::DeferredChannelMessageWithSource,
            data: None,
        }
    }

    /// For component interactions: acknowledge without editing the message.
    pub fn ack_component() -> Self {
        Self {
            kind: InteractionResponseType::DeferredUpdateMessage,
            data: None,
        }
    }
}

/// A single choice shown in an autocomplete dropdown.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutocompleteChoice {
    /// The display name shown to the user (max 100 chars).
    pub name: String,
    /// The value submitted when the user selects this choice.
    pub value: serde_json::Value,
}

impl AutocompleteChoice {
    /// A choice with a string value.
    pub fn string(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value: serde_json::Value::String(value.into()),
        }
    }

    /// A choice with an integer value.
    pub fn integer(name: impl Into<String>, value: i64) -> Self {
        Self {
            name: name.into(),
            value: serde_json::Value::from(value),
        }
    }
}
