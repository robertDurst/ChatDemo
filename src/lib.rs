//! Crypto Math is a crate containing basic cryptographic primitives and encryption schemes.

extern crate num;
extern crate num_traits;
extern crate rand;
extern crate wasm_bindgen;

#[macro_use]
extern crate lazy_static;

/// Currently this module contains basic crypto math primitives (GCD, modular inverse, etc.) and
/// RSA encryption/decryption.
pub mod crypto_math;
