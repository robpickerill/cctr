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
