use crate::charmap::CharMap;

pub struct Replacer {
    map: CharMap,
}

impl Replacer {
    pub fn new(source: &str, dest: &str) -> Self {
        let map = CharMap::new(source, dest);

        Self { map }
    }

    pub fn replace(&self, input: &str) -> String {
        input
            .chars()
            .map(|c| self.map.get(c).unwrap_or(&c).to_owned())
            .collect()
    }
}
