use std::ops::Rem;

use crate::Number;

impl Rem for Number {
    fn rem(self, rhs: Self) -> Self::Output {
        self.divmod(rhs).1
    }

    type Output = Self;
}
