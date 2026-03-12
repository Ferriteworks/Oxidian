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

//! Discord mention string formatters.
//!
//! These produce the raw Discord markdown strings that Discord renders as
//! formatted mentions in message content.

/// Mention a user: `<@user_id>`
pub fn user(id: u64) -> String {
    format!("<@{id}>")
}

/// Mention a role: `<@&role_id>`
pub fn role(id: u64) -> String {
    format!("<@&{id}>")
}

/// Mention a channel: `<#channel_id>`
pub fn channel(id: u64) -> String {
    format!("<#{id}>")
}

/// Mention a top-level slash command: `</name:command_id>`
pub fn slash_command(name: &str, id: u64) -> String {
    format!("</{name}:{id}>")
}

/// Mention a slash command subcommand: `</command subcommand:command_id>`
pub fn slash_subcommand(command: &str, subcommand: &str, id: u64) -> String {
    format!("</{command} {subcommand}:{id}>")
}

/// Mention a slash command subcommand group:
/// `</command group subcommand:command_id>`
pub fn slash_subcommand_group(
    command: &str,
    group: &str,
    subcommand: &str,
    id: u64,
) -> String {
    format!("</{command} {group} {subcommand}:{id}>")
}

/// A static custom emoji: `<:name:emoji_id>`
pub fn custom_emoji(name: &str, id: u64) -> String {
    format!("<:{name}:{id}>")
}

/// An animated custom emoji: `<a:name:emoji_id>`
pub fn animated_emoji(name: &str, id: u64) -> String {
    format!("<a:{name}:{id}>")
}
