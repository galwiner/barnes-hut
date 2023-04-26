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

#[macro_use]
mod macros;
mod application;
mod created;
mod drawing;
mod physics;
mod simulation;
mod view_state;

pub use application::*;

// Entry point for wasm
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    console_log::init_with_level(log::Level::Debug).unwrap();

    use log::info;
    info!("Logging works!");

    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    run_sync();
    Ok(())
}
