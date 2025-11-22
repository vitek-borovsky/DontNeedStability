use std::net::{Ipv4Addr, Ipv6Addr};
use std::str::FromStr;

use super::db::record::{RecordClass, RecordData, RecordType, ResourceRecord, Zone};

pub struct ZoneParser;

impl ZoneParser {
    pub fn parse_zone_file(content: &str, default_origin: String, default_ttl: u32) -> Result<Zone, String> {
        let mut origin = default_origin;
        let mut ttl = default_ttl;
        let mut records: Vec<ResourceRecord> = Vec::new();

        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with(';') {
                continue;
            }

            let parts: Vec<&str> = line.split_whitespace().collect();

            if parts[0].starts_with('$') {
                // Handle directives
                match parts[0] {
                    "$ORIGIN" => {
                        if parts.len() < 2 {
                            return Err(format!("Invalid $ORIGIN directive: {}", line));
                        }
                        origin = parts[1].to_string();
                    }
                    "$TTL" => {
                        if parts.len() < 2 {
                            return Err(format!("Invalid $TTL directive: {}", line));
                        }
                        ttl = parts[1].parse::<u32>().map_err(|e| format!("Invalid $TTL value: {}: {}", parts[1], e))?;
                    }
                    _ => eprintln!("Unknown directive: {}", parts[0]),
                }
            } else {
                // Handle resource records
                let mut current_name = "@".to_string();
                let mut current_ttl = ttl;
                let mut current_class = RecordClass::IN;
                let mut i = 0;

                if !parts[i].starts_with('@') && !parts[i].ends_with('.') {
                    current_name = parts[i].to_string();
                    i += 1;
                }

                if i < parts.len() && parts[i].parse::<u32>().is_ok() {
                    current_ttl = parts[i].parse::<u32>().unwrap();
                    i += 1;
                }

                if i < parts.len() && RecordClass::from_str(parts[i]).is_ok() {
                    current_class = RecordClass::from_str(parts[i]).unwrap();
                    i += 1;
                }

                let record_type_str = if i < parts.len() && RecordType::from_str(parts[i]).is_ok() {
                    let r_type = parts[i];
                    i += 1;
                    r_type
                } else {
                    return Err(format!("Missing or invalid record type: {}", line));
                };

                let record_data_str = parts[i..].join(" ");
                let record_data = match record_type_str {
                    "A" => RecordData::A(Ipv4Addr::from_str(&record_data_str).map_err(|e| format!("Invalid A record data: {}: {}", record_data_str, e))?),
                    "AAAA" => RecordData::AAAA(Ipv6Addr::from_str(&record_data_str).map_err(|e| format!("Invalid AAAA record data: {}: {}", record_data_str, e))?),
                    "CNAME" => RecordData::CNAME(record_data_str),
                    "NS" => RecordData::NS(record_data_str),
                    "MX" => {
                        let mx_parts: Vec<&str> = record_data_str.split_whitespace().collect();
                        if mx_parts.len() < 2 {
                            return Err(format!("Invalid MX record data: {}", record_data_str));
                        }
                        let preference = mx_parts[0].parse::<u16>().map_err(|e| format!("Invalid MX preference: {}: {}", mx_parts[0], e))?;
                        let exchange = mx_parts[1..].join(" ");
                        RecordData::MX { preference, exchange }
                    },
                    "SOA" => {
                        let soa_parts: Vec<&str> = record_data_str.split_whitespace().collect();
                        if soa_parts.len() < 7 {
                            return Err(format!("Invalid SOA record data: {}", record_data_str));
                        }
                        RecordData::SOA {
                            mname: soa_parts[0].to_string(),
                            rname: soa_parts[1].to_string(),
                            serial: soa_parts[2].parse::<u32>().map_err(|e| format!("Invalid SOA serial: {}: {}", soa_parts[2], e))?,
                            refresh: soa_parts[3].parse::<u32>().map_err(|e| format!("Invalid SOA refresh: {}: {}", soa_parts[3], e))?,
                            retry: soa_parts[4].parse::<u32>().map_err(|e| format!("Invalid SOA retry: {}: {}", soa_parts[4], e))?,
                            expire: soa_parts[5].parse::<u32>().map_err(|e| format!("Invalid SOA expire: {}: {}", soa_parts[5], e))?,
                            minimum: soa_parts[6].parse::<u32>().map_err(|e| format!("Invalid SOA minimum: {}: {}", soa_parts[6], e))?,
                        }
                    },
                    "TXT" => RecordData::TXT(record_data_str),
                    _ => return Err(format!("Unsupported record type: {}", record_type_str)),
                };

                let fqdn = if current_name == "@" {
                    origin.clone()
                } else if current_name.ends_with('.') {
                    current_name
                } else {
                    format!("{}.{}", current_name, origin)
                };

                records.push(ResourceRecord {
                    name: fqdn,
                    ttl: current_ttl,
                    class: current_class,
                    data: record_data,
                });
            }
        }

        Ok(Zone {
            origin,
            ttl,
            records,
        })
    }
}

impl FromStr for RecordType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "A" => Ok(RecordType::A),
            "AAAA" => Ok(RecordType::AAAA),
            "CNAME" => Ok(RecordType::CNAME),
            "MX" => Ok(RecordType::MX),
            "NS" => Ok(RecordType::NS),
            "PTR" => Ok(RecordType::PTR),
            "SOA" => Ok(RecordType::SOA),
            "SRV" => Ok(RecordType::SRV),
            "TXT" => Ok(RecordType::TXT),
            _ => Err(format!("Unknown RecordType: {}", s)),
        }
    }
}

impl FromStr for RecordClass {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "IN" => Ok(RecordClass::IN),
            _ => Err(format!("Unknown RecordClass: {}", s)),
        }
    }
}
