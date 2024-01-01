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
}
