extern crate wasm_bindgen;
extern crate num;
extern crate num_traits;

pub mod crypto_math;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Why hello there, {}!", name));
}


