# Configuration Guide

This document explains how to configure the DontNeedStability DNS server. Currently, configuration is primarily handled by directly modifying the `src/main.rs` file.

## 1. Server Address and Port

The server's listening IP address and port are defined within the `src/main.rs` file. To change where the server listens for incoming DNS queries, you need to modify the `SocketAddr` creation line.

**How to Change:**

Locate the following line in `src/main.rs`:

```rust
let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
```

*   To change the IP address, modify the array `[127, 0, 0, 1]` to your desired IPv4 address (e.g., `[0, 0, 0, 0]` to listen on all available network interfaces).
*   To change the port, modify `8080` to your preferred port number.

**Example:** To make the server listen on all interfaces on port `5353`:

```rust
let addr = SocketAddr::from(([0, 0, 0, 0], 5353));
```

## 2. Database Implementation

By default, the DontNeedStability server uses an `InMemoryDatabase`, which stores all DNS records in the server's volatile memory. This is suitable for testing and non-persistent use cases.

**How to Change:**

If you were to implement a different database backend (e.g., a persistent database), you would modify the `main` function in `src/main.rs` to instantiate and use your new database implementation.

**Current Default (in `src/main.rs`):**

```rust
// Initializes the in-memory database
let db = InMemoryDatabase::new();
// Creates the application instance with the in-memory database
let app = App::new(db);
```

## Future Configuration Enhancements

For more flexible configuration without requiring code changes, future versions of the server could incorporate:

*   **Command-line Arguments**: Allowing users to specify settings directly when launching the server (e.g., using libraries like `clap`).
*   **Configuration Files**: Reading settings from external files (e.g., `config.toml`, `config.yaml`) using libraries like `config`.
*   **Environment Variables**: Enabling configuration through environment variables, which is common in containerized deployments.
