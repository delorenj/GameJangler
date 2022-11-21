use crate::scraper::{PlatformInstance, Scrapable};
use crate::scraper::steam::Steam;

#[test]
fn test_steam_scraper_can_scrape_for_platform() {
    let mut result: Vec<PlatformInstance> = Vec::new();
    Steam.start_scrape(&mut result, 'C');
}

#[test]
fn test_steam_scraper_can_put_something_in_result_vec() {
    let mut result: Vec<PlatformInstance> = Vec::new();
    assert_eq!(0, result.len());
    Steam.start_scrape(&mut result, 'C');
    assert_ne!(0, result.len());
}
