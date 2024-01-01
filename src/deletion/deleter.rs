use std::collections::HashSet;

pub struct Deleter {
    chars: HashSet<char>,
}

impl Deleter {
    pub fn new(chars: &str) -> Self {
        let chars = chars.chars().collect();

        Self { chars }
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
}
