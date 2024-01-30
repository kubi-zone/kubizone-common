use std::fmt::{Debug, Display, Write};

use schemars::JsonSchema;
use serde::{de::Error, Deserialize, Serialize};
use thiserror::Error;

use crate::segment::{DomainSegment, DomainSegmentError};

#[derive(Error, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum FullyQualifiedDomainNameError {
    #[error("domain is partially qualified")]
    DomainIsPartiallyQualified,
    #[error("{0}")]
    SegmentError(#[from] DomainSegmentError),
}

#[derive(Default, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct FullyQualifiedDomainName(Vec<DomainSegment>);

impl FullyQualifiedDomainName {
    pub fn iter(&self) -> impl Iterator<Item = &DomainSegment> + '_ {
        self.0.iter()
    }

    pub fn is_subdomain_of(&self, parent: &FullyQualifiedDomainName) -> bool {
        self.0.ends_with(parent.as_ref())
    }
}

impl FromIterator<DomainSegment> for FullyQualifiedDomainName {
    fn from_iter<T: IntoIterator<Item = DomainSegment>>(iter: T) -> Self {
        FullyQualifiedDomainName(iter.into_iter().collect())
    }
}

impl TryFrom<String> for FullyQualifiedDomainName {
    type Error = FullyQualifiedDomainNameError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

impl TryFrom<&str> for FullyQualifiedDomainName {
    type Error = FullyQualifiedDomainNameError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if !value.ends_with('.') {
            Err(FullyQualifiedDomainNameError::DomainIsPartiallyQualified)
        } else {
            let segments = Result::from_iter(
                value
                    .trim_end_matches('.')
                    .split('.')
                    .map(DomainSegment::try_from),
            )?;

            Ok(FullyQualifiedDomainName(segments))
        }
    }
}

impl Display for FullyQualifiedDomainName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for segment in &self.0 {
            write!(f, "{}", segment)?;
            f.write_char('.')?;
        }

        Ok(())
    }
}

impl AsRef<[DomainSegment]> for FullyQualifiedDomainName {
    fn as_ref(&self) -> &[DomainSegment] {
        self.0.as_ref()
    }
}

impl JsonSchema for FullyQualifiedDomainName {
    fn schema_name() -> String {
        <String as schemars::JsonSchema>::schema_name()
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        <String as schemars::JsonSchema>::json_schema(gen)
    }
}

impl<'de> Deserialize<'de> for FullyQualifiedDomainName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;

        Self::try_from(value).map_err(D::Error::custom)
    }
}

impl Serialize for FullyQualifiedDomainName {
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
        fqdn::FullyQualifiedDomainNameError, segment::DomainSegment, FullyQualifiedDomainName,
    };

    #[test]
    fn construct_fqdn() {
        assert_eq!(
            FullyQualifiedDomainName::try_from("example.org."),
            Ok(FullyQualifiedDomainName::from_iter([
                DomainSegment::try_from("example").unwrap(),
                DomainSegment::try_from("org").unwrap()
            ]))
        );
    }

    #[test]
    fn fqdn_from_pqdn_fails() {
        assert_eq!(
            FullyQualifiedDomainName::try_from("example.org"),
            Err(FullyQualifiedDomainNameError::DomainIsPartiallyQualified)
        );
    }
}