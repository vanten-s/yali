use std::fmt::Write;

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
            return self.clone()
        } else {
            return other.clone()
        }
    }

    pub fn min(&self, other: &Self) -> Self {
        if self <= other {
            return self.clone()
        } else {
            return other.clone()
        }
    }

    fn length_without_leading_zeroes(&self) -> usize {
        for i in (0..self.body.len()).rev() {
            if self.body[i] == 0 {
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
            if self.body[i] != other.body[i] {
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
            if self.body[i] < other.body[i] {
                return !self.positive;
            }
            if self.body[i] > other.body[i] {
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
            if self.body[i] < other.body[i] {
                return !self.positive;
            }
            if self.body[i] > other.body[i] {
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
            if self.body[i] > other.body[i] {
                return !self.positive;
            }
            if self.body[i] < other.body[i] {
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
            if self.body[i] > other.body[i] {
                return !self.positive;
            }
            if self.body[i] < other.body[i] {
                return self.positive;
            }
        }
        return true;
    }

    fn partial_cmp(&self, _other: &Self) -> Option<std::cmp::Ordering> {
        todo!()
    }
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
        assert_eq!(Number::from(0x1010i128).max(&Number::from(-1i128)), Number::from(0x1010i128));
    }
}
