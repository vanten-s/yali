use std::ops::Shl;

use crate::Number;

impl<const N: usize> Shl<usize> for Number<N> {
    fn shl(mut self, rhs: usize) -> Self::Output {
        if rhs & 63 == 0 {
            self.body.rotate_left(rhs/64);
            for i in (N-rhs/64)..N {
                self.body[i] = 0;
            }
            return self;
        }

        let mut index = 0;
        while index < rhs {
            let mut carry = 0;
            for index in (0..N).rev() {
                self.body[index] = self.body[index].rotate_left(1);
                let carry_next = self.body[index] & 0b1;
                if carry > 0 {
                    self.body[index] = self.body[index] | 0b1;
                } else {
                    self.body[index] = self.body[index] & (u64::MAX - 1);
                }
                carry = carry_next
            }

            index += 1;
        }
        return self;
    }

    type Output = Self;
}
