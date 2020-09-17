use wasm_bindgen::prelude::*;
use log::{Level, info};

#[wasm_bindgen(start)]
pub fn run() {
    console_log::init_with_level(Level::Debug).expect("failed to init logging");

    info!("Hello world!");
}
