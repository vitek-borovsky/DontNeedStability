# DontNeedStability

DNS server

## Usage

To run the DNS server, use the following command:

```bash
cargo run
```

By default, the server will listen on `127.0.0.1:8080` and use an in-memory database. You can configure the server by modifying `src/main.rs`.

Example query using `dig`:

```bash
dig @127.0.0.1 -p 8080 example.com
```

## Architecture

The project utilizes a `Database` trait for abstracting database operations, located in `src/db/mod.rs`. An in-memory implementation, `InMemoryDatabase`, is provided in `src/db/in_memory.rs` for development and testing purposes.
