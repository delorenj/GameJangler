#[cfg(test)]
mod tests;

use crate::scraper::PlatformInstance;
use crate::settings::LoadSettingsError::NONE;
use nanoid::nanoid;
use serde::{Deserialize, Serialize};
use std::{fs::DirEntry, path::PathBuf};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct SettingsSchema {
    pub platforms: Option<Vec<PlatformInstance>>,
}

pub enum LoadSettingsError {
    NONE,
    EMPTY,
    CORRUPT,
}

pub const SETTINGS_FILE: &str = "settings.json";

pub struct SettingsManager;

impl SettingsManager {
    pub fn load(app_dir: Option<PathBuf>) -> Result<SettingsSchema, LoadSettingsError> {
        let data = "balls";
        fs::write(app_dir.unwrap().into_os_string().into_string().unwrap(), data).expect("Unable to write file");

        Err(NONE)
    }
}
