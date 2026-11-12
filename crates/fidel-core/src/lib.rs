//! Core data structures, schemas, and registry for Fidel Amharic.

#![deny(missing_docs)]
#![deny(unsafe_code)]

pub mod error;
pub mod profile;
pub mod registry;

pub use error::CoreError;
pub use profile::{MappingRule, Profile, ProfileMetadata, ProfileOptions};
pub use registry::ProfileRegistry;

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_profile() -> Profile {
        Profile {
            metadata: ProfileMetadata {
                id: "test_amharic".to_string(),
                display_name: "Test Amharic".to_string(),
                language: "am".to_string(),
                script: "Ethi".to_string(),
                version: "1.0.0".to_string(),
                license: "MIT".to_string(),
                author: Some("Fidel Team".to_string()),
                description: Some("Test layout".to_string()),
            },
            options: ProfileOptions::default(),
            rules: vec![
                MappingRule {
                    keys: "he".to_string(),
                    output: "ሀ".to_string(),
                    order: Some(1),
                },
                MappingRule {
                    keys: "hu".to_string(),
                    output: "ሁ".to_string(),
                    order: Some(2),
                },
            ],
        }
    }

    #[test]
    fn profile_validation_succeeds_for_valid_profile() {
        let profile = sample_profile();
        assert!(profile.validate().is_ok());
    }

    #[test]
    fn profile_validation_fails_on_duplicate_keys() {
        let mut profile = sample_profile();
        profile.rules.push(MappingRule {
            keys: "he".to_string(),
            output: "ህ".to_string(),
            order: Some(6),
        });
        assert!(matches!(
            profile.validate(),
            Err(CoreError::DuplicateRule(_))
        ));
    }

    #[test]
    fn profile_roundtrips_through_toml() {
        let profile = sample_profile();
        let toml_str = profile.to_toml().expect("failed to serialize to TOML");
        let parsed = Profile::from_toml(&toml_str).expect("failed to deserialize from TOML");
        assert_eq!(profile, parsed);
    }

    #[test]
    fn registry_manages_active_profile() {
        let registry = ProfileRegistry::new();
        let profile = sample_profile();
        registry
            .register(profile.clone())
            .expect("failed to register");

        let active = registry.get_active().expect("active profile should exist");
        assert_eq!(active.metadata.id, "test_amharic");
    }
}
