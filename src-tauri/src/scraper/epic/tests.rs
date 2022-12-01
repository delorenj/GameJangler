use crate::scraper::epic::Epic;
use crate::scraper::{PlatformInstance, Scrapable};

#[test]
fn test_epic_scraper_can_scrape_for_platform() {
    let mut result: Vec<PlatformInstance> = Vec::new();
    Epic.start_scrape(&mut result, &vec!["C:/"]);
}

#[test]
fn test_epic_scraper_can_put_something_in_result_vec() {
    let mut result: Vec<PlatformInstance> = Vec::new();
    assert_eq!(0, result.len());
    Epic.start_scrape(&mut result, &vec!["C:/"]);
    assert_ne!(0, result.len());
}
