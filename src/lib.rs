#![doc = include_str!("../README.md")]

/// A number that automatically allocates extra memory when it needs to, which means it can be as
/// large as you want
#[derive(Hash, Clone, PartialEq, Eq)]
pub struct Number {
    body: Vec<u8>,
}

mod comparisons;
mod fromstr;
mod operations;

#[cfg(test)]
mod tests;

impl Default for Number {
    fn default() -> Self {
        Self::new()
    }
}

impl Number {
    // Generates an new Number equal to 0;
    pub fn new() -> Self {
        Self { body: Vec::new() }
    }

    fn optimise(self) -> Self {
        let total_length = self.body.len();

        if self.body.len() == 0 {
            return self;
        }

        if self.body[0] != 0 {
            return self;
        }

        let mut active_length = 0;
        for i in self.body.iter() {
            if i != &0 {
                break;
            }
            active_length += 1;
        }

        let start = active_length;
        let end = total_length;

        let body = self.body.get(start..end).unwrap_or(&[0]).to_owned();

        return Self { body };
    }

    fn split_at(&self, index: usize) -> (Number, Number) {
        if index > self.body.len() {
            return (Number::new(), self.clone());
        }

        let body_len = self.body.len();
        let split_point = body_len - index;

        let mut high = vec![0; split_point];
        let mut low = vec![0; index];

        let mut i = 0;
        while i < body_len {
            if i < split_point {
                high[i] = self.body[i];
            } else {
                low[i - split_point] = self.body[i];
            }
            i += 1;
        }

        let number_high = Number { body: high };
        let number_low = Number { body: low };
        return (number_high, number_low);
    }

    fn pad(mut self, amount: impl Into<isize> + Copy) -> Self {
        self.body.reverse();

        self.body.reserve(amount.into() as usize);
        for _ in 0..amount.into() {
            self.body.push(0);
        }

        self.body.reverse();

        self
    }

    fn pad_back(mut self, amount: impl Into<isize> + Copy) -> Self {
        self.body.reserve(amount.into() as usize);
        for _ in 0..amount.into() {
            self.body.push(0);
        }
        self
    }

    fn num_bits(&self) -> usize {
        let length = (self.body.len() - 1) * 8;
        for i in (0..8).rev() {
            if self.body[0] & (1 << i) != 0 {
                return length + i + 1;
            }
        }
        length
    }
}

impl<T: Into<i128>> From<T> for Number {
    /// Converts `value` to `Number`
    fn from(value: T) -> Self {
        let value: i128 = value.into();
        Self {
            body: value.to_be_bytes().to_vec(),
        }
        .optimise()
    }
}

impl std::fmt::Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("0x")?;
        for i in self.body.iter() {
            f.write_str(&format!(" {i:02x}"))?;
        }
        Ok(())
    }
}

impl std::fmt::Binary for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("0b")?;
        for i in self.body.iter() {
            f.write_str(&format!(" {i:08b}"))?;
        }
        Ok(())
    }
}

impl std::fmt::Debug for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{self}"))
    }
}
