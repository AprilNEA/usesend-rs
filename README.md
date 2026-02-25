<div align="center">

# usesend-rs

**Rust SDK for [useSend](https://usesend.com) — an open-source Resend alternative.**

[![Crates.io (usesend)](https://img.shields.io/crates/v/usesend.svg)](https://crates.io/crates/usesend)
[![Crates.io (usesend-api)](https://img.shields.io/crates/v/usesend-api.svg)](https://crates.io/crates/usesend-api)
[![docs.rs](https://docs.rs/usesend/badge.svg)](https://docs.rs/usesend)
[![CI](https://github.com/AprilNEA/usesend-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/AprilNEA/usesend-rs/actions)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE-MIT)

</div>

## Installation

```toml
[dependencies]
usesend = "0.1"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

## Quick Start

```rust,no_run
use usesend::types::email::SendEmailRequest;

#[tokio::main]
async fn main() -> usesend::ApiResult<()> {
    let client = usesend::UseSend::new("us_api_key");

    let email = SendEmailRequest::builder()
        .from("hello@example.com")
        .to("user@example.com")
        .subject("Hello from useSend!")
        .html("<h1>Welcome!</h1>")
        .build();

    let resp = client.emails.send(&email).await?;
    println!("Sent: {}", resp.email_id);
    Ok(())
}
```

## Features

- **Builder pattern** — Fluent API for composing emails, contacts, campaigns, and more
- **Sub-service architecture** — Organized access via `client.emails`, `client.domains`, `client.contacts`, etc.
- **Rate limiting** — Built-in [governor](https://crates.io/crates/governor)-based rate limiter with automatic 429 retry
- **Typed IDs** — `DomainId`, `EmailId`, and other strongly-typed identifiers
- **Self-hosted support** — Point to your own useSend instance with `UseSend::with_base_url()`
- **Environment variable** — Read API key from `USESEND_API_KEY` via `UseSend::from_env()`

👉 **[Full usage guide →](USAGE.md)** — Domains, Contacts, Campaigns, Feature Flags, and more.

## Crate Structure

| Crate | Description |
|-------|-------------|
| [`usesend`](https://crates.io/crates/usesend) | High-level SDK with builder pattern — **use this** |
| [`usesend-api`](https://crates.io/crates/usesend-api) | Low-level typed HTTP client and request/response models |

Most users should depend on **`usesend`**. Use `usesend-api` directly only if you need raw access to the API types or want to build your own abstraction layer.

## Minimum Supported Rust Version

The MSRV is **1.88** (Rust edition 2024).

## Contributing

Contributions are welcome! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## License

Licensed under either of [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE) at your option.
