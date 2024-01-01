use std::{collections::HashSet, error::Error};

use crate::classes::Class;

pub struct Deleter {
    chars: HashSet<char>,
}

impl Deleter {
    pub fn new(chars: &str) -> Self {
        Self {
            chars: Deleter::process_input(chars).into_iter().collect(),
        }
    }

    fn process_input(input: &str) -> Vec<char> {
        if let Ok(class) = Deleter::derive_class(input) {
            return class;
        };

        if let Ok(range) = Deleter::derive_range(input) {
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

    pub fn delete(&self, input: &str) -> String {
        input.chars().filter(|c| !self.chars.contains(c)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delete() {
        let deleter = Deleter::new("abc");

        assert_eq!(deleter.delete("abc"), "");
        assert_eq!(deleter.delete("cba"), "");
        assert_eq!(deleter.delete("abccba"), "");
    }

    #[test]
    fn test_delete_with_repeated_chars() {
        let deleter = Deleter::new("aa");

        assert_eq!(deleter.delete("aa"), "");
        assert_eq!(deleter.delete("aaaa"), "");
        assert_eq!(deleter.delete("aaaaaa"), "");
    }

    #[test]
    fn test_delete_with_no_match() {
        let deleter = Deleter::new("abc");

        assert_eq!(deleter.delete("def"), "def");
        assert_eq!(deleter.delete("fed"), "fed");
        assert_eq!(deleter.delete("defabc"), "def");
    }

    #[test]
    fn test_delete_unicode() {
        let deleter = Deleter::new("👍");

        assert_eq!(deleter.delete("👍"), "");
        assert_eq!(deleter.delete("👍👍👍"), "");
        assert_eq!(deleter.delete("👍👎👍"), "👎");
    }

    #[test]
    fn test_delete_class() {
        let deleter = Deleter::new(":upper:");

        assert_eq!(deleter.delete("ABC"), "");
        assert_eq!(deleter.delete("ABCabc"), "abc");
        assert_eq!(deleter.delete("abcABC"), "abc");
    }

    #[test]
    fn test_delete_range() {
        let deleter = Deleter::new("a-z");

        assert_eq!(deleter.delete("abc"), "");
        assert_eq!(deleter.delete("abcABC"), "ABC");
        assert_eq!(deleter.delete("ABCabc"), "ABC");
    }
}
