use std::{str::FromStr, time::Duration};

use criterion::{criterion_group, criterion_main, Criterion};
use rand::Rng;
use yet_another_largeint::*;

fn operations_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("decryption");

    group.sample_size(10);

    group.bench_function(&format!("Decryption with 32 bit"), |b| {
        b.iter(|| {
            Number::from_str("3e7e499b").unwrap().mod_exponentiation(
                Number::from_str("bfdfff59").unwrap(),
                Number::from_str("e498ae9f").unwrap(),
            )
        })
    });
    group.bench_function(&format!("Decryption with 128 bit"), |b| {
        b.iter(|| {
            Number::from_str("8962971de09fdb99b34aa8975a0d08f2")
                .unwrap()
                .mod_exponentiation(
                    Number::from_str("6bac928a1b5c8fd237a24a8c4ed1666d").unwrap(),
                    Number::from_str("9da6c7db9d0c926dd750e95685ebae4c").unwrap(),
                )
        })
    });
    group.bench_function(&format!("Decryption with 256 bit"), |b| {
        b.iter(|| {
            Number::from_str("74b9aa8c2208cb33f38c26df5aab6740badb26f23ca3630b4ee864f5802ff658")
                .unwrap()
                .mod_exponentiation(
                    Number::from_str(
                        "a8d5146dfe683f1052a716fd8a48368b9b64c9cba06c553307a5ea4afdd3f3d9",
                    )
                    .unwrap(),
                    Number::from_str(
                        "b2251ff38c3d8049eedc1845591270890550e91a6421b5159ab134eb6bf5be93",
                    )
                    .unwrap(),
                )
        })
    });
    group.bench_function(&format!("Decryption with 1024 bit"), |b| {
        b.iter(|| {
            Number::from_str("5b6e20f60e63bd4c678ea65bdfef7953ecd1735657325fd31ae229d1ea272f6c35a0ee0b2d10f7748ef150bfe957278953acab4dc03557a452a7079947f21bbf0441fb46455f53e508c5a4322a2ebc35fc4e18f36bfee093a8d918132173e6b60ed582af9356382424aa7deed97d2efc3ba3944907e3299a4fbeee9b095f974b").unwrap().mod_exponentiation(
                Number::from_str("cc3c668290dfecc01967bbd69192ef86a6afc6336c6dc4745f844c61f6cf6a87f1b74698202c57fb87c41f5542fe5e89a6f7cc06e05ec3cc75d3820dae8c82b4be2d9205af808eff9000e88200d53e01d505d2f7edad7c578eb98a8cf1f2ef5ac5ef3a23615a80fde90087f1c4601589c8df27dfb812df16be15eb2cb9233701").unwrap(),
                Number::from_str("f7a1d1e2374ae092217e544bc44167028c8ff712bfb03582911ea6eec1a8d29fd6782fd04a966bfbfb8703c6cc47777c04eeb7b83867fd05a26002e29e9d96913b329d60c3278c550db19e9c9efb52edf790b174802a8aa792fff5761dd0fc7567a6a6c7062d24fb2a1b78f8796505f0fe274b38c7aee791ad55aac704ca1b1d").unwrap(),
            )
        })
    });
}

fn mul_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("mul");

    group.warm_up_time(Duration::from_secs(3));
    group.measurement_time(Duration::from_secs(5));
    group.sample_size(2000);

    const CHARSET: &[u8] = b"0123456789ABCDEF";
    group.bench_function(&format!("Multiplication with 1024 bits"), |b| {
        b.iter(|| {
            let mut rng = rand::thread_rng();
            let one_char = || CHARSET[rng.gen_range(0..CHARSET.len())] as char;
            let num: String = std::iter::repeat_with(one_char).take(256).collect();
            let num: Number = num.parse().unwrap();
            let _ = num.clone() * num;
        })
    });
    group.bench_function(&format!("Old Multiplication with 1024 bits"), |b| {
        b.iter(|| {
            let mut rng = rand::thread_rng();
            let one_char = || CHARSET[rng.gen_range(0..CHARSET.len())] as char;
            let num: String = std::iter::repeat_with(one_char).take(256).collect();
            let num: Number = num.parse().unwrap();
            return num.clone().pow(num, &|a, b| a + b, 0.into(), false);
        })
    });
}

criterion_group!(benches, mul_benchmark);
criterion_main!(benches);
