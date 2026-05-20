#![doc = include_str!("../README.md")]

/// A number that automatically allocates extra memory when it needs to, which means it can be as
/// large as you want
#[derive(Hash, Clone, Copy)]
pub struct Number<const N: usize> {
    body: [u64; N],
}

mod comparisons;
mod fromstr;
mod operations;

#[cfg(test)]
mod tests;

impl<const N: usize> Number<N> {
    // Generates an new Number equal to 0, with length coerced from type;
    pub fn new() -> Self {
        Self { body: [0; N] }
    }

    fn get_body_length(self) -> usize {
        let mut index = 0;

        while index < N {
            if self.body[index] != 0 {
                break;
            }
            index += 1;
        }

        N - index
    }
}

impl<const N: usize> From<[u64; N]> for Number<N> {
    /// Converts `value` to `Number`
    fn from(body: [u64; N]) -> Self {
        Self { body }
    }
}

impl<const N: usize> From<u64> for Number<N> {
    /// Converts `value` to `Number`
    fn from(value: u64) -> Self {
        let mut body = [0; N];
        body[N - 1] = value;
        Self { body }
    }
}

impl<const N: usize> From<u128> for Number<N> {
    /// Converts `value` to `Number`
    fn from(value: u128) -> Self {
        let mut body = [0; N];
        body[N - 1] = value as u64;
        body[N - 2] = (value >> 64) as u64;
        Self { body }
    }
}

impl<const N: usize> std::fmt::Display for Number<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("0x")?;
        for i in self.body.iter() {
            f.write_str(&format!(" {i:016x}"))?;
        }
        Ok(())
    }
}

impl<const N: usize> std::fmt::Binary for Number<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("0b")?;
        for i in self.body.iter() {
            f.write_str(&format!(" {i:064b}"))?;
        }
        Ok(())
    }
}

impl<const N: usize> std::fmt::Debug for Number<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{self}"))
    }
}




