use crate::scraper::{GameInstance, PlatformInstance, Scrapable};
#[cfg(test)]
mod tests;

pub struct Epic;

impl Scrapable<PlatformInstance> for Epic {
    fn start_scrape(&self, result: &mut Vec<PlatformInstance>, drive_letter: char) -> Result<(), ()>{
        println!("Scanning for Epic platforms on drive letter {}", drive_letter);
        let test = PlatformInstance::new("Test".to_owned(), "C:/some/location".to_owned());
        result.push(test);
        Ok(())
    }
}

impl Scrapable<GameInstance> for Epic {
    fn start_scrape(&self, result: &mut Vec<GameInstance>, drive_letter: char) -> Result<(), ()>{
        println!("Scanning for Epic games on drive letter {}", drive_letter);
        Ok(())
    }
}
