use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/js_bindings/navigate.js")]
extern "C" {
    pub fn change_location_to(location: &str);
}
