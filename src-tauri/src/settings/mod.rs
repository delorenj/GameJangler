#[cfg(test)]
mod tests;

use crate::scraper::PlatformInstance;
use crate::settings::LoadSettingsError::NONE;
use nanoid::nanoid;
use serde::{Deserialize, Serialize};
use simplelog::info;
use std::path::Path;
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

pub struct SettingsManager {
    pub settings_path: PathBuf,
    pub settings: Option<SettingsSchema>
}

impl SettingsManager {
    pub fn new(app_dir: Option<PathBuf>) -> Self {
        Self {
            settings_path: app_dir.unwrap(),
            settings: None
        }
    }
}

pub trait Loadable {
    fn load(&self) -> Result<SettingsSchema, LoadSettingsError>;
}

impl Loadable for SettingsManager {
    fn load(&self) -> Result<SettingsSchema, LoadSettingsError> {
        let result = fs::read_to_string(self.settings_path.join(Path::new(SETTINGS_FILE)));
        
            match result {
                Ok(s) => {
                    info!("Loaded settings!");
                }
                Err(e) => {
                    error!("Nope")
                }
            }
        Err(NONE)
    }
}    

    // pub fn init() -> Result<SettingsSchema, LoadSettingsError> {
    //     let data = "balls";
    //     fs::write(app_dir
    //         .unwrap()
    //         .join(Path::new(SETTINGS_FILE)), data).expect("Unable to write file");

    //     Err(NONE)
    // }
