use std::fs::File;
use simplelog::*;


const LOG_FILE: &str = "app.log";

pub fn init_log() {
    CombinedLogger::init(
        vec![
            WriteLogger::new(LevelFilter::Info, Config::default(), File::create(LOG_FILE).unwrap()),
        ]
    ).unwrap();
}
