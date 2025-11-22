# DontNeedStability

DNS server

## Usage

To run the DNS server, use the following command:

```bash
cargo run -- -c config.toml
```

Alternatively, you can omit the `-c` flag to use the default `config.toml` in the project root:

```bash
cargo run
```

The server's port and zones directory are configured via `config.toml`. By default, it listens on `127.0.0.1:8080` and uses an in-memory database, loading zone files from the `zones/` directory.

Example query using `dig`:

```bash
dig @127.0.0.1 -p 8080 example.com
```

## Architecture

The project utilizes a `Database` trait for abstracting database operations, located in `src/db/mod.rs`. An in-memory implementation, `InMemoryDatabase`, is provided in `src/db/in_memory.rs` for development and testing purposes. The main application logic is now consolidated in `src/app.rs`.
