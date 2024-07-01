# yali

Yali is a library for representing, and doing arithmetic with, large numbers.

## Examples

Parse hex value from string:
```rust
let num_hex = "ab32fa1689fbc2c2631d4343bad3ab2155d";
let num = Number::from(num_hex.to_owned());
```

Regular exponentiation
```rust
let a = Number::from(2);
let b = Number::from(4);
let c = a ^ b;
```

Modular exponentiation
```rust
let a: Number = "ab32fa1689fbc2c2631d4343bad3ab2155d".to_owned().into();
let b: Number = "10001".to_owned().into();
let n: Number = "4343bad3ab2155d89fbc28c2631d".to_owned().into();
let num = a.mod_exponentiation(b, n);
```

