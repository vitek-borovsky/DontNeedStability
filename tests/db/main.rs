use dontNeedStability::db::{Database, InMemoryDatabase, Record, RecordType};
use std::net::Ipv4Addr;

#[test]
fn test_lookup_non_existent_record() {
    let db = InMemoryDatabase::new();
    let domain = "nonexistent.com";

    let retrieved_record = db.lookup_record(domain, RecordType::A);
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
    let retrieved_record = db.lookup_record(domain, RecordType::A);
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
