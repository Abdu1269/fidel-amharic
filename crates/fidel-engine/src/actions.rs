//! Engine output actions resulting from key processing.

use serde::{Deserialize, Serialize};

/// The action to be taken by the native platform adapter in response to a key event.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EngineAction {
    /// Commit final text directly to the active application context.
    /// Clears any active composition.
    Commit {
        /// The completed Unicode text to insert.
        text: String,
    },

    /// Update the current inline composition / preedit text.
    SetComposition {
        /// The text to display visually (e.g., underlined or with cursor).
        display: String,
        /// The raw ASCII buffer being composed.
        raw: String,
    },

    /// Commit the currently pending composition, and immediately start a new
    /// composition with the remainder.
    CommitAndCompose {
        /// The text to commit to the document.
        committed: String,
        /// The new composition string to display.
        display: String,
        /// The raw ASCII buffer of the new composition.
        raw: String,
    },

    /// Clear the active composition without committing any text.
    ClearComposition,

    /// The engine does not handle this key event; let the native OS / application handle it.
    PassThrough,

    /// The engine handled the key event, but no text change or composition update is required.
    Consumed,
}
