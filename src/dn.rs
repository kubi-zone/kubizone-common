use std::fmt::Display;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    fqdn::FullyQualifiedDomainNameError,
    segment::{DomainSegment, DomainSegmentError},
    FullyQualifiedDomainName, PartiallyQualifiedDomainName,
};

/// Either a [`FullyQualifiedDomainName`] or a [`PartiallyQualifiedDomainName`].
#[derive(Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[serde(untagged)]
pub enum DomainName {
    /// Domain name is fully qualified.
    Full(FullyQualifiedDomainName),
    /// Domain name is partially qualified.
    Partial(PartiallyQualifiedDomainName),
}

impl DomainName {
    /// Returns true if domain is fully qualified.
    pub fn is_fully_qualified(&self) -> bool {
        match self {
            DomainName::Full(_) => true,
            DomainName::Partial(_) => false,
        }
    }

    /// Returns true if domain is only partially qualified.
    pub fn is_partially_qualified(&self) -> bool {
        match self {
            DomainName::Full(_) => false,
            DomainName::Partial(_) => true,
        }
    }

    /// Returns [`None`] if fully qualified, or a reference to the contained partially qualified domain otherwise.
    pub fn as_partial(&self) -> Option<&PartiallyQualifiedDomainName> {
        match self {
            DomainName::Partial(partial) => Some(partial),
            _ => None,
        }
    }

    /// Returns [`None`] if partially qualified, or a reference to the contained fully qualified domain otherwise.
    pub fn as_full(&self) -> Option<&FullyQualifiedDomainName> {
        match self {
            DomainName::Full(full) => Some(full),
            _ => None,
        }
    }

    /// Returns the length of the domain.
    ///
    /// Note that fully qualified domain names will include the trailing dot
    /// in this measurement.
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        match self {
            DomainName::Full(full) => full.len(),
            DomainName::Partial(partial) => partial.len(),
        }
    }
}

impl Default for DomainName {
    fn default() -> Self {
        DomainName::Partial(PartiallyQualifiedDomainName::default())
    }
}

impl From<PartiallyQualifiedDomainName> for DomainName {
    fn from(value: PartiallyQualifiedDomainName) -> Self {
        DomainName::Partial(value)
    }
}

impl From<FullyQualifiedDomainName> for DomainName {
    fn from(value: FullyQualifiedDomainName) -> Self {
        DomainName::Full(value)
    }
}

impl TryFrom<String> for DomainName {
    type Error = DomainSegmentError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

impl TryFrom<&str> for DomainName {
    type Error = DomainSegmentError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match FullyQualifiedDomainName::try_from(value) {
            Ok(fqdn) => Ok(DomainName::Full(fqdn)),
            Err(FullyQualifiedDomainNameError::DomainIsPartiallyQualified) => Ok(
                DomainName::Partial(PartiallyQualifiedDomainName::try_from(value).unwrap()),
            ),
            Err(FullyQualifiedDomainNameError::SegmentError(err)) => Err(err),
        }
    }
}

impl Display for DomainName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DomainName::Full(full) => full.fmt(f),
            DomainName::Partial(partial) => partial.fmt(f),
        }
    }
}

impl AsRef<[DomainSegment]> for DomainName {
    fn as_ref(&self) -> &[DomainSegment] {
        match self {
            DomainName::Full(full) => full.as_ref(),
            DomainName::Partial(partial) => partial.as_ref(),
        }
    }
}

impl PartialEq<PartiallyQualifiedDomainName> for DomainName {
    fn eq(&self, other: &PartiallyQualifiedDomainName) -> bool {
        match self {
            DomainName::Full(_) => false,
            DomainName::Partial(partial) => partial.eq(other),
        }
    }
}

impl PartialEq<FullyQualifiedDomainName> for DomainName {
    fn eq(&self, other: &FullyQualifiedDomainName) -> bool {
        match self {
            DomainName::Full(full) => full.eq(other),
            DomainName::Partial(_) => false,
        }
    }
}

impl JsonSchema for DomainName {
    fn schema_name() -> String {
        <String as JsonSchema>::schema_name()
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        <String as JsonSchema>::json_schema(gen)
    }
}

#[cfg(test)]
mod tests {
    use crate::{DomainName, FullyQualifiedDomainName, PartiallyQualifiedDomainName};

    #[test]
    fn deser() {
        let fqdn = DomainName::from(FullyQualifiedDomainName::try_from("example.org.").unwrap());
        let pqdn = DomainName::from(PartiallyQualifiedDomainName::try_from("example.org").unwrap());

        println!("fqdn: {}", serde_yaml::to_string(&fqdn).unwrap());
        println!("pqdn: {}", serde_yaml::to_string(&pqdn).unwrap());

        assert_eq!(
            serde_yaml::from_str::<DomainName>(&serde_yaml::to_string(&fqdn).unwrap()).unwrap(),
            fqdn
        );

        assert_eq!(
            serde_yaml::from_str::<DomainName>(&serde_yaml::to_string(&pqdn).unwrap()).unwrap(),
            pqdn
        );
    }
}
