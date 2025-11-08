use std::collections::HashMap;

use super::{Database, Record, RecordType};

pub struct InMemoryDatabase {
    records: HashMap<String, HashMap<String, String>>,
}

impl InMemoryDatabase {
    pub fn new() -> Self {
         InMemoryDatabase {
            records: HashMap::new(),
        }
    }
}

impl Database for InMemoryDatabase {
    fn lookup_record(&self, domain: &str, record_type: RecordType) -> Option<Record> {
        self.records.get(domain)
            .and_then(|domain_records| domain_records.get(&record_type.to_string()))
            .and_then(|record_str| {
                // This is a placeholder for actual deserialization
                // In a real application, you'd use a proper serialization library like serde
                // For now, we'll just try to parse it back into a Record based on its type
                match record_type {
                    RecordType::A => record_str.parse::<std::net::Ipv4Addr>().ok().map(Record::A),
                    RecordType::AAAA => record_str.parse::<std::net::Ipv6Addr>().ok().map(Record::AAAA),
                    RecordType::CNAME => Some(Record::CNAME(record_str.clone())),
                    RecordType::NS => Some(Record::NS(record_str.clone())),
                    RecordType::PTR => Some(Record::PTR(record_str.clone())),
                    RecordType::TXT => Some(Record::TXT(record_str.clone())),
                    // For complex records, this simple parsing won't work. 
                    // A proper serialization library would be needed here.
                    _ => None, // Placeholder for MX, SOA, SRV
                }
            })
    }

    fn insert_record(&mut self, domain: &str, record: Record) -> Result<(), String> {
        let record_type_str = record.get_type();
        let record_value_str = match record {
            Record::A(ip) => ip.to_string(),
            Record::AAAA(ip) => ip.to_string(),
            Record::CNAME(name) => name,
            Record::MX { preference, exchange } => format!("{} {}", preference, exchange),
            Record::NS(name) => name,
            Record::PTR(name) => name,
            Record::SOA { mname, rname, serial, refresh, retry, expire, minimum } => {
                format!("{} {} {} {} {} {} {}", mname, rname, serial, refresh, retry, expire, minimum)
            },
            Record::SRV { priority, weight, port, target } => {
                format!("{} {} {} {}", priority, weight, port, target)
            },
            Record::TXT(text) => text,
        };

        self.records.entry(domain.to_string())
            .or_insert_with(HashMap::new)
            .insert(record_type_str, record_value_str);
        Ok(())
    }
}
