use yali::*;

fn main() {
    let public_key = "704ed906c08eba1777796e8548c7dca75623941837b86e05e5683cff8a0d17fd725f99b9132b1133c7aff55b5c0980573038240f72c1203956ada0d990b2868da2cc623cdec9faff59c91c2f7d1700198e50f2e65f5be63ccb5f7317f120c33873e433984ac7a3ee35268e442470dc8580c54bef3d4955d452accbd886ced921";
    let secret_key = "41c75cc3e0aa7802609ec605d8cffec41fe6acc9d670f6380bda96c79123042715033b7533ede8a06c57fa1c2d76ff2d91f77fdf7075ff1ab652f1c01c49041749be41242082d62315d0267530117a582a3d21ce16ff567a8ace9b86a35c4d949eb6f34bf1306036ff47d461dd292660c8d4030256cbb764006ec66c800b5f45";
    let public_key: Number = public_key.parse().unwrap();
    let secret_key: Number = secret_key.parse().unwrap();

    let public_exponent: Number = 0x_01_00_01.into();

    let message: Number = 0x7e19df.into();

    let encrypted = message.mod_exponentiation(public_exponent, public_key.clone());
    dbg!(&encrypted);

    let decrypted = dbg!(encrypted.clone()).mod_exponentiation(dbg!(secret_key), dbg!(public_key));
    dbg!(&decrypted);
}
