#[cfg(test)]
mod tests;

use crate::scanner::PlatformInstance;
use serde::{Deserialize, Serialize};
use simplelog::info;
use std::env::consts::OS;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, strum::EnumIter, Serialize, Deserialize)]
pub enum OsType {
    WINDOWS(String),
    DARWIN,
    LINUX,
    UNSUPPORTED,
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct SettingsSchema {
    pub platforms: Option<Vec<PlatformInstance>>,
    pub os: Option<String>,
}

pub const SETTINGS_FILE: &str = "settings.json";

pub struct SettingsManager {
    pub settings_path: PathBuf,
    pub settings: Option<SettingsSchema>,
}

pub fn get_os_type() -> String {
    return match std::env::consts::OS {
        "windows" => "windows".to_string(),
        "macos" => "darwin".to_string(),
        "linux" => "linux".to_string(),
        &_ => "unsupported".to_string(),
    };
}

impl SettingsManager {
    pub fn new(app_dir: Option<PathBuf>) -> Self {
        Self {
            settings_path: app_dir.unwrap(),
            settings: None,
        }
    }

    pub fn write_platforms(data: Vec<PlatformInstance>) {
        let mut schema = self.lo
    }
}

pub trait Loadable {
    fn load(&self) -> SettingsSchema;
    fn init(&self) -> SettingsSchema;
    fn save(&self, data: &SettingsSchema) -> SettingsSchema;
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
                    Err(_) => return self.init(),
                };
            }
            Err(_) => return self.init(),
        };
    }

    fn init(&self) -> SettingsSchema {
        let mut data = SettingsSchema::default();
        data.os = Some(get_os_type());

        if !self.settings_path.exists() {
            fs::create_dir(&self.settings_path).expect("Error creating config directory!");
        }

        fs::write(
            self.settings_path.join(Path::new(SETTINGS_FILE)),
            serde_json::to_string_pretty(&data).unwrap().to_owned(),
        )
        .expect("Unable to write file: ");
        self.load()
    }

    fn save(&self, data: &SettingsSchema) -> SettingsSchema {
        fs::write(
            self.settings_path.join(Path::new(SETTINGS_FILE)),
            serde_json::to_string_pretty(data).unwrap().to_owned(),
        )
        .expect("Unable to write file: ");
        self.load()
    }
}
