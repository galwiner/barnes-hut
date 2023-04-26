#[macro_use]
extern crate derivative;
extern crate env_logger;
#[macro_use]
extern crate log;
extern crate nannou;
#[cfg(feature = "parallel")]
extern crate rayon;
#[cfg(test)]
#[allow(unused_imports)]
#[macro_use]
extern crate static_assertions;

pub use application::*;

#[macro_use]
mod macros;
mod application;
mod created;
mod drawing;
mod physics;
mod simulation;
mod view_state;

pub fn env_logger_config() -> env_logger::Builder {
    use log::LevelFilter::*;
    let mut builder = env_logger::Builder::new();
    builder
        .filter_level(Warn)
        .filter_module(module_path!(), Debug)
        .filter_module("wgpu_hal::dx12::instance", Error)
        .format_timestamp(None)
        .parse_default_env();
    builder
}

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

// Entry point for wasm
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    //console_log::init_with_level(log::Level::Warn).unwrap();
    env_logger_config().init();

    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    run_sync();
    Ok(())
}
