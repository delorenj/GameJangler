use std::{env};
use app::scraper::{PlatformInstance, Scrapable};
use app::scraper::steam::Steam;
use simplelog::*;
use std::fs::File;

fn main() -> Result<(), ()> {
    CombinedLogger::init(
        vec![
            //TermLogger::new(LevelFilter::Warn, simplelog::Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
            WriteLogger::new(LevelFilter::Info, simplelog::Config::default(), File::create("my_rust_binary.log").unwrap()),
        ]
    ).unwrap();
    info!("Starting scraper app...");
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let mut platforms: Vec<PlatformInstance> = Vec::new();
    Steam.start_scrape(&mut platforms, 'C');
    Ok(())
}
