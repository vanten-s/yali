use std::ops::{BitXor, Mul};

use crate::Number;

impl<const N: usize> BitXor for Number<N> {
    fn bitxor(self, rhs: Self) -> Self::Output {
        self.pow(rhs, &Self::mul, 1u16.into(), false)
    }

    type Output = Self;
}

impl<const N: usize> Number<N> {
    pub(in crate::operations) fn pow(
        mut self,
        mut rhs: Self,
        function: &impl Fn(Self, Self) -> Self,
        start: Self,
        _debug: bool,
    ) -> Self {
        let mut result = start;
        let zero = 0u16.into();
        while rhs > zero {
            if rhs.body[rhs.body.len() - 1] & 1 == 1 {
                if _debug {
                    dbg!(result);
                    dbg!(self);
                }
                result = function(result, self);
                if _debug {
                    println!("=");
                    dbg!(result);
                }
            }
            if _debug {
                dbg!(self);
            }
            self = function(self, self);
            if _debug {
                println!("=");
                dbg!(self);
            }
            rhs = rhs >> 1;
        }

        return result;
    }
}
