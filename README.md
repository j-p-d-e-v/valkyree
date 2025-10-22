# 🦀 Valkeyree

> *“The messenger between realms.”*

Valkeyree rides the async winds of **Tokio**, carrying your commands from **Rust** to **Valkey** in pure RESP form — fearless, fast, and forged in Rust. ⚡  
A modern, async library for **crafting**, **sending**, and **decoding** Valkey commands — elegant in design and type-safe in execution.

---

## 📦 Installation

```bash
cargo add valkeyree
```

📚 **Crates.io:** [https://crates.io/crates/valkeyree](https://crates.io/crates/valkeyree)

---

## ✨ Features

- ⚡ **Async I/O** powered by [Tokio](https://tokio.rs/)
- 🧱 Low-level **RESP2** encoder / decoder *(RESP3 support planned)*
- 🧩 Command builders & typed result enums
- 🔐 Authentication & ACL-ready
- 🚀 Designed for pipelining and future Valkey releases

---

## 🧠 Usage Example

```rust
use valkeyree::transport::{
    connection::ConnectionConfig,
    client::Client,
};
use valkeyree::types::command_kind::CommandKind;
use serde_json::Value;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Connection configuration
    let config = ConnectionConfig {
        address: "127.0.0.1:6379".to_string(),
        username: Some("myapp".to_string()),
        password: Some("password123".to_string()),
    };

    // Create a new client
    let client = Client::new(config).await?;

    // Send a SET command
    let set_command = CommandKind::Set("myclient".into(), Value::String("hey".into()));
    client.send(set_command).await?;

    // Send a GET command
    let get_command = CommandKind::Get("myclient".into());
    let result = client.send(get_command).await?;

    println!("Value: {:?}", result);

    Ok(())
}
```

---

## 📘 Documentation

See the list of supported Valkey commands here:  
👉 [**Supported Commands**](./docs/supported-commands.md)

---

## ⚖️ License

This project is licensed under the terms of the [MIT License](./LICENSE).

---

## 👤 Developer

**JP Mateo**  
*Rust Developer & Network Automation Enthusiast*  
[GitHub: j-p-d-e-v](https://github.com/j-p-d-e-v)
