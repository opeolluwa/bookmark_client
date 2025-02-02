use wasm_bindgen::prelude::*;

// create rust functions from the javascript functions
#[wasm_bindgen(module = "/src/js_bindings/add.js")]
extern "C" {
    pub fn add(a: u8, b: u8) -> u8;
}
