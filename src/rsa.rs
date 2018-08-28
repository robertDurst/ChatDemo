use crypto_math::{generate_prime, lcm, mod_inverse, number_to_string, string_to_number};
use num::bigint::BigInt;
use num_traits::ToPrimitive;
use wasm_bindgen::prelude::*;

// Why lazy_static you may ask? Well, for one, try to compile this without lazy_static. You will
// get an error saying statics can't be the result of an executed function. So, as per the crate
// docs, with lazy_static we get Using this macro, it is possible to have statics that require
// code to be executed at runtime in order to be initialized.
lazy_static! {
    static ref ZERO: BigInt = string_to_number("0");
    static ref ONE: BigInt = string_to_number("1");
    static ref TWO: BigInt = string_to_number("2");
}

/// stores the information for a given RSA keypair.
#[wasm_bindgen]
#[derive(Debug)]
pub struct Keypair {
    /// Public key
    e: String,
    /// Private key
    d: String,
    /// Modulo (both public and private)
    n: String,
}

#[wasm_bindgen]
impl Keypair {
    /// randomly generates a new keypair based on two seeds.
    pub fn new(seed_one: &[u8], seed_two: &[u8]) -> Keypair {
        // Hardcoded to 256-bits with 1000 tries for now
        let q_num = generate_prime(256, 1000, &seed_one).unwrap();

        // Hardcoded to 256-bits with 1000 tries for now
        let p_num = generate_prime(256, 1000, &seed_two).unwrap();

        let n_num = &p_num * &q_num;

        let p_minus_one = &p_num - &*ONE;
        let q_minus_one = &q_num - &*ONE;

        let phi_num = lcm(&p_minus_one, &q_minus_one);
        // Hard Code in 65537.
        // Ref: https://www.reddit.com/r/crypto/comments/6363di/how_do_computers_choose_the_rsa_value_for_e/
        let e_num = string_to_number("65537");

        let mut d_num = mod_inverse(&e_num, &phi_num).unwrap();

        if d_num < *ZERO {
            d_num += &phi_num;
        }

        Keypair {
            e: number_to_string(&e_num),
            d: number_to_string(&d_num),
            n: number_to_string(&n_num),
        }
    }

    /// nicely outputs a formatted public key for use in the javascript code.
    pub fn public_key_display_wasm(&self) -> String {
        format!("({}, {})", self.e, self.n)
    }

    /// given a ciphertext, attempts to decrypt based on the private key and modulo from this keypair. Performs
    /// simple decryption based on RSA algorithm.
    pub fn decrypt(&self, ciphertext: &str) -> String {
        let private_key = string_to_number(&self.d);
        let modulus = string_to_number(&self.n);

        let mut decrypted_values: Vec<char> = Vec::new();

        for c in ciphertext.split(',') {
            let to_decrypt = string_to_number(c);
            let decrypted = to_decrypt.modpow(&private_key, &modulus);
            let decrypted_u8 = decrypted.to_u8();
            if let Some(d_u8) = decrypted_u8 {
                decrypted_values.push(d_u8 as char)
            }
        }

        decrypted_values.iter().collect()
    }
}

#[cfg(test)]
mod test_generate_key {
    use super::*;

    #[test]
    fn works_with_simple_encrypt_decrypt() {
        // You need two different seeds (p and q must be different)
        let seed_one = &[
            10, 16, 51, 42, 123, 31, 212, 31, 233, 15, 9, 7, 41, 32, 4, 3, 144, 122, 1, 35, 1, 13,
            55, 23, 1, 33, 1, 1, 1, 1, 2, 1,
        ];
        let seed_two = &[
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1,
        ];

        // Generate a keypair
        let k = Keypair::new(seed_one, seed_two);

        // Capture all the variables for encryption and decryption
        let e = string_to_number(&k.e);
        let d = string_to_number(&k.d);
        let n = string_to_number(&k.n);

        // Message and ciphertext
        let plaintext = string_to_number("72");
        let ciphertext = plaintext.modpow(&e, &n);

        let decrypted = ciphertext.modpow(&d, &n);
        assert_eq!(plaintext, decrypted);
    }
}

/// given a public key (e, n), encrypts message m for this public key using RSA.
#[wasm_bindgen]
pub fn encrypt(m: &str, e: &str, n: &str) -> String {
    let public_key = string_to_number(e);
    let modulus = string_to_number(n);

    let mut encrypted_values = String::default();

    for c in m.bytes() {
        let c_str = c.to_string();
        let to_encrypt = string_to_number(&c_str);
        let encrypted = to_encrypt.modpow(&public_key, &modulus);

        encrypted_values = format!("{},{}", encrypted_values, number_to_string(&encrypted));
    }

    encrypted_values
}

#[cfg(test)]
mod test_encrypt_decrypt {
    use super::*;

    #[test]
    fn complete_encrypt_and_decrypt() {
        // You need two different seeds (p and q must be different)
        let seed_one = &[
            10, 16, 51, 42, 123, 31, 212, 31, 233, 15, 9, 7, 41, 32, 4, 3, 144, 122, 1, 35, 1, 13,
            55, 23, 1, 33, 1, 1, 1, 1, 2, 1,
        ];
        let seed_two = &[
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1,
        ];

        // Generate a keypair
        let k = Keypair::new(seed_one, seed_two);

        // Message and ciphertext
        let plaintext = "HelloWorld!";
        let ciphertext = encrypt(plaintext, &k.e, &k.n);
        let decrypted = k.decrypt(&ciphertext[1..]);

        assert_eq!(plaintext, decrypted);
    }
}
