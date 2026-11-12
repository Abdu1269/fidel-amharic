//! Comprehensive golden tests verifying typing with the Power Geez Unicode profile.

use fidel_core::profile::Profile;
use fidel_engine::{Composer, EngineAction, KeyEvent, SpecialKey};
use std::fs;
use std::path::Path;

fn load_power_geez_composer() -> Composer {
    let path = Path::new("../../profiles/power_geez_phonetic_unicode.toml");
    let content = fs::read_to_string(path)
        .or_else(|_| fs::read_to_string("profiles/power_geez_phonetic_unicode.toml"))
        .expect("Failed to read power_geez_phonetic_unicode.toml");

    let profile = Profile::from_toml(&content).expect("Failed to parse profile");
    Composer::new(&profile)
}

/// Helper function to simulate a string of key presses and collect the committed text.
fn type_sequence(composer: &mut Composer, sequence: &str) -> String {
    let mut output = String::new();

    for ch in sequence.chars() {
        let action = composer.process_key(&KeyEvent::from_char(ch));
        match action {
            EngineAction::Commit { text } => {
                output.push_str(&text);
            }
            EngineAction::CommitAndCompose { committed, .. } => {
                output.push_str(&committed);
            }
            EngineAction::SetComposition { .. } => {}
            EngineAction::ClearComposition => {}
            EngineAction::PassThrough => {
                output.push(ch);
            }
            EngineAction::Consumed => {}
        }
    }

    // If composition is still pending at the end, commit with Enter or Space
    if composer.is_composing() {
        if let EngineAction::Commit { text } =
            composer.process_key(&KeyEvent::from_special(SpecialKey::Enter))
        {
            output.push_str(&text);
        }
    }

    output
}

#[test]
fn test_halehame_family_vowel_orders() {
    let mut composer = load_power_geez_composer();

    // 1st order: 'h' + Enter -> 'ሀ' (U+1200)
    assert_eq!(type_sequence(&mut composer, "h"), "ሀ");
    // 2nd order: 'hu' -> 'ሁ' (U+1201)
    assert_eq!(type_sequence(&mut composer, "hu"), "ሁ");
    // 3rd order: 'hi' -> 'ሂ' (U+1202)
    assert_eq!(type_sequence(&mut composer, "hi"), "ሂ");
    // 4th order: 'ha' -> 'ሃ' (U+1203)
    assert_eq!(type_sequence(&mut composer, "ha"), "ሃ");
    // 5th order: 'hy' -> 'ሄ' (U+1204)
    assert_eq!(type_sequence(&mut composer, "hy"), "ሄ");
    // 6th order: 'he' -> 'ህ' (U+1205)
    assert_eq!(type_sequence(&mut composer, "he"), "ህ");
    // 7th order: 'ho' -> 'ሆ' (U+1206)
    assert_eq!(type_sequence(&mut composer, "ho"), "ሆ");
}

#[test]
fn test_lewe_family_vowel_orders() {
    let mut composer = load_power_geez_composer();

    assert_eq!(type_sequence(&mut composer, "l"), "ለ");
    assert_eq!(type_sequence(&mut composer, "lu"), "ሉ");
    assert_eq!(type_sequence(&mut composer, "li"), "ሊ");
    assert_eq!(type_sequence(&mut composer, "la"), "ላ");
    assert_eq!(type_sequence(&mut composer, "ly"), "ሌ");
    assert_eq!(type_sequence(&mut composer, "le"), "ል");
    assert_eq!(type_sequence(&mut composer, "lo"), "ሎ");
}

#[test]
fn test_ma_family_vowel_orders() {
    let mut composer = load_power_geez_composer();

    assert_eq!(type_sequence(&mut composer, "m"), "መ");
    assert_eq!(type_sequence(&mut composer, "mu"), "ሙ");
    assert_eq!(type_sequence(&mut composer, "mi"), "ሚ");
    assert_eq!(type_sequence(&mut composer, "ma"), "ማ");
    assert_eq!(type_sequence(&mut composer, "my"), "ሜ");
    assert_eq!(type_sequence(&mut composer, "me"), "ም");
    assert_eq!(type_sequence(&mut composer, "mo"), "ሞ");
}

#[test]
fn test_se_and_she_families() {
    let mut composer = load_power_geez_composer();

    // 's' family
    assert_eq!(type_sequence(&mut composer, "s"), "ሰ");
    assert_eq!(type_sequence(&mut composer, "su"), "ሱ");
    assert_eq!(type_sequence(&mut composer, "si"), "ሲ");
    assert_eq!(type_sequence(&mut composer, "sa"), "ሳ");
    assert_eq!(type_sequence(&mut composer, "sy"), "ሴ");
    assert_eq!(type_sequence(&mut composer, "se"), "ስ");
    assert_eq!(type_sequence(&mut composer, "so"), "ሶ");

    // 'sh' family
    assert_eq!(type_sequence(&mut composer, "sh"), "ሸ");
    assert_eq!(type_sequence(&mut composer, "shu"), "ሹ");
    assert_eq!(type_sequence(&mut composer, "shi"), "ሺ");
    assert_eq!(type_sequence(&mut composer, "sha"), "ሻ");
    assert_eq!(type_sequence(&mut composer, "shy"), "ሼ");
    assert_eq!(type_sequence(&mut composer, "she"), "ሽ");
    assert_eq!(type_sequence(&mut composer, "sho"), "ሾ");
}

#[test]
fn test_be_and_te_and_che_families() {
    let mut composer = load_power_geez_composer();

    // 'b' family
    assert_eq!(type_sequence(&mut composer, "ba"), "ባ");
    assert_eq!(type_sequence(&mut composer, "be"), "ብ");

    // 't' family
    assert_eq!(type_sequence(&mut composer, "ta"), "ታ");
    assert_eq!(type_sequence(&mut composer, "te"), "ት");

    // 'ch' family
    assert_eq!(type_sequence(&mut composer, "cha"), "ቻ");
    assert_eq!(type_sequence(&mut composer, "che"), "ች");
}

#[test]
fn test_qe_and_ke_and_ge_families() {
    let mut composer = load_power_geez_composer();

    assert_eq!(type_sequence(&mut composer, "qa"), "ቃ");
    assert_eq!(type_sequence(&mut composer, "qe"), "ቅ");

    assert_eq!(type_sequence(&mut composer, "ka"), "ካ");
    assert_eq!(type_sequence(&mut composer, "ke"), "ክ");

    assert_eq!(type_sequence(&mut composer, "ga"), "ጋ");
    assert_eq!(type_sequence(&mut composer, "ge"), "ግ");
}

#[test]
fn test_punctuation_commit() {
    let mut composer = load_power_geez_composer();

    assert_eq!(type_sequence(&mut composer, ":"), "፡");
    assert_eq!(type_sequence(&mut composer, "::"), "፨");
    assert_eq!(type_sequence(&mut composer, "."), "።");
    assert_eq!(type_sequence(&mut composer, ";"), "፤");
    assert_eq!(type_sequence(&mut composer, ":-"), "፦");
}

#[test]
fn test_ethiopic_numerals() {
    let mut composer = load_power_geez_composer();

    assert_eq!(type_sequence(&mut composer, "1*"), "፩");
    assert_eq!(type_sequence(&mut composer, "2*"), "፪");
    assert_eq!(type_sequence(&mut composer, "3*"), "፫");
    assert_eq!(type_sequence(&mut composer, "10*"), "፲");
    assert_eq!(type_sequence(&mut composer, "100*"), "፻");
}

#[test]
fn test_full_amharic_word_typing() {
    let mut composer = load_power_geez_composer();

    // "ሰላም" (selam) in Power Geez:
    // 's' -> ሰ, 'l' + 'a' -> ላ, 'm' + 'e' -> ም
    let word = type_sequence(&mut composer, "slame");
    assert_eq!(word, "ሰላም");

    // "አማርኛ" (amarinya):
    // 'a' -> ኣ (or 'x' -> አ), 'm' + 'a' -> ማ, 'r' + 'e' -> ር, 'N' + 'a' -> ኛ
    let word2 = type_sequence(&mut composer, "xmareNa");
    assert_eq!(word2, "አማርኛ");
}
