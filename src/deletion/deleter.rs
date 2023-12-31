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
