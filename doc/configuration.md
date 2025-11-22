# Configuration Guide

This document explains how to configure the DontNeedStability DNS server.

## 1. Specifying the Configuration File

By default, the server looks for `config.toml` in the project root. You can specify a different configuration file using the `-c` or `--config` command-line argument:

```bash
cargo run -- -c /path/to/your/custom_config.toml
```

## 2. Server Address, Port, and Zones Directory

The server's listening IP address, port, and the directory where DNS zone files are located are configured via `config.toml`. By default, the server listens on `127.0.0.1:8080` and loads zone files from the `zones/` directory.

**`config.toml` example:**

```toml
# Configuration for DontNeedStability DNS Server

[server]
port = 8080
zones_directory = "zones" # Path to the directory containing zone files
```

To change the port or zones directory, modify the respective values in `config.toml`.

## 3. DNS Zones and Records

DNS zones and their associated records are defined using standard BIND-style zone files. These files should be placed in the directory specified by `zones_directory` in `config.toml`.

Each zone file should represent a single DNS zone and be named after the zone (e.g., `example.com.zone`).

**Example `zones/example.com.zone`:**

```dns
$ORIGIN example.com.
$TTL 3600

@       IN      SOA     ns1.example.com. admin.example.com. (
                        2023112201 ; Serial
                        7200       ; Refresh
                        3600       ; Retry
                        1209600    ; Expire
                        3600       ; Minimum TTL
                        )

@       IN      NS      ns1.example.com.
@       IN      NS      ns2.example.com.

ns1     IN      A       192.0.2.1
ns2     IN      A       192.0.2.2

www     IN      A       192.0.2.3
mail    IN      A       192.0.2.4
        IN      MX      10 mail.example.com.
```

### Supported Record Types:

The zone parser currently supports the following DNS record types:

*   `A` (IPv4 Address)
*   `AAAA` (IPv6 Address)
*   `CNAME` (Canonical Name)
*   `MX` (Mail Exchange)
*   `NS` (Name Server)
*   `SOA` (Start of Authority)
*   `TXT` (Text Record)

## 4. Database Implementation

By default, the DontNeedStability server uses an `InMemoryDatabase`, which stores all DNS records in the server's volatile memory. This is suitable for testing and non-persistent use cases.

**How to Change:**

If you were to implement a different database backend (e.g., a persistent database), you would modify the `main` function in `src/main.rs` to instantiate and use your new database implementation.

**Current Default (in `src/main.rs`):**

```rust
// Initializes the in-memory database
let db = InMemoryDatabase::new();
// Creates the application instance with the in-memory database
let app = App::new(Box::new(db), socket);
```