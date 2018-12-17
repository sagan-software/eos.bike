use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "../static/wallet")]
extern "C" {
    pub fn connect();
}
