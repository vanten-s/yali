use std::ops::Sub;

use crate::Number;

impl Sub for Number {
    fn sub(self, rhs: Self) -> Self::Output {
        let max = self;
        let min = rhs;

        if min > max {
            dbg!(&min, &max);
            panic!("Subtraction overflow :exploding_head:");
        }

        let max = max.body;
        let min = min.body;

        let min_len = min.len();
        let max_len = max.len();

        let len_difference = max_len - min_len;

        let mut body = vec![0; min_len];
        let mut carry = 0i16;
        let mut index = min_len as usize;
        while index > 0 {
            index -= 1;
            let index_max = index + len_difference;

            // Figure out carry
            let current_carry: i16;
            if max[index_max] as i16 + carry < min[index] as i16 {
                current_carry = 256 + carry;
                carry = -1;
            } else {
                current_carry = carry;
                carry = 0;
            }

            // Subtract everything
            let diff = (max[index_max] as i16) + current_carry - (min[index] as i16);

            let diff = diff.to_be_bytes()[1];

            body[min_len - index - 1] = diff as u8;
        }

        for index in (0..len_difference).rev() {
            if max[index] == 0 && carry == -1 {
                body.push(255);
                carry = -1;
            } else {
                body.push((max[index] as i16 + carry) as u8);
                carry = 0;
            }
        }

        body.reverse();

        Self { body }.optimise()
    }

    type Output = Self;
}
