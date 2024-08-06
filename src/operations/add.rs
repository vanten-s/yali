use std::ops::{Add, AddAssign};

use crate::Number;

impl<const N: usize> Add for Number<N> {
    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self 
    }

    type Output = Self;
}

impl<const N: usize> AddAssign for Number<N> {
    fn add_assign(&mut self, rhs: Self) {
        let mut carry = 0;
        let mut index = N;
        let mut max_length = std::cmp::max(self.get_body_length(), rhs.get_body_length()) + 1;
        
        if max_length > N {
            max_length = N;
        } 

        while index > N - max_length {
            index -= 1;
            // Add everything
            let sum = self.body[index] as u128 + rhs.body[index] as u128 + carry as u128;

            // Figure out carry
            carry = (sum >> 64) as u64;
            let sum = sum as u64;

            self.body[index] = sum;
        }
    }
}
