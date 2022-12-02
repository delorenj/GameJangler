use crate::scanner::{GameInstance, PlatformInstance, Scannable};
#[cfg(test)]
mod tests;

pub struct Epic;

impl Scannable<PlatformInstance> for Epic {
    fn start_scan(&self, result: &mut Vec<PlatformInstance>, root_paths: &Vec<&str>) {
        println!("Scanning for Epic platforms on root paths {:?}", root_paths);
        let test = PlatformInstance::new("Test".to_owned(), "C:/some/location".to_owned());
        result.push(test);
    }
}

impl Scannable<GameInstance> for Epic {
    fn start_scan(&self, result: &mut Vec<GameInstance>, root_paths: &Vec<&str>) {
        println!("Scanning for Epic games on root paths {:?}", root_paths);
    }
}
