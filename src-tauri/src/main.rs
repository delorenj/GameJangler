#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use app::scanner::{MetaScannable, Platform, PlatformInstance, PlatformSet, ScanManager};
use app::settings::{Loadable, SettingsManager, SettingsSchema};
use std::path::PathBuf;
use std::process::Command;
use tauri::Manager;
use tauri::Wry;
use tokio::sync::mpsc;
use tokio::sync::Mutex;
use tracing::info;
use tracing_subscriber;

struct AsyncProcInputTx {
    inner: Mutex<mpsc::Sender<String>>,
}

#[tauri::command]
fn scan_for_platform(platform: Platform) -> Vec<PlatformInstance> {
    let mut result: Vec<PlatformInstance> = Vec::new();
    let platform = PlatformInstance::new(platform, PathBuf::from("C:/Some/Test/Path"));
    result.push(platform);
    return result;
}

#[tauri::command]
async fn scan_for_platforms(
    app_handle: tauri::AppHandle<Wry>,
    root_paths: Vec<&str>,
    platform_set: Option<PlatformSet>,
) -> Result<Vec<PlatformInstance>, String> {
    let mut result: Vec<PlatformInstance> = Vec::new();
    let app_dir = app_handle.path_resolver().app_dir();
    let manager = SettingsManager::new(app_dir);
    let scanner = ScanManager {};
    scanner.start_scan(&app_handle, &mut result, &root_paths, platform_set);
    println!("{:?}", result);
    manager.write_platforms(&result);
    return Ok(result);
}

#[tauri::command]
async fn scan_for_drives(app_handle: tauri::AppHandle<Wry>) -> Result<Vec<PathBuf>, String> {
    let mut result: Vec<PathBuf> = Vec::new();
    result.push(PathBuf::from("C:/"));
    println!("{:?}", result);
    return Ok(result);
}

#[tauri::command]
fn load_settings(app_handle: tauri::AppHandle<Wry>) -> SettingsSchema {
    let app_dir = app_handle.path_resolver().app_dir();
    let manager = SettingsManager::new(app_dir);
    manager.load()
}

#[tauri::command]
fn show_in_folder(path: String) {
    #[cfg(target_os = "windows")]
    {
        Command::new("explorer")
            .args(["/select,", &path]) // The comma after select is not a typo
            .spawn()
            .unwrap();
    }

    #[cfg(target_os = "linux")]
    {
        if path.contains(",") {
            // see https://gitlab.freedesktop.org/dbus/dbus/-/issues/76
            let new_path = match metadata(&path).unwrap().is_dir() {
                true => path,
                false => {
                    let mut path2 = PathBuf::from(path);
                    path2.pop();
                    path2.into_os_string().into_string().unwrap()
                }
            };
            Command::new("xdg-open").arg(&new_path).spawn().unwrap();
        } else {
            Command::new("dbus-send")
                .args([
                    "--session",
                    "--dest=org.freedesktop.FileManager1",
                    "--type=method_call",
                    "/org/freedesktop/FileManager1",
                    "org.freedesktop.FileManager1.ShowItems",
                    format!("array:string:\"file://{path}\"").as_str(),
                    "string:\"\"",
                ])
                .spawn()
                .unwrap();
        }
    }

    #[cfg(target_os = "macos")]
    {
        Command::new("open").args(["-R", &path]).spawn().unwrap();
    }
}

fn main() {
    tracing_subscriber::fmt::init();

    let (async_proc_input_tx, async_proc_input_rx) = mpsc::channel(1);
    let (async_proc_output_tx, mut async_proc_output_rx) = mpsc::channel(1);

    tauri::Builder::default()
        .manage(AsyncProcInputTx {
            inner: Mutex::new(async_proc_input_tx),
        })
        .invoke_handler(tauri::generate_handler![
            load_settings,
            scan_for_platform,
            scan_for_platforms,
            show_in_folder
        ])
        .setup(|app| {
            tauri::async_runtime::spawn(async move {
                async_process_model(async_proc_input_rx, async_proc_output_tx).await
            });

            let app_handle = app.handle();
            tauri::async_runtime::spawn(async move {
                loop {
                    if let Some(output) = async_proc_output_rx.recv().await {
                        rs2js(output, &app_handle);
                    }
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

pub fn rs2js<R: tauri::Runtime>(message: String, manager: &impl Manager<R>) {
    info!(?message, "rs2js");
    manager
        .emit_all("rs2js", format!("rs: {}", message))
        .unwrap();
}

#[tauri::command]
async fn js2rs(message: String, state: tauri::State<'_, AsyncProcInputTx>) -> Result<(), String> {
    info!(?message, "js2rs");
    let async_proc_input_tx = state.inner.lock().await;
    async_proc_input_tx
        .send(message)
        .await
        .map_err(|e| e.to_string())
}

async fn async_process_model(
    mut input_rx: mpsc::Receiver<String>,
    output_tx: mpsc::Sender<String>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    while let Some(input) = input_rx.recv().await {
        let output = input;
        output_tx.send(output).await?;
    }

    Ok(())
}
