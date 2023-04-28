#[macro_use]
extern crate derivative;
#[macro_use]
extern crate log;
#[cfg(test)]
#[allow(unused_imports)]
#[macro_use]
extern crate static_assertions;

#[cfg(target_arch = "wasm32")]
pub use wasm::*;

#[macro_use]
mod macros;
pub mod application;
mod created;
mod drawing;
mod physics;
mod simulation;
mod view_state;
#[cfg(target_arch = "wasm32")]
mod wasm;

pub const MODULE_PATH: &str = module_path!();
