use std::ops::{BitXor, Mul};

use crate::Number;

impl BitXor for Number {
    fn bitxor(self, rhs: Self) -> Self::Output {
        self.pow(rhs, &Self::mul, 1.into(), false)
    }

    type Output = Self;
}

impl Number {
    pub(in crate::operations) fn pow(
        mut self,
        mut rhs: Self,
        function: &impl Fn(Number, Number) -> Number,
        start: Number,
        _debug: bool,
    ) -> Number {
        let mut result = start.clone();
        let zero = 0.into();
        while rhs > zero {
            if _debug == true {
                dbg!(&result);
                dbg!(&self);
                dbg!(&rhs);
            }
            if rhs.body[rhs.body.len() - 1] & 1 == 1 {
                result = function(result, self.clone());
            }
            self = function(self.clone(), self);
            rhs = rhs >> 1;
            if _debug == true {
                dbg!(&result);
                dbg!(&self);
                dbg!(&rhs);
            }
        }
        if _debug == true {
            dbg!(&result);
            dbg!(&self);
            dbg!(&rhs);
        }

        return result;
    }
}
