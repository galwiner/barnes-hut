extern crate fern;

use wasm_bindgen::prelude::*;

// Entry point for wasm
#[wasm_bindgen(start)]
pub fn wasm_entrypoint() -> Result<(), JsValue> {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    configure_logging().unwrap();
    crate::application::run_sync();
    Ok(())
}

fn configure_logging() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {}] {}",
                record.level(),
                record.target(),
                message
            ))
        })
        .level(log::LevelFilter::Warn)
        .level_for(crate::MODULE_PATH, log::LevelFilter::Debug)
        .level_for("wgpu_hal::dx12::instance", log::LevelFilter::Error)
        .chain(fern::Output::call(console_log::log))
        .apply()?;
    Ok(())
}
