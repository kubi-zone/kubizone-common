use std::fmt::Display;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    fqdn::FullyQualifiedDomainNameError,
    segment::{DomainSegment, DomainSegmentError},
    FullyQualifiedDomainName, PartiallyQualifiedDomainName,
};

#[derive(
    Clone, Debug, Serialize, Deserialize, JsonSchema, Hash, PartialEq, Eq, PartialOrd, Ord,
)]
pub enum DomainName {
    Full(FullyQualifiedDomainName),
    Partial(PartiallyQualifiedDomainName),
}

impl DomainName {
    pub fn is_fully_qualified(&self) -> bool {
        match self {
            DomainName::Full(_) => true,
            DomainName::Partial(_) => false,
        }
    }

    pub fn is_partially_qualified(&self) -> bool {
        match self {
            DomainName::Full(_) => false,
            DomainName::Partial(_) => true,
        }
    }

    pub fn as_partial(&self) -> Option<&PartiallyQualifiedDomainName> {
        match self {
            DomainName::Partial(partial) => Some(partial),
            _ => None,
        }
    }

    pub fn as_full(&self) -> Option<&FullyQualifiedDomainName> {
        match self {
            DomainName::Full(full) => Some(full),
            _ => None,
        }
    }
}

impl Default for DomainName {
    fn default() -> Self {
        DomainName::Full(FullyQualifiedDomainName::default())
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
