mod class;
mod dn;
mod fqdn;
mod pattern;
mod pqdn;
mod segment;
mod r#type;

pub use class::Class;
pub use dn::DomainName;
pub use fqdn::FullyQualifiedDomainName;
pub use pattern::{Pattern, PatternSegment};
pub use pqdn::PartiallyQualifiedDomainName;
pub use r#type::Type;

pub mod error {
    pub use crate::dn::DomainNameError;
    pub use crate::fqdn::FullyQualifiedDomainNameError;
    pub use crate::pattern::PatternSegmentError;
    pub use crate::pqdn::PartiallyQualifiedDomainNameError;
}
