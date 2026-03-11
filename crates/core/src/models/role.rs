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

/// A Discord guild role.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Role {
    /// The role's unique snowflake ID.
    pub id: Snowflake,
    /// The role's display name.
    pub name: String,
    /// The role's display colour as a packed RGB integer (`0xRRGGBB`).
    #[serde(default)]
    pub color: u32,
    /// Whether members with this role are shown separately in the member list.
    #[serde(default)]
    pub hoist: bool,
    /// The role's position in the hierarchy (higher = more powerful).
    #[serde(default)]
    pub position: i32,
    /// The permissions bit-string for this role.
    pub permissions: String,
    /// Whether this role is managed by an integration and cannot be manually assigned.
    #[serde(default)]
    pub managed: bool,
    /// Whether anyone can mention this role with `@role`.
    #[serde(default)]
    pub mentionable: bool,
}
