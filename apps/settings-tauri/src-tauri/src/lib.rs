//! Application library and Tauri command handlers for Fidel Amharic Settings.

use serde::{Deserialize, Serialize};
use std::sync::Mutex;

use fidel_core::profile::{Profile, ProfileMetadata};
use fidel_diagnostics::SafeDiagnosticsReport;
use fidel_engine::{Composer, EngineAction, KeyEvent, SpecialKey};

const EMBEDDED_PROFILE: &str =
    include_str!("../../../../profiles/power_geez_phonetic_unicode.toml");

/// A row in the Ethiopic Fidel matrix representing a consonant family across 7 vowel orders.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FidelRow {
    pub key: String,
    pub name: String,
    pub orders: [String; 7],
}

struct AppState {
    profile: Profile,
    test_composer: Mutex<Composer>,
}

#[tauri::command]
fn get_diagnostics(state: tauri::State<'_, AppState>) -> Result<SafeDiagnosticsReport, String> {
    Ok(SafeDiagnosticsReport::collect(
        Some(state.profile.metadata.id.clone()),
        Some(state.profile.metadata.version.clone()),
    ))
}

#[tauri::command]
fn get_active_profile(state: tauri::State<'_, AppState>) -> Result<ProfileMetadata, String> {
    Ok(state.profile.metadata.clone())
}

#[tauri::command]
fn list_available_profiles(state: tauri::State<'_, AppState>) -> Result<Vec<ProfileMetadata>, String> {
    Ok(vec![
        state.profile.metadata.clone(),
        ProfileMetadata {
            id: "fidel_modern_phonetic".to_string(),
            display_name: "Fidel Modern Phonetic".to_string(),
            language: "am".to_string(),
            script: "Ethi".to_string(),
            version: "0.1.0".to_string(),
            license: "MIT".to_string(),
            author: Some("Fidel Amharic Project".to_string()),
            description: Some("Intuitive modern phonetic layout for Amharic".to_string()),
        },
    ])
}

#[tauri::command]
fn simulate_typing(input: String, state: tauri::State<'_, AppState>) -> Result<String, String> {
    let mut composer = state.test_composer.lock().map_err(|e| e.to_string())?;
    composer.reset();

    let mut output = String::new();
    for ch in input.chars() {
        let action = composer.process_key(&KeyEvent::from_char(ch));
        match action {
            EngineAction::Commit { text } => output.push_str(&text),
            EngineAction::CommitAndCompose { committed, .. } => output.push_str(&committed),
            EngineAction::PassThrough => output.push(ch),
            _ => {}
        }
    }

    if composer.is_composing() {
        if let EngineAction::Commit { text } =
            composer.process_key(&KeyEvent::from_special(SpecialKey::Enter))
        {
            output.push_str(&text);
        }
    }

    Ok(output)
}

#[tauri::command]
fn get_fidel_matrix() -> Result<Vec<FidelRow>, String> {
    let matrix = vec![
        FidelRow { key: "h".into(), name: "Hä (ሀ)".into(), orders: ["ሀ".into(), "ሁ".into(), "ሂ".into(), "ሃ".into(), "ሄ".into(), "ህ".into(), "ሆ".into()] },
        FidelRow { key: "l".into(), name: "Lä (ለ)".into(), orders: ["ለ".into(), "ሉ".into(), "ሊ".into(), "ላ".into(), "ሌ".into(), "ል".into(), "ሎ".into()] },
        FidelRow { key: "H".into(), name: "Ḥä (ሐ)".into(), orders: ["ሐ".into(), "ሑ".into(), "ሒ".into(), "ሓ".into(), "ሔ".into(), "ሕ".into(), "ሖ".into()] },
        FidelRow { key: "m".into(), name: "Mä (መ)".into(), orders: ["መ".into(), "ሙ".into(), "ሚ".into(), "ማ".into(), "ሜ".into(), "ም".into(), "ሞ".into()] },
        FidelRow { key: "s".into(), name: "Sä (ሰ)".into(), orders: ["ሰ".into(), "ሱ".into(), "ሲ".into(), "ሳ".into(), "ሴ".into(), "ስ".into(), "ሶ".into()] },
        FidelRow { key: "sh".into(), name: "Šä (ሸ)".into(), orders: ["ሸ".into(), "ሹ".into(), "ሺ".into(), "ሻ".into(), "ሼ".into(), "ሽ".into(), "ሾ".into()] },
        FidelRow { key: "r".into(), name: "Rä (ረ)".into(), orders: ["ረ".into(), "ሩ".into(), "ሪ".into(), "ራ".into(), "ሬ".into(), "ር".into(), "ሮ".into()] },
        FidelRow { key: "q".into(), name: "Qä (ቀ)".into(), orders: ["ቀ".into(), "ቁ".into(), "ቂ".into(), "ቃ".into(), "ቄ".into(), "ቅ".into(), "ቆ".into()] },
        FidelRow { key: "b".into(), name: "Bä (በ)".into(), orders: ["በ".into(), "ቡ".into(), "ቢ".into(), "ባ".into(), "ቤ".into(), "ብ".into(), "ቦ".into()] },
        FidelRow { key: "v".into(), name: "Vä (ቨ)".into(), orders: ["ቨ".into(), "ቩ".into(), "ቪ".into(), "ቫ".into(), "ቬ".into(), "ቭ".into(), "ቮ".into()] },
        FidelRow { key: "t".into(), name: "Tä (ተ)".into(), orders: ["ተ".into(), "ቱ".into(), "ቲ".into(), "ታ".into(), "ቴ".into(), "ት".into(), "ቶ".into()] },
        FidelRow { key: "ch".into(), name: "Čä (ቸ)".into(), orders: ["ቸ".into(), "ቹ".into(), "ቺ".into(), "ቻ".into(), "ቼ".into(), "ች".into(), "ቾ".into()] },
        FidelRow { key: "n".into(), name: "Nä (ነ)".into(), orders: ["ነ".into(), "ኑ".into(), "ኒ".into(), "ና".into(), "ኔ".into(), "ን".into(), "ኖ".into()] },
        FidelRow { key: "ny".into(), name: "Ñä (ኘ)".into(), orders: ["ኘ".into(), "ኙ".into(), "ኚ".into(), "ኛ".into(), "ኜ".into(), "ኝ".into(), "ኞ".into()] },
        FidelRow { key: "k".into(), name: "Kä (ከ)".into(), orders: ["ከ".into(), "ኩ".into(), "ኪ".into(), "ካ".into(), "ኬ".into(), "ክ".into(), "ኮ".into()] },
        FidelRow { key: "kh".into(), name: "Ḫä (ኸ)".into(), orders: ["ኸ".into(), "ኹ".into(), "ኺ".into(), "ኻ".into(), "ኼ".into(), "ኽ".into(), "ኾ".into()] },
        FidelRow { key: "w".into(), name: "Wä (ወ)".into(), orders: ["ወ".into(), "ዉ".into(), "ዊ".into(), "ዋ".into(), "ዌ".into(), "ው".into(), "ዎ".into()] },
        FidelRow { key: "z".into(), name: "Zä (ዘ)".into(), orders: ["ዘ".into(), "ዙ".into(), "ዚ".into(), "ዛ".into(), "ዜ".into(), "ዝ".into(), "ዞ".into()] },
        FidelRow { key: "zh".into(), name: "Žä (ዠ)".into(), orders: ["ዠ".into(), "ዡ".into(), "ዢ".into(), "ዣ".into(), "ዤ".into(), "ዥ".into(), "ዦ".into()] },
        FidelRow { key: "y".into(), name: "Yä (የ)".into(), orders: ["የ".into(), "ዩ".into(), "ዪ".into(), "ያ".into(), "ዬ".into(), "ይ".into(), "ዮ".into()] },
        FidelRow { key: "d".into(), name: "Dä (ደ)".into(), orders: ["ደ".into(), "ዱ".into(), "ዲ".into(), "ዳ".into(), "ዴ".into(), "ድ".into(), "ዶ".into()] },
        FidelRow { key: "j".into(), name: "Jä (ጀ)".into(), orders: ["ጀ".into(), "ጁ".into(), "ጂ".into(), "ጃ".into(), "ጄ".into(), "ጅ".into(), "ጆ".into()] },
        FidelRow { key: "g".into(), name: "Gä (ገ)".into(), orders: ["ገ".into(), "ጉ".into(), "ጊ".into(), "ጋ".into(), "ጌ".into(), "ግ".into(), "ጎ".into()] },
        FidelRow { key: "T".into(), name: "Ṭä (ጠ)".into(), orders: ["ጠ".into(), "ጡ".into(), "ጢ".into(), "ጣ".into(), "ጤ".into(), "ጥ".into(), "ጦ".into()] },
        FidelRow { key: "C".into(), name: "Č̣ä (ጨ)".into(), orders: ["ጨ".into(), "ጩ".into(), "ጪ".into(), "ጫ".into(), "ጬ".into(), "ጭ".into(), "ጮ".into()] },
        FidelRow { key: "P".into(), name: "P̣ä (ጰ)".into(), orders: ["ጰ".into(), "ጱ".into(), "ጲ".into(), "ጳ".into(), "ጴ".into(), "ጵ".into(), "ጶ".into()] },
        FidelRow { key: "ts".into(), name: "Ṣä (ጸ)".into(), orders: ["ጸ".into(), "ጹ".into(), "ጺ".into(), "ጻ".into(), "ጼ".into(), "ጽ".into(), "ጾ".into()] },
        FidelRow { key: "f".into(), name: "Fä (ፈ)".into(), orders: ["ፈ".into(), "ፉ".into(), "ፊ".into(), "ፋ".into(), "ፌ".into(), "ፍ".into(), "ፎ".into()] },
        FidelRow { key: "p".into(), name: "Pä (ፐ)".into(), orders: ["ፐ".into(), "ፑ".into(), "ፒ".into(), "ፓ".into(), "ፔ".into(), "ፕ".into(), "ፖ".into()] },
    ];
    Ok(matrix)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let profile = Profile::from_toml(EMBEDDED_PROFILE)
        .expect("Embedded Power Geez profile must be valid");
    let test_composer = Mutex::new(Composer::new(&profile));

    tauri::Builder::default()
        .manage(AppState {
            profile,
            test_composer,
        })
        .invoke_handler(tauri::generate_handler![
            get_diagnostics,
            get_active_profile,
            list_available_profiles,
            simulate_typing,
            get_fidel_matrix,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
