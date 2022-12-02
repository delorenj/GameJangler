pub mod epic;
pub mod steam;

use std::borrow::Borrow;
use strum::{IntoEnumIterator, EnumIter};

#[cfg(test)]
mod tests;
use nanoid::nanoid;
use serde::{Deserialize, Serialize};
use std::fs::DirEntry;

use self::{steam::Steam, epic::Epic};

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
    platforms: Vec<Platform>
}

impl Default for PlatformSet {
    fn default() -> PlatformSet {
        PlatformSet {
         platforms: Platform::iter().collect::<Vec<Platform>>()
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlatformInstance {
    id: String, //UUID
    pub name: String,
    pub location: String,
}

impl PlatformInstance {
    pub fn new(name: String, location: String) -> Self {
        Self {
            id: nanoid!(),
            name,
            location,
        }
    }
}

pub trait Scannable<ScanType> {
    fn start_scan(&self, result: &mut Vec<ScanType>, root_paths: &Vec<&str>);
}

pub trait MetaScannable<ScanType> {
    fn start_scan(&self, result: &mut Vec<ScanType>, root_paths: &Vec<&str>, platform_set: Option<PlatformSet>);
}

pub struct ScanManager {}

impl MetaScannable<PlatformInstance> for ScanManager {
    fn start_scan(&self, result: &mut Vec<PlatformInstance>, root_paths: &Vec<&str>, platform_set: Option<PlatformSet>) {
        let platforms = platform_set.unwrap_or_default();
        for platform in platforms.platforms.iter() {
            match platform {
              Platform::STEAM => {
                Steam.start_scan(result, root_paths)
              } 
              Platform::EPIC => {
                Epic.start_scan(result, root_paths)
              }
            }
        }
    }
}
