#[cfg(test)]
mod tests;

use crate::scraper::PlatformInstance;
use crate::settings::LoadSettingsError::NONE;
use nanoid::nanoid;
use serde::{Deserialize, Serialize};
use std::fs::DirEntry;

#[derive(Serialize, Deserialize, Debug)]
pub struct SettingsSchema {
    pub platforms: Option<Vec<PlatformInstance>>,
}

pub enum LoadSettingsError {
    NONE,
    EMPTY,
    CORRUPT,
}
pub struct SettingsManager;

impl SettingsManager {
    pub fn load() -> Result<SettingsSchema, LoadSettingsError> {
        Err(NONE)
    }
}
