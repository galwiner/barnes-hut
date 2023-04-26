extern crate env_logger;

use log::LevelFilter::{Debug, Error, Warn};

use barnes_hut::{env_logger_config, run_sync};

fn main() {
    env_logger_config().init();
    run_sync()
}
