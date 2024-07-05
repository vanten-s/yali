use std::ops::Div;

use crate::Number;

impl<const N: usize> Div for Number<N> {
    fn div(self, rhs: Self) -> Self::Output {
        self.divmod(rhs).0
    }

    type Output = Self;
}
