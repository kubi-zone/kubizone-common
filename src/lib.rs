use std::fmt::Display;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct DomainIsPartiallyQualifiedError;

#[derive(Debug, Clone)]
pub struct DomainIsFullyQualifiedError;

#[derive(
    Serialize, Deserialize, Clone, Debug, Default, JsonSchema, Hash, PartialEq, Eq, PartialOrd, Ord,
)]
#[serde(transparent)]
pub struct FullyQualifiedDomainName(pub String);

impl TryFrom<String> for FullyQualifiedDomainName {
    type Error = DomainIsPartiallyQualifiedError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.ends_with('.') {
            Err(DomainIsPartiallyQualifiedError)
        } else {
            Ok(FullyQualifiedDomainName(value))
        }
    }
}

impl Display for FullyQualifiedDomainName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl AsRef<str> for FullyQualifiedDomainName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[derive(
    Serialize, Deserialize, Clone, Debug, Default, JsonSchema, Hash, PartialEq, Eq, PartialOrd, Ord,
)]
#[serde(transparent)]
pub struct PartiallyQualifiedDomainName(pub String);

impl TryFrom<String> for PartiallyQualifiedDomainName {
    type Error = DomainIsFullyQualifiedError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.ends_with('.') {
            Err(DomainIsFullyQualifiedError)
        } else {
            Ok(PartiallyQualifiedDomainName(value))
        }
    }
}

impl Display for PartiallyQualifiedDomainName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl AsRef<str> for PartiallyQualifiedDomainName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, JsonSchema, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum DomainName {
    Full(FullyQualifiedDomainName),
    Partial(PartiallyQualifiedDomainName),
}

impl DomainName {
    pub fn partial(&self) -> Option<&PartiallyQualifiedDomainName> {
        match self {
            DomainName::Partial(partial) => Some(partial),
            _ => None,
        }
    }

    pub fn full(&self) -> Option<&FullyQualifiedDomainName> {
        match self {
            DomainName::Full(full) => Some(full),
            _ => None,
        }
    }
}

impl From<String> for DomainName {
    fn from(domain: String) -> Self {
        if domain.ends_with('.') {
            DomainName::Full(FullyQualifiedDomainName(domain))
        } else {
            DomainName::Partial(PartiallyQualifiedDomainName(domain))
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

impl AsRef<str> for DomainName {
    fn as_ref(&self) -> &str {
        match self {
            DomainName::Full(full) => full.as_ref(),
            DomainName::Partial(partial) => partial.as_ref(),
        }
    }
}

impl<'de> Deserialize<'de> for DomainName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let domain = String::deserialize(deserializer)?;

        Ok(DomainName::from(domain))
    }
}

impl Serialize for DomainName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.as_ref().serialize(serializer)
    }
}
