use std::ops::{BitXor, Mul};

use crate::Number;

impl BitXor for Number {
    fn bitxor(self, rhs: Self) -> Self::Output {
        self.pow(rhs, &Self::mul, 1.into(), false)
    }

    type Output = Self;
}
