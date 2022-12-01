use std::fs::DirEntry;
use crate::scraper::{GameInstance, PlatformInstance, Scrapable};

extern crate finder;

use finder::Finder;

#[cfg(test)]
mod tests;

pub struct Steam;

fn is_vdf_file(e: &DirEntry) -> bool {
    if let Some(s) = e.path().file_name() {
        let name = String::from(s.to_str().unwrap());
        if name.ends_with(".vdf") {
            return true;
        }
    }
    false
}

fn is_libraryfolders_file(e: &DirEntry) -> bool {
    if let Some(s) = e.path().file_name() {
        let name = String::from(s.to_str().unwrap());
        if name.ends_with("libraryfolders.vdf") {
            return true;
        }
    }
    false
}

impl Scrapable<PlatformInstance> for Steam {
    fn start_scrape(&self, result: &mut Vec<PlatformInstance>, root_paths: &Vec<&str>) {
        println!("Scanning for Steam platforms on root paths {:?}", root_paths);
        let finders = Finder::new("C:\\");
        for i in finders.filter(&is_libraryfolders_file).into_iter() {
            println!("{}", i.path().to_str().unwrap());
        }
    }
}

impl Scrapable<GameInstance> for Steam {
    fn start_scrape(&self, result: &mut Vec<GameInstance>, root_paths: &Vec<&str>) {
        println!("Scanning for Steam games on root paths {:?}", root_paths);
    }
}
