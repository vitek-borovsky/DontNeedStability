pub mod in_memory;
pub mod record;
pub use in_memory::InMemoryDatabase;
pub use record::Record;

/// Represents the type of a DNS record for querying.
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum RecordType {
    A,
    AAAA,
    CNAME,
    MX,
    NS,
    PTR,
    SOA,
    SRV,
    TXT,
}

impl ToString for RecordType {
    fn to_string(&self) -> String {
        match self {
            RecordType::A => "A".to_string(),
            RecordType::AAAA => "AAAA".to_string(),
            RecordType::CNAME => "CNAME".to_string(),
            RecordType::MX => "MX".to_string(),
            RecordType::NS => "NS".to_string(),
            RecordType::PTR => "PTR".to_string(),
            RecordType::SOA => "SOA".to_string(),
            RecordType::SRV => "SRV".to_string(),
            RecordType::TXT => "TXT".to_string(),
        }
    }
}

/// A trait for database operations.
pub trait Database {
    /// Looks up a record in the database.
    ///
    /// # Arguments
    ///
    /// * `domain` - The domain name to look up.
    /// * `record_type` - The type of record to look up (e.g., `RecordType::A`).
    ///
    /// # Returns
    ///
    /// An `Option` containing the `Record` if found, otherwise `None`.
    fn lookup_record(&self, domain: &str, record_type: RecordType) -> Option<Record>;
    /// Inserts a record into the database.
    ///
    /// # Arguments
    ///
    /// * `domain` - The domain name for the record.
    /// * `record` - The `Record` to insert.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success (`Ok(())`) or an error (`Err(String)`) if the insertion fails.
    fn insert_record(&mut self, domain: &str, record: Record) -> Result<(), String>;
}
