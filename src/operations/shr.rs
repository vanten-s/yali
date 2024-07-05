use std::ops::Shr;

use crate::Number;

impl<const N: usize> Shr<u128> for Number<N> {
    fn shr(mut self, rhs: u128) -> Self::Output {
        if rhs & 0b111111 == 0 {
            let mut index = 0;
            while index < rhs {
                let mut n = N - 1;
                while n > 0 {
                    n -= 1;
                    self.body[n + 1] = self.body[n];
                }
                index += 64;
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
