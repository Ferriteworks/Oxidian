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

//! Autocomplete interaction support.
//!
//! When a user is typing a slash-command option marked as `autocomplete = true`,
//! Discord sends an `ApplicationCommandAutocomplete` interaction.  Respond with
//! an [`AutocompleteResponse`] containing up to 25 choices.

use oxidian_core::models::interaction::AutocompleteChoice;

/// The response body for an autocomplete interaction.
///
/// # Example
///
/// ```rust,ignore
/// use oxidian::interactions::autocomplete::AutocompleteResponse;
/// use oxidian::core::models::interaction::AutocompleteChoice;
///
/// let response = AutocompleteResponse::new([
///     AutocompleteChoice::string("Option A", "a"),
///     AutocompleteChoice::string("Option B", "b"),
/// ]);
/// ```
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AutocompleteResponse {
    /// Up to 25 choices to show the user.
    pub choices: Vec<AutocompleteChoice>,
}

impl AutocompleteResponse {
    /// Build a response from an iterable of choices.
    pub fn new(choices: impl IntoIterator<Item = AutocompleteChoice>) -> Self {
        Self {
            choices: choices.into_iter().collect(),
        }
    }

    /// Add a single choice.
    pub fn push(&mut self, choice: AutocompleteChoice) {
        self.choices.push(choice);
    }
}
