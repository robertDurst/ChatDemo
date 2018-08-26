//! Crypto Math is a crate containing basic cryptographic primitives and encryption schemes.

extern crate num;
extern crate num_traits;
extern crate rand;
extern crate wasm_bindgen;

#[macro_use]
extern crate lazy_static;

/// The crypto_math module contains basic crypto math primitives (GCD, modular inverse, etc.).
pub mod crypto_math;

/// The rsa module contains methods for encryption/decryption of messages utilizng RSA asymmetric,
/// public key cryptosystem.
pub mod rsa;
