use std::ops::Rem;

use crate::Number;

impl<const N: usize> Rem for Number<N> {
    fn rem(self, rhs: Self) -> Self::Output {
        self.divmod(rhs).1
    }

    type Output = Self;
}
