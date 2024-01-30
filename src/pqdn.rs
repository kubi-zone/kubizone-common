use std::{
    fmt::{Display, Write},
    ops::Add,
};

use schemars::JsonSchema;
use serde::{de::Error, Deserialize, Serialize};
use thiserror::Error;

use crate::{
    segment::{DomainSegment, DomainSegmentError},
    FullyQualifiedDomainName,
};

#[derive(Error, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum PartiallyQualifiedDomainNameError {
    #[error("domain is fully qualified")]
    DomainIsFullyQualified,
    #[error("{0}")]
    SegmentError(#[from] DomainSegmentError),
}

#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct PartiallyQualifiedDomainName(Vec<DomainSegment>);

impl PartiallyQualifiedDomainName {
    pub fn iter(&self) -> impl Iterator<Item = &DomainSegment> + '_ {
        self.0.iter()
    }
}

impl FromIterator<DomainSegment> for PartiallyQualifiedDomainName {
    fn from_iter<T: IntoIterator<Item = DomainSegment>>(iter: T) -> Self {
        PartiallyQualifiedDomainName(iter.into_iter().collect())
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
            if index != self.0.len() {
                f.write_char('.')?;
            }
        }

        Ok(())
    }
}

impl Add<&FullyQualifiedDomainName> for &PartiallyQualifiedDomainName {
    type Output = FullyQualifiedDomainName;

    fn add(self, rhs: &FullyQualifiedDomainName) -> Self::Output {
        FullyQualifiedDomainName::from_iter(self.0.iter().chain(rhs.iter()).cloned())
    }
}

impl Add for &PartiallyQualifiedDomainName {
    type Output = PartiallyQualifiedDomainName;

    fn add(self, rhs: &PartiallyQualifiedDomainName) -> Self::Output {
        PartiallyQualifiedDomainName::from_iter(self.0.iter().chain(rhs.iter()).cloned())
    }
}

impl AsRef<[DomainSegment]> for PartiallyQualifiedDomainName {
    fn as_ref(&self) -> &[DomainSegment] {
        self.0.as_ref()
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
        error::PartiallyQualifiedDomainNameError, segment::DomainSegment,
        PartiallyQualifiedDomainName,
    };

    #[test]
    fn construct_pqdn() {
        assert_eq!(
            PartiallyQualifiedDomainName::try_from("example.org"),
            Ok(PartiallyQualifiedDomainName::from_iter([
                DomainSegment::try_from("example").unwrap(),
                DomainSegment::try_from("org").unwrap()
            ]))
        );
    }

    #[test]
    fn pqdn_from_fqdn_fails() {
        assert_eq!(
            PartiallyQualifiedDomainName::try_from("example.org."),
            Err(PartiallyQualifiedDomainNameError::DomainIsFullyQualified)
        );
    }
}
