//! Profile definitions, validation, and serialization.

use serde::{Deserialize, Serialize};
use std::collections::HashSet;

use crate::error::CoreError;

/// Metadata describing an input profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProfileMetadata {
    /// Unique identifier for this profile (e.g., "power_geez_phonetic_unicode").
    pub id: String,
    /// Human-friendly display name (e.g., "Power Geez Phonetic-compatible (Unicode)").
    pub display_name: String,
    /// BCP-47 language tag (e.g., "am" for Amharic, "ti" for Tigrinya).
    pub language: String,
    /// ISO 15924 script code (e.g., "Ethi").
    pub script: String,
    /// Semantic version of this profile specification.
    pub version: String,
    /// Software license for this profile data (e.g., "MIT").
    pub license: String,
    /// Attribution and author information.
    pub author: Option<String>,
    /// Long-form description or usage guide.
    pub description: Option<String>,
}

/// Execution options that customize typing and composition behavior.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProfileOptions {
    /// Whether key matching is sensitive to uppercase/lowercase distinctions.
    #[serde(default = "default_case_sensitive")]
    pub case_sensitive: bool,

    /// Explicit character sequences that force immediate commitment of pending composition.
    #[serde(default)]
    pub terminators: Vec<char>,

    /// Timeout in milliseconds after which pending composition commits automatically (0 disables).
    #[serde(default)]
    pub auto_commit_timeout_ms: u64,
}

fn default_case_sensitive() -> bool {
    true
}

impl Default for ProfileOptions {
    fn default() -> Self {
        Self {
            case_sensitive: true,
            terminators: vec!['\''],
            auto_commit_timeout_ms: 0,
        }
    }
}

/// An individual phonetic mapping rule from a key sequence to Unicode output.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MappingRule {
    /// Latin key sequence entered by the user (e.g., "ha", "she").
    pub keys: String,
    /// Corresponding Unicode output string (e.g., "ሃ", "ሸ").
    pub output: String,
    /// Optional Ethiopic vowel order (1 to 7) for classification and visual keyboard aid.
    pub order: Option<u8>,
}

/// A complete keyboard layout profile containing metadata, options, and mapping rules.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Profile {
    /// Metadata header.
    pub metadata: ProfileMetadata,
    /// Operational preferences.
    #[serde(default)]
    pub options: ProfileOptions,
    /// List of phonetic mapping rules.
    pub rules: Vec<MappingRule>,
}

impl Profile {
    /// Validates the profile for internal consistency:
    /// - Non-empty rules
    /// - No duplicate key sequences
    /// - Non-empty keys and outputs
    /// - Valid Unicode code points
    pub fn validate(&self) -> Result<(), CoreError> {
        if self.rules.is_empty() {
            return Err(CoreError::EmptyProfile(self.metadata.id.clone()));
        }

        let mut seen = HashSet::with_capacity(self.rules.len());
        for rule in &self.rules {
            if rule.keys.is_empty() {
                return Err(CoreError::InvalidRule {
                    keys: rule.keys.clone(),
                    reason: "Key sequence must not be empty".to_string(),
                });
            }
            if rule.output.is_empty() {
                return Err(CoreError::InvalidRule {
                    keys: rule.keys.clone(),
                    reason: "Output string must not be empty".to_string(),
                });
            }
            if !seen.insert(&rule.keys) {
                return Err(CoreError::DuplicateRule(rule.keys.clone()));
            }
        }

        Ok(())
    }

    /// Deserializes a `Profile` from a TOML string and validates it.
    pub fn from_toml(source: &str) -> Result<Self, CoreError> {
        let profile: Self =
            toml::from_str(source).map_err(|e| CoreError::ParseError(e.to_string()))?;
        profile.validate()?;
        Ok(profile)
    }

    /// Serializes the profile to a formatted TOML string.
    pub fn to_toml(&self) -> Result<String, CoreError> {
        toml::to_string_pretty(self).map_err(|e| CoreError::ParseError(e.to_string()))
    }

    /// Deserializes a `Profile` from a JSON string and validates it.
    pub fn from_json(source: &str) -> Result<Self, CoreError> {
        let profile: Self =
            serde_json::from_str(source).map_err(|e| CoreError::ParseError(e.to_string()))?;
        profile.validate()?;
        Ok(profile)
    }

    /// Serializes the profile to a formatted JSON string.
    pub fn to_json(&self) -> Result<String, CoreError> {
        serde_json::to_string_pretty(self).map_err(|e| CoreError::ParseError(e.to_string()))
    }
}
