extern crate env_logger;

use log::LevelFilter::{Debug, Error, Warn};

use barnes_hut;

fn main() {
    env_logger::Builder::new()
        .filter_level(Warn)
        .filter_module(module_path!(), Debug)
        .filter_module("wgpu_hal::dx12::instance", Error)
        .format_timestamp(None)
        .parse_default_env()
        .init();

    barnes_hut::run_sync()
}
