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
