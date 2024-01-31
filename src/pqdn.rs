use std::{
    fmt::{Display, Write},
    ops::Add,
};

use schemars::JsonSchema;
use serde::{de::Error, Deserialize, Serialize};
use thiserror::Error;

use crate::{
    fqdn::FullyQualifiedDomainNameError,
    segment::{DomainSegment, DomainSegmentError},
    FullyQualifiedDomainName,
};

/// Produced when attempting to construct a [`PartiallyQualifiedDomainName`]
/// from an invalid string.
#[derive(Error, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum PartiallyQualifiedDomainNameError {
    /// The parsed string is not partially qualified. That is, it contains
    /// a trailing dot making it fully qualified.
    #[error("domain is fully qualified")]
    DomainIsFullyQualified,
    /// One or more of the segments of the domain specified in the string
    /// are invalid.
    #[error("{0}")]
    SegmentError(#[from] DomainSegmentError),
    /// Domain contains origin (@) segment in a non-terminal
    /// domain segment.
    #[error("non-terminal segment contains origin (@) segment")]
    OriginInNonTerminalSegment,
}

/// Partially qualified domain name (PQDN).
///
/// A partially qualified domain name is an incomplete domain, meaning
/// the domain name is (potentially) a subdomain of another unknown domain.
/// Unlike fully qualified domain names, PQDNs indicate only some of the
/// path within the domain name system.
///
/// Partially qualified domain names are often used when the root of the
/// domain name is not known, or specified elsewhere.
///
/// See also [`FullyQualifiedDomainName`]
#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct PartiallyQualifiedDomainName(Vec<DomainSegment>);

impl PartiallyQualifiedDomainName {
    /// Attempt to construct a PartiallyQualifiedDomainName from an iterator
    /// over [`DomainSegment`]s. Fails if the iterator contains an origin (@)
    /// segment anywhere but the last segment.
    pub fn try_from_segments<T: IntoIterator<Item = DomainSegment>>(
        iter: T,
    ) -> Result<Self, PartiallyQualifiedDomainNameError> {
        let segments: Vec<DomainSegment> = iter.into_iter().collect();

        if segments.iter().rev().skip(1).any(DomainSegment::is_origin) {
            Err(PartiallyQualifiedDomainNameError::OriginInNonTerminalSegment)
        } else {
            Ok(PartiallyQualifiedDomainName(segments))
        }
    }

    /// Iterates over all [`DomainSegment`]s that make up the domain name.
    pub fn iter(&self) -> impl Iterator<Item = &DomainSegment> + '_ {
        self.0.iter()
    }

    /// Length of the fully qualified domain name as a string.
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.0.iter().map(|segment| segment.len()).sum::<usize>() + self.0.len()
    }
}

impl TryFrom<String> for PartiallyQualifiedDomainName {
    type Error = PartiallyQualifiedDomainNameError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

impl TryFrom<&str> for PartiallyQualifiedDomainName {
    type Error = PartiallyQualifiedDomainNameError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.ends_with('.') {
            Err(PartiallyQualifiedDomainNameError::DomainIsFullyQualified)
        } else {
            let segments = Result::from_iter(value.split('.').map(DomainSegment::try_from))?;

            Ok(PartiallyQualifiedDomainName(segments))
        }
    }
}

impl Display for PartiallyQualifiedDomainName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (index, segment) in self.0.iter().enumerate() {
            segment.fmt(f)?;
            if index != self.0.len() - 1 {
                f.write_char('.')?;
            }
        }

        Ok(())
    }
}

impl Add<&FullyQualifiedDomainName> for &PartiallyQualifiedDomainName {
    type Output = Result<FullyQualifiedDomainName, FullyQualifiedDomainNameError>;

    fn add(self, rhs: &FullyQualifiedDomainName) -> Self::Output {
        FullyQualifiedDomainName::try_from_segments(self.0.iter().chain(rhs.iter()).cloned())
    }
}

impl Add for &PartiallyQualifiedDomainName {
    type Output = Result<PartiallyQualifiedDomainName, PartiallyQualifiedDomainNameError>;

    fn add(self, rhs: &PartiallyQualifiedDomainName) -> Self::Output {
        PartiallyQualifiedDomainName::try_from_segments(self.0.iter().chain(rhs.iter()).cloned())
    }
}

impl AsRef<[DomainSegment]> for PartiallyQualifiedDomainName {
    fn as_ref(&self) -> &[DomainSegment] {
        self.0.as_ref()
    }
}

impl PartialEq<String> for PartiallyQualifiedDomainName {
    fn eq(&self, other: &String) -> bool {
        self.to_string().eq(other)
    }
}

impl PartialEq<str> for PartiallyQualifiedDomainName {
    fn eq(&self, other: &str) -> bool {
        self.to_string().eq(other)
    }
}

impl JsonSchema for PartiallyQualifiedDomainName {
    fn schema_name() -> String {
        <String as schemars::JsonSchema>::schema_name()
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        <String as schemars::JsonSchema>::json_schema(gen)
    }
}

impl<'de> Deserialize<'de> for PartiallyQualifiedDomainName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;

        Self::try_from(value).map_err(D::Error::custom)
    }
}

impl Serialize for PartiallyQualifiedDomainName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.to_string().serialize(serializer)
    }
}

#[cfg(test)]
mod test {
    use crate::{
        error::PartiallyQualifiedDomainNameError, segment::DomainSegment, FullyQualifiedDomainName,
        PartiallyQualifiedDomainName,
    };

    #[test]
    fn construct_pqdn() {
        assert_eq!(
            PartiallyQualifiedDomainName::try_from("example.org"),
            PartiallyQualifiedDomainName::try_from_segments([
                DomainSegment::try_from("example").unwrap(),
                DomainSegment::try_from("org").unwrap()
            ])
        );
    }

    #[test]
    fn pqdn_from_fqdn_fails() {
        assert_eq!(
            PartiallyQualifiedDomainName::try_from("example.org."),
            Err(PartiallyQualifiedDomainNameError::DomainIsFullyQualified)
        );
    }

    #[test]
    fn addition() {
        assert_eq!(
            &PartiallyQualifiedDomainName::try_from("test").unwrap()
                + &FullyQualifiedDomainName::try_from("example.org.").unwrap(),
            Ok(FullyQualifiedDomainName::try_from("test.example.org.").unwrap())
        )
    }

    #[test]
    fn pqdn_addition() {
        assert_eq!(
            &PartiallyQualifiedDomainName::try_from("test").unwrap()
                + &PartiallyQualifiedDomainName::try_from("example").unwrap(),
            Ok(PartiallyQualifiedDomainName::try_from("test.example").unwrap())
        )
    }
}
