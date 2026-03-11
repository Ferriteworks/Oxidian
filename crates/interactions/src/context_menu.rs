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

//! Context-menu command definitions.
//!
//! Context-menu commands surface as "Apps" in the right-click menu on users
//! ([`UserCommand`]) or messages ([`MessageCommand`]).  They are registered the
//! same way as slash commands but with `type = 2` or `type = 3`.

use serde::{Deserialize, Serialize};

use oxidian_core::{models::interaction::ApplicationCommandType, snowflake::Snowflake};

/// Definition for a **user** context-menu command (right-click a user → Apps).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserCommand {
    /// The command name (1–32 chars).
    pub name: String,
    /// Command type — always [`ApplicationCommandType::User`].
    #[serde(rename = "type")]
    pub kind: ApplicationCommandType,
    /// Required member permissions as a bitfield string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_member_permissions: Option<String>,
    /// The command ID (only set on responses from Discord).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Snowflake>,
}

impl UserCommand {
    /// Create a new user context-menu command definition.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            kind: ApplicationCommandType::User,
            default_member_permissions: None,
            id: None,
        }
    }

    /// Restrict this command to members with the given permission bitfield.
    pub fn default_member_permissions(mut self, perms: impl Into<String>) -> Self {
        self.default_member_permissions = Some(perms.into());
        self
    }
}

/// Definition for a **message** context-menu command (right-click a message → Apps).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageCommand {
    /// The command name (1–32 chars).
    pub name: String,
    /// Command type — always [`ApplicationCommandType::Message`].
    #[serde(rename = "type")]
    pub kind: ApplicationCommandType,
    /// Required member permissions as a bitfield string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_member_permissions: Option<String>,
    /// The command ID (only set on responses from Discord).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Snowflake>,
}

impl MessageCommand {
    /// Create a new message context-menu command definition.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            kind: ApplicationCommandType::Message,
            default_member_permissions: None,
            id: None,
        }
    }

    /// Restrict this command to members with the given permission bitfield.
    pub fn default_member_permissions(mut self, perms: impl Into<String>) -> Self {
        self.default_member_permissions = Some(perms.into());
        self
    }
}
