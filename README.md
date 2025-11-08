# DontNeedStability
DNS server

## Architecture

The project utilizes a `Database` trait for abstracting database operations, located in `src/db/mod.rs`. An in-memory implementation, `InMemoryDatabase`, is provided in `src/db/in_memory.rs` for development and testing purposes.
