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
    fn lookup_meta_records(&self, fqdn: &str, record_type: RecordType) -> Option<Vec<&Record>> {
        if ! self.meta_records_types.contains(&record_type) {
            // TODO Throw exception
        }
        self.meta_records.get(&(fqdn.to_string(), record_type)).map(|vec_records| vec_records.iter().collect())
    }

    fn lookup_resource_record(&self, fqdn: &str, record_type: RecordType) -> Option<&Record> {
        if ! self.resource_records_types.contains(&record_type) {
            // TODO Throw exception
        }
        self.resource_records.get(&(fqdn.to_string(), record_type))
    }

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
