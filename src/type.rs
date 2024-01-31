use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(
    Serialize, Deserialize, JsonSchema, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord,
)]
pub enum Type {
    /// Address record
    ///
    /// Returns a 32-bit IPv4 address, most commonly used to map hostnames to an IP address of the host, but it is also used for DNSBLs, storing subnet masks in RFC 1101, etc.
    ///
    /// [1035](https://datatracker.ietf.org/doc/html/rfc1035)
    #[allow(clippy::upper_case_acronyms)]
    A,
    /// IPv6 address record
    ///
    /// Returns a 128-bit IPv6 address, most commonly used to map hostnames to an IP address of the host.
    ///
    /// [3596](https://datatracker.ietf.org/doc/html/rfc3596)
    #[allow(clippy::upper_case_acronyms)]
    AAAA,
    /// AFS database record
    ///
    /// Location of database servers of an AFS cell. This record is commonly used by AFS clients to contact AFS cells outside their local domain. A subtype of this record is used by the obsolete DCE/DFS file system.
    ///
    /// [1183](https://datatracker.ietf.org/doc/html/rfc1183)
    #[allow(clippy::upper_case_acronyms)]
    AFSDB,
    /// Address Prefix List
    ///
    /// Specify lists of address ranges, e.g. in CIDR format, for various address families. Experimental.
    ///
    /// [3123](https://datatracker.ietf.org/doc/html/rfc3123)
    #[allow(clippy::upper_case_acronyms)]
    APL,
    /// Certification Authority Authorization
    ///
    /// DNS Certification Authority Authorization, constraining acceptable CAs for a host/domain
    ///
    /// [6844](https://datatracker.ietf.org/doc/html/rfc6844)
    #[allow(clippy::upper_case_acronyms)]
    CAA,
    /// Child copy of DNSKEY record, for transfer to parent
    ///
    /// [7344](https://datatracker.ietf.org/doc/html/rfc7344)
    #[allow(clippy::upper_case_acronyms)]
    CDNSKEY,
    /// Child DS
    ///
    /// Child copy of DS record, for transfer to parent
    ///
    /// [7344](https://datatracker.ietf.org/doc/html/rfc7344)
    #[allow(clippy::upper_case_acronyms)]
    CDS,
    /// Certificate record
    ///
    /// Stores PKIX, SPKI, PGP, etc.
    ///
    /// [4398](https://datatracker.ietf.org/doc/html/rfc4398)
    #[allow(clippy::upper_case_acronyms)]
    CERT,
    /// Canonical name record
    ///
    /// Alias of one name to another: the DNS lookup will continue by retrying the lookup with the new name.
    ///
    /// [1035](https://datatracker.ietf.org/doc/html/rfc1035)
    #[allow(clippy::upper_case_acronyms)]
    CNAME,
    /// Child-to-Parent Synchronization
    ///
    /// Specify a synchronization mechanism between a child and a parent DNS zone. Typical example is declaring the same NS records in the parent and the child zone
    ///
    /// [7477](https://datatracker.ietf.org/doc/html/rfc7477)
    #[allow(clippy::upper_case_acronyms)]
    CSYNC,
    /// DHCP identifier
    ///
    /// Used in conjunction with the FQDN option to DHCP
    ///
    /// [4701](https://datatracker.ietf.org/doc/html/rfc4701)
    #[allow(clippy::upper_case_acronyms)]
    DHCID,
    /// DNSSEC Lookaside Validation record
    ///
    /// For publishing DNSSEC trust anchors outside of the DNS delegation chain. Uses the same format as the DS record. RFC 5074 describes a way of using these records.
    ///
    /// [4431](https://datatracker.ietf.org/doc/html/rfc4431)
    #[allow(clippy::upper_case_acronyms)]
    DLV,
    /// Delegation name record
    ///
    /// Alias for a name and all its subnames, unlike CNAME, which is an alias for only the exact name. Like a CNAME record, the DNS lookup will continue by retrying the lookup with the new name.
    ///
    /// [6672](https://datatracker.ietf.org/doc/html/rfc6672)
    #[allow(clippy::upper_case_acronyms)]
    DNAME,
    /// DNS Key record
    ///
    /// The key record used in DNSSEC. Uses the same format as the KEY record.
    ///
    /// [4034](https://datatracker.ietf.org/doc/html/rfc4034)
    #[allow(clippy::upper_case_acronyms)]
    DNSKEY,
    /// Delegation signer
    ///
    /// The record used to identify the DNSSEC signing key of a delegated zone
    ///
    /// [4034](https://datatracker.ietf.org/doc/html/rfc4034)
    #[allow(clippy::upper_case_acronyms)]
    DS,
    /// MAC address (EUI-48)
    ///
    /// A 48-bit IEEE Extended Unique Identifier.
    ///
    /// [7043](https://datatracker.ietf.org/doc/html/rfc7043)
    #[allow(clippy::upper_case_acronyms)]
    EUI48,
    /// MAC address (EUI-64)
    ///
    /// A 64-bit IEEE Extended Unique Identifier.
    ///
    /// [7043](https://datatracker.ietf.org/doc/html/rfc7043)
    #[allow(clippy::upper_case_acronyms)]
    EUI64,
    /// Host Information
    ///
    /// Providing Minimal-Sized Responses to DNS Queries That Have QTYPE=ANY
    ///
    /// [8482](https://datatracker.ietf.org/doc/html/rfc8482)
    #[allow(clippy::upper_case_acronyms)]
    HINFO,
    /// Host Identity Protocol
    ///
    /// Method of separating the end-point identifier and locator roles of IP addresses.
    ///
    /// [8005](https://datatracker.ietf.org/doc/html/rfc8005)
    #[allow(clippy::upper_case_acronyms)]
    HIP,
    /// HTTPS Binding
    ///
    /// RR that improves performance for clients that need to resolve many resources to access a domain.
    ///
    /// [9460](https://datatracker.ietf.org/doc/html/rfc9460)
    #[allow(clippy::upper_case_acronyms)]
    HTTPS,
    /// IPsec Key
    ///
    /// Key record that can be used with IPsec
    ///
    /// [4025](https://datatracker.ietf.org/doc/html/rfc4025)
    #[allow(clippy::upper_case_acronyms)]
    IPSECKEY,
    /// Key record
    ///
    /// Used only for SIG(0) (RFC 2931) and TKEY (RFC 2930).[5] RFC 3445 eliminated their use for application keys and limited their use to DNSSEC.[6] RFC 3755 designates DNSKEY as the replacement within DNSSEC.[7] RFC 4025 designates IPSECKEY as the replacement for use with IPsec.[8]
    ///
    /// [2535](https://datatracker.ietf.org/doc/html/rfc2535)
    /// [2930](https://datatracker.ietf.org/doc/html/rfc2930)
    #[allow(clippy::upper_case_acronyms)]
    KEY,
    /// Key Exchanger record
    ///
    /// Used with some cryptographic systems (not including DNSSEC) to identify a key management agent for the associated domain-name. Note that this has nothing to do with DNS Security. It is Informational status, rather than being on the IETF standards-track. It has always had limited deployment, but is still in use.
    ///
    /// [2230](https://datatracker.ietf.org/doc/html/rfc2230)
    #[allow(clippy::upper_case_acronyms)]
    KX,
    /// Location record
    ///
    /// Specifies a geographical location associated with a domain name
    ///
    /// [1876](https://datatracker.ietf.org/doc/html/rfc1876)
    #[allow(clippy::upper_case_acronyms)]
    LOC,
    /// Mail exchange record
    ///
    /// List of mail exchange servers that accept email for a domain
    ///
    /// [1035](https://datatracker.ietf.org/doc/html/rfc1035)
    /// [7505](https://datatracker.ietf.org/doc/html/rfc7505)
    #[allow(clippy::upper_case_acronyms)]
    MX,
    /// Naming Authority Pointer
    ///
    /// Allows regular-expression-based rewriting of domain names which can then be used as URIs, further domain names to lookups, etc.
    ///
    /// [3403](https://datatracker.ietf.org/doc/html/rfc3403)
    #[allow(clippy::upper_case_acronyms)]
    NAPTR,
    /// Name server record
    ///
    /// Delegates a DNS zone to use the given authoritative name servers
    ///
    /// [1035](https://datatracker.ietf.org/doc/html/rfc1035)
    #[allow(clippy::upper_case_acronyms)]
    NS,
    /// Next Secure record
    ///
    /// Part of DNSSEC—used to prove a name does not exist. Uses the same format as the (obsolete) NXT record.
    ///
    /// [4034](https://datatracker.ietf.org/doc/html/rfc4034)
    #[allow(clippy::upper_case_acronyms)]
    NSEC,
    /// Next Secure record version 3
    ///
    /// An extension to DNSSEC that allows proof of nonexistence for a name without permitting zonewalking
    ///
    /// [5155](https://datatracker.ietf.org/doc/html/rfc5155)
    #[allow(clippy::upper_case_acronyms)]
    NSEC3,
    /// NSEC3 parameters
    ///
    /// Parameter record for use with NSEC3
    ///
    /// [5155](https://datatracker.ietf.org/doc/html/rfc5155)
    #[allow(clippy::upper_case_acronyms)]
    NSEC3PARAM,
    /// OpenPGP public key record
    ///
    /// A DNS-based Authentication of Named Entities (DANE) method for publishing and locating OpenPGP public keys in DNS for a specific email address using an OPENPGPKEY DNS resource record.
    ///
    /// [7929](https://datatracker.ietf.org/doc/html/rfc7929)
    #[allow(clippy::upper_case_acronyms)]
    OPENPGPKEY,
    /// PTR Resource Record
    ///
    /// Pointer to a canonical name. Unlike a CNAME, DNS processing stops and just the name is returned. The most common use is for implementing reverse DNS lookups, but other uses include such things as DNS-SD.
    ///
    /// [1035](https://datatracker.ietf.org/doc/html/rfc1035)
    #[allow(clippy::upper_case_acronyms)]
    PTR,
    /// DNSSEC signature
    ///
    /// Signature for a DNSSEC-secured record set. Uses the same format as the SIG record.
    ///
    /// [4034](https://datatracker.ietf.org/doc/html/rfc4034)
    #[allow(clippy::upper_case_acronyms)]
    RRSIG,
    /// Responsible Person
    ///
    /// Information about the responsible person(s) for the domain. Usually an email address with the @ replaced by a .
    ///
    /// [1183](https://datatracker.ietf.org/doc/html/rfc1183)
    #[allow(clippy::upper_case_acronyms)]
    RP,
    /// Signature
    ///
    /// Signature record used in SIG(0) (RFC 2931) and TKEY (RFC 2930).[7] RFC 3755 designated RRSIG as the replacement for SIG for use within DNSSEC.[7]
    ///
    /// [2535](https://datatracker.ietf.org/doc/html/rfc2535)
    #[allow(clippy::upper_case_acronyms)]
    SIG,
    /// S/MIME cert association
    ///
    /// Associates an S/MIME certificate with a domain name for sender authentication.
    ///
    /// [8162](https://datatracker.ietf.org/doc/html/rfc8162)
    #[allow(clippy::upper_case_acronyms)]
    SMIMEA,
    /// Start of [a zone of] authority record
    ///
    /// Specifies authoritative information about a DNS zone, including the primary name server, the email of the domain administrator, the domain serial number, and several timers relating to refreshing the zone.
    ///
    /// [1035](https://datatracker.ietf.org/doc/html/rfc1035)
    /// [2308](https://datatracker.ietf.org/doc/html/rfc2308)
    #[allow(clippy::upper_case_acronyms)]
    SOA,
    /// Service locator
    ///
    /// Generalized service location record, used for newer protocols instead of creating protocol-specific records such as MX.
    ///
    /// [2782](https://datatracker.ietf.org/doc/html/rfc2782)
    #[allow(clippy::upper_case_acronyms)]
    SRV,
    /// SSH Public Key Fingerprint
    ///
    /// Resource record for publishing SSH public host key fingerprints in the DNS, in order to aid in verifying the authenticity of the host. RFC 6594 defines ECC SSH keys and SHA-256 hashes. See the IANA SSHFP RR parameters registry for details.
    ///
    /// [4255](https://datatracker.ietf.org/doc/html/rfc4255)
    #[allow(clippy::upper_case_acronyms)]
    SSHFP,
    /// Service Binding
    ///
    /// RR that improves performance for clients that need to resolve many resources to access a domain.
    ///
    /// [9460](https://datatracker.ietf.org/doc/html/rfc9460)
    #[allow(clippy::upper_case_acronyms)]
    SVCB,
    /// DNSSEC Trust Authorities
    ///
    /// Part of a deployment proposal for DNSSEC without a signed DNS root. See the IANA database and Weiler Spec for details. Uses the same format as the DS record.
    #[allow(clippy::upper_case_acronyms)]
    TA,
    /// Transaction Key record
    ///
    /// A method of providing keying material to be used with TSIG that is encrypted under the public key in an accompanying KEY RR.[12]
    ///
    /// [2930](https://datatracker.ietf.org/doc/html/rfc2930)
    #[allow(clippy::upper_case_acronyms)]
    TKEY,
    /// TLSA certificate association
    ///
    /// A record for DANE. RFC 6698 defines "The TLSA DNS resource record is used to associate a TLS server certificate or public key with the domain name where the record is found, thus forming a 'TLSA certificate association'".
    ///
    /// [6698](https://datatracker.ietf.org/doc/html/rfc6698)
    #[allow(clippy::upper_case_acronyms)]
    TLSA,
    /// Transaction Signature
    ///
    /// Can be used to authenticate dynamic updates as coming from an approved client, or to authenticate responses as coming from an approved recursive name server[13] similar to DNSSEC.
    ///
    /// [2845](https://datatracker.ietf.org/doc/html/rfc2845)
    #[allow(clippy::upper_case_acronyms)]
    TSIG,
    /// Text record
    ///
    /// Originally for arbitrary human-readable text in a DNS record. Since the early 1990s, however, this record more often carries machine-readable data, such as specified by RFC 1464, opportunistic encryption, Sender Policy Framework, DKIM, DMARC, DNS-SD, etc.
    ///
    /// [1035](https://datatracker.ietf.org/doc/html/rfc1035)
    #[allow(clippy::upper_case_acronyms)]
    TXT,
    /// Uniform Resource Identifier
    ///
    /// Can be used for publishing mappings from hostnames to URIs.
    ///
    /// [7553](https://datatracker.ietf.org/doc/html/rfc7553)
    #[allow(clippy::upper_case_acronyms)]
    URI,
    /// Message Digests for DNS Zones
    ///
    /// Provides a cryptographic message digest over DNS zone data at rest.
    ///
    /// [8976](https://datatracker.ietf.org/doc/html/rfc8976)
    #[allow(clippy::upper_case_acronyms)]
    ZONEMD,
}
