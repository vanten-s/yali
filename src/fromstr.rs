use std::{num::ParseIntError, str::FromStr};

use crate::Number;

impl FromStr for Number {
    fn from_str(s: &str) -> Result<Self, ParseIntError> {
        let mut s = s.replace(|a| !"0123456789abcdef".contains(a), "");
        if s.len() % 2 != 0 {
            s = s.chars().rev().collect();
            s.push('0');
            s = s.chars().rev().collect();
        }
        let mut body = Vec::new();
        let mut current_pair = Vec::new();
        for i in s.chars() {
            current_pair.push(i);
            if current_pair.len() == 2 {
                let string: String = current_pair.iter().collect();
                let byte = u8::from_str_radix(&string, 16)?;
                body.push(byte);
                current_pair = Vec::new()
            }
        }

        Ok(Self { body })
    }

    type Err = ParseIntError;
}
