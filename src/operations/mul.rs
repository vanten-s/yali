use crate::Number;
use std::ops::Mul;

impl Mul for Number {
    fn mul(self, rhs: Self) -> Self::Output {
        // Karatsuba Algorithm
        let m = std::cmp::max(self.body.len(), rhs.body.len()) / 2;

        if std::cmp::min(self.body.len(), rhs.body.len()) <= 700 {
            return self.pow(rhs, &|a, b| a + b, 0.into(), false);
        } else {
            dbg!(&self);
        }

        let (x1, x0) = self.split_at(m);
        let (y1, y0) = rhs.split_at(m);

        let x0 = x0.optimise();
        let y0 = y0.optimise();

        let z0 = x0.clone() * y0.clone();
        let z2 = x1.clone() * y1.clone();
        let z3 = (x1.clone() + x0.clone()) * (y1.clone() + y0.clone());
        let z1 = z3 - z2.clone() - z0.clone();

        let shift_bytes = |a: Number, b: usize| a << (8 * b as u128);
        return shift_bytes(z2, m << 1) + shift_bytes(z1, m) + z0;
    }

    type Output = Self;
}
