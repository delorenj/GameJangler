use crate::scraper::{GameInstance, PlatformInstance, Scrapable};
#[cfg(test)]
mod tests;

pub struct Epic;

impl Scrapable<PlatformInstance> for Epic {
    fn start_scrape(&self, result: &mut Vec<PlatformInstance>, root_paths: &Vec<&str>) {
        println!("Scanning for Epic platforms on root paths {:?}", root_paths);
        let test = PlatformInstance::new("Test".to_owned(), "C:/some/location".to_owned());
        result.push(test);
    }
}

impl Scrapable<GameInstance> for Epic {
    fn start_scrape(&self, result: &mut Vec<GameInstance>, root_paths: &Vec<&str>) {
        println!("Scanning for Epic games on root paths {:?}", root_paths);
    }
}
