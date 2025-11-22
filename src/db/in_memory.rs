use std::collections::HashMap;

use super::{Database, Record, RecordType};

pub struct InMemoryDatabase {
    meta_records: HashMap<(String, RecordType), Vec<Record>>,
    meta_records_types: Vec<RecordType>,

    resource_records: HashMap<(String, RecordType), Record>,
    resource_records_types: Vec<RecordType>,
}

impl InMemoryDatabase {
    pub fn new() -> Self {
         InMemoryDatabase {
            meta_records: HashMap::new(),
            meta_records_types: vec![RecordType::SOA, RecordType::NS],

            resource_records: HashMap::new(),
            resource_records_types: vec![RecordType::A, RecordType::AAAA],
        }
    }
}

impl Database for InMemoryDatabase {
    /// Looks up meta records (SOA, NS) for a given fully qualified domain name (FQDN) and record type.
    ///
    /// # Arguments
    ///
    /// * `fqdn` - The fully qualified domain name to lookup.
    /// * `record_type` - The type of meta record to lookup (e.g., `RecordType::SOA`, `RecordType::NS`).
    ///
    /// # Returns
    ///
    /// A `Result` which is:
    /// - `Ok(Some(Vec<&Record>))` if records are found.
    /// - `Ok(None)` if no records are found.
    /// - `Err(String)` if an invalid record type is provided (e.g., a resource record type).
    ///
    /// # Examples
    ///
    /// ```
    /// use std::net::Ipv4Addr;
    /// use dont_need_stability::db::{Database, InMemoryDatabase, Record, RecordType};
    ///
    /// let mut db = InMemoryDatabase::new();
    /// let domain = "example.com";
    ///
    /// // Insert an SOA record
    /// let soa_record = Record::SOA {
    ///     mname: "ns1.example.com".to_string(),
    ///     rname: "hostmaster.example.com".to_string(),
    ///     serial: 2023102701,
    ///     refresh: 3600,
    ///     retry: 1800,
    ///     expire: 604800,
    ///     minimum: 600,
    /// };
    /// db.insert_record(domain, soa_record.clone()).unwrap();
    ///
    /// // Lookup the SOA record
    /// let retrieved_soa = db.lookup_meta_records(domain, RecordType::SOA).unwrap().unwrap();
    /// assert_eq!(retrieved_soa.len(), 1);
    /// assert_eq!(retrieved_soa[0], &soa_record);
    ///
    /// // Insert multiple NS records
    /// let ns1 = Record::NS("ns1.example.com".to_string());
    /// let ns2 = Record::NS("ns2.example.com".to_string());
    /// db.insert_record(domain, ns1.clone()).unwrap();
    /// db.insert_record(domain, ns2.clone()).unwrap();
    ///
    /// // Lookup NS records
    /// let retrieved_ns = db.lookup_meta_records(domain, RecordType::NS).unwrap().unwrap();
    /// assert_eq!(retrieved_ns.len(), 2);
    /// assert!(retrieved_ns.contains(&&ns1));
    /// assert!(retrieved_ns.contains(&&ns2));
    ///
    /// // Lookup a non-existent meta record
    /// let non_existent = db.lookup_meta_records("nonexistent.com", RecordType::SOA).unwrap();
    /// assert!(non_existent.is_none());
    ///
    /// // Attempt to lookup a resource record type (should error)
    /// let error_result = db.lookup_meta_records(domain, RecordType::A);
    /// assert!(error_result.is_err());
    /// ```
    fn lookup_meta_records(&self, fqdn: &str, record_type: RecordType) -> Result<Option<Vec<&Record>>, String> {
        if ! self.meta_records_types.contains(&record_type) {
            return Err(format!(
                "Cannot use lookup_meta_records with RecordType={:?}, try using lookup_resource_record",
                record_type
            ));
        }
        Ok(self.meta_records
            .get(&(fqdn.to_string(), record_type))
            .map(|vec_records| vec_records.iter().collect()))
    }

    /// Looks up a resource record (A, AAAA) for a given fully qualified domain name (FQDN) and record type.
    ///
    /// # Arguments
    ///
    /// * `fqdn` - The fully qualified domain name to lookup.
    /// * `record_type` - The type of resource record to lookup (e.g., `RecordType::A`, `RecordType::AAAA`).
    ///
    /// # Returns
    ///
    /// A `Result` which is:
    /// - `Ok(Some(&Record))` if a record is found.
    /// - `Ok(None)` if no record is found.
    /// - `Err(String)` if an invalid record type is provided (e.g., a meta record type).
    ///
    /// # Examples
    ///
    /// ```
    /// use std::net::Ipv4Addr;
    /// use dont_need_stability::db::{Database, InMemoryDatabase, Record, RecordType};
    ///
    /// let mut db = InMemoryDatabase::new();
    /// let domain = "example.com";
    /// let ip_addr = Ipv4Addr::new(192, 168, 1, 1);
    /// let a_record = Record::A(ip_addr);
    ///
    /// // Insert an A record
    /// db.insert_record(domain, a_record.clone()).unwrap();
    ///
    /// // Lookup the A record
    /// let retrieved_a = db.lookup_resource_record(domain, RecordType::A).unwrap().unwrap();
    /// assert_eq!(retrieved_a, &a_record);
    ///
    /// // Lookup a non-existent resource record
    /// let non_existent = db.lookup_resource_record("nonexistent.com", RecordType::A).unwrap();
    /// assert!(non_existent.is_none());
    ///
    /// // Attempt to lookup a meta record type (should error)
    /// let error_result = db.lookup_resource_record(domain, RecordType::SOA);
    /// assert!(error_result.is_err());
    /// ```
    fn lookup_resource_record(&self, fqdn: &str, record_type: RecordType) -> Result<Option<&Record>, String> {
        if ! self.resource_records_types.contains(&record_type) {
            return Err(format!(
                "Cannot use lookup_resource_record with RecordType={:?}, try using lookup_meta_records",
                record_type
            ));
        }
        Ok(self.resource_records
            .get(&(fqdn.to_string(), record_type)))
    }

    /// Inserts a DNS record into the database.
    ///
    /// # Arguments
    ///
    /// * `fqdn` - The fully qualified domain name for which to insert the record.
    /// * `record` - The `Record` to insert.
    ///
    /// # Returns
    ///
    /// A `Result` which is:
    /// - `Ok(())` if the record was successfully inserted.
    /// - `Err(String)` if the record already exists (for A, AAAA, SOA) or if there's a type mismatch.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::net::Ipv4Addr;
    /// use dont_need_stability::db::{Database, InMemoryDatabase, Record, RecordType};
    ///
    /// let mut db = InMemoryDatabase::new();
    /// let domain = "example.com";
    ///
    /// // Insert an A record
    /// let a_record = Record::A(Ipv4Addr::new(192, 168, 1, 1));
    /// assert!(db.insert_record(domain, a_record.clone()).is_ok());
    ///
    /// // Attempt to insert a duplicate A record
    /// assert!(db.insert_record(domain, a_record.clone()).is_err());
    ///
    /// // Insert an SOA record
    /// let soa_record = Record::SOA {
    ///     mname: "ns1.example.com".to_string(),
    ///     rname: "hostmaster.example.com".to_string(),
    ///     serial: 2023102701,
    ///     refresh: 3600,
    ///     retry: 1800,
    ///     expire: 604800,
    ///     minimum: 600,
    /// };
    /// assert!(db.insert_record(domain, soa_record.clone()).is_ok());
    ///
    /// // Attempt to insert a duplicate SOA record
    /// assert!(db.insert_record(domain, soa_record.clone()).is_err());
    ///
    /// // Insert multiple NS records
    /// let ns1 = Record::NS("ns1.example.com".to_string());
    /// let ns2 = Record::NS("ns2.example.com".to_string());
    /// assert!(db.insert_record(domain, ns1.clone()).is_ok());
    /// assert!(db.insert_record(domain, ns2.clone()).is_ok());
    ///
    /// // Attempt to insert a duplicate NS record (should not error, but not add a duplicate)
    /// assert!(db.insert_record(domain, ns1.clone()).is_ok());
    /// let retrieved_ns = db.lookup_meta_records(domain, RecordType::NS).unwrap().unwrap();
    /// assert_eq!(retrieved_ns.len(), 2); // Still 2 records
    /// ```
    fn insert_record(&mut self, fqdn: &str, record: Record) -> Result<(), String> {
        let record_type = record.get_type();
        let fqdn_string = fqdn.to_string();
        let key = (fqdn_string.clone(), record_type.clone());

        match record_type {
            RecordType::NS => {
                let records_vec = self.meta_records.entry(key).or_insert_with(Vec::new);
                if let Record::NS(new_ns) = record {
                    if !records_vec.iter().any(|r| match r { Record::NS(existing_ns) => existing_ns == &new_ns, _ => false }) {
                        records_vec.push(Record::NS(new_ns));
                    }
                } else {
                    return Err(format!("Type mismatch: Expected NS record, got {:?}", record));
                }
            }
            RecordType::SOA => {
                if self.meta_records.contains_key(&key) {
                    return Err(format!(
                        "Cannot insert {:?}, SOA record already exists for {}",
                        record,
                        fqdn
                    ));
                }
                self.meta_records.insert(key, vec![record]);
            }
            RecordType::A | RecordType::AAAA=> {
                if self.resource_records.contains_key(&key) {
                    return Err(format!(
                        "Cannot insert {:?}, record already exists for {}",
                        record,
                        fqdn
                    ));
                }
                self.resource_records.insert(key, record);
            }
            _ => eprintln!("Undefined RecordType"),
        }
        Ok(())
    }
}

