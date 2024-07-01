use std::ops::Shr;

use crate::Number;

impl Shr<u128> for Number {
    fn shr(mut self, rhs: u128) -> Self::Output {
        let mut index = 0;
        while index < rhs {
            let mut carry = 0;
            for index in 0..self.body.len() {
                self.body[index] = self.body[index].rotate_right(1);
                let carry_next = self.body[index] & 0b10000000;
                if carry > 0 {
                    self.body[index] = self.body[index] | 0b10000000;
                } else {
                    self.body[index] = self.body[index] & 0b01111111;
                }
                carry = carry_next
            }

            index += 1;
        }
        return self.optimise();
    }

    type Output = Self;
}
