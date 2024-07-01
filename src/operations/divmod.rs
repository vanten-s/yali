use crate::Number;

impl Number {
    /// Calculates quotient and remainder
    /// `divmod(a, b) -> (a / b, a % b)`
    pub fn divmod(self, rhs: Self) -> (Number, Number) {
        let mut quotient: Number = Number {
            body: self.body.iter().map(|_| 0).collect(),
        };
        let mut remainder: Number = quotient.clone();

        for j in 0..self.num_bits() {
            let i = j + self.body.len() * 8 - self.num_bits();
            remainder = remainder << 1;
            if remainder.body.len() == 0 {
                remainder = remainder.pad(1u8);
            }
            let remainder_body_length = remainder.body.len();
            if self.body[i >> 3] & (1 << (7 - (i % 8))) != 0 {
                remainder.body[remainder_body_length - 1] |= 1;
            } else {
                remainder.body[remainder_body_length - 1] &= 0b11111110;
            }
            if remainder >= rhs {
                remainder = remainder - rhs.clone();
                quotient.body[i >> 3] |= 1 << (7 - (i % 8));
            }
        }

        (quotient.optimise(), remainder.optimise())
    }
}
