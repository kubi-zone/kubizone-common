use std::fmt::Display;

use thiserror::Error;

use crate::segment::DomainSegment;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PatternSegment(String);

impl PatternSegment {
    pub fn matches(&self, domain_segment: &DomainSegment) -> bool {
        if self.0 == domain_segment.as_ref() {
            return true;
        }

        if let Some((head, tail)) = self.0.split_once('*') {
            return domain_segment.as_ref().starts_with(head)
                && domain_segment.as_ref().ends_with(tail);
        }

        false
    }
}

#[derive(Error, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum PatternSegmentError {
    #[error("illegal hyphen at position {0}")]
    IllegalHyphen(usize),
    #[error("invalid character {0}")]
    InvalidCharacter(char),
    #[error("pattern too long {0} > 63")]
    TooLong(usize),
    #[error("pattern is an empty string")]
    EmptyString,
    #[error("patterns can only have one wildcard")]
    MultipleWildcards,
    #[error("origins must be standalone")]
    NonStandaloneOrigin,
}

const VALID_CHARACTERS: &str = "-0123456789abcdefghijklmnopqrstuvwxyz*@";

impl TryFrom<&str> for PatternSegment {
    type Error = PatternSegmentError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = value.to_ascii_lowercase();

        if value.is_empty() {
            return Err(PatternSegmentError::EmptyString);
        }

        if value.len() > 63 {
            return Err(PatternSegmentError::TooLong(value.len()));
        }

        if let Some(character) = value.chars().find(|c| !VALID_CHARACTERS.contains(*c)) {
            return Err(PatternSegmentError::InvalidCharacter(character));
        }

        if value.starts_with('-') {
            return Err(PatternSegmentError::IllegalHyphen(1));
        }

        if value.ends_with('-') {
            return Err(PatternSegmentError::IllegalHyphen(value.len()));
        }

        if value.get(2..4) == Some("--") {
            return Err(PatternSegmentError::IllegalHyphen(3));
        }

        if value.chars().filter(|c| *c == '*').count() > 1 {
            return Err(PatternSegmentError::MultipleWildcards)
        }

        if value.contains('@') && value.len() != 1 {
            return Err(PatternSegmentError::NonStandaloneOrigin)
        }

        Ok(PatternSegment(value))
    }
}

impl TryFrom<String> for PatternSegment {
    type Error = PatternSegmentError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

impl Display for PatternSegment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl AsRef<str> for PatternSegment {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

#[cfg(test)]
mod tests {
    use crate::{pattern::PatternSegment, segment::DomainSegment};

    #[test]
    fn literal_matches() {
        assert!(PatternSegment::try_from("example")
            .unwrap()
            .matches(&DomainSegment::try_from("example").unwrap()))
    }

    #[test]
    fn wildcard() {
        assert!(PatternSegment::try_from("*")
            .unwrap()
            .matches(&DomainSegment::try_from("example").unwrap()))
    }

    #[test]
    fn leading_wildcard() {
        assert!(PatternSegment::try_from("*ample")
            .unwrap()
            .matches(&DomainSegment::try_from("example").unwrap()))
    }

    #[test]
    fn trailing_wildcard() {
        assert!(PatternSegment::try_from("examp*")
            .unwrap()
            .matches(&DomainSegment::try_from("example").unwrap()))
    }

    #[test]
    fn splitting_wildcard() {
        assert!(PatternSegment::try_from("ex*le")
            .unwrap()
            .matches(&DomainSegment::try_from("example").unwrap()))
    }

    #[test]
    fn multiple_wildcards() {
        assert!(!PatternSegment::try_from("*amp*")
            .unwrap()
            .matches(&DomainSegment::try_from("example").unwrap()))
    }

    #[test]
    fn origins() {
        assert!(!PatternSegment::try_from("@")
            .unwrap()
            .matches(&DomainSegment::try_from("example").unwrap()))
    }
}
