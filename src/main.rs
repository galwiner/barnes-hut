extern crate env_logger;
use nannou_egui::{self};

fn main() {
    configure_logging();
    barnes_hut::application::run_sync()
}

fn configure_logging() {
    use log::LevelFilter::*;
    env_logger::Builder::new()
        .filter_level(Warn)
        .filter_module(barnes_hut::MODULE_PATH, Debug)
        .filter_module("wgpu_hal::dx12::instance", Error)
        .format_timestamp(None)
        .parse_default_env()
        .init();
}
