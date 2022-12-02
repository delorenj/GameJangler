#[cfg(test)]
mod tests;

use crate::scanner::PlatformInstance;
use serde::{Deserialize, Serialize};
use simplelog::info;
use std::path::{Path, PathBuf};
use std::fs;

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct SettingsSchema {
    pub platforms: Option<Vec<PlatformInstance>>,
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
    fn load(&self) -> SettingsSchema;
    fn init(&self) -> SettingsSchema;
}

impl Loadable for SettingsManager {
    fn load(&self) -> SettingsSchema {
        let result = fs::read_to_string(self.settings_path.join(Path::new(SETTINGS_FILE)));
        
            match result {
                Ok(s) => {
                    info!("Loaded settings!");
                    let settings: Result<SettingsSchema, _> = serde_json::from_str(&s); 
                    match settings {
                        Ok(s) => return s,
                        Err(_) => return self.init()
                    };

                }
                Err(_) =>  return self.init()
            };
    }
    
    fn init(&self) -> SettingsSchema {
        let data = SettingsSchema::default();

        if !self.settings_path.exists() {
            fs::create_dir(&self.settings_path).expect("Error creating config directory!");
        }

        fs::write(
            self.settings_path.join(Path::new(SETTINGS_FILE)), 
            serde_json::to_string_pretty(&data).unwrap().to_owned())
        .expect("Unable to write file: ");
        self.load()

    }
}
