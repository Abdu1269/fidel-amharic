//! Deterministic composition engine for Ethiopic Fidel scripts.

#![deny(missing_docs)]
#![deny(unsafe_code)]

pub mod actions;
pub mod composer;
pub mod keycodes;
pub mod trie;
pub mod unicode;

pub use actions::EngineAction;
pub use composer::Composer;
pub use keycodes::{KeyEvent, Modifiers, SpecialKey};
pub use trie::{Trie, TrieLookup};

#[cfg(test)]
mod tests {
    use super::*;
    use fidel_core::profile::{MappingRule, Profile, ProfileMetadata, ProfileOptions};

    fn create_test_profile() -> Profile {
        Profile {
            metadata: ProfileMetadata {
                id: "amharic_test".to_string(),
                display_name: "Amharic Test Profile".to_string(),
                language: "am".to_string(),
                script: "Ethi".to_string(),
                version: "1.0.0".to_string(),
                license: "MIT".to_string(),
                author: Some("Fidel Amharic Team".to_string()),
                description: None,
            },
            options: ProfileOptions {
                case_sensitive: true,
                terminators: vec!['\''],
                auto_commit_timeout_ms: 0,
            },
            rules: vec![
                // H family
                MappingRule {
                    keys: "h".into(),
                    output: "ህ".into(),
                    order: Some(6),
                },
                MappingRule {
                    keys: "he".into(),
                    output: "ሀ".into(),
                    order: Some(1),
                },
                MappingRule {
                    keys: "hu".into(),
                    output: "ሁ".into(),
                    order: Some(2),
                },
                MappingRule {
                    keys: "hi".into(),
                    output: "ሂ".into(),
                    order: Some(3),
                },
                MappingRule {
                    keys: "ha".into(),
                    output: "ሃ".into(),
                    order: Some(4),
                },
                MappingRule {
                    keys: "hE".into(),
                    output: "ሄ".into(),
                    order: Some(5),
                },
                MappingRule {
                    keys: "ho".into(),
                    output: "ሆ".into(),
                    order: Some(7),
                },
                // S and SH families
                MappingRule {
                    keys: "s".into(),
                    output: "ስ".into(),
                    order: Some(6),
                },
                MappingRule {
                    keys: "se".into(),
                    output: "ሰ".into(),
                    order: Some(1),
                },
                MappingRule {
                    keys: "sa".into(),
                    output: "ሳ".into(),
                    order: Some(4),
                },
                MappingRule {
                    keys: "sh".into(),
                    output: "ሽ".into(),
                    order: Some(6),
                },
                MappingRule {
                    keys: "she".into(),
                    output: "ሸ".into(),
                    order: Some(1),
                },
                MappingRule {
                    keys: "shu".into(),
                    output: "ሹ".into(),
                    order: Some(2),
                },
                MappingRule {
                    keys: "sha".into(),
                    output: "ሻ".into(),
                    order: Some(4),
                },
                // M family
                MappingRule {
                    keys: "m".into(),
                    output: "ም".into(),
                    order: Some(6),
                },
                MappingRule {
                    keys: "me".into(),
                    output: "መ".into(),
                    order: Some(1),
                },
                MappingRule {
                    keys: "ma".into(),
                    output: "ማ".into(),
                    order: Some(4),
                },
                // Punctuation
                MappingRule {
                    keys: ":".into(),
                    output: "፡".into(),
                    order: None,
                },
                MappingRule {
                    keys: "::".into(),
                    output: "።".into(),
                    order: None,
                },
            ],
        }
    }

    #[test]
    fn single_consonant_enters_composition_state() {
        let profile = create_test_profile();
        let mut composer = Composer::new(&profile);

        let action = composer.process_key(&KeyEvent::from_char('h'));
        assert_eq!(
            action,
            EngineAction::SetComposition {
                display: "ህ".to_string(),
                raw: "h".to_string(),
            }
        );
        assert!(composer.is_composing());
    }

    #[test]
    fn consonant_followed_by_vowel_commits_syllable() {
        let profile = create_test_profile();
        let mut composer = Composer::new(&profile);

        composer.process_key(&KeyEvent::from_char('h'));
        let action = composer.process_key(&KeyEvent::from_char('a'));

        assert_eq!(
            action,
            EngineAction::Commit {
                text: "ሃ".to_string()
            }
        );
        assert!(!composer.is_composing());
    }

    #[test]
    fn multi_key_consonant_disambiguation_works() {
        let profile = create_test_profile();
        let mut composer = Composer::new(&profile);

        // 's' -> matches 'ስ' with more possibilities (like 'sh')
        let a1 = composer.process_key(&KeyEvent::from_char('s'));
        assert_eq!(
            a1,
            EngineAction::SetComposition {
                display: "ስ".to_string(),
                raw: "s".to_string(),
            }
        );

        // 'h' -> matches 'ሽ' with more possibilities ('she', 'sha', etc.)
        let a2 = composer.process_key(&KeyEvent::from_char('h'));
        assert_eq!(
            a2,
            EngineAction::SetComposition {
                display: "ሽ".to_string(),
                raw: "sh".to_string(),
            }
        );

        // 'e' -> exact terminal match for 'ሸ'
        let a3 = composer.process_key(&KeyEvent::from_char('e'));
        assert_eq!(
            a3,
            EngineAction::Commit {
                text: "ሸ".to_string()
            }
        );
        assert!(!composer.is_composing());
    }

    #[test]
    fn sequential_consonants_trigger_commit_and_compose() {
        let profile = create_test_profile();
        let mut composer = Composer::new(&profile);

        // Type 'h'
        composer.process_key(&KeyEvent::from_char('h'));

        // Type 'm' (consonant transition: 'h' + 'm' is NoMatch, so commits 'ህ' and starts composing 'm')
        let action = composer.process_key(&KeyEvent::from_char('m'));
        assert_eq!(
            action,
            EngineAction::CommitAndCompose {
                committed: "ህ".to_string(),
                display: "ም".to_string(),
                raw: "m".to_string(),
            }
        );
        assert!(composer.is_composing());
    }

    #[test]
    fn backspace_pops_composition_buffer() {
        let profile = create_test_profile();
        let mut composer = Composer::new(&profile);

        composer.process_key(&KeyEvent::from_char('s'));
        composer.process_key(&KeyEvent::from_char('h'));
        assert_eq!(composer.raw_buffer(), "sh");

        // First backspace: 'sh' -> 's'
        let a1 = composer.process_key(&KeyEvent::from_special(SpecialKey::Backspace));
        assert_eq!(
            a1,
            EngineAction::SetComposition {
                display: "ስ".to_string(),
                raw: "s".to_string(),
            }
        );

        // Second backspace: 's' -> empty -> ClearComposition
        let a2 = composer.process_key(&KeyEvent::from_special(SpecialKey::Backspace));
        assert_eq!(a2, EngineAction::ClearComposition);
        assert!(!composer.is_composing());
    }

    #[test]
    fn escape_clears_active_composition() {
        let profile = create_test_profile();
        let mut composer = Composer::new(&profile);

        composer.process_key(&KeyEvent::from_char('h'));
        assert!(composer.is_composing());

        let action = composer.process_key(&KeyEvent::from_special(SpecialKey::Escape));
        assert_eq!(action, EngineAction::ClearComposition);
        assert!(!composer.is_composing());
    }

    #[test]
    fn explicit_terminator_commits_pending_composition() {
        let profile = create_test_profile();
        let mut composer = Composer::new(&profile);

        composer.process_key(&KeyEvent::from_char('s'));
        let action = composer.process_key(&KeyEvent::from_char('\''));

        assert_eq!(
            action,
            EngineAction::Commit {
                text: "ስ".to_string()
            }
        );
        assert!(!composer.is_composing());
    }

    #[test]
    fn punctuation_sequence_composition_works() {
        let profile = create_test_profile();
        let mut composer = Composer::new(&profile);

        // First ':'
        let a1 = composer.process_key(&KeyEvent::from_char(':'));
        assert_eq!(
            a1,
            EngineAction::SetComposition {
                display: "፡".to_string(),
                raw: ":".to_string(),
            }
        );

        // Second ':' -> '::' = '።' (Ethiopic full stop)
        let a2 = composer.process_key(&KeyEvent::from_char(':'));
        assert_eq!(
            a2,
            EngineAction::Commit {
                text: "።".to_string()
            }
        );
    }
}
