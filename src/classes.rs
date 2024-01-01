pub enum Error {
    InvalidClass(String),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidClass(input) => write!(f, "invalid class: {}", input),
        }
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidClass(input) => write!(f, "invalid class: {}", input),
        }
    }
}

pub enum Class {
    Upper,
    Lower,
    Digit,
}

impl Class {
    pub fn chars(&self) -> Vec<char> {
        match self {
            Class::Upper => ('A'..='Z').collect(),
            Class::Lower => ('a'..='z').collect(),
            Class::Digit => ('0'..='9').collect(),
        }
    }
}

impl TryFrom<&str> for Class {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            ":upper:" => Ok(Class::Upper),
            ":lower:" => Ok(Class::Lower),
            ":digit:" => Ok(Class::Digit),
            _ => Err(Error::InvalidClass(value.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_upper() {
        let class = Class::try_from(":upper:").unwrap();
        assert_eq!(
            class.chars(),
            vec![
                'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P',
                'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'
            ]
        );
    }

    #[test]
    fn test_lower() {
        let class = Class::try_from(":lower:").unwrap();
        assert_eq!(
            class.chars(),
            vec![
                'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
                'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'
            ]
        );
    }
}
