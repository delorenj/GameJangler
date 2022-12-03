use crate::events::rs2js;
use crate::scanner::{GameInstance, Platform, PlatformInstance, Scannable};
use std::{fs::DirEntry, ops::Deref};

extern crate finder;

use finder::Finder;
use tauri::Wry;

#[cfg(test)]
mod tests;

pub struct Steam;

fn is_libraryfolders_file(e: &DirEntry) -> bool {
    if let Some(s) = e.path().file_name() {
        let name = String::from(s.to_str().unwrap());
        if name.ends_with("libraryfolders.vdf") {
            return true;
        }
    }
    false
}

impl Scannable<PlatformInstance> for Steam {
    fn start_scan(
        &self,
        app_handle: &tauri::AppHandle<Wry>,
        result: &mut Vec<PlatformInstance>,
        root_paths: &Vec<&str>,
    ) {
        println!(
            "Scanning for Steam platforms on root paths {:?}",
            root_paths
        );
        let finders = Finder::new("/Users/jaraddelorenzo/Library/Application Support");
        for i in finders.filter(&is_libraryfolders_file).into_iter() {
            println!("{}", i.path().to_str().unwrap());
            rs2js("poop".to_owned(), app_handle);
            result.push(PlatformInstance::new(
                Platform::STEAM,
                i.path().to_path_buf(),
            ))
        }
    }
}

impl Scannable<GameInstance> for Steam {
    fn start_scan(
        &self,
        app_handle: &tauri::AppHandle<Wry>,
        result: &mut Vec<GameInstance>,
        root_paths: &Vec<&str>,
    ) {
        println!("Scanning for Steam games on root paths {:?}", root_paths);
    }
}
