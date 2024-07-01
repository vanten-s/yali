use std::ops::{Add, AddAssign};

use crate::Number;

impl Add for Number {
    fn add(self, rhs: Self) -> Self::Output {
        let mut min = rhs;
        let mut max = self;
        if min > max {
            (min, max) = (max, min);
        }

        let (min, max) = (min.body, max.body);

        let min_len = min.len();
        let max_len = max.len();

        let len_difference = max_len - min_len;

        let mut body = vec![0; min_len];
        let mut carry = 0u8;
        let mut index = min_len as usize;
        while index > 0 {
            index -= 1;
            // Add everything
            let sum = max[index + len_difference] as u16 + min[index] as u16 + carry as u16;

            // Figure out carry
            carry = (sum >> 8) as u8;
            let sum = sum as u8;

            body[min_len - index - 1] = sum;
        }

        for index in (0..len_difference).rev() {
            // Add everything
            let sum = max[index] as u16 + carry as u16;

            // Figure out carry
            let bytes = sum.to_be_bytes();
            carry = bytes[0];
            let sum = bytes[1];

            body.push(sum);
        }

        body.push(carry);
        body.reverse();

        Number { body }.optimise()
    }

    type Output = Number;
}

impl AddAssign for Number {
    fn add_assign(&mut self, rhs: Self) {
        *self = self.to_owned() + rhs;
    }
}
