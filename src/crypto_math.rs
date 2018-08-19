use num::bigint::BigInt;
use num_traits::{Zero, One};

macro_rules! Number {
    ($e: expr) => {
        BigInt::parse_bytes(String::from($e).as_bytes(), 10).unwrap();
    };
}

#[cfg(test)]
mod test_number_macro {
    use super::*;

    #[test]
    fn negative_small() {
        let a = Number!("-5");
        let b = BigInt::parse_bytes(b"-5", 10).unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn negative_large() {
        let a = Number!("-523892389328392");
        let b = BigInt::parse_bytes(b"-523892389328392", 10).unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn miniscule() {
        let a = Number!("0");
        let b = BigInt::parse_bytes(b"0", 10).unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn tiny() {
        let a = Number!("10");
        let b = BigInt::parse_bytes(b"10", 10).unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn small() {
        let a = Number!("123");
        let b = BigInt::parse_bytes(b"123", 10).unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn medium() {
        let a = Number!("123456789");
        let b = BigInt::parse_bytes(b"123456789", 10).unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn large() {
        let a = Number!("123456789123456789");
        let b = BigInt::parse_bytes(b"123456789123456789", 10).unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn x_large() {
        let a = Number!("123456789123456789123456789123456789123456789123456789");
        let b = BigInt::parse_bytes(
            b"123456789123456789123456789123456789123456789123456789",
            10,
        ).unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn xx_large() {
        let a = Number!("123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789");
        let b = BigInt::parse_bytes(b"123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789", 10).unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn xxx_large() {
        let a = Number!("123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789");
        let b = BigInt::parse_bytes(b"123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789", 10).unwrap();
        assert_eq!(a, b);
    }
}

// Ported from: http://www.maths.dk/teaching/courses/math398-spring2017/code/cryptomath.txt
pub fn gcd(a: BigInt, b: BigInt) -> BigInt {
    let mut x = a;
    let mut y = b;

    while y != Zero::zero() {
        let r = x % y.clone();
        x = y;
        y = r;
    }
    return x;
}

#[cfg(test)]
mod test_gcd {
    use super::*;

    #[test]
    fn miniscule() {
        let a = Number!("10");
        let b = Number!("5");
        let expected = Number!("5");
        assert_eq!(gcd(a, b), expected);
    }

    #[test]
    fn tiny() {
        let a = Number!("29943");
        let b = Number!("29738");
        let expected = Number!("1");
        assert_eq!(gcd(a, b), expected);
    }

    #[test]
    fn small() {
        let a = Number!("299429203");
        let b = Number!("827382738");
        let expected = Number!("1");
        assert_eq!(gcd(a, b), expected);
    }

    #[test]
    fn medium() {
        let a = Number!("1672976127961212891");
        let b = Number!("3378278237328723873");
        let expected = Number!("3");
        assert_eq!(gcd(a, b), expected);
    }

    #[test]
    fn large() {
        let a = Number!("16729761279612128911672976127961212891");
        let b = Number!("33782782373287238731672976127961212891");
        let expected = Number!("3");
        assert_eq!(gcd(a, b), expected);
    }

    #[test]
    fn x_large() {
        let a = Number!(
            "1873817317893712873298173982173982173897128738912738217371897381374891378943789"
        );
        let b = Number!(
            "9188937128738173912371837981739817238917246812647812678394619836281693618963297"
        );
        let expected = Number!("1");
        assert_eq!(gcd(a, b), expected);
    }

    #[test]
    fn xx_large() {
        let a = Number!("18273781798371987398173891273871293762178362308763217863871263826817067830612083612876307916239721638916398216398613892168903681293610639120368219732891372189361287361986371863218763017236270362896319038213");
        let b = Number!("82726226362376138712678923161327863279136912363261786391287273961273967239678123623623672369236872671268723672167267612727198623872637892632186267386219627823169783627819623761983627816378263178639821687326");
        let expected = Number!("1");
        assert_eq!(gcd(a, b), expected);
    }

    #[test]
    fn xxx_large() {
        let a = Number!("1827378179837198739817389127387129376217836230876321786387126382681706783061208361287630791623972163891639821639861389216890368129361063912036821973289137218936128736198637186321876301723627036289631903821318273781798371987398173891273871293762178362308763217863871263826817067830612083612876307916239721638916398216398613892168903681293610639120368219732891372189361287361986371863218763017236270362896319038213");
        let b = Number!("8272622636237613871267892316132786327913691236326178639128727396127396723967812362362367236923687267126872367216726761272719862387263789263218626738621962782316978362781962376198362781637826317863982168732618273781798371987398173891273871293762178362308763217863871263826817067830612083612876307916239721638916398216398613892168903681293610639120368219732891372189361287361986371863218763017236270362896319038213");
        let expected = Number!("1");
        assert_eq!(gcd(a, b), expected);
    }
}

// Based on pseudocode from: https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm
pub fn extended_gcd(a: BigInt, b: BigInt) -> (BigInt, BigInt) {
    let mut r = a;
    let mut old_r = b;

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

    return (old_t, old_s);
}

#[cfg(test)]
mod test_extended_gcd {
    use super::*;

    #[test]
    fn miniscule() {
        let a = Number!("12");
        let b = Number!("17");
        let expected_a = Number!("-7");
        let expected_b= Number!("5");
        assert_eq!(extended_gcd(a, b), (expected_a, expected_b));
    }

    #[test]
    fn tiny() {
        let a = Number!("180");
        let b = Number!("150");
        let expected_a = Number!("1");
        let expected_b= Number!("-1");
        assert_eq!(extended_gcd(a, b), (expected_a, expected_b));
    }

    #[test]
    fn small() {
        let a = Number!("39392");
        let b = Number!("9372");
        let expected_a = Number!("950");
        let expected_b= Number!("-3993");
        assert_eq!(extended_gcd(a, b), (expected_a, expected_b));
    }

    #[test]
    fn medium() {
        let a = Number!("29837362344");
        let b = Number!("20938934792");
        let expected_a = Number!("70028498");
        let expected_b= Number!("-99788537");
        assert_eq!(extended_gcd(a, b), (expected_a, expected_b));
    }

    #[test]
    fn large() {
        let a = Number!("298373623442234243");
        let b = Number!("224224424293284938");
        let expected_a = Number!("29778059398942417");
        let expected_b= Number!("-39625422207881285");
        assert_eq!(extended_gcd(a, b), (expected_a, expected_b));
    }

    #[test]
    fn x_large() {
        let a = Number!("1873817317893712873298173982173982173897128738912738217371897381374891378943789");
        let b = Number!("9188937128738173912371837981739817238917246812647812678394619836281693618963297");
        let expected_a = Number!("-1486080468736810267748473400213603987572344688308283414544810257982300340964938");
        let expected_b= Number!("303043026531734365807887908508346161442903254640489390676789939813131430349539");
        assert_eq!(extended_gcd(a, b), (expected_a, expected_b));
    }
}
