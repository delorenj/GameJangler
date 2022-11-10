use crate::scraper::{GameInstance, PlatformInstance, Scrapable};

#[cfg(test)]
mod tests;

pub struct Steam;

impl Scrapable<PlatformInstance> for Steam {
    fn start_scrape(&self, result: &mut Vec<PlatformInstance>, drive_letter: char) {
        println!("Scanning for Steam platforms on drive letter {}", drive_letter);

    }
}

impl Scrapable<GameInstance> for Steam {
    fn start_scrape(&self, result: &mut Vec<GameInstance>, drive_letter: char) {
        println!("Scanning for Steam games on drive letter {}", drive_letter);
    }
}
