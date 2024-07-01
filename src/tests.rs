#[allow(unused_imports)]
use crate::Number;

#[test]
fn optimise() {
    let num = Number {
        body: vec![0x00, 0x10, 0x20],
    };
    let num_optimised = Number {
        body: vec![0x10, 0x20],
    };
    assert_eq!(num.optimise(), num_optimised)
}

#[test]
fn from() {
    let num_i128 = 0x0100i128;
    let num_number = Number::from(num_i128);
    println!("{num_number}");
    assert_eq!(num_number.body, vec![0x01, 0x00]);
}

#[test]
fn compare() {
    let num_a = Number::from(100 as i128);
    let num_b = Number::from(200 as i128);
    assert_eq!(num_a < num_b, true);
    assert_eq!(num_a < num_a, false);
    assert_eq!(num_a > num_b, false);
    assert_eq!(num_a > num_a, false);
    assert_eq!(num_a <= num_b, true);
    assert_eq!(num_a <= num_a, true);
    assert_eq!(num_a >= num_b, false);
    assert_eq!(num_a >= num_a, true);
}

#[test]
fn operations() {
    // all_operations(0xdea120, 0xab0a24);
    // all_operations(0xffff, 0x02);
    all_operations(0x200, 0x20);
    // all_operations(0x20, 0x20);
    // all_operations(0x21, 0x20);
    // all_operations(0x20000, 0x20);
}

fn all_operations(a: impl Into<i128> + Clone, b: impl Into<i128> + Clone) {
    let a_i128: i128 = a.clone().into();
    let b_i128: i128 = b.clone().into();
    let a = Number::from(a);
    let b = Number::from(b);
    assert_eq!(a.clone() + b.clone(), (a_i128 + b_i128).into());
    assert_eq!(a.clone() - b.clone(), (a_i128 - b_i128).into());
    assert_eq!(a.clone() * b.clone(), (a_i128 * b_i128).into());
    assert_eq!(a.clone() / b.clone(), (a_i128 / b_i128).into());
    assert_eq!(a.clone() % b.clone(), (a_i128 % b_i128).into());
    assert_eq!(a.clone() ^ b.clone(), (a_i128.pow(b_i128 as u32)).into());
}

#[test]
fn bit_length() {
    assert_eq!(Number::from(1).num_bits(), 1);
    assert_eq!(Number::from(2).num_bits(), 2);
    assert_eq!(Number::from(256).num_bits(), 9);
    assert_eq!(Number::from(255).num_bits(), 8);
}

#[test]
fn from_str() {
    assert_eq!("abcd".parse::<Number>().unwrap(), Number::from(0xabcd));
    assert_eq!("abcde".parse::<Number>().unwrap(), Number::from(0xabcde));
}

#[test]
fn split_at() {
    let num = Number::from(0x_11_22_33_44_55i128);
    let (high, low) = num.split_at(3);
    assert_eq!(high, Number::from(0x_11_22));
    assert_eq!(low, Number::from(0x_33_44_55));
}

#[test]
fn rsa() {
    let public_key = "704ed906c08eba1777796e8548c7dca75623941837b86e05e5683cff8a0d17fd725f99b9132b1133c7aff55b5c0980573038240f72c1203956ada0d990b2868da2cc623cdec9faff59c91c2f7d1700198e50f2e65f5be63ccb5f7317f120c33873e433984ac7a3ee35268e442470dc8580c54bef3d4955d452accbd886ced921";
    let secret_key = "41c75cc3e0aa7802609ec605d8cffec41fe6acc9d670f6380bda96c79123042715033b7533ede8a06c57fa1c2d76ff2d91f77fdf7075ff1ab652f1c01c49041749be41242082d62315d0267530117a582a3d21ce16ff567a8ace9b86a35c4d949eb6f34bf1306036ff47d461dd292660c8d4030256cbb764006ec66c800b5f45";
    let public_key: Number = public_key.parse().unwrap();
    let secret_key: Number = secret_key.parse().unwrap();

    let public_exponent: Number = 0x_01_00_01.into();

    let message: Number = 0x7e19df.into();
    dbg!(&message);

    let encrypted = message
        .clone()
        .mod_exponentiation(public_exponent, public_key.clone());
    dbg!(&encrypted);

    let decrypted = encrypted
        .clone()
        .mod_exponentiation(dbg!(secret_key), dbg!(public_key));
    dbg!(&decrypted);

    assert_eq!(message, decrypted);
}
