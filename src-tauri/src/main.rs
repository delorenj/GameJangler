#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::borrow::Borrow;
use app::scanner::{Platform, PlatformInstance, ScanManager, MetaScannable, PlatformSet};
use app::settings::{Loadable, SettingsManager, SettingsSchema};
use log::error;
    use simplelog::info;
use std::result;
use std::time::{SystemTime, UNIX_EPOCH};
use tauri::Wry;

#[tauri::command]
fn scan_for_platform(platform: String) -> Vec<PlatformInstance> {
    let mut result: Vec<PlatformInstance> = Vec::new();
    let platform =
        PlatformInstance::new("Test Platform".to_string(), "C:/Some/Test/Path".to_string());
    result.push(platform);
    return result;
}

#[tauri::command]
fn scan_for_platforms(root_paths: Vec<&str>, platform_set: Option<PlatformSet>) -> Vec<PlatformInstance> {
    println!("{:?}", platform_set.borrow());

    let mut result: Vec<PlatformInstance> = Vec::new();
    let scanner = ScanManager {};
    scanner.start_scan(&mut result, &root_paths, platform_set);
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
        .invoke_handler(tauri::generate_handler![load_settings, scan_for_platform, scan_for_platforms])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
