#[cfg(test)]
mod tests;

use nanoid::nanoid;

pub struct GameInstance {
    id: String,
    title: String,
    location: String
}

impl GameInstance {
    pub fn new(title: String, location: String) -> Self {
        Self { id: nanoid!(), title, location }
    }
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


trait Scrapable<ScrapeType> {
    fn start_scrape(&self, result: &mut Vec<ScrapeType>, drive_letter: char);
}

struct Steam;

impl Scrapable<PlatformInstance> for Steam {
    fn start_scrape(&self, result: &mut Vec<PlatformInstance>, drive_letter: char) {
        println!("Scanning for Steam platforms on drive letter {}", drive_letter);
        let test = PlatformInstance::new("Test".to_owned(), "C:/some/location".to_owned());
        result.push(test);
    }
}

impl Scrapable<GameInstance> for Steam {
    fn start_scrape(&self, result: &mut Vec<GameInstance>, drive_letter: char) {
        println!("Scanning for Steam games on drive letter {}", drive_letter);
    }
}


