use crate::Number;

impl<const N: usize> Number<N> {
    /// Calculates quotient and remainder
    /// `divmod(a, b) -> (a / b, a % b)`
    pub fn divmod(self, rhs: Self) -> (Self, Self) {
        let mut quotient = Number { body: [0; N] };
        let mut remainder = Number { body: [0; N] };

        for j in 0..(64 * N) {
            remainder = remainder << 1;

            if self.body[j >> 6] & (1 << (63 - (j % 64))) != 0 {
                remainder.body[N - 1] |= 1;
            } else {
                remainder.body[N - 1] &= u64::MAX - 1;
            }

            if remainder >= rhs {
                remainder = remainder - rhs;
                quotient.body[j >> 6] |= 1 << (63 - (j % 64));
            }
        }

        (quotient, remainder)
    }
}
