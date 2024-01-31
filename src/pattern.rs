use std::fmt::{Display, Write};

use schemars::JsonSchema;
use serde::{de::Error, Deserialize, Serialize};
use thiserror::Error;

use crate::{segment::DomainSegment, DomainName, FullyQualifiedDomainName};

#[derive(Error, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum PatternError {}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Pattern(Vec<PatternSegment>);

impl Pattern {
    /// Iterates over the [`PatternSegment`]s of the pattern.
    pub fn iter(&self) -> impl Iterator<Item = &PatternSegment> + '_ {
        self.0.iter()
    }

    /// Replaces trailing `@` domain segments with the provided fully qualified domain name.
    pub fn with_origin(&self, origin: &FullyQualifiedDomainName) -> Pattern {
        let mut pattern = self.clone();

        if self.0.last().is_some_and(PatternSegment::is_origin) {
            pattern.0.pop();
            pattern
                .0
                .extend(origin.iter().cloned().map(PatternSegment::from));
        }

        pattern
    }

    /// Returns true if the papttern matches the given domain.
    pub fn matches(&self, domain: &DomainName) -> bool {
        let domain_segments = domain.as_ref().iter().rev();
        let pattern_segments = self.0[..].iter().rev();

        if domain_segments.len() < pattern_segments.len() {
            // Patterns longer than the domain segment cannot possibly match.
            return false;
        }

        if domain_segments.len() > pattern_segments.len()
            // Domains longer than patterns can never match, unless the first
            // segment of the pattern is a standalone wildcard (*)
            && !self.0.first().is_some_and(|pattern| pattern.as_ref() == "*")
        {
            return false;
        }

        for (pattern, domain) in pattern_segments.zip(domain_segments) {
            // If we have hit a pattern segment containing only a wildcard, the rest of the
            // domain segments are automatically matched.
            if pattern.as_ref() == "*" {
                return true;
            }

            if !pattern.matches(domain) {
                return false;
            }
        }

        true
    }
}

impl FromIterator<PatternSegment> for Pattern {
    fn from_iter<T: IntoIterator<Item = PatternSegment>>(iter: T) -> Self {
        Pattern(iter.into_iter().collect())
    }
}

impl TryFrom<&str> for Pattern {
    type Error = PatternSegmentError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let segments = Result::from_iter(
            value
                .trim_end_matches('.')
                .split('.')
                .map(PatternSegment::try_from),
        )?;
        Ok(Pattern(segments))
    }
}

impl TryFrom<String> for Pattern {
    type Error = PatternSegmentError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_ref())
    }
}

impl Display for Pattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for segment in &self.0 {
            write!(f, "{}", segment)?;
            f.write_char('.')?;
        }

        Ok(())
    }
}

impl JsonSchema for Pattern {
    fn schema_name() -> String {
        <String as schemars::JsonSchema>::schema_name()
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        <String as schemars::JsonSchema>::json_schema(gen)
    }
}

impl<'de> Deserialize<'de> for Pattern {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;

        Self::try_from(value).map_err(D::Error::custom)
    }
}

impl Serialize for Pattern {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.to_string().serialize(serializer)
    }
}

/// Segment of a pattern.
/// 
/// Used for matching against a single [`DomainSegment`].
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PatternSegment(String);

impl PatternSegment {
    /// Returns true if the pattern segment matches the provided domain segment.
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

    /// Returns true if this pattern segment is just the origin (@) symbol,
    /// and nothing else.
    pub fn is_origin(&self) -> bool {
        self.0 == "@"
    }

    // Segments cannot be empty.
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

/// Produced when attempting to construct a [`PatternSegment`]
/// from an invalid string.
#[derive(Error, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum PatternSegmentError {
    /// Domain name segments (and therefore pattern segments)
    /// can contain hyphens, but crucially:
    /// 
    /// * Not at the beginning of a segment.
    /// * Not at the end of a segment.
    /// * Not at the 3rd and 4th position *simultaneously* (used for [Punycode encoding](https://en.wikipedia.org/wiki/Punycode))
    #[error("illegal hyphen at position {0}")]
    IllegalHyphen(usize),
    /// Segment contains invalid character.
    #[error("invalid character {0}")]
    InvalidCharacter(char),
    /// Domain segment is longer than the permitted 63 characters.
    #[error("pattern too long {0} > 63")]
    TooLong(usize),
    /// Domain segment is empty.
    #[error("pattern is an empty string")]
    EmptyString,
    /// Pattern contains more than one wildcard (*) character.
    #[error("patterns can only have one wildcard")]
    MultipleWildcards,
    /// Patterns matching an origin (@) cannot contain any other characters.
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
            return Err(PatternSegmentError::MultipleWildcards);
        }

        if value.contains('@') && value.len() != 1 {
            return Err(PatternSegmentError::NonStandaloneOrigin);
        }

        Ok(PatternSegment(value))
    }
}

impl From<DomainSegment> for PatternSegment {
    fn from(value: DomainSegment) -> Self {
        PatternSegment(value.as_ref().to_string())
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
    use crate::{
        error::PatternSegmentError, pattern::PatternSegment, segment::DomainSegment, DomainName,
        FullyQualifiedDomainName, Pattern,
    };

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
        assert_eq!(
            PatternSegment::try_from("*amp*"),
            Err(PatternSegmentError::MultipleWildcards)
        );
    }

    #[test]
    fn origins() {
        assert!(!PatternSegment::try_from("@")
            .unwrap()
            .matches(&DomainSegment::try_from("example").unwrap()))
    }

    #[test]
    fn simple_pattern_match() {
        assert!(Pattern::try_from("*.example.org")
            .unwrap()
            .matches(&DomainName::try_from("www.example.org").unwrap()));
    }

    #[test]
    fn longer_pattern_than_domain() {
        assert!(!Pattern::try_from("*.*.example.org")
            .unwrap()
            .matches(&DomainName::try_from("www.example.org").unwrap()));
    }

    #[test]
    fn longer_domain_than_pattern() {
        assert!(Pattern::try_from("*.example.org")
            .unwrap()
            .matches(&DomainName::try_from("www.sub.test.dev.example.org").unwrap()));
    }

    #[test]
    fn wildcard_segments() {
        let pattern = Pattern::try_from("dev*.example.org").unwrap();

        assert!(pattern.matches(&DomainName::try_from("dev.example.org").unwrap()));
        assert!(pattern.matches(&DomainName::try_from("dev-1.example.org").unwrap()));
        assert!(pattern.matches(&DomainName::try_from("dev-hello.example.org").unwrap()));
        assert!(!pattern.matches(&DomainName::try_from("de.example.org").unwrap()));
        assert!(!pattern.matches(&DomainName::try_from("www.dev-1.example.org").unwrap()));
    }

    #[test]
    fn patterns_assumed_wildcard() {
        let fqdn = Pattern::try_from("example.org.").unwrap();
        let pqdn = Pattern::try_from("example.org").unwrap();
        assert_eq!(fqdn, pqdn);

        assert_eq!(
            fqdn.matches(&DomainName::try_from("example.org.").unwrap()),
            pqdn.matches(&DomainName::try_from("example.org.").unwrap())
        );
    }

    #[test]
    fn origin_insertion() {
        let pattern = Pattern::try_from("example.@").unwrap();

        assert!(!pattern.matches(&DomainName::try_from("example.org.").unwrap()));

        assert!(pattern
            .with_origin(&FullyQualifiedDomainName::try_from("org.").unwrap())
            .matches(&DomainName::try_from("example.org.").unwrap()));
    }
}
