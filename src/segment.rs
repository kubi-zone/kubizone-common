use std::fmt::Display;

use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DomainSegment(String);

#[derive(Error, Debug, Clone)]
pub enum DomainSegmentError {
    #[error("illegal hyphen at position {0}")]
    IllegalHyphen(usize),
    #[error("invalid character {0}")]
    InvalidCharacter(char),
    #[error("segment too long {0} > 63")]
    TooLong(usize),
    #[error("segment is an empty string")]
    EmptyString,
}

pub const VALID_CHARACTERS: &str = "-0123456789abcdefghijklmnopqrstuvwxyz";

impl TryFrom<&str> for DomainSegment {
    type Error = DomainSegmentError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = value.to_ascii_lowercase();

        if value.is_empty() {
            return Err(DomainSegmentError::EmptyString);
        }

        if value.len() > 63 {
            return Err(DomainSegmentError::TooLong(value.len()));
        }

        if let Some(character) = value.chars().find(|c| !VALID_CHARACTERS.contains(*c)) {
            return Err(DomainSegmentError::InvalidCharacter(character));
        }

        if value.starts_with('-') {
            return Err(DomainSegmentError::IllegalHyphen(1));
        }

        if value.ends_with('-') {
            return Err(DomainSegmentError::IllegalHyphen(value.len()));
        }

        if value.get(2..4) == Some("--") {
            return Err(DomainSegmentError::IllegalHyphen(3));
        }

        Ok(DomainSegment(value))
    }
}

impl TryFrom<String> for DomainSegment {
    type Error = DomainSegmentError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

impl Display for DomainSegment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl AsRef<str> for DomainSegment {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}
