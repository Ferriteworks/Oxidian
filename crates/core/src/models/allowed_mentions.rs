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

/// Controls which mentions Discord should parse in a message.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AllowedMentions {
    /// Mention types Discord should parse automatically.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parse: Vec<String>,
    /// Explicitly allowed role mentions.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub roles: Vec<Snowflake>,
    /// Explicitly allowed user mentions.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub users: Vec<Snowflake>,
    /// Whether the author of the referenced message should be mentioned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replied_user: Option<bool>,
}

/// Builder for [`AllowedMentions`].
#[derive(Debug, Clone, Default)]
pub struct AllowedMentionsBuilder {
    inner: AllowedMentions,
}

impl AllowedMentionsBuilder {
    /// Start building a new [`AllowedMentions`] value.
    pub fn new() -> Self {
        Self::default()
    }

    /// Allow `@everyone` / `@here` mentions.
    pub fn everyone(mut self) -> Self {
        self.inner.parse.push("everyone".to_owned());
        self
    }

    /// Allow user mentions to be parsed automatically.
    pub fn users(mut self) -> Self {
        self.inner.parse.push("users".to_owned());
        self
    }

    /// Allow role mentions to be parsed automatically.
    pub fn roles(mut self) -> Self {
        self.inner.parse.push("roles".to_owned());
        self
    }

    /// Restrict allowed user mentions to specific user IDs.
    pub fn allow_user(mut self, user_id: Snowflake) -> Self {
        self.inner.users.push(user_id);
        self
    }

    /// Restrict allowed role mentions to specific role IDs.
    pub fn allow_role(mut self, role_id: Snowflake) -> Self {
        self.inner.roles.push(role_id);
        self
    }

    /// Control whether replies mention the author of the referenced message.
    pub fn replied_user(mut self, replied_user: bool) -> Self {
        self.inner.replied_user = Some(replied_user);
        self
    }

    /// Finish the builder.
    pub fn build(self) -> AllowedMentions {
        self.inner
    }
}
