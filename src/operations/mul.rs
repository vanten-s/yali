use crate::Number;
use std::ops::Mul;

impl<const N: usize> Mul for Number<N> {
    /// Implemented recursively
    /// m splits integer evenly
    /// x1 = self, shifted right by m words
    /// x0 = self, with m top bytes 0
    /// z3 = (x1 + x0) * (y1 + y0)
    /// z2 = x1 * y1
    /// z1 = x1 * y0 + x0 * y1 = z3 - z2 - z0
    /// z0 = x0 * y0
    /// x * y = z2 << m

    fn mul(self, rhs: Self) -> Self::Output {
        let prod = self.mul_back(rhs, N/2);
        prod
    }

    type Output = Self;
}

impl<const N: usize> Number<N> {
    pub fn mul_back(self, rhs: Self, mut m: usize) -> Self {
        if m == 0 {
            if self.body[N-2] != 0 || rhs.body[N-2] != 0 {
                let z0 = self.body[N-1] as u128 * rhs.body[N-1] as u128;
                let z1 = self.body[N-2] as u128 * rhs.body[N-1] as u128 + self.body[N-1] as u128 * rhs.body[N-2] as u128;
                let z2 = self.body[N-2] as u128 * rhs.body[N-2] as u128;

                let z0: Self = z0.into();
                let z1: Self = z1.into();
                let z2: Self = z2.into();

                return (z2 << 128) + (z1 << 64) + z0;
                 
            }
            let product = self.body[N-1] as u128 * rhs.body[N-1] as u128;
            let carry = product >> 64;
            let mut body = [0;N];
            body[N-1] = product as u64;
            body[N-2] = carry as u64;
            return Self { body };
        }

        if m < N/2 {
            if self.body[N-(m*2)-1] != 0 || rhs.body[N-(m*2)-1] != 0 {
                m += 1;
            }
        }

        let mut x1 = self;
        x1 = x1 >> m*64;
        let mut x0 = self;
        let mut y1 = rhs;
        y1 = y1 >> m*64;
        let mut y0 = rhs;
        for i in m..N {
            x0.body[N-i-1] = 0;
            x1.body[N-i-1] = 0;
            y0.body[N-i-1] = 0;
            y1.body[N-i-1] = 0;
        }

        let mut z2 = x1.mul_back(y1, m / 2);
        let z0 = x0.mul_back(y0, m / 2);
        let z3 = (x1 + x0).mul_back(y1 + y0, m / 2);
        let mut z1 = z3 - z2 - z0;

        z2.body.rotate_left(2 * m);
        z1.body.rotate_left(m);

        return z2 + z1 + z0;
    }
}
