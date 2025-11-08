use std::collections::HashMap;

use super::{Database, Record, RecordType};

pub struct InMemoryDatabase {
    /// (Domain, RecordType) -> Record
    records: HashMap<(String, RecordType), Record>,
}

impl InMemoryDatabase {
    pub fn new() -> Self {
         InMemoryDatabase {
            records: HashMap::new(),
        }
    }
}

impl Database for InMemoryDatabase {
    fn lookup_record(&self, fqdn: &str, record_type: RecordType) -> Option<&Record> {
        self.records.get(&(fqdn.to_string(), record_type))
    }

    fn insert_record(&mut self, fqdn: &str, record: Record) -> Result<(), String> {
        let record_type: RecordType = record.get_type();
        let lookup: Option<&Record> = self.lookup_record(fqdn, record_type.clone());
        if let Some(querried_record) = lookup {
            return Err(format!(
                "Cannot insert {:?}, record alerady exists {:?}",
                record,
                querried_record
            ));
        }

        self.records.insert(
            (fqdn.to_string(), record_type),
            record
        );

        Ok(())
    }
}
