use crate::Number;

impl Number {
    /// Performs Modular exponentiation
    /// `mod_exponentiation(a, b, n) = (a ^ b) % n`
    pub fn mod_exponentiation(self, rhs: Self, modulus: Self) -> Self {
        self.pow(rhs, &|a, b| (a * b) % modulus.clone(), 1.into(), false)
    }
}
