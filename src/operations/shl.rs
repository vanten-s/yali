use std::ops::Shl;

use crate::Number;

impl Shl<u128> for Number {
    fn shl(mut self, rhs: u128) -> Self::Output {
        self.body.reverse();
        self = self.pad_back((rhs as isize >> 3) + 1);
        self.body = self.body.iter().map(|x| x.reverse_bits()).collect();

        let len_begin = self.body.len() as isize;

        self = self >> rhs;

        let len_after = self.body.len() as isize;
        self = self.pad(len_begin - len_after);

        self.body = self.body.iter().map(|x| x.reverse_bits()).collect();
        self.body.reverse();

        self.optimise()
    }

    type Output = Self;
}
