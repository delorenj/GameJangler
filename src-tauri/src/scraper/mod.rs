pub mod epic;
pub mod steam;

#[cfg(test)]
mod tests;
use nanoid::nanoid;
use serde::{Deserialize, Serialize};
use std::fs::DirEntry;

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

pub enum Platform {
    STEAM,
    EPIC,
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

pub trait Scrapable<ScrapeType> {
    fn start_scrape(&self, result: &mut Vec<ScrapeType>, root_paths: &Vec<&str>);
}

pub struct ScrapeManager {}

impl Scrapable<PlatformInstance> for ScrapeManager {
    fn start_scrape(&self, result: &mut Vec<PlatformInstance>, root_paths: &Vec<&str>) {
        todo!()
    }
}