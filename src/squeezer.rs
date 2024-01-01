use std::{collections::HashSet, error::Error};

use crate::classes::Class;

pub struct Squeezer {
    chars: HashSet<char>,
}

impl Squeezer {
    pub fn new(source: &str) -> Self {
        Self {
            chars: Self::process_input(source).into_iter().collect(),
        }
    }

    fn process_input(input: &str) -> Vec<char> {
        if let Ok(class) = Self::derive_class(input) {
            return class;
        };

        if let Ok(range) = Self::derive_range(input) {
            return range;
        };

        input.chars().collect()
    }

    fn derive_class(input: &str) -> Result<Vec<char>, Box<dyn Error>> {
        let class = Class::try_from(input)?;
        Ok(class.chars())
    }

    fn derive_range(input: &str) -> Result<Vec<char>, Box<dyn Error>> {
        let chars = input.chars().collect::<Vec<_>>();

        if chars.len() == 3 && chars[1] == '-' && chars[2] as u32 > chars[0] as u32 {
            return Ok((chars[0]..=chars[2]).collect());
        }

        Err(Box::from(
            format!("failed to parse: {} as a range", input).to_string(),
        ))
    }

    pub fn squeeze(&self, input: &str) -> String {
        let mut output = String::new();
        let mut last_char = None;

        for c in input.chars() {
            if self.chars.contains(&c) {
                if last_char == Some(c) {
                    continue; // Skip this character, as it's a repeat
                }
            }
            output.push(c);
            last_char = Some(c);
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_squeeze() {
        let squeezer = Squeezer::new("a");

        assert_eq!(squeezer.squeeze("aaaaabc"), "abc");
        assert_eq!(squeezer.squeeze("cbaaaaa"), "cba");
        assert_eq!(squeezer.squeeze("aaabccbaaa"), "abccba");
    }
}
