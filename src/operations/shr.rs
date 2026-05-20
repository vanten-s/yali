use std::ops::Shr;

use crate::Number;

impl<const N: usize> Shr<usize> for Number<N> {
    fn shr(mut self, rhs: usize) -> Self::Output {
        if rhs & 63 == 0 {
            self.body.rotate_right(rhs / 64);
            for i in 0..rhs/64 {
                self.body[i] = 0;
            }
            return self;
        }

        let mut index = 0;
        while index < rhs {
            let mut carry = 0;
            for index in 0..N {
                self.body[index] = self.body[index].rotate_right(1);
                let carry_next = self.body[index] & ((u64::MAX >> 1) + 1);
                if carry > 0 {
                    self.body[index] = self.body[index] | ((u64::MAX >> 1) + 1);
                } else {
                    self.body[index] = self.body[index] & !((u64::MAX >> 1) + 1);
                }
                carry = carry_next
            }

            index += 1;
        }
        return self;
    }

    type Output = Self;
}
