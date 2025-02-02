use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/js_bindings/is_active_route.js")]
extern "C" {
    pub fn is_active_route(route: &str) -> bool;
}
