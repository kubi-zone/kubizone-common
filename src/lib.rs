mod fqdn;
mod pqdn;
mod segment;

pub use fqdn::FullyQualifiedDomainName;
pub use pqdn::PartiallyQualifiedDomainName;

pub mod error {
    pub use crate::fqdn::FullyQualifiedDomainNameError;
    pub use crate::pqdn::PartiallyQualifiedDomainNameError;
}