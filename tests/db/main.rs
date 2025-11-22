use dont_need_stability::db::{Database, InMemoryDatabase, RecordData, RecordType, ResourceRecord, Zone};
use dont_need_stability::db::record::RecordClass;
use std::net::Ipv4Addr;

#[test]
fn test_lookup_non_existent_record() {
    let db = InMemoryDatabase::new();
    let domain = "nonexistent.com.";

    let retrieved_record = db.lookup_resource_record(domain, RecordType::A).unwrap();
    assert!(retrieved_record.is_none());
}

#[test]
fn test_insert_and_lookup_a_record() {
    let mut db = InMemoryDatabase::new();
    let domain = "example.com.";
    let ip_addr = Ipv4Addr::new(192, 168, 1, 1);
    let record_data = RecordData::A(ip_addr);
    let resource_record = ResourceRecord {
        name: domain.to_string(),
        ttl: 3600,
        class: RecordClass::IN,
        data: record_data.clone(),
    };
    let zone = Zone {
        origin: domain.to_string(),
        ttl: 3600,
        records: vec![resource_record.clone()],
    };

    // Test insertion
    let result = db.add_zone(zone.clone());
    assert!(result.is_ok());

    // Test lookup
    let retrieved_record = db.lookup_resource_record(domain, RecordType::A).unwrap();
    assert!(retrieved_record.is_some());
    assert_eq!(retrieved_record.unwrap(), &record_data);
}

#[test]
fn test_insert_duplicate_record() {
    let mut db = InMemoryDatabase::new();
    let domain = "example.com.";
    let ip_addr = Ipv4Addr::new(192, 168, 1, 1);
    let record_data = RecordData::A(ip_addr);
    let resource_record = ResourceRecord {
        name: domain.to_string(),
        ttl: 3600,
        class: RecordClass::IN,
        data: record_data.clone(),
    };
    let zone = Zone {
        origin: domain.to_string(),
        ttl: 3600,
        records: vec![resource_record.clone()],
    };

    // Insert the record once
    let result1 = db.add_zone(zone.clone());
    assert!(result1.is_ok());

    // Try to insert the same record again (by adding the same zone)
    let result2 = db.add_zone(zone.clone());
    assert!(result2.is_err());
}

#[test]
fn test_insert_and_lookup_ns_record() {
    let mut db = InMemoryDatabase::new();
    let domain = "example.com.";
    let ns1_str = "ns1.example.com.".to_string();
    let ns2_str = "ns2.example.com.".to_string();

    let record_data1 = RecordData::NS(ns1_str.clone());
    let resource_record1 = ResourceRecord {
        name: domain.to_string(),
        ttl: 3600,
        class: RecordClass::IN,
        data: record_data1.clone(),
    };

    let record_data2 = RecordData::NS(ns2_str.clone());
    let resource_record2 = ResourceRecord {
        name: domain.to_string(),
        ttl: 3600,
        class: RecordClass::IN,
        data: record_data2.clone(),
    };

    let zone = Zone {
        origin: domain.to_string(),
        ttl: 3600,
        records: vec![resource_record1.clone(), resource_record2.clone()],
    };

    // Insert zone with NS records
    let result = db.add_zone(zone.clone());
    assert!(result.is_ok());

    // Lookup NS records
    let retrieved_records = db.lookup_meta_records(domain, RecordType::NS).unwrap();
    assert!(retrieved_records.is_some());
    let records_vec = retrieved_records.unwrap();
    assert_eq!(records_vec.len(), 2);
    assert!(records_vec.contains(&&record_data1));
    assert!(records_vec.contains(&&record_data2));
}

#[test]
fn test_insert_duplicate_soa_record() {
    let mut db = InMemoryDatabase::new();
    let domain = "example.com.";
    let soa_record_data = RecordData::SOA {
        mname: "ns1.example.com.".to_string(),
        rname: "hostmaster.example.com.".to_string(),
        serial: 2023102701,
        refresh: 3600,
        retry: 1800,
        expire: 604800,
        minimum: 600,
    };
    let resource_record = ResourceRecord {
        name: domain.to_string(),
        ttl: 3600,
        class: RecordClass::IN,
        data: soa_record_data.clone(),
    };
    let zone = Zone {
        origin: domain.to_string(),
        ttl: 3600,
        records: vec![resource_record.clone()],
    };

    // Insert SOA record once
    let result1 = db.add_zone(zone.clone());
    assert!(result1.is_ok());

    // Try to insert the same SOA record again
    let result2 = db.add_zone(zone.clone());
    assert!(result2.is_err());
}
