#[macro_use]
extern crate derivative;
extern crate env_logger;
#[macro_use]
extern crate log;
extern crate nannou;
#[cfg(feature = "parallel")]
extern crate rayon;
#[cfg(test)]
#[macro_use]
extern crate static_assertions;

use log::LevelFilter::{Debug, Error, Warn};

#[macro_use]
mod macros;
mod application;
mod created;
mod drawing;
mod physics;
mod quad_tree;
mod simulation;
mod view_state;

fn main() {
    env_logger::Builder::new()
        .filter_level(Warn)
        .filter_module(module_path!(), Debug)
        .filter_module("wgpu_hal::dx12::instance", Error)
        .parse_default_env()
        .init();
    nannou::app(application::init_app)
        .update(application::update)
        .run()
}
