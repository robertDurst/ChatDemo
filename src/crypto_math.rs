use num;
use num::bigint::{BigInt, RandBigInt, ToBigInt};
use num_traits::{One, Zero};
use wasm_bindgen::prelude::*;
use rand::{SeedableRng, StdRng};

static SMALL_PRIMES: &'static [i32] = &[
    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97,
    101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181, 191, 193,
    197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281, 283, 293, 307,
    311, 313, 317, 331, 337, 347, 349, 353, 359, 367, 373, 379, 383, 389, 397, 401, 409, 419, 421,
    431, 433, 439, 443, 449, 457, 461, 463, 467, 479, 487, 491, 499, 503, 509, 521, 523, 541, 547,
    557, 563, 569, 571, 577, 587, 593, 599, 601, 607, 613, 617, 619, 631, 641, 643, 647, 653, 659,
    661, 673, 677, 683, 691, 701, 709, 719, 727, 733, 739, 743, 751, 757, 761, 769, 773, 787, 797,
    809, 811, 821, 823, 827, 829, 839, 853, 857, 859, 863, 877, 881, 883, 887, 907, 911, 919, 929,
    937, 941, 947, 953, 967, 971, 977, 983, 991, 997,
];

static BASES: &'static [i32] = &[2, 3, 5, 7, 11];

macro_rules! StringToNumber {
    ($e: expr) => {
        BigInt::parse_bytes($e.as_bytes(), 10).unwrap();
    };
}

macro_rules! NumberToString {
    ($e: expr) => {
        format!("{}", $e);
    };
}

#[cfg(test)]
mod test_number_macro {
    use super::*;

    #[test]
    fn negative_small() {
        let a = StringToNumber!("-5");
        let b = BigInt::parse_bytes(b"-5", 10).unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn negative_large() {
        let a = StringToNumber!("-523892389328392");
        let b = BigInt::parse_bytes(b"-523892389328392", 10).unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn miniscule() {
        let a = StringToNumber!("0");
        let b = BigInt::parse_bytes(b"0", 10).unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn tiny() {
        let a = StringToNumber!("10");
        let b = BigInt::parse_bytes(b"10", 10).unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn small() {
        let a = StringToNumber!("123");
        let b = BigInt::parse_bytes(b"123", 10).unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn medium() {
        let a = StringToNumber!("123456789");
        let b = BigInt::parse_bytes(b"123456789", 10).unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn large() {
        let a = StringToNumber!("123456789123456789");
        let b = BigInt::parse_bytes(b"123456789123456789", 10).unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn x_large() {
        let a = StringToNumber!("123456789123456789123456789123456789123456789123456789");
        let b = BigInt::parse_bytes(
            b"123456789123456789123456789123456789123456789123456789",
            10,
        ).unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn xx_large() {
        let a = StringToNumber!("123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789");
        let b = BigInt::parse_bytes(b"123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789", 10).unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn xxx_large() {
        let a = StringToNumber!("123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789");
        let b = BigInt::parse_bytes(b"123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789", 10).unwrap();
        assert_eq!(a, b);
    }
}

// Ported from: http://www.maths.dk/teaching/courses/math398-spring2017/code/cryptomath.txt
pub fn gcd(a: &str, b: &str) -> String {
    let mut x = StringToNumber!(a);
    let mut y = StringToNumber!(b);

    while y != Zero::zero() {
        let r = x % &y;
        x = y;
        y = r;
    }

    return NumberToString!(x);
}

#[cfg(test)]
mod test_gcd {
    use super::*;

    #[test]
    fn miniscule() {
        let a = "10";
        let b = "5";
        let expected = "5";
        assert_eq!(gcd(a, b), expected);
    }

    #[test]
    fn tiny() {
        let a = "29943";
        let b = "29738";
        let expected = "1";
        assert_eq!(gcd(a, b), expected);
    }

    #[test]
    fn small() {
        let a = "299429203";
        let b = "827382738";
        let expected = "1";
        assert_eq!(gcd(a, b), expected);
    }

    #[test]
    fn medium() {
        let a = "1672976127961212891";
        let b = "3378278237328723873";
        let expected = "3";
        assert_eq!(gcd(a, b), expected);
    }

    #[test]
    fn large() {
        let a = "16729761279612128911672976127961212891";
        let b = "33782782373287238731672976127961212891";
        let expected = "3";
        assert_eq!(gcd(a, b), expected);
    }

    #[test]
    fn x_large() {
        let a = "1873817317893712873298173982173982173897128738912738217371897381374891378943789";
        let b = "9188937128738173912371837981739817238917246812647812678394619836281693618963297";
        let expected = "1";
        assert_eq!(gcd(a, b), expected);
    }

    #[test]
    fn xx_large() {
        let a = "18273781798371987398173891273871293762178362308763217863871263826817067830612083612876307916239721638916398216398613892168903681293610639120368219732891372189361287361986371863218763017236270362896319038213";
        let b = "82726226362376138712678923161327863279136912363261786391287273961273967239678123623623672369236872671268723672167267612727198623872637892632186267386219627823169783627819623761983627816378263178639821687326";
        let expected = "1";
        assert_eq!(gcd(a, b), expected);
    }

    #[test]
    fn xxx_large() {
        let a = "1827378179837198739817389127387129376217836230876321786387126382681706783061208361287630791623972163891639821639861389216890368129361063912036821973289137218936128736198637186321876301723627036289631903821318273781798371987398173891273871293762178362308763217863871263826817067830612083612876307916239721638916398216398613892168903681293610639120368219732891372189361287361986371863218763017236270362896319038213";
        let b = "8272622636237613871267892316132786327913691236326178639128727396127396723967812362362367236923687267126872367216726761272719862387263789263218626738621962782316978362781962376198362781637826317863982168732618273781798371987398173891273871293762178362308763217863871263826817067830612083612876307916239721638916398216398613892168903681293610639120368219732891372189361287361986371863218763017236270362896319038213";
        let expected = "1";
        assert_eq!(gcd(a, b), expected);
    }
}

pub fn lcm(a: &str, b: &str) -> String {
    let mut x = StringToNumber!(a);
    let mut y = StringToNumber!(b);

    let numerator = &x * &y;
    let denominator = StringToNumber!(gcd(a, b));

    let lcm = numerator / denominator;

    NumberToString!(lcm)
}

#[cfg(test)]
mod test_lcm {
    use super::*;

    #[test]
    fn miniscule() {
        let a = "5";
        let b = "2";
        let expected = "10";
        assert_eq!(lcm(a, b), expected);
    }

    #[test]
    fn tiny() {
        let a = "15";
        let b = "20";
        let expected = "60";
        assert_eq!(lcm(a, b), expected);
    }

    #[test]
    fn small() {
        let a = "299429203";
        let b = "827382738";
        let expected = "247742553815297814";
        assert_eq!(lcm(a, b), expected);
    }

    #[test]
    fn medium() {
        let a = "1672976127961212891";
        let b = "3378278237328723873";
        let expected = "1883926281553946627336368887435682281";
        assert_eq!(lcm(a, b), expected);
    }
}


// Based on pseudocode from: https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm
pub fn extended_gcd(a: &str, b: &str) -> (String, String) {
    let mut r = StringToNumber!(a);
    let mut old_r = StringToNumber!(b);

    let mut old_s: BigInt = One::one();
    let mut s: BigInt = Zero::zero();

    let mut old_t: BigInt = Zero::zero();
    let mut t: BigInt = One::one();

    while r != Zero::zero() {
        let q = &old_r / &r;

        let temp_r = r.clone();
        r = old_r - &q * r;
        old_r = temp_r;

        let temp_s = s.clone();
        s = old_s - &q * s;
        old_s = temp_s;

        let temp_t = t.clone();
        t = old_t - &q * t;
        old_t = temp_t;
    }

    (NumberToString!(old_t), NumberToString!(old_s))
}

#[cfg(test)]
mod test_extended_gcd {
    use super::*;

    #[test]
    fn miniscule() {
        let a = "12";
        let b = "17";
        let expected_a = "-7".to_string();
        let expected_b = "5".to_string();
        assert_eq!(extended_gcd(a, b), (expected_a, expected_b));
    }

    #[test]
    fn tiny() {
        let a = "180";
        let b = "150";
        let expected_a = "1".to_string();
        let expected_b = "-1".to_string();
        assert_eq!(extended_gcd(a, b), (expected_a, expected_b));
    }

    #[test]
    fn small() {
        let a = "39392";
        let b = "9372";
        let expected_a = "950".to_string();
        let expected_b = "-3993".to_string();
        assert_eq!(extended_gcd(a, b), (expected_a, expected_b));
    }

    #[test]
    fn medium() {
        let a = "29837362344";
        let b = "20938934792";
        let expected_a = "70028498".to_string();
        let expected_b = "-99788537".to_string();
        assert_eq!(extended_gcd(a, b), (expected_a, expected_b));
    }

    #[test]
    fn large() {
        let a = "298373623442234243";
        let b = "224224424293284938";
        let expected_a = "29778059398942417".to_string();
        let expected_b = "-39625422207881285".to_string();
        assert_eq!(extended_gcd(a, b), (expected_a, expected_b));
    }

    #[test]
    fn x_large() {
        let a = "1873817317893712873298173982173982173897128738912738217371897381374891378943789";
        let b = "9188937128738173912371837981739817238917246812647812678394619836281693618963297";
        let expected_a =
            "-1486080468736810267748473400213603987572344688308283414544810257982300340964938"
                .to_string();
        let expected_b =
            "303043026531734365807887908508346161442903254640489390676789939813131430349539"
                .to_string();
        assert_eq!(extended_gcd(a, b), (expected_a, expected_b));
    }
}

// Ported from: http://www.maths.dk/teaching/courses/math398-spring2017/code/cryptomath.txt
pub fn mod_inverse(a: &str, m: &str) -> Option<String> {
    let gcd_num = StringToNumber!(gcd(&a, &m));
    if gcd_num != One::one() {
        return None;
    }

    let (u, _) = extended_gcd(&a, &m);
    let u_num = StringToNumber!(u);
    let m_num = StringToNumber!(m);
    Some(NumberToString!(u_num % m_num))
}

#[cfg(test)]
mod test_mod_inverse {
    use super::*;

    #[test]
    fn miniscule() {
        let a = "3";
        let m = "26";
        let expected = Some("9".to_string());
        assert_eq!(mod_inverse(a, m), expected);
    }

    #[test]
    fn tiny() {
        let a = "333";
        let m = "2613";
        let expected = None;
        assert_eq!(mod_inverse(a, m), expected);
    }

    #[test]
    fn small() {
        let a = "333213";
        let m = "261312334";
        let expected = Some("66480691".to_string());
        assert_eq!(mod_inverse(a, m), expected);
    }

    #[test]
    fn medium() {
        let a = "3332131312321";
        let m = "261312334131135465";
        let expected = Some("63643812378874741".to_string());
        assert_eq!(mod_inverse(a, m), expected);
    }

    #[test]
    fn large() {
        let a = "33321313123211923123812";
        let m = "261312334131135465912381278381238";
        let expected = None;
        assert_eq!(mod_inverse(a, m), expected);
    }

    #[test]
    fn x_large() {
        let a = "1873817317893712873298173982173982173897128738912738217371897381374891378";
        let m = "9188937128738173912371837981739817238917246812647812678394619836281693618963297";
        let expected = Some(
            "-996417904483222556354083958060155179719360472118047976841259037232297184027911"
                .to_string(),
        );
        assert_eq!(mod_inverse(a, m), expected);
    }
}

// Check out: https://rosettacode.org/wiki/Miller%E2%80%93Rabin_primality_test
pub fn miller_rabin(n: &str, seed: Vec<u8>) -> bool {
    let zero: BigInt = StringToNumber!("0");
    let one: BigInt = StringToNumber!("1");
    let two: BigInt = StringToNumber!("2");

    let n_num: BigInt = StringToNumber!(n);
    let n_minus_one = &n_num - &one;

    if n_num == two {
        return true;
    }

    if n_num < two || &n_num % &two == zero {
        return false;
    }

    let mut s: BigInt = zero.clone();
    let mut d: BigInt = &n_num - &one;

    while &d % &two == zero {
        s += &one;
        d /= &two;
    }

    let mut rng: StdRng = SeedableRng::from_seed(from_slice(&seed));
    
    // 50 here is a parameter for accuracy
    for _ in 0..50 {
        let a_num = rng.gen_bigint_range(&two, &n_minus_one);
        let a_str = NumberToString!(&a_num);

        let gcd_str = gcd(&a_str, &n);
        let gcd_num = StringToNumber!(gcd_str);

        if gcd_num != one {
            return false;
        }

        let mut x_num = a_num.modpow(&d, &n_num);

        if x_num == one || x_num == n_minus_one {
            continue;
        }

        let mut is_witness = true;
        let mut r = one.clone();

        while r < s && is_witness {
            x_num = x_num.modpow(&two, &n_num);

            if x_num == n_minus_one {
                is_witness = false;
            }

            r += &one;
        }

        if is_witness {
            return false;
        }
    }

    true
}

pub fn is_prime(n: &str, seed: Vec<u8>) -> bool {
    let zero = StringToNumber!("0");
    let one = StringToNumber!("1");
    let two = StringToNumber!("2");

    let n_num = StringToNumber!(n);

    let n_minus_one = &n_num - &one;

    if n_num < two {
        return false;
    }

    let small_primes_as_bigints: Vec<BigInt> = SMALL_PRIMES
        .iter()
        .map(|x| x.to_bigint().unwrap())
        .collect();
    let is_small_prime = small_primes_as_bigints.contains(&n_num);

    if is_small_prime {
        return true;
    }

    for prime in small_primes_as_bigints {
        if &n_num % &prime == zero {
            return false;
        }
    }

    let bases_as_bigints: Vec<BigInt> = BASES.iter().map(|x| x.to_bigint().unwrap()).collect();

    for base in &bases_as_bigints {
        if base.modpow(&n_minus_one, &n_num) != StringToNumber!("1") {
            return false;
        }
    }

    miller_rabin(n, seed)
}

#[cfg(test)]
mod test_is_prime_and_rabin_miller {
    use super::*;

    #[test]
    fn miniscule_prime() {
        let a = "3";
        let expected = true;
        assert_eq!(is_prime(a, test_seed()), expected);
    }

    #[test]
    fn miniscule_not_prime() {
        let a = "4";
        let expected = false;
        assert_eq!(is_prime(a, test_seed()), expected);
    }

    #[test]
    fn tiny_prime() {
        let a = "1049";
        let expected = true;
        assert_eq!(is_prime(a, test_seed()), expected);
    }

    #[test]
    fn tiny_not_prime() {
        let a = "1050";
        let expected = false;
        assert_eq!(is_prime(a, test_seed()), expected);
    }

    #[test]
    fn small_prime() {
        let a = "100103";
        let expected = true;
        assert_eq!(is_prime(a, test_seed()), expected);
    }

    #[test]
    fn small_not_prime() {
        let a = "100105";
        let expected = false;
        assert_eq!(is_prime(a, test_seed()), expected);
    }

    #[test]
    fn medium_prime() {
        let a = "100000015333";
        let expected = true;
        assert_eq!(is_prime(a, test_seed()), expected);
    }

    #[test]
    fn medium_not_prime() {
        let a = "100000015334";
        let expected = false;
        assert_eq!(is_prime(a, test_seed()), expected);
    }

    #[test]
    fn large_prime() {
        let a = "335184372088831";
        let expected = true;
        assert_eq!(is_prime(a, test_seed()), expected);
    }

    #[test]
    fn large_not_prime() {
        let a = "335184372088832";
        let expected = false;
        assert_eq!(is_prime(a, test_seed()), expected);
    }
}

#[wasm_bindgen]
pub fn generate_prime(bits: usize, tries: usize, seed: Vec<u8>) -> Option<String> {
    let bits_minus_one = bits - 1;
    let x = num::pow(StringToNumber!("2"), bits_minus_one);
    let y = StringToNumber!("2") * &x;

    let mut rng: StdRng = SeedableRng::from_seed(from_slice(&seed));

    for _ in 0..tries {
        let mut n = rng.gen_bigint_range(&x, &y);

        if &n % StringToNumber!("2") == StringToNumber!("0") {
            n += 1;
        }

        let num_str = &NumberToString!(&n);
        let q = is_prime(num_str, seed.clone());

        if q {
            return Some(NumberToString!(n));
        }
    }

    None
}

#[cfg(test)]
mod test_generate_prime {
    use super::*;

    #[test]
    fn miniscule_prime() {
        let prime = generate_prime(2, 1000, test_seed());
        assert_eq!(prime, Some("3".to_string()));
    }

    #[test]
    fn tiny_prime() {
        let prime = generate_prime(8, 1000, test_seed());
        assert_eq!(prime, Some("193".to_string()));
    }

    #[test]
    fn medium_prime() {
        let prime = generate_prime(64, 1000, test_seed());
        assert_eq!(prime, Some("10057321802802702503".to_string()));
    }

    #[test]
    fn large_prime() {
        let prime = generate_prime(256, 1000, test_seed());
        assert_eq!(prime, Some("91585194753718779240055081770127290880143452499556598946529982336565467053363".to_string()));
    }
}

// Ref: https://stackoverflow.com/questions/29570607/is-there-a-good-way-to-convert-a-vect-to-an-array
fn from_slice(bytes: &[u8]) -> [u8; 32] {
    let mut array = [0; 32];
    let bytes = &bytes[..array.len()]; // panics if not enough data
    array.copy_from_slice(bytes); 
    array
}

// Fixes: https://github.com/ColbyCypherSociety/ChatDemo/issues/21
// Ref: https://stackoverflow.com/questions/46378637/how-to-make-a-variable-with-a-scope-lifecycle-for-all-test-functions-in-a-rust-t
fn test_seed() -> Vec<u8> {
    vec![1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1]
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct Keydouble {
    v: String,
    n: String,
    isPrivate: bool,
}

#[wasm_bindgen]
impl Keydouble {
    pub fn new(v: String, n: String, isPrivate: bool) -> Keydouble {
        Keydouble {
            v,
            n,
            isPrivate,
        }
    }
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct Keypair {
    public: Keydouble,
    private: Keydouble,
}

#[wasm_bindgen]
impl Keypair {
    pub fn new(seed_one: Vec<u8>, seed_two: Vec<u8>) -> Keypair {
        let one = StringToNumber!("1");
        let two = StringToNumber!("2");

        // Hardcoded to 64-bits for now
        let q_str = generate_prime(8, 1000, seed_one.clone()).unwrap();
        let q_num = StringToNumber!(q_str);

        // Hardcoded to 64-bits for now
        let p_str = generate_prime(8, 1000, seed_two.clone()).unwrap();
        let p_num = StringToNumber!(p_str);

        let n_num = &p_num * &q_num;
        let n_str = NumberToString!(n_num);

        let p_minus_one_str = NumberToString!(&p_num - &one);
        let q_minus_one_str = NumberToString!(&q_num - &one);

        let phi_str = lcm(&p_minus_one_str, &q_minus_one_str);
        let phi_num = StringToNumber!(&phi_str);

        let mut e_found = false;

        let mut rng: StdRng = SeedableRng::from_seed(from_slice(&seed_one));

        let mut e_str = String::default();

        while !e_found {
            let e_num = rng.gen_bigint_range(&two, &(&phi_num - &two));

            e_str = NumberToString!(&e_num);
            if gcd(&e_str, &phi_str) == "1".to_string() {
                e_found = true;
            }
        }

        let mut d_str = mod_inverse(&e_str, &phi_str).unwrap();

        if StringToNumber!(d_str) < StringToNumber!("0") {
            let d_num = &n_num + StringToNumber!(d_str);
            d_str = NumberToString!(d_num);
        }

        Keypair {
            public: Keydouble::new(e_str, n_str.clone(), false),
            private: Keydouble::new(d_str, n_str.clone(), true),
        }
    }

    pub fn public_key_display_wasm(&self) -> String {
        format!("({}, {})", self.public.v, self.public.n)
    }
}


#[cfg(test)]
mod test_generate_key{
    use super::*;

    #[test]
    fn works_with_simple_encrypt_decrypt() {
        // You need two different seeds (p and q must be different)
        let seed_one = vec![10,16,51,42,123,31,212,31,233,15,9,7,41,32,4,3,144,122,1,35,1,13,55,23,1,33,1,1,1,1,2,1];
        let seed_two = test_seed();

        // Generate a keypair
        let k = Keypair::new(seed_one, seed_two);

        // Capture all the variables for encryption and decryption
        let e = StringToNumber!(k.public.v);
        let d = StringToNumber!(k.private.v);
        let n = StringToNumber!(k.public.n);

        // Message and ciphertext
        let plaintext = StringToNumber!("72");
        let ciphertext =  plaintext.modpow(&e, &n);
        
        let decrypted = ciphertext.modpow(&d, &n);
        assert_eq!(plaintext, decrypted);
    }
}
