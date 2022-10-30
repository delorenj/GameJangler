pub struct GameInstance {
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
    pub fn new() -> Self {
        Self.id = nanite!();
        return Self;
    }
}
trait Scrapable<ScrapeType> {
    fn start_scrape(&self, &mut result: Vec<ScrapeType>, drive_letter: &char);
}

struct Steam;

impl Scrapable<ScrapeType> for Steam {
    fn start_scrape(&self, &mut result: Vec<ScrapeType>, drive_letter: &char) {
        println!("Scanning for Steam platforms on drive letter {}", drive_letter);
    }
}

struct Epic;
impl Scrapable<ScrapeType> for Steam {
    fn start_scrape(&self, &mut result: Vec<T>, drive_letter: &char) {
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
    pub fn scrape(&self, &mut result: Vec<ScrapeType>, drive_letter: &char) {
        self.platform_type.start_scrape(result, drive_letter);
    }
}


