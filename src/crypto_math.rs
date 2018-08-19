use num::bigint::BigInt;
use num_traits::{Zero, One};
use wasm_bindgen::prelude::*;

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
        let r = x % y.clone();
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

    return (NumberToString!(old_t), NumberToString!(old_s));
}

#[cfg(test)]
mod test_extended_gcd {
    use super::*;

    #[test]
    fn miniscule() {
        let a = "12";
        let b = "17";
        let expected_a = "-7".to_string();
        let expected_b= "5".to_string();
        assert_eq!(extended_gcd(a, b), (expected_a, expected_b));
    }

    #[test]
    fn tiny() {
        let a = "180";
        let b = "150";
        let expected_a = "1".to_string();
        let expected_b= "-1".to_string();
        assert_eq!(extended_gcd(a, b), (expected_a, expected_b));
    }

    #[test]
    fn small() {
        let a = "39392";
        let b = "9372";
        let expected_a = "950".to_string();
        let expected_b="-3993".to_string();
        assert_eq!(extended_gcd(a, b), (expected_a, expected_b));
    }

    #[test]
    fn medium() {
        let a = "29837362344";
        let b = "20938934792";
        let expected_a = "70028498".to_string();
        let expected_b= "-99788537".to_string();
        assert_eq!(extended_gcd(a, b), (expected_a, expected_b));
    }

    #[test]
    fn large() {
        let a = "298373623442234243";
        let b = "224224424293284938";
        let expected_a = "29778059398942417".to_string();
        let expected_b= "-39625422207881285".to_string();
        assert_eq!(extended_gcd(a, b), (expected_a, expected_b));
    }

    #[test]
    fn x_large() {
        let a = "1873817317893712873298173982173982173897128738912738217371897381374891378943789";
        let b = "9188937128738173912371837981739817238917246812647812678394619836281693618963297";
        let expected_a = "-1486080468736810267748473400213603987572344688308283414544810257982300340964938".to_string();
        let expected_b= "303043026531734365807887908508346161442903254640489390676789939813131430349539".to_string();
        assert_eq!(extended_gcd(a, b), (expected_a, expected_b));
    }
}

// Ported from: http://www.maths.dk/teaching/courses/math398-spring2017/code/cryptomath.txt
pub fn mod_inverse(a: &str, m: &str) -> Option<String> {
    let gcd_num = StringToNumber!(gcd(a.clone(),m.clone()));
    if gcd_num != One::one() {
        return None
    }

    let (u, _) = extended_gcd(a.clone(), m.clone());
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
        let expected = Some("-996417904483222556354083958060155179719360472118047976841259037232297184027911".to_string());
        assert_eq!(mod_inverse(a, m), expected);
    }
}
