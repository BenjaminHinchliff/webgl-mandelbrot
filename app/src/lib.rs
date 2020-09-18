use wasm_bindgen::prelude::*;
use log::{Level, info};

#[wasm_bindgen]
pub fn render_mandelbrot() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(Level::Debug).expect("failed to init logging");

    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

    let canvas = document.get_element_by_id("screen").expect("element with #screen must exist");

    info!("{:#?}", canvas);
}
