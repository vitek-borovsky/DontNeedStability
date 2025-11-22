# API Documentation

This document provides a human-readable overview of the key traits and data structures that form the public interface of the DontNeedStability DNS server.

## 1. `Database` Trait

**Location:** `src/db/mod.rs`

The `Database` trait is a fundamental contract that defines how the DNS server interacts with any underlying data storage. It abstracts away the specifics of *how* records are stored, focusing instead on *what* operations can be performed. This allows for easy swapping of database implementations (e.g., from in-memory to a persistent disk-based solution) without altering the core server logic.

**Purpose:** To provide a consistent interface for retrieving and storing DNS records.

**Key Methods:**

*   **`get_record(domain: &str) -> Option<Record>`**
    *   **Description:** Attempts to find and retrieve a DNS record for a given domain name.
    *   **Parameters:**
        *   `domain`: A string slice representing the domain name to look up (e.g., "example.com").
    *   **Returns:**
        *   `Some(Record)`: If a matching DNS record is found.
        *   `None`: If no record exists for the specified domain.

*   **`set_record(&mut self, record: Record)`**
    *   **Description:** Stores a new DNS record or updates an existing one. The database implementation is responsible for handling how duplicates or updates are managed.
    *   **Parameters:**
        *   `record`: The `Record` struct containing all the details of the DNS entry to be stored.

### Current Implementations:

*   **`InMemoryDatabase`**: (Located in `src/db/in_memory.rs`) This is the default implementation, storing all DNS records in the server's memory. It's ideal for testing and scenarios where data persistence isn't critical.

## 2. `Record` Struct

**Location:** `src/db/record.rs`

The `Record` struct is the primary data structure used to represent a single DNS entry. It encapsulates all the necessary information for a DNS record.

**Purpose:** To define the structure and content of a DNS record.

**Key Fields:**

*   **`domain: String`**: The domain name to which this record applies (e.g., "example.com").
*   **`ip_address: String`**: The IP address associated with the domain (e.g., "192.168.1.1" for A records, or a CNAME target).
*   **`record_type: RecordType`**: Specifies the type of DNS record (e.g., A, AAAA, CNAME). This is an enum.
*   **`ttl: u32`**: Time-To-Live. The duration (in seconds) that a DNS resolver should cache this record.

### `RecordType` Enum

This enumeration defines the various types of DNS records supported by the system:

*   `A`: Address record (IPv4)
*   `AAAA`: IPv6 Address record
*   `CNAME`: Canonical Name record
*   `MX`: Mail Exchange record
*   `NS`: Name Server record
*   `PTR`: Pointer record
*   `SOA`: Start of Authority record
*   `SRV`: Service record
*   `TXT`: Text record

## 3. `App` Struct

**Location:** `src/app/app.rs`

The `App` struct serves as the central application context. It holds the shared state and provides methods to interact with the core functionalities, primarily the database. By holding a generic `Database` trait object, `App` remains flexible and decoupled from specific database implementations.

**Purpose:** To manage the application's state and provide high-level access to core services.

**Key Functionality:**

*   **`App::new(db: D)`**: Constructor for creating a new `App` instance, requiring a database implementation `D` that satisfies the `Database` trait.
*   **`get_record(&self, domain: &str) -> Option<Record>`**: A convenience method that delegates the call to the underlying database to retrieve a record.
*   **`set_record(&self, record: Record)`**: A convenience method that delegates the call to the underlying database to store or update a record.