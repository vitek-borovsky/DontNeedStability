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

impl ToString for Record {
    fn to_string(&self) -> String {
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
