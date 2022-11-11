pub mod steam;
pub mod epic;

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

pub trait Scrapable<ScrapeType> {
    fn start_scrape(&self, result: &mut Vec<ScrapeType>, drive_letter: char) -> Result<(), ()>;
}



