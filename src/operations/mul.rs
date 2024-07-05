use crate::Number;
use std::ops::Mul;

impl<const N: usize> Mul for Number<N> {
    fn mul(self, rhs: Self) -> Self::Output {
        return self.pow(rhs, &|a, b| a + b, 0u8.into(), false);
    }

    type Output = Self;
}
