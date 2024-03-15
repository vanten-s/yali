use std::{fmt::Write, ops::Index};

#[derive(Clone)]
pub struct Number {
    body: Vec<u8>,
    positive: bool, // If the number is positive: true, if negative: false
}

impl Number {
    pub fn new() -> Self {
        Self {
            body: Vec::new(),
            positive: true,
        }
    }

    pub fn max(&self, other: &Self) -> Self {
        if self >= other {
            return self.clone();
        } else {
            return other.clone();
        }
    }

    pub fn min(&self, other: &Self) -> Self {
        if self <= other {
            return self.clone();
        } else {
            return other.clone();
        }
    }

    pub fn div(&self, other: &Self) -> Self {
        todo!()
    }

    fn length_without_leading_zeroes(&self) -> usize {
        for i in (0..self.body.len()).rev() {
            if self[i] == 0 {
                continue;
            }
            return i;
        }
        return 0;
    }
}

impl From<i128> for Number {
    fn from(value: i128) -> Self {
        Self {
            body: value.abs().to_le_bytes().to_vec(),
            positive: value > 0,
        }
    }
}

impl From<u128> for Number {
    fn from(value: u128) -> Self {
        Self {
            body: value.to_le_bytes().to_vec(),
            positive: true,
        }
    }
}

impl From<usize> for Number {
    fn from(value: usize) -> Self {
        Self::from(value as u128)
    }
}

impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        if self.length_without_leading_zeroes() != other.length_without_leading_zeroes() {
            return false;
        }
        for i in 0..self.body.len().min(other.body.len()) {
            if self[i] != other[i] {
                return false;
            }
        }
        return true;
    }

    fn ne(&self, other: &Self) -> bool {
        return !(self == other);
    }
}

impl PartialOrd for Number {
    fn gt(&self, other: &Self) -> bool {
        if self.positive != other.positive {
            return self.positive;
        }
        if self.length_without_leading_zeroes() < other.length_without_leading_zeroes() {
            return false;
        }
        if self.length_without_leading_zeroes() > other.length_without_leading_zeroes() {
            return true;
        }
        for i in (0..self.body.len().min(other.body.len())).rev() {
            if self[i] < other[i] {
                return !self.positive;
            }
            if self[i] > other[i] {
                return self.positive;
            }
        }
        return false;
    }

    fn ge(&self, other: &Self) -> bool {
        if self.positive != other.positive {
            return self.positive;
        }
        if self.length_without_leading_zeroes() < other.length_without_leading_zeroes() {
            return false;
        }
        if self.length_without_leading_zeroes() > other.length_without_leading_zeroes() {
            return true;
        }
        for i in (0..self.body.len().min(other.body.len())).rev() {
            if self[i] < other[i] {
                return !self.positive;
            }
            if self[i] > other[i] {
                return self.positive;
            }
        }
        return true;
    }

    fn lt(&self, other: &Self) -> bool {
        if self.positive != other.positive {
            return other.positive;
        }
        if self.length_without_leading_zeroes() > other.length_without_leading_zeroes() {
            return false;
        }
        if self.length_without_leading_zeroes() < other.length_without_leading_zeroes() {
            return true;
        }
        for i in (0..self.body.len().min(other.body.len())).rev() {
            dbg!(i);
            if self[i] > other[i] {
                return !self.positive;
            }
            if self[i] < other[i] {
                return self.positive;
            }
        }
        return false;
    }

    fn le(&self, other: &Self) -> bool {
        if self.positive != other.positive {
            return other.positive;
        }
        if self.length_without_leading_zeroes() > other.length_without_leading_zeroes() {
            return false;
        }
        if self.length_without_leading_zeroes() < other.length_without_leading_zeroes() {
            return true;
        }
        for i in (0..self.body.len().min(other.body.len())).rev() {
            if self[i] > other[i] {
                return !self.positive;
            }
            if self[i] < other[i] {
                return self.positive;
            }
        }
        return true;
    }

    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        todo!()
    }
}

impl std::ops::Add for Number {
    fn add(self, rhs: Self) -> Self::Output {
        let max_length = self
            .length_without_leading_zeroes()
            .max(rhs.length_without_leading_zeroes());

        let (mut a, mut b) = (self.max(&rhs), self.min(&rhs));

        while b.body.len() < max_length {
            b.body.push(0);
        }

        let mut positive = true;
        let mut subtract = false;

        match (a.positive, b.positive) {
            (false, false) => {
                positive = false;
            }
            (true, false) => {
                panic!("A is not bigger than B");
            }
            (false, true) => {
                subtract = true;
            }
            (true, true) => {
                positive = true;
            }
        }

        let mut carry = 0;
        let mut body = Vec::new();

        if subtract {
            return a - b;
        }

        for i in 0..max_length {
            let mut result = (a[i] as i16) + (b[i] as i16) + carry;
            carry = 0;
            if result > u8::max as i16 {
                result -= 256;
                carry = 1;
            }
            let result = result as u8;
            body.push(result)
        }
        if carry == 1 {
            body.push(carry as u8);
        }

        Number { body, positive }
    }

    type Output = Self;
}

impl std::ops::Sub for Number {
    fn sub(self, rhs: Self) -> Self::Output {
        let max_length = self.length_without_leading_zeroes().max(rhs.length_without_leading_zeroes());

        let mut carry = 0;
        let mut body = Vec::new();
        let mut positive = false;
        for i in 0..max_length {
            let mut result = (self[i] as i16) - (rhs[i] as i16) + carry;
            let result = result as u8;
            body.push(result)
        }

        Number {
            body,
            positive,
        }
    }

    type Output = Self;
}

impl Index<usize> for Number {
    fn index(&self, index: usize) -> &Self::Output {
        &self.body[index]
    }

    type Output = u8;
}

impl std::fmt::Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("0x")?;
        if !self.positive {
            f.write_char('-')?;
        }
        for i in self.body.iter().rev() {
            f.write_str(&format!("{i:x}"))?;
        }
        Ok(())
    }
}

impl std::fmt::Debug for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{self}"))
    }
}

#[cfg(test)]
mod tests {
    use crate::Number;

    #[test]
    fn length_without_leading_zeroes() {
        assert_eq!(
            Number::from(0x1010i128).max(&Number::from(-1i128)),
            Number::from(0x1010i128)
        );
    }
}
