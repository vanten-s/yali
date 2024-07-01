use std::ops::Div;

use crate::Number;

impl Div for Number {
    fn div(self, rhs: Self) -> Self::Output {
        self.divmod(rhs).0
    }

    type Output = Self;
}
