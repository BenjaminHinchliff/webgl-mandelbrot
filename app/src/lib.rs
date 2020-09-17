use wasm_bindgen::prelude::*;
use log::{Level, info};

#[wasm_bindgen(start)]
pub fn run() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(Level::Debug).expect("failed to init logging");

    info!("Hello world!");
    panic!("hello panic!");
}
