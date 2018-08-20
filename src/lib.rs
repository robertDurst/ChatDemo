extern crate num;
extern crate num_bigint as biggint;
extern crate num_traits;
extern crate rand;
extern crate wasm_bindgen;

pub mod crypto_math;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Why hello there, {}!", name));
}
