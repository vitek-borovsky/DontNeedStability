# Database Interface

The `Database` trait, defined in `src/db/mod.rs`, provides a common interface for database operations within the DNS server. It uses `RecordType` for querying and the `Record` enum for storing and returning DNS records, encapsulating both the record type and its associated data.

An in-memory implementation, `InMemoryDatabase`, is also provided in `src/db/in_memory.rs` for development and testing. This implementation uses a `HashMap` to store DNS records.

```rust
/// Represents the type of a DNS record for querying.
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum RecordType {
    A,
    AAAA,
    CNAME,
    MX,
    NS,
    PTR,
    SOA,
    SRV,
    TXT,
}

impl ToString for RecordType {
    fn to_string(&self) -> String {
        match self {
            RecordType::A => "A".to_string(),
            RecordType::AAAA => "AAAA".to_string(),
            RecordType::CNAME => "CNAME".to_string(),
            RecordType::MX => "MX".to_string(),
            RecordType::NS => "NS".to_string(),
            RecordType::PTR => "PTR".to_string(),
            RecordType::SOA => "SOA".to_string(),
            RecordType::SRV => "SRV".to_string(),
            RecordType::TXT => "TXT".to_string(),
        }
    }
}

use std::net::{Ipv4Addr, Ipv6Addr};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Record {
    A(Ipv4Addr),
    AAAA(Ipv6Addr),
    CNAME(String),
    MX { preference: u16, exchange: String },
    NS(String),
    PTR(String),
    SOA {
        mname: String,
        rname: String,
        serial: u32,
        refresh: u32,
        retry: u32,
        expire: u32,
        minimum: u32,
    },
    SRV { priority: u16, weight: u16, port: u16, target: String },
    TXT(String),
}

impl Record {
    pub fn get_type(&self) -> String {
        match self {
            Record::A(_) => "A".to_string(),
            Record::AAAA(_) => "AAAA".to_string(),
            Record::CNAME(_) => "CNAME".to_string(),
            Record::MX { .. } => "MX".to_string(),
            Record::NS(_) => "NS".to_string(),
            Record::PTR(_) => "PTR".to_string(),
            Record::SOA { .. } => "SOA".to_string(),
            Record::SRV { .. } => "SRV".to_string(),
            Record::TXT(_) => "TXT".to_string(),
        }
    }
}

pub trait Database {
    fn lookup_record(&self, domain: &str, record_type: RecordType) -> Option<Record>;
    fn insert_record(&mut self, domain: &str, record: Record) -> Result<(), String>;
}
```
