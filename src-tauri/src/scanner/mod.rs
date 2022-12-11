pub mod epic;
pub mod steam;

use strum::{EnumIter, IntoEnumIterator};

#[cfg(test)]
mod tests;
use crate::settings::SettingsManager;
use nanoid::nanoid;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tauri::Wry;

use self::{epic::Epic, steam::Steam};

#[derive(Serialize, Deserialize, Debug)]
pub struct GameInstance {
    id: String,
    title: String,
    location: String,
}

impl GameInstance {
    pub fn new(title: String, location: String) -> Self {
        Self {
            id: nanoid!(),
            title,
            location,
        }
    }
}

#[derive(Debug, EnumIter, Serialize, Deserialize)]
pub enum Platform {
    STEAM,
    EPIC,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlatformSet {
    platforms: Vec<Platform>,
}

impl Default for PlatformSet {
    fn default() -> PlatformSet {
        PlatformSet {
            platforms: Platform::iter().collect::<Vec<Platform>>(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlatformInstance {
    id: String, //UUID
    pub platform: Platform,
    pub location: PathBuf,
}

impl PlatformInstance {
    pub fn new(platform: Platform, location: PathBuf) -> Self {
        Self {
            id: nanoid!(),
            platform,
            location,
        }
    }
}

pub trait Scannable<ScanType> {
    fn start_scan(
        &self,
        app_handle: &tauri::AppHandle<Wry>,
        result: &mut Vec<ScanType>,
        root_paths: &Vec<&str>,
    );
}

pub trait MetaScannable<ScanType> {
    fn start_scan(
        &self,
        app_handle: &tauri::AppHandle<Wry>,
        result: &mut Vec<ScanType>,
        root_paths: &Vec<&str>,
        platform_set: Option<PlatformSet>,
    );
}

pub struct ScanManager {}

impl MetaScannable<PlatformInstance> for ScanManager {
    fn start_scan(
        &self,
        app_handle: &tauri::AppHandle<Wry>,
        result: &mut Vec<PlatformInstance>,
        root_paths: &Vec<&str>,
        platform_set: Option<PlatformSet>,
    ) {
        let platforms = platform_set.unwrap_or_default();
        for platform in platforms.platforms.iter() {
            match platform {
                Platform::STEAM => Steam.start_scan(app_handle, result, root_paths),
                Platform::EPIC => Epic.start_scan(app_handle, result, root_paths),
            }
        }
        SettingsManager::write_platforms(&result)
    }
}
