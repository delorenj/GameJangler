use crate::scraper::{PlatformInstance, Scraper, Steam};

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
    let scraper = Scraper::new(Steam);
    let mut result = Vec::new();
    assert_eq!(0, result.len());
    scraper.scrape(&result, 'C');
    assert_ne!(0, result.len())
}
