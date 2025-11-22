# Configuration Guide

This document explains how to configure the DontNeedStability DNS server.

## 1. Server Address and Port

The server's listening IP address and port are configured via `config.toml` at the project root. By default, the server listens on `127.0.0.1:8080`.

**`config.toml` example:**

```toml
# Configuration for DontNeedStability DNS Server

[server]
port = 8080
```

To change the port, modify the `port` value in `config.toml`.

## 2. DNS Zones and Records

DNS zones and their associated records are defined using standard BIND-style zone files. These files should be placed in the `zones/` directory at the project root.

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

## 3. Database Implementation

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