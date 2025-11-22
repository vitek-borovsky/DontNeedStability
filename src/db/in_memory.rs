use std::collections::HashMap;

use super::{Database, RecordData, RecordType, Zone};

pub struct InMemoryDatabase {
    zones: HashMap<String, Zone>,
}

impl InMemoryDatabase {
    pub fn new() -> Self {
        InMemoryDatabase {
            zones: HashMap::new(),
        }
    }

    pub fn add_zone(&mut self, zone: Zone) -> Result<(), String> {
        if self.zones.contains_key(&zone.origin) {
            return Err(format!("Zone {} already exists", zone.origin));
        }
        self.zones.insert(zone.origin.clone(), zone);
        Ok(())
    }
}

impl Database for InMemoryDatabase {
    fn lookup_meta_records(&self, fqdn: &str, record_type: RecordType) -> Result<Option<Vec<&RecordData>>, String> {
        if let Some(zone) = self.zones.get(fqdn) {
            let records: Vec<&RecordData> = zone.records.iter()
                .filter(|rec| rec.data.get_type() == record_type)
                .map(|rec| &rec.data)
                .collect();
            if records.is_empty() {
                Ok(None)
            } else {
                Ok(Some(records))
            }
        } else {
            Ok(None)
        }
    }

    fn lookup_resource_record(&self, fqdn: &str, record_type: RecordType) -> Result<Option<&RecordData>, String> {
        if let Some(zone) = self.zones.get(fqdn) {
            let record = zone.records.iter()
                .find(|rec| rec.data.get_type() == record_type)
                .map(|rec| &rec.data);
            Ok(record)
        } else {
            Ok(None)
        }
    }

    fn add_zone(&mut self, zone: Zone) -> Result<(), String> {
        InMemoryDatabase::add_zone(self, zone)
    }
}