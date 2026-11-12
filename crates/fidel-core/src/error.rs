//! Error types for fidel-core operations.

use thiserror::Error;

/// Errors that can occur within `fidel-core`.
#[derive(Debug, Error)]
pub enum CoreError {
    /// Failed to parse a profile from serialized data.
    #[error("Failed to parse profile: {0}")]
    ParseError(String),

    /// A profile validation rule was violated.
    #[error("Invalid rule for key sequence '{keys}': {reason}")]
    InvalidRule {
        /// The key sequence that caused the error.
        keys: String,
        /// Description of why the rule is invalid.
        reason: String,
    },

    /// The profile contains duplicate mapping definitions for the same sequence.
    #[error("Duplicate mapping detected for key sequence: '{0}'")]
    DuplicateRule(String),

    /// The profile does not contain any mapping rules.
    #[error("Profile '{0}' contains no mapping rules")]
    EmptyProfile(String),

    /// Profile with the requested identifier was not found in the registry.
    #[error("Profile '{0}' was not found in the registry")]
    ProfileNotFound(String),

    /// An I/O error occurred while reading or writing profile assets.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}
