# yali

Yali is a library for representing, and doing arithmetic with, large numbers.

## Warnings
1. This library only supports positive integers.
2. This isn't the most efficient library. This is just one of my small side projects.

## Examples

Parse hex value from string:
```rust
use yali::Number;

let num_hex = "ab32fa1689fbc2c2631d4343bad3ab2155d";
let num: Number<16> = num_hex.parse().unwrap();
```

Regular exponentiation
```rust
use yali::Number;

let a = Number::<16>::from(2u64);
let b = Number::<16>::from(4u64);
let c = a ^ b;
assert_eq!(c, Number::<16>::from(16u64));
```

Modular exponentiation
```rust
use yali::Number;

let a: Number<16> = "ab32fa1689fbc2c2631d4343bad3ab2155d".parse().unwrap();
let b: Number<16> = "10001".parse().unwrap();
let n: Number<16> = "4343bad3ab2155d89fbc28c2631d".parse().unwrap();
let num = a.mod_exponentiation(b, n);
```

