# Architecture

The DontNeedStability project is a DNS server built with Rust, designed for modularity and extensibility.

## Core Components and Data Flow

### 1. Database Trait (`src/db/mod.rs`)

At the heart of the data management is the `Database` trait. This trait acts as an abstraction layer, defining a standard interface for all database operations. This design allows the DNS server to work with various database backends without needing to know their specific implementation details. Any database solution, whether in-memory or persistent, can be integrated by simply implementing this trait.

**Key Operations:**
*   **`get_record(domain: &str)`**: Retrieves a DNS record associated with a given domain name.
*   **`set_record(record: Record)`**: Stores or updates a DNS record.

### 2. In-Memory Database (`src/db/in_memory.rs`)

`InMemoryDatabase` is a concrete implementation of the `Database` trait. It's designed for development, testing, and scenarios where data persistence across server restarts isn't required. It stores DNS records in a simple hash map, providing fast lookups and updates within the server's runtime.

### 3. Application State (`src/app/app.rs`)

The `App` struct encapsulates the overall application state. Crucially, it holds a reference to an instance of a type that implements the `Database` trait. This means the `App` can interact with any database backend through the unified `Database` interface, without being coupled to a specific implementation like `InMemoryDatabase`.

### 4. Server (`src/server.rs`)

The `Server` component is the entry point for all DNS queries. Its primary responsibilities include:
*   **Listening for Queries**: It continuously monitors a specified network address and port for incoming DNS requests.
*   **Query Processing**: Upon receiving a query, the server parses the incoming data to identify the requested domain name.
*   **Database Interaction**: It then uses the `App`'s database interface to `get_record` for the requested domain.
*   **Response Generation**: Based on the database's response (either a found record or an indication that the record doesn't exist), the server constructs an appropriate DNS response packet.
*   **Sending Response**: Finally, the server sends the crafted response back to the client that initiated the query.

## Overall Data Flow

1.  **Server Initialization**: When the application starts (`src/main.rs`), an `InMemoryDatabase` (by default) is created and wrapped within an `App` instance. The `Server` is then initialized with this `App` instance.
2.  **Client Query**: A DNS client sends a query (e.g., for `example.com`) to the server's listening address and port.
3.  **Server Receives Query**: The `Server` component receives and parses the incoming query, extracting the domain name.
4.  **Application Lookup**: The `Server` delegates the domain lookup to the `App` instance, which in turn calls the `get_record` method on its internal `Database` trait object.
5.  **Database Response**: The `InMemoryDatabase` (or any other implemented database) searches for the record and returns it to the `App`.
6.  **Server Responds**: The `App` passes the record back to the `Server`. The `Server` then formats this information into a standard DNS response and sends it back to the client.

This modular design ensures that the server logic is separated from database concerns, making the system easier to test, maintain, and extend with different database solutions in the future.