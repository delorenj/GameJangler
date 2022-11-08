use crate::scraper::{PlatformInstance, PlatformScraper, Steam};

#[test]
fn test_platform_instance_constructor_has_unique_id_with_length_21() {
    let platform = PlatformInstance::new(
        "test".to_string(),
        "C:/some/path".to_string()
    );
    assert_eq!(21, platform.id.len());
}

#[test]
fn test_steam_scraper_can_scrape_for_platform() {
    let scraper = PlatformScraper::new(Steam);
    let mut result = Vec::new();
    scraper.scrape(&mut result, 'C')
}

#[test]
fn test_steam_scraper_can_put_something_in_result_vec() {
    let scraper = PlatformScraper::new(Steam);
    let mut result = Vec::new();
    assert_eq!(0, result.len());
    scraper.scrape(&mut result, 'C');
    assert_ne!(0, result.len());
}
