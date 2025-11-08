use dontNeedStability::db::{Database, InMemoryDatabase, Record, RecordType};
use std::net::Ipv4Addr;

#[test]
fn test_lookup_non_existent_record() {
    let db = InMemoryDatabase::new();
    let domain = "nonexistent.com";

    let retrieved_record = db.lookup_resource_record(domain, RecordType::A);
    assert!(retrieved_record.is_none());
}

#[test]
fn test_insert_and_lookup_a_record() {
    let mut db = InMemoryDatabase::new();
    let domain = "example.com";
    let ip_addr = Ipv4Addr::new(192, 168, 1, 1);
    let record = Record::A(ip_addr);

    // Test insertion
    let result = db.insert_record(domain, record.clone());
    assert!(result.is_ok());

    // Test lookup
    let retrieved_record = db.lookup_resource_record(domain, RecordType::A);
    assert!(retrieved_record.is_some());
    assert_eq!(retrieved_record.unwrap(), &record);
}

#[test]
fn test_insert_duplicate_record() {
    let mut db = InMemoryDatabase::new();
    let domain = "example.com";
    let ip_addr = Ipv4Addr::new(192, 168, 1, 1);
    let record = Record::A(ip_addr);

    // Insert the record once
    let result1 = db.insert_record(domain, record.clone());
    assert!(result1.is_ok());

    // Try to insert the same record again
    let result2 = db.insert_record(domain, record.clone());
    assert!(result2.is_err());
}

#[test]
fn test_insert_and_lookup_ns_record() {
    let mut db = InMemoryDatabase::new();
    let domain = "example.com";
    let ns1 = "ns1.example.com".to_string();
    let ns2 = "ns2.example.com".to_string();

    let record1 = Record::NS(ns1.clone());
    let record2 = Record::NS(ns2.clone());

    // Insert first NS record
    let result1 = db.insert_record(domain, record1.clone());
    assert!(result1.is_ok());

    // Insert second NS record
    let result2 = db.insert_record(domain, record2.clone());
    assert!(result2.is_ok());

    // Lookup NS records
    let retrieved_records = db.lookup_meta_records(domain, RecordType::NS);
    assert!(retrieved_records.is_some());
    let records_vec = retrieved_records.unwrap();
    assert_eq!(records_vec.len(), 2);
    assert!(records_vec.contains(&&record1));
    assert!(records_vec.contains(&&record2));
}

#[test]
fn test_insert_duplicate_soa_record() {
    let mut db = InMemoryDatabase::new();
    let domain = "example.com";
    let soa_record = Record::SOA {
        mname: "ns1.example.com".to_string(),
        rname: "hostmaster.example.com".to_string(),
        serial: 2023102701,
        refresh: 3600,
        retry: 1800,
        expire: 604800,
        minimum: 600,
    };

    // Insert SOA record once
    let result1 = db.insert_record(domain, soa_record.clone());
    assert!(result1.is_ok());

    // Try to insert the same SOA record again
    let result2 = db.insert_record(domain, soa_record.clone());
    assert!(result2.is_err());
}
