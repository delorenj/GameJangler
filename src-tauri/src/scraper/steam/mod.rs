use crate::scraper::{GameInstance, PlatformInstance, Scrapable};
extern crate finder;
use finder::Finder;

#[cfg(test)]
mod tests;

pub struct Steam;

impl Scrapable<PlatformInstance> for Steam {
    fn start_scrape(&self, result: &mut Vec<PlatformInstance>, drive_letter: char) {
        println!("Scanning for Steam platforms on drive letter {}", drive_letter);
        let finders = Finder::new("C:\\Program Files (x86)\\Steam\\steamapps");
        for i in finders.into_iter() {
            println!("{}", i.path().to_str().unwrap());
        }
    }
}

impl Scrapable<GameInstance> for Steam {
    fn start_scrape(&self, result: &mut Vec<GameInstance>, drive_letter: char) {
        println!("Scanning for Steam games on drive letter {}", drive_letter);
    }
}
