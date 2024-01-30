mod dn;
mod fqdn;
mod pattern;
mod pqdn;
mod segment;

pub use dn::DomainName;
pub use fqdn::FullyQualifiedDomainName;
pub use pqdn::PartiallyQualifiedDomainName;
pub use pattern::PatternSegment;

pub mod error {
    pub use crate::fqdn::FullyQualifiedDomainNameError;
    pub use crate::pqdn::PartiallyQualifiedDomainNameError;
    pub use crate::pattern::PatternSegmentError;
}
