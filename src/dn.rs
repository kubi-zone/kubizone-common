
#[derive(Clone, Debug, JsonSchema, Hash, PartialEq, Eq, PartialOrd, Ord)]
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

impl From<String> for DomainName {
    fn from(domain: String) -> Self {
        if domain.ends_with('.') {
            DomainName::Full(FullyQualifiedDomainName(domain))
        } else {
            DomainName::Partial(PartiallyQualifiedDomainName(domain))
        }
    }
}

impl From<&str> for DomainName {
    fn from(domain: &str) -> Self {
        if domain.ends_with('.') {
            DomainName::Full(FullyQualifiedDomainName(domain.to_string()))
        } else {
            DomainName::Partial(PartiallyQualifiedDomainName(domain.to_string()))
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