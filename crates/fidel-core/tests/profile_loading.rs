//! Integration tests verifying bundled keyboard profiles load and validate correctly.

use fidel_core::profile::Profile;
use std::fs;
use std::path::Path;

#[test]
fn loads_and_validates_power_geez_profile() {
    let profile_path = Path::new("../../profiles/power_geez_phonetic_unicode.toml");
    let content = fs::read_to_string(profile_path)
        .or_else(|_| fs::read_to_string("profiles/power_geez_phonetic_unicode.toml"))
        .expect("Failed to read power_geez_phonetic_unicode.toml");

    let profile = Profile::from_toml(&content).expect("Failed to parse Power Geez profile");
    assert_eq!(profile.metadata.id, "power_geez_phonetic_unicode");
    assert_eq!(profile.metadata.language, "am");
    assert_eq!(profile.metadata.script, "Ethi");
    assert!(
        profile.rules.len() >= 500,
        "Expected at least 500 rules, found {}",
        profile.rules.len()
    );
}
