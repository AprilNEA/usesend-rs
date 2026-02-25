# usesend-rs

[![Crates.io (usesend)](https://img.shields.io/crates/v/usesend.svg)](https://crates.io/crates/usesend)
[![Crates.io (usesend-api)](https://img.shields.io/crates/v/usesend-api.svg)](https://crates.io/crates/usesend-api)
[![docs.rs](https://docs.rs/usesend/badge.svg)](https://docs.rs/usesend)
[![CI](https://github.com/AprilNEA/usesend-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/AprilNEA/usesend-rs/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

Rust SDK for [useSend](https://usesend.com) — an open-source Resend alternative.

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
usesend = "0.1"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

Send an email:

```rust,no_run
#[tokio::main]
async fn main() -> usesend::ApiResult<()> {
    let client = usesend::UseSend::new("us_api_key");

    let resp = client.emails.build()
        .from("hello@example.com")
        .to("user@example.com")
        .subject("Hello!")
        .html("<h1>Hi</h1>")
        .send()
        .await?;

    println!("Sent: {}", resp.email_id);
    Ok(())
}
```

## Feature Flags

| Flag | Default | Description |
|------|---------|-------------|
| `native-tls` | ✅ | Use the platform-native TLS backend (OpenSSL / Schannel / Secure Transport) |
| `rustls-tls` | ❌ | Use [rustls](https://github.com/rustls/rustls) for TLS (pure Rust, no system dependency) |

To use `rustls-tls` instead of the default:

```toml
[dependencies]
usesend = { version = "0.1", default-features = false, features = ["rustls-tls"] }
```

## Crate Structure

This repository contains two crates:

| Crate | Description |
|-------|-------------|
| [`usesend`](./usesend) | High-level, ergonomic SDK with builder-based API |
| [`usesend-api`](./usesend-api) | Low-level typed HTTP client and request/response models |

Most users should depend on **`usesend`**. Use `usesend-api` directly only if you need raw access to the API types or want to build your own abstraction.

## License

[MIT](LICENSE) © 2024-2026 AprilNEA
