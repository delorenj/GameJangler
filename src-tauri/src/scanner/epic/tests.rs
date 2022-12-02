use crate::scanner::epic::Epic;
use crate::scanner::{PlatformInstance, Scannable};

#[test]
fn test_epic_scanner_can_scrape_for_platform() {
    let mut result: Vec<PlatformInstance> = Vec::new();
    Epic.start_scan(&mut result, &vec!["C:/"]);
}

#[test]
fn test_epic_scanner_can_put_something_in_result_vec() {
    let mut result: Vec<PlatformInstance> = Vec::new();
    assert_eq!(0, result.len());
    Epic.start_scan(&mut result, &vec!["C:/"]);
    assert_ne!(0, result.len());
}
