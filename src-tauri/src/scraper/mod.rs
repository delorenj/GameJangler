struct GameInstance {
    title: String,
    location: String,
    owning_platform: PlatformInstance
}

struct PlatformInstance{
    id: String, //UUID
    name: String,
    location: String
}

trait Scrapable<T> {
    fn scrape() -> Vec<T>;
}

struct Scraper<T> {
    drive_letter: char,
    result: Vec<T>
}

impl Scraper<PlatformInstance> {
    fn new(drive_letter: &char) -> Self {
        Self.drive_letter = *drive_letter;
        let mut result = Vec::new();
        return Self
    }
}
