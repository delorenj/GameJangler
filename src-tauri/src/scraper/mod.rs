#[cfg(test)]
mod tests;

use nanoid::nanoid;

pub struct GameInstance {
    id: String,
    title: String,
    location: String,
    owning_platform: PlatformInstance
}

pub struct PlatformInstance{
    id: String, //UUID
    pub name: String,
    pub location: String
}

impl PlatformInstance {
    pub fn new(name: String, location: String) -> Self {
        Self { id: nanoid!(), name, location }
    }
}

impl GameInstance {
    pub fn new(title: String, location: String, owning_platform: String) -> Self {
        Self { id: nanoid!(), title, location, owning_platform }
    }
}

trait Scrapable<ScrapeType> {
    fn start_scrape(&self, result: &mut Vec<ScrapeType>, drive_letter: char);
}

struct Steam;

impl Scrapable<ScrapeType> for Steam {
    fn start_scrape(&self, result: &mut Vec<ScrapeType>, drive_letter: char) {
        result.push({})
        println!("Scanning for Steam platforms on drive letter {}", drive_letter);
    }
}

struct Epic;
impl Scrapable<ScrapeType> for Epic {
    fn start_scrape(&self, result: &mut Vec<ScrapeType>, drive_letter: char) {
        println!("Scanning for Epic platforms on drive letter {}", drive_letter);
    }
}

struct Scraper<PlatformType: Scrapable<ScrapeType>> {
    platform_type: PlatformType
}

impl<PlatformType: Scrapable<ScrapeType>> Scraper<PlatformType> {
    pub fn new(platform_type: PlatformType) -> Self {
        Self { platform_type }
    }
    pub fn scrape(&self, result: &mut Vec<ScrapeType>, drive_letter: char) {
        self.platform_type.start_scrape(result, drive_letter);
    }
}


