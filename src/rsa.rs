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
            d_num += &n_num;
        }

        Keypair {
            d: number_to_string(&d_num),
            n: number_to_string(&n_num),
        }
    }

    /// nicely outputs a formatted public key for use in the javascript code.
    /// improved since 0.2.0. Now outputs just n as a radix 32 string similar
    /// to how it is done here: http://gauss.ececs.uc.edu/Courses/c653/project/radix_32.html
    pub fn public_key_display_wasm(&self) -> String {
        format!("{}", string_to_number(&self.n).to_str_radix(32))
    }

    /// given a ciphertext, attempts to decrypt based on the private key and modulo from this keypair. Performs
    /// simple decryption based on RSA algorithm.
    pub fn decrypt(&self, c: &str) -> i32 {
        let private_key = string_to_number(&self.d);
        let modulus = string_to_number(&self.n);
        let ciphertext = string_to_number(c);

        let decrypted = ciphertext.modpow(&private_key, &modulus);
        
        let decrypted_i32 = decrypted.to_i32();
        if let Some(d_i32) = decrypted_i32 {
            return d_i32
        }
        
        return -1;
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
        let e = string_to_number("65537");
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
pub fn encrypt(m: i32, n: &str) -> String {
    let plaintext = string_to_number(&format!("{}", m));
    // receive n as hex and convert back to decimal
    let modulus = BigInt::parse_bytes(n.as_bytes(), 32).unwrap();
    let public_key = string_to_number("65537");
    
    let encrypted = plaintext.modpow(&public_key, &modulus);

    number_to_string(&encrypted)
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
        let plaintext = 32;
        let modulus = k.public_key_display_wasm();
        
        let ciphertext = encrypt(plaintext, &modulus);
        let decrypted = k.decrypt(&ciphertext);

        assert_eq!(plaintext, decrypted);
    }
}
