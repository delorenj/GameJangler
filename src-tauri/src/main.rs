#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use app::scraper::PlatformInstance;
use app::settings::{Loadable, SettingsManager, SettingsSchema};
use log::error;
use simplelog::info;
use std::time::{SystemTime, UNIX_EPOCH};
use tauri::Wry;

#[tauri::command]
fn on_button_clicked() -> String {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis();
    format!(
        "on_button_clicked called from Rust! (timestamp: {}ms)",
        since_the_epoch
    )
}

#[tauri::command]
fn scan_for_platform(platform_name: String) -> Vec<PlatformInstance> {
    let mut result: Vec<PlatformInstance> = Vec::new();
    let platform =
        PlatformInstance::new("Test Platform".to_string(), "C:/Some/Test/Path".to_string());
    result.push(platform);
    return result;
}

#[tauri::command]
fn load_settings(app_handle: tauri::AppHandle<Wry>) -> SettingsSchema {
    let app_dir = app_handle.path_resolver().app_dir();
    let manager = SettingsManager::new(app_dir);
    manager.load()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![on_button_clicked, load_settings])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
