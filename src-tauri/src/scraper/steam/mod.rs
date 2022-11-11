use crate::scraper::{GameInstance, PlatformInstance, Scrapable};
use error_chain::error_chain;
use glob::{glob_with, MatchOptions};

#[cfg(test)]
mod tests;

pub struct Steam;

error_chain! {
    foreign_links {
        Glob(glob::GlobError);
        Pattern(glob::PatternError);
    }
}

impl Scrapable<PlatformInstance> for Steam {
    fn start_scrape(&self, result: &mut Vec<PlatformInstance>, drive_letter: char) -> Result<()> {
        println!("Scanning for Steam platforms on drive letter {}", drive_letter);
        let options = MatchOptions {
            case_sensitive: false,
            ..Default::default()
        };

        for entry in glob_with("steam/libraryfolders.vdf", options)? {
            println!("{}", entry?.display());
        }
        Ok(())
    }
}

impl Scrapable<GameInstance> for Steam {
    fn start_scrape(&self, result: &mut Vec<GameInstance>, drive_letter: char) -> Result<()>{
        println!("Scanning for Steam games on drive letter {}", drive_letter);
        Ok(())
    }
}
