use std::{env};
use app::scraper::{PlatformInstance, Scrapable};
use app::scraper::steam::Steam;

fn main() -> Result<(), ()>  {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let mut platforms: Vec<PlatformInstance> = Vec::new();
    Steam.start_scrape(&mut platforms, 'C');
    Ok(())

}
