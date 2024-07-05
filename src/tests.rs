#[allow(unused_imports)]
use crate::Number;

#[test]
fn from() {
    let num_u64 = 0x0100u64;
    let num_number: Number<2> = Number::from(num_u64);
    println!("{num_number}");
    assert_eq!(num_number.body, [0, 0x100]);
}

#[test]
fn compare() {
    let num_a: Number<1> = Number::from(100 as u64);
    let num_b: Number<1> = Number::from(200 as u64);
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
    all_operations(0xdea120u64, 0xab0a24u64);
    all_operations(0xffffu64, 0x02u64);
    all_operations(0x200u64, 0x20u64);
    all_operations(0x20u64, 0x20u64);
    all_operations(0x21u64, 0x20u64);
    all_operations(0x20000u64, 0x20u64);
}

fn all_operations(a: impl Into<u64> + Clone, b: impl Into<u64> + Clone) {
    let a_i128: u64 = a.clone().into();
    let b_i128: u64 = b.clone().into();
    let a: Number<32> = Number::from(a_i128 as u64);
    let b: Number<32> = Number::from(b_i128 as u64);
    assert_eq!(a.clone() + b.clone(), (a_i128 + b_i128).into());
    assert_eq!(a.clone() - b.clone(), (a_i128 - b_i128).into());
    assert_eq!(a.clone() * b.clone(), (a_i128 * b_i128).into());
    assert_eq!(a.clone() / b.clone(), (a_i128 / b_i128).into());
    assert_eq!(a.clone() % b.clone(), (a_i128 % b_i128).into());
}

#[test]
fn from_str() {
    assert_eq!(
        "abcd".parse::<Number<2>>().unwrap(),
        Number::from(0xabcdu64)
    );
    assert_eq!(
        "abcde".parse::<Number<2>>().unwrap(),
        Number::from(0xabcdeu64)
    );
}

#[test]
fn mod_exponentiation() {
    let a: Number<64> = "787878".parse().unwrap();
    let b: Number<64> = "10".parse().unwrap();
    let m: Number<64> = "704ed906c08eba3fa2".parse().unwrap();

    let expected_result = "53324dc9600ec1dcd4".parse().unwrap();

    assert_eq!(a.mod_exponentiation(b, m), expected_result);
}

#[test]
fn rsa() {
    let public_key = "704ed906c08eba1777796e8548c7dca75623941837b86e05e5683cff8a0d17fd725f99b9132b1133c7aff55b5c0980573038240f72c1203956ada0d990b2868da2cc623cdec9faff59c91c2f7d1700198e50f2e65f5be63ccb5f7317f120c33873e433984ac7a3ee35268e442470dc8580c54bef3d4955d452accbd886ced921";
    let secret_key = "41c75cc3e0aa7802609ec605d8cffec41fe6acc9d670f6380bda96c79123042715033b7533ede8a06c57fa1c2d76ff2d91f77fdf7075ff1ab652f1c01c49041749be41242082d62315d0267530117a582a3d21ce16ff567a8ace9b86a35c4d949eb6f34bf1306036ff47d461dd292660c8d4030256cbb764006ec66c800b5f45";
    let public_key: Number<64> = public_key.parse().unwrap();
    let secret_key: Number<64> = secret_key.parse().unwrap();

    let public_exponent: Number<64> = 0x_01_00_01u64.into();

    let message: Number<64> = 0x7e19dfu64.into();
    dbg!(&message);

    let encrypted = message
        .clone()
        .mod_exponentiation(dbg!(public_exponent), public_key.clone());
    assert_eq!(encrypted, "6c565e45d0448b0a0acd2d2b165b09837da3fe29ad3b91ceceb046004372635aef1f8c5763688853f89829f4467bec34d9e399faa3e6763d8c5467f146764351ff44fcf2b3cd2b53d64189d2823a965e2b73409499a91034f45a2fe28641f94bfb8d3b17d30a18d25d22361f53584dbbd5207ba649f85af6e22538c336215cdc".parse().unwrap());
    dbg!(&encrypted);

    let decrypted = encrypted
        .clone()
        .mod_exponentiation(dbg!(secret_key), dbg!(public_key));
    dbg!(&decrypted);

    assert_eq!(message, decrypted);
}
