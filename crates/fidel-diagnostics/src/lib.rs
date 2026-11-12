//! Safe, privacy-preserving diagnostics for Fidel Amharic.
//!
//! STRICT PRIVACY POLICY:
//! This crate NEVER records, inspects, caches, or transmits typed text,
//! keystrokes, buffer lengths, or document contents. Diagnostics are strictly
//! limited to system architecture, engine version, and profile identifiers.

#![deny(missing_docs)]
#![deny(unsafe_code)]

use serde::{Deserialize, Serialize};

/// Safe diagnostic snapshot containing non-identifying technical metadata.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SafeDiagnosticsReport {
    /// Version of the Fidel engine crate.
    pub engine_version: String,
    /// Operating system platform (e.g., "linux", "windows", "macos").
    pub platform: String,
    /// Target CPU architecture (e.g., "x86_64", "aarch64").
    pub architecture: String,
    /// Active profile identifier, if any.
    pub active_profile_id: Option<String>,
    /// Active profile version, if any.
    pub active_profile_version: Option<String>,
}

impl SafeDiagnosticsReport {
    /// Collects a safe diagnostic report without any user or typed data.
    pub fn collect(
        active_profile_id: Option<String>,
        active_profile_version: Option<String>,
    ) -> Self {
        Self {
            engine_version: env!("CARGO_PKG_VERSION").to_string(),
            platform: std::env::consts::OS.to_string(),
            architecture: std::env::consts::ARCH.to_string(),
            active_profile_id,
            active_profile_version,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn diagnostics_does_not_contain_text_and_collects_valid_metadata() {
        let report = SafeDiagnosticsReport::collect(
            Some("power_geez_phonetic_unicode".to_string()),
            Some("1.2.6".to_string()),
        );

        assert_eq!(report.platform, std::env::consts::OS);
        assert_eq!(report.architecture, std::env::consts::ARCH);
        assert_eq!(
            report.active_profile_id,
            Some("power_geez_phonetic_unicode".to_string())
        );
    }
}
