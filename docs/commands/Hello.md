# `Hello`

Negotiates protocol version and returns connection information.

## 🧩 Description
Initiates a HELLO exchange (RESP3 capable servers) and may return server/client info.

## 🧠 Example
```rust
use valkeyree::types::command_kind::CommandKind;
let cmd = CommandKind::Hello.build()?;
```

## 🔗 References
- 📘 [HELLO](https://valkey.io/commands/hello/)
- 🧾 Source: [`src/types/command_kind.rs`](../../src/types/command_kind.rs)
