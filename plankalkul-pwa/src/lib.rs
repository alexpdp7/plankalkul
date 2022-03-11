#![allow(clippy::unused_unit)] // https://github.com/rustwasm/wasm-bindgen/issues/2774#issuecomment-1030747023
mod app;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    yew::start_app::<app::Model>();

    Ok(())
}
