use yali::*;

fn main() {
    let public_key = "704ed906c08eba1777796e8548c7dca75623941837b86e05e5683cff8a0d17fd725f99b9132b1133c7aff55b5c0980573038240f72c1203956ada0d990b2868da2cc623cdec9faff59c91c2f7d1700198e50f2e65f5be63ccb5f7317f120c33873e433984ac7a3ee35268e442470dc8580c54bef3d4955d452accbd886ced921";
    let secret_key = "41c75cc3e0aa7802609ec605d8cffec41fe6acc9d670f6380bda96c79123042715033b7533ede8a06c57fa1c2d76ff2d91f77fdf7075ff1ab652f1c01c49041749be41242082d62315d0267530117a582a3d21ce16ff567a8ace9b86a35c4d949eb6f34bf1306036ff47d461dd292660c8d4030256cbb764006ec66c800b5f45";
    let public_key: Number<32> = public_key.parse().unwrap();
    let secret_key: Number<32> = secret_key.parse().unwrap();

    let public_exponent: Number<32> = 0x_01_00_01u64.into();

    let message: Number<32> = 0x7e19dfu64.into();
    dbg!(&message);

    let encrypted = message.mod_exponentiation(dbg!(public_exponent), public_key);
    assert_eq!(encrypted, "6c565e45d0448b0a0acd2d2b165b09837da3fe29ad3b91ceceb046004372635aef1f8c5763688853f89829f4467bec34d9e399faa3e6763d8c5467f146764351ff44fcf2b3cd2b53d64189d2823a965e2b73409499a91034f45a2fe28641f94bfb8d3b17d30a18d25d22361f53584dbbd5207ba649f85af6e22538c336215cdc".parse().unwrap());
    dbg!(&encrypted);

    let decrypted = encrypted.mod_exponentiation(dbg!(secret_key), dbg!(public_key));
    dbg!(&decrypted);

    assert_eq!(message, decrypted);
}
