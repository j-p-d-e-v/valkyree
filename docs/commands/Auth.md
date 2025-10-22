# `Auth(AuthConfig)`

Authenticates the current Valkey connection.

## 🧩 Description
Verifies a client’s credentials using `username` and `password` provided via `AuthConfig`.

## 🧠 Example
```rust
use valkeyree::types::command_kind::CommandKind;
use valkeyree::builder::commands::AuthConfig;
let cmd = CommandKind::Auth(AuthConfig {
    username: config.username,
    password: config.password,
}).build()?;
```

## 🔗 References
- 📘 [AUTH](https://valkey.io/commands/auth/)
- 🧾 Source: [`src/types/command_kind.rs`](../../src/types/command_kind.rs)
