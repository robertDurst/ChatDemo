use num::{
    bigint::{BigInt, RandBigInt, ToBigInt},
    pow,
};
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

// Why lazy_static you may ask? Well, for one, try to compile this without lazy_static. You will
// get an error saying statics can't be the result of an executed function. So, as per the crate
// docs, with lazy_static we get Using this macro, it is possible to have statics that require
// code to be executed at runtime in order to be initialized.
lazy_static! {
    static ref ZERO: BigInt = string_to_number("0");
    static ref ONE: BigInt = string_to_number("1");
    static ref TWO: BigInt = string_to_number("2");
}

/// a helper function for going from a BigInt struct to a String.
pub fn number_to_string(num: &BigInt) -> String {
    format!("{}", num)
}

#[cfg(test)]
mod test_number_to_string {
    use super::*;

    #[test]
    fn negative_small() {
        let num = BigInt::parse_bytes(b"-5", 10).unwrap();
        let a = number_to_string(&num);
        let b = "-5".to_string();
        assert_eq!(a, b);
    }

    #[test]
    fn negative_large() {
        let num = BigInt::parse_bytes(b"-523892389328392", 10).unwrap();
        let a = number_to_string(&num);
        let b = "-523892389328392".to_string();
        assert_eq!(a, b);
    }

    #[test]
    fn miniscule() {
        let num = BigInt::parse_bytes(b"0", 10).unwrap();
        let a = number_to_string(&num);
        let b = "0";
        assert_eq!(a, b);
    }

    #[test]
    fn tiny() {
        let num = BigInt::parse_bytes(b"10", 10).unwrap();
        let a = number_to_string(&num);
        let b = "10".to_string();
        assert_eq!(a, b);
    }

    #[test]
    fn small() {
        let num = BigInt::parse_bytes(b"123", 10).unwrap();
        let a = number_to_string(&num);
        let b = "123".to_string();
        assert_eq!(a, b);
    }

    #[test]
    fn medium() {
        let num = BigInt::parse_bytes(b"123456789", 10).unwrap();
        let a = number_to_string(&num);
        let b = "123456789".to_string();
        assert_eq!(a, b);
    }

    #[test]
    fn large() {
        let num = BigInt::parse_bytes(b"123456789123456789", 10).unwrap();
        let a = number_to_string(&num);
        let b = "123456789123456789".to_string();
        assert_eq!(a, b);
    }

    #[test]
    fn x_large() {
        let num = BigInt::parse_bytes(
            b"123456789123456789123456789123456789123456789123456789",
            10,
        ).unwrap();
        let a = number_to_string(&num);
        let b = "123456789123456789123456789123456789123456789123456789".to_string();
        assert_eq!(a, b);
    }

    #[test]
    fn xx_large() {
        let num = BigInt::parse_bytes(b"123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789", 10).unwrap();
        let a = number_to_string(&num);
        let b = "123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789".to_string();
        assert_eq!(a, b);
    }

    #[test]
    fn xxx_large() {
        let num = BigInt::parse_bytes(b"123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789", 10).unwrap();
        let a = number_to_string(&num);
        let b = "123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789".to_string();
        assert_eq!(a, b);
    }
}

/// a helper function for going from a String to a BigInt struct.
pub fn string_to_number(s: &str) -> BigInt {
    BigInt::parse_bytes(s.as_bytes(), 10).unwrap()
}

#[cfg(test)]
mod test_string_to_number_macro {
    use super::*;

    #[test]
    fn negative_small() {
        let a = string_to_number("-5");
        let b = BigInt::parse_bytes(b"-5", 10).unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn negative_large() {
        let a = string_to_number("-523892389328392");
        let b = BigInt::parse_bytes(b"-523892389328392", 10).unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn miniscule() {
        let a = string_to_number("0");
        let b = BigInt::parse_bytes(b"0", 10).unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn tiny() {
        let a = string_to_number("10");
        let b = BigInt::parse_bytes(b"10", 10).unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn small() {
        let a = string_to_number("123");
        let b = BigInt::parse_bytes(b"123", 10).unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn medium() {
        let a = string_to_number("123456789");
        let b = BigInt::parse_bytes(b"123456789", 10).unwrap();
        assert_eq!(a, b);
    }
    #[test]
    fn large() {
        let a = string_to_number("123456789123456789");
        let b = BigInt::parse_bytes(b"123456789123456789", 10).unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn x_large() {
        let a = string_to_number("123456789123456789123456789123456789123456789123456789");
        let b = BigInt::parse_bytes(
            b"123456789123456789123456789123456789123456789123456789",
            10,
        ).unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn xx_large() {
        let a = string_to_number("123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789");
        let b = BigInt::parse_bytes(b"123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789", 10).unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn xxx_large() {
        let a = string_to_number("123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789");
        let b = BigInt::parse_bytes(b"123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789", 10).unwrap();
        assert_eq!(a, b);
    }
}

/// calculates and returns the greatest common denominator of two BigInt's.
///
/// ## Reference
/// Ported from: [http://www.maths.dk/teaching/courses/math398-spring2017/code/cryptomath.txt](http://www.maths.dk/teaching/courses/math398-spring2017/code/cryptomath.txt)
pub fn gcd<'a>(a: &'a BigInt, b: &'a BigInt) -> BigInt {
    let mut x = a.clone();
    let mut y = b.clone();

    while y != *ZERO {
        let remainder = &x % &y;
        x = y;
        y = remainder;
    }

    x
}

#[cfg(test)]
mod test_gcd {
    use super::*;

    #[test]
    fn miniscule() {
        let a = &string_to_number("10");
        let b = &string_to_number("5");
        let expected = string_to_number("5");
        assert_eq!(gcd(a, b), expected);
    }

    #[test]
    fn tiny() {
        let a = &string_to_number("29943");
        let b = &string_to_number("29738");
        let expected = string_to_number("1");
        assert_eq!(gcd(a, b), expected);
    }

    #[test]
    fn small() {
        let a = &string_to_number("299429203");
        let b = &string_to_number("827382738");
        let expected = string_to_number("1");
        assert_eq!(gcd(a, b), expected);
    }

    #[test]
    fn medium() {
        let a = &string_to_number("1672976127961212891");
        let b = &string_to_number("3378278237328723873");
        let expected = string_to_number("3");
        assert_eq!(gcd(a, b), expected);
    }

    #[test]
    fn large() {
        let a = &string_to_number("16729761279612128911672976127961212891");
        let b = &string_to_number("33782782373287238731672976127961212891");
        let expected = string_to_number("3");
        assert_eq!(gcd(a, b), expected);
    }

    #[test]
    fn x_large() {
        let a = &string_to_number(
            "1873817317893712873298173982173982173897128738912738217371897381374891378943789",
        );
        let b = &string_to_number(
            "9188937128738173912371837981739817238917246812647812678394619836281693618963297",
        );
        let expected = string_to_number("1");
        assert_eq!(gcd(a, b), expected);
    }

    #[test]
    fn xx_large() {
        let a = &string_to_number("18273781798371987398173891273871293762178362308763217863871263826817067830612083612876307916239721638916398216398613892168903681293610639120368219732891372189361287361986371863218763017236270362896319038213");
        let b = &string_to_number("82726226362376138712678923161327863279136912363261786391287273961273967239678123623623672369236872671268723672167267612727198623872637892632186267386219627823169783627819623761983627816378263178639821687326");
        let expected = string_to_number("1");
        assert_eq!(gcd(a, b), expected);
    }

    #[test]
    fn xxx_large() {
        let a = &string_to_number("1827378179837198739817389127387129376217836230876321786387126382681706783061208361287630791623972163891639821639861389216890368129361063912036821973289137218936128736198637186321876301723627036289631903821318273781798371987398173891273871293762178362308763217863871263826817067830612083612876307916239721638916398216398613892168903681293610639120368219732891372189361287361986371863218763017236270362896319038213");
        let b = &string_to_number("8272622636237613871267892316132786327913691236326178639128727396127396723967812362362367236923687267126872367216726761272719862387263789263218626738621962782316978362781962376198362781637826317863982168732618273781798371987398173891273871293762178362308763217863871263826817067830612083612876307916239721638916398216398613892168903681293610639120368219732891372189361287361986371863218763017236270362896319038213");
        let expected = string_to_number("1");
        assert_eq!(gcd(a, b), expected);
    }
}

/// calculates and returns the lowest common multiple of two ints.
///
/// ## Reference
/// Ref: [https://www.geeksforgeeks.org/program-to-find-lcm-of-two-numbers/](https://www.geeksforgeeks.org/program-to-find-lcm-of-two-numbers/)
pub fn lcm<'a>(a: &'a BigInt, b: &'a BigInt) -> BigInt {
    let numerator = a * b;
    let denominator = gcd(a, b);

    numerator / denominator
}

#[cfg(test)]
mod test_lcm {
    use super::*;

    #[test]
    fn miniscule() {
        let a = &string_to_number("5");
        let b = &string_to_number("2");
        let expected = string_to_number("10");
        assert_eq!(lcm(a, b), expected);
    }

    #[test]
    fn tiny() {
        let a = &string_to_number("15");
        let b = &string_to_number("20");
        let expected = string_to_number("60");
        assert_eq!(lcm(a, b), expected);
    }

    #[test]
    fn small() {
        let a = &string_to_number("299429203");
        let b = &string_to_number("827382738");
        let expected = string_to_number("247742553815297814");
        assert_eq!(lcm(a, b), expected);
    }

    #[test]
    fn medium() {
        let a = &string_to_number("1672976127961212891");
        let b = &string_to_number("3378278237328723873");
        let expected = string_to_number("1883926281553946627336368887435682281");
        assert_eq!(lcm(a, b), expected);
    }
}

/// an extension of the gcd function, the extended euclidean algorithm derives the Bézout coefficients for
/// two BigInt's.
///
/// ## Reference
/// Based on pseudocode from: [https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm](https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm).
pub fn extended_gcd(a: &BigInt, b: &BigInt) -> (BigInt, BigInt) {
    let mut a_num = a.clone();
    let mut b_num = b.clone();

    let mut old_s: BigInt = ONE.clone();
    let mut s: BigInt = ZERO.clone();

    let mut old_t: BigInt = ZERO.clone();
    let mut t: BigInt = ONE.clone();

    while a_num != *ZERO {
        let quotient = &b_num / &a_num;

        let temp_r = a_num.clone();
        a_num = &b_num - &quotient * &a_num;
        b_num = temp_r;

        let temp_s = s.clone();
        s = old_s - &quotient * s;
        old_s = temp_s;

        let temp_t = t.clone();
        t = old_t - &quotient * t;
        old_t = temp_t;
    }

    (old_t, old_s)
}

#[cfg(test)]
mod test_extended_gcd {
    use super::*;

    #[test]
    fn miniscule() {
        let a = &string_to_number("12");
        let b = &string_to_number("17");
        let expected_a = string_to_number("-7");
        let expected_b = string_to_number("5");
        assert_eq!(extended_gcd(a, b), (expected_a, expected_b));
    }

    #[test]
    fn tiny() {
        let a = &string_to_number("180");
        let b = &string_to_number("150");
        let expected_a = string_to_number("1");
        let expected_b = string_to_number("-1");
        assert_eq!(extended_gcd(a, b), (expected_a, expected_b));
    }

    #[test]
    fn small() {
        let a = &string_to_number("39392");
        let b = &string_to_number("9372");
        let expected_a = string_to_number("950");
        let expected_b = string_to_number("-3993");
        assert_eq!(extended_gcd(a, b), (expected_a, expected_b));
    }

    #[test]
    fn medium() {
        let a = &string_to_number("29837362344");
        let b = &string_to_number("20938934792");
        let expected_a = string_to_number("70028498");
        let expected_b = string_to_number("-99788537");
        assert_eq!(extended_gcd(a, b), (expected_a, expected_b));
    }

    #[test]
    fn large() {
        let a = &string_to_number("298373623442234243");
        let b = &string_to_number("224224424293284938");
        let expected_a = string_to_number("29778059398942417");
        let expected_b = string_to_number("-39625422207881285");
        assert_eq!(extended_gcd(a, b), (expected_a, expected_b));
    }

    #[test]
    fn x_large() {
        let a = &string_to_number(
            "1873817317893712873298173982173982173897128738912738217371897381374891378943789",
        );
        let b = &string_to_number(
            "9188937128738173912371837981739817238917246812647812678394619836281693618963297",
        );
        let expected_a = string_to_number(
            "-1486080468736810267748473400213603987572344688308283414544810257982300340964938",
        );
        let expected_b = string_to_number(
            "303043026531734365807887908508346161442903254640489390676789939813131430349539",
        );
        assert_eq!(extended_gcd(a, b), (expected_a, expected_b));
    }
}

/// returns the modular inverse of a BigInt modulo n (another BigInt).
///
/// ## Reference
/// Ported from: [http://www.maths.dk/teaching/courses/math398-spring2017/code/cryptomath.txt](http://www.maths.dk/teaching/courses/math398-spring2017/code/cryptomath.txt)
pub fn mod_inverse(a: &BigInt, m: &BigInt) -> Option<BigInt> {
    let gcd_num = &gcd(a, m);
    if gcd_num != &*ONE {
        return None;
    }

    let (u, _) = extended_gcd(a, m);

    Some(u % m)
}

#[cfg(test)]
mod test_mod_inverse {
    use super::*;

    #[test]
    fn miniscule() {
        let a = &string_to_number("3");
        let m = &string_to_number("26");
        let expected = Some(string_to_number("9"));
        assert_eq!(mod_inverse(a, m), expected);
    }

    #[test]
    fn tiny() {
        let a = &string_to_number("333");
        let m = &string_to_number("2613");
        let expected = None;
        assert_eq!(mod_inverse(a, m), expected);
    }

    #[test]
    fn small() {
        let a = &string_to_number("333213");
        let m = &string_to_number("261312334");
        let expected = Some(string_to_number("66480691"));
        assert_eq!(mod_inverse(a, m), expected);
    }

    #[test]
    fn medium() {
        let a = &string_to_number("3332131312321");
        let m = &string_to_number("261312334131135465");
        let expected = Some(string_to_number("63643812378874741"));
        assert_eq!(mod_inverse(a, m), expected);
    }

    #[test]
    fn large() {
        let a = &string_to_number("33321313123211923123812");
        let m = &string_to_number("261312334131135465912381278381238");
        let expected = None;
        assert_eq!(mod_inverse(a, m), expected);
    }

    #[test]
    fn x_large() {
        let a = &string_to_number(
            "1873817317893712873298173982173982173897128738912738217371897381374891378",
        );
        let m = &string_to_number(
            "9188937128738173912371837981739817238917246812647812678394619836281693618963297",
        );
        let expected = Some(string_to_number(
            "-996417904483222556354083958060155179719360472118047976841259037232297184027911",
        ));
        assert_eq!(mod_inverse(a, m), expected);
    }
}

/// probabilistically deduces whether a given number is prime.
///
/// ## Reference
/// Check out: [https://rosettacode.org/wiki/Miller%E2%80%93Rabin_primality_test](https://rosettacode.org/wiki/Miller%E2%80%93Rabin_primality_test)
pub fn miller_rabin(n: &BigInt, seed: &[u8]) -> bool {
    let n_minus_one = n - &*ONE;

    if n == &*TWO {
        return true;
    }

    if *n < *TWO || n % &*TWO == *ZERO {
        return false;
    }

    let mut s: BigInt = ZERO.clone();
    let mut d: BigInt = n - &*ONE;

    while &d % &*TWO == *ZERO {
        s += &*ONE;
        d /= &*TWO;
    }

    let mut rng: StdRng = SeedableRng::from_seed(from_slice(&seed));

    // 50 here is a parameter for accuracy
    for _ in 0..50 {
        let a_num = rng.gen_bigint_range(&*TWO, &n_minus_one);

        let gcd_num = gcd(&a_num, n);

        if gcd_num != *ONE {
            return false;
        }

        let mut x_num = a_num.modpow(&d, n);

        if x_num == *ONE || x_num == n_minus_one {
            continue;
        }

        let mut is_witness = true;
        let mut r = ONE.clone();

        while r < s && is_witness {
            x_num = x_num.modpow(&*TWO, n);

            if x_num == n_minus_one {
                is_witness = false;
            }

            r += &*ONE;
        }

        if is_witness {
            return false;
        }
    }

    true
}

/// determines whether a given number is prime by first running some simple primality tests through a small set of
/// known primes and bases, and then, if all these basic tests pass, returns the result of the Miller-Rabin test.
///
/// ## Reference
/// Ported from: [http://www.maths.dk/teaching/courses/math398-spring2017/code/cryptomath.txt](http://www.maths.dk/teaching/courses/math398-spring2017/code/cryptomath.txt)
pub fn is_prime(n: &BigInt, seed: &[u8]) -> bool {
    let n_minus_one = n - &*ONE;

    if *n < *TWO {
        return false;
    }

    let small_primes_as_bigints: Vec<BigInt> = SMALL_PRIMES
        .iter()
        .map(|x| x.to_bigint().unwrap())
        .collect();
    let is_small_prime = small_primes_as_bigints.contains(n);

    if is_small_prime {
        return true;
    }

    for prime in small_primes_as_bigints {
        if n % &prime == *ZERO {
            return false;
        }
    }

    let bases_as_bigints: Vec<BigInt> = BASES.iter().map(|x| x.to_bigint().unwrap()).collect();

    for base in &bases_as_bigints {
        if base.modpow(&n_minus_one, n) != *ONE {
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
        let a = &string_to_number("3");
        let expected = true;
        assert_eq!(is_prime(a, test_seed()), expected);
    }

    #[test]
    fn miniscule_not_prime() {
        let a = &string_to_number("4");
        let expected = false;
        assert_eq!(is_prime(a, test_seed()), expected);
    }

    #[test]
    fn tiny_prime() {
        let a = &string_to_number("1049");
        let expected = true;
        assert_eq!(is_prime(a, test_seed()), expected);
    }

    #[test]
    fn tiny_not_prime() {
        let a = &string_to_number("1050");
        let expected = false;
        assert_eq!(is_prime(a, test_seed()), expected);
    }

    #[test]
    fn small_prime() {
        let a = &string_to_number("100103");
        let expected = true;
        assert_eq!(is_prime(a, test_seed()), expected);
    }

    #[test]
    fn small_not_prime() {
        let a = &string_to_number("100105");
        let expected = false;
        assert_eq!(is_prime(a, test_seed()), expected);
    }

    #[test]
    fn medium_prime() {
        let a = &string_to_number("100000015333");
        let expected = true;
        assert_eq!(is_prime(a, test_seed()), expected);
    }

    #[test]
    fn medium_not_prime() {
        let a = &string_to_number("100000015334");
        let expected = false;
        assert_eq!(is_prime(a, test_seed()), expected);
    }

    #[test]
    fn large_prime() {
        let a = &string_to_number("335184372088831");
        let expected = true;
        assert_eq!(is_prime(a, test_seed()), expected);
    }

    #[test]
    fn large_not_prime() {
        let a = &string_to_number("335184372088832");
        let expected = false;
        assert_eq!(is_prime(a, test_seed()), expected);
    }
}

/// generates a random prime for a given seed.
///
/// ## Reference
/// Ported from: [http://www.maths.dk/teaching/courses/math398-spring2017/code/cryptomath.txt](http://www.maths.dk/teaching/courses/math398-spring2017/code/cryptomath.txt)
pub fn generate_prime(bits: usize, tries: usize, seed: &[u8]) -> Option<BigInt> {
    let bits_minus_one = bits - 1;
    let x = pow(TWO.clone(), bits_minus_one);
    let y = &*TWO * &x;

    let mut rng: StdRng = SeedableRng::from_seed(from_slice(&seed));

    for _ in 0..tries {
        let mut n = rng.gen_bigint_range(&x, &y);

        if &n % &*TWO == *ZERO {
            n += 1;
        }

        let q = is_prime(&n, seed);

        if q {
            return Some(n);
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
        assert_eq!(prime, Some(string_to_number("3")));
    }

    #[test]
    fn tiny_prime() {
        let prime = generate_prime(8, 1000, test_seed());
        assert_eq!(prime, Some(string_to_number("193")));
    }

    #[test]
    fn medium_prime() {
        let prime = generate_prime(64, 1000, test_seed());
        assert_eq!(prime, Some(string_to_number("10057321802802702503")));
    }

    #[test]
    fn large_prime() {
        let prime = generate_prime(256, 1000, test_seed());
        assert_eq!(
            prime,
            Some(string_to_number(
                "91585194753718779240055081770127290880143452499556598946529982336565467053363"
            ))
        );
    }
}

/// converts a vector of bytes to an array of u8's.
///
/// ## Reference
/// Ref: https://stackoverflow.com/questions/29570607/is-there-a-good-way-to-convert-a-vect-to-an-array
fn from_slice(bytes: &[u8]) -> [u8; 32] {
    let mut array = [0; 32];
    let bytes = &bytes[..array.len()]; // panics if not enough data
    array.copy_from_slice(bytes);
    array
}

// Fixes: https://github.com/ColbyCypherSociety/ChatDemo/issues/21
// Ref: https://stackoverflow.com/questions/46378637/how-to-make-a-variable-with-a-scope-lifecycle-for-all-test-functions-in-a-rust-t
#[allow(dead_code)]
pub fn test_seed<'a>() -> &'a [u8] {
    &[
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1,
    ]
}
