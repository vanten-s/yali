use std::{num::ParseIntError, str::FromStr};

use crate::Number;

impl<const N: usize> FromStr for Number<N> {
    fn from_str(s: &str) -> Result<Self, ParseIntError> {
        let mut s = s.replace(|a| !"0123456789abcdef".contains(a), "");

        while s.len() % 16 != 0 {
            s = s.chars().rev().collect();
            s.push('0');
            s = s.chars().rev().collect();
        }

        let mut body = [0; N];
        let mut current_pair = Vec::new();
        let mut index = N - s.len() / 16;

        for i in s.chars() {
            current_pair.push(i);
            if current_pair.len() == 16 {
                let string: String = current_pair.iter().collect();
                let byte = u64::from_str_radix(&string, 16)?;
                body[index] = byte;
                current_pair = Vec::new();
                index += 1;
            }
        }

        Ok(Self { body })
    }

    type Err = ParseIntError;
}
