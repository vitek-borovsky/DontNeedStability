use std::net::{Ipv4Addr, Ipv6Addr};

/// Represents the type of a DNS record for querying.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
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

/// Represents the class of a DNS record.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum RecordClass {
    IN, // Internet
    // Add other classes if needed
}

impl ToString for RecordClass {
    fn to_string(&self) -> String {
        match self {
            RecordClass::IN => "IN".to_string(),
        }
    }
}

/// Represents the data associated with a DNS record.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum RecordData {
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

impl RecordData {
    pub fn get_type(&self) -> RecordType {
        match self {
            RecordData::A(_) => RecordType::A,
            RecordData::AAAA(_) => RecordType::AAAA,
            RecordData::CNAME(_) => RecordType::CNAME,
            RecordData::MX { .. } => RecordType::MX,
            RecordData::NS(_) => RecordType::NS,
            RecordData::PTR(_) => RecordType::PTR,
            RecordData::SOA { .. } => RecordType::SOA,
            RecordData::SRV { .. } => RecordType::SRV,
            RecordData::TXT(_) => RecordType::TXT,
        }
    }
}

/// Represents a complete DNS resource record.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct ResourceRecord {
    pub name: String,
    pub ttl: u32,
    pub class: RecordClass,
    pub data: RecordData,
}

/// Represents a DNS zone, containing its origin, default TTL, and resource records.
#[derive(Debug, Clone)]
pub struct Zone {
    pub origin: String,
    pub ttl: u32, // Default TTL for records in this zone
    pub records: Vec<ResourceRecord>,
}
