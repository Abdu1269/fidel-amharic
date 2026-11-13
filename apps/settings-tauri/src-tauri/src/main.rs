//! Thin executable passthrough for Fidel Amharic Settings.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    settings_tauri_lib::run();
}
