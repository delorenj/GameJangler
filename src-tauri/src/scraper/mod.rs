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

trait Scrapable<T> {
    fn start_scrape(&self, result: &mut Vec<T>, drive_letter: char);
}

struct Steam;


impl Scrapable<PlatformInstance> for Steam {
    fn start_scrape(&self, result: &mut Vec<PlatformInstance>, drive_letter: char) {
        println!("Scanning for Steam platforms on drive letter {}", drive_letter);
    }
}

struct Epic;
impl Scrapable<PlatformInstance> for Epic {
    fn start_scrape(&self, result: &mut Vec<PlatformInstance>, drive_letter: char) {
        println!("Scanning for Epic platforms on drive letter {}", drive_letter);
    }
}

trait Platform {

}
impl Platform for Steam {

}

impl Platform for Epic {

}

struct PlatformScraper<T: Platform>;

impl Scrapable<PlatformInstance> for PlatformScraper<Steam> {
    fn scrape(&self, result: &mut Vec<PlatformInstance>, drive_letter: char) {
        self.platform_type.start_scrape(result, drive_letter);
    }
}

impl Scrapable<PlatformInstance> for PlatformScraper<Epic> {
    fn scrape(&self, result: &mut Vec<PlatformInstance>, drive_letter: char) {
        self.platform_type.start_scrape(result, drive_letter);
    }
}

