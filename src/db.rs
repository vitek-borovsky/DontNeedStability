pub mod in_memory;
pub mod record;

pub use self::record::{RecordType, RecordData, ResourceRecord, Zone};
pub use self::in_memory::InMemoryDatabase;

/// A trait for database operations.
pub trait Database {
    fn lookup_meta_records(&self, fqdn: &str, record_type: RecordType) -> Result<Option<Vec<&RecordData>>, String>;
    /// Looks up a record in the database.
    ///
    /// # Arguments
    ///
    /// * `fqdn` - The fully qualified domain name to look up.
    /// * `record_type` - The type of record to look up (e.g., `RecordType::A`).
    ///
    /// # Returns
    ///
    /// An `Option` containing the `RecordData` if found, otherwise `None`.
    fn lookup_resource_record(&self, fqdn: &str, record_type: RecordType) -> Result<Option<&RecordData>, String>;
    fn add_zone(&mut self, zone: Zone) -> Result<(), String>;
}


