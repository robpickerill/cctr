use std::{collections::HashMap, error::Error, iter::zip};

use crate::classes::Class;

pub struct CharMap {
    map: HashMap<char, char>,
}

impl CharMap {
    pub fn new(source: &str, dest: &str) -> Self {
        let map = zip(
            CharMap::process_input(source).iter().cloned().cycle(),
            CharMap::process_input(dest),
        )
        .collect();

        Self { map }
    }

    fn process_input(input: &str) -> Vec<char> {
        if let Ok(class) = CharMap::derive_class(input) {
            return class;
        };

        if let Ok(range) = CharMap::derive_range(input) {
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

    pub fn get(&self, key: char) -> Option<&char> {
        self.map.get(&key)
    }
}
