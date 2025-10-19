# Valkeyree

Valkeyree — the messenger between realms.

She rides the async winds of Tokio, carrying your commands from Rust to Valkey and back in pure RESP form.
With Serde as her charm, she translates your data effortlessly — no scrolls, no spells, just type-safe speed. ⚡

A modern Rust library for crafting, sending, and decoding Valkey commands — elegant, async, and fearless.

## Installing
```
cargo add valkeyree
```

### ✨ Features
- ⚡ Async I/O powered by **Tokio**
- 🧱 Low-level **RESP2/RESP3(In the future)** encoder & decoder
- 🧩 Command builders and typed result enums
- 🔐 Authentication & ACL-ready
- 🧠 **Serde**-friendly data serialization
- 🚀 Built for pipelining and future Valkey releases

### Usage
```rust
let config = ConnectionConfig {
    address: "127.0.0.1:6379".to_string(),
    username: Some("myapp".to_string()),
    password: Some("password123".to_string()),
};
// Create a new client
let client = Client::new(config).await?;
// Send a SET command
let set_command = CommandKind::Set("myclient".into(), Value::String("hey".into()));
let result = client.send(set_command).await?;
// Send a GET command
let get_command = CommandKind::Get("myclient".into());
let result = client.send(get_command).await?;
```

# License 
See [LICENSE](/LICENSE) for details.
