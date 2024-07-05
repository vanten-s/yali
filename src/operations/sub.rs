use std::ops::Sub;

use crate::Number;

impl<const N: usize> Sub for Number<N> {
    fn sub(self, rhs: Self) -> Self::Output {
        let max = self;
        let min = rhs;

        if min > max {
            dbg!(&min, &max);
            panic!("Subtraction overflow :exploding_head:");
        }

        let mut body = [0; N];
        let mut carry = 0i128;
        let mut index = N;

        let mut max_length = std::cmp::max(self.get_body_length(), rhs.get_body_length()) + 1;
        
        if max_length > N {
            max_length = N;
        } 

        while index > N - max_length {
            index -= 1;
            // Figure out carry
            let current_carry: i128;
            if self.body[index] as i128 + carry < rhs.body[index] as i128 {
                current_carry = u64::MAX as i128 + carry + 1;
                carry = -1;
            } else {
                current_carry = carry;
                carry = 0;
            }

            // Subtract everything
            let diff = (self.body[index] as i128) + current_carry - (rhs.body[index] as i128);

            body[index] = diff as u64;
        }

        Self { body }
    }

    type Output = Self;
}
