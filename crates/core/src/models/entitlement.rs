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

/// A Discord entitlement representing a user or guild's access to a premium
/// offering (SKU) for your application.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Entitlement {
    /// The entitlement's unique ID.
    pub id: Snowflake,
    /// ID of the SKU.
    pub sku_id: Snowflake,
    /// ID of the application the entitlement belongs to.
    pub application_id: Snowflake,
    /// ID of the user that is granted access to the entitlement's SKU.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<Snowflake>,
    /// Type of entitlement (see `EntitlementType`).
    #[serde(rename = "type")]
    pub kind: u8,
    /// Whether the entitlement has been deleted.
    #[serde(default)]
    pub deleted: bool,
    /// Start date at which the entitlement is valid (ISO 8601).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starts_at: Option<String>,
    /// Date at which the entitlement is no longer valid (ISO 8601).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ends_at: Option<String>,
    /// ID of the guild that is granted access to the entitlement's SKU.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<Snowflake>,
    /// Whether the entitlement was consumed. Only applies to consumable items.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumed: Option<bool>,
}

/// Entitlement type codes.
///
/// | Value | Name                     | Description                                  |
/// |-------|--------------------------|----------------------------------------------|
/// | 8     | APPLICATION_SUBSCRIPTION | Entitlement purchased by a user              |
pub struct EntitlementType;

impl EntitlementType {
    pub const APPLICATION_SUBSCRIPTION: u8 = 8;
}

/// A Discord SKU (stock-keeping unit) representing a premium offering.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Sku {
    /// The SKU's unique ID.
    pub id: Snowflake,
    /// Type of SKU (5 = SUBSCRIPTION, 6 = SUBSCRIPTION_GROUP).
    #[serde(rename = "type")]
    pub kind: u8,
    /// ID of the parent application.
    pub application_id: Snowflake,
    /// Customer-facing name of the premium offering.
    pub name: String,
    /// System-generated URL slug.
    pub slug: String,
    /// SKU flags as a bitfield.
    #[serde(default)]
    pub flags: u64,
}
