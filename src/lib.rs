mod dn;
mod fqdn;
mod pattern;
mod pqdn;
mod segment;

pub use dn::DomainName;
pub use fqdn::FullyQualifiedDomainName;
pub use pattern::{Pattern, PatternSegment};
pub use pqdn::PartiallyQualifiedDomainName;

pub mod error {
    pub use crate::fqdn::FullyQualifiedDomainNameError;
    pub use crate::pattern::PatternSegmentError;
    pub use crate::pqdn::PartiallyQualifiedDomainNameError;
}
