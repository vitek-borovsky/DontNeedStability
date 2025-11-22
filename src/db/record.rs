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
    pub fn get_type(&self) -> RecordType {
        match self {
            Record::A(_) => RecordType::A,
            Record::AAAA(_) => RecordType::AAAA,
            Record::CNAME(_) => RecordType::CNAME,
            Record::MX { .. } => RecordType::MX,
            Record::NS(_) => RecordType::NS,
            Record::PTR(_) => RecordType::PTR,
            Record::SOA { .. } => RecordType::SOA,
            Record::SRV { .. } => RecordType::SRV,
            Record::TXT(_) => RecordType::TXT,
        }
    }
}
