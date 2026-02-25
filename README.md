<div align="center">

# usesend-rs

**Rust SDK for [useSend](https://usesend.com) — an open-source Resend alternative.**

[![Crates.io (usesend)](https://img.shields.io/crates/v/usesend.svg)](https://crates.io/crates/usesend)
[![Crates.io (usesend-api)](https://img.shields.io/crates/v/usesend-api.svg)](https://crates.io/crates/usesend-api)
[![docs.rs](https://docs.rs/usesend/badge.svg)](https://docs.rs/usesend)
[![CI](https://github.com/AprilNEA/usesend-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/AprilNEA/usesend-rs/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

</div>

## Installation

Add **`usesend`** to your project:

```toml
[dependencies]
usesend = "0.1"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

## Quick Start

```rust,no_run
#[tokio::main]
async fn main() -> usesend::ApiResult<()> {
    let client = usesend::UseSend::new("us_api_key");

    let resp = client.emails.build()
        .from("hello@example.com")
        .to("user@example.com")
        .subject("Hello from useSend!")
        .html("<h1>Welcome!</h1>")
        .send()
        .await?;

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

## Usage

### Send an Email

```rust,no_run
# async fn example() -> usesend::ApiResult<()> {
let client = usesend::UseSend::new("us_api_key");

client.emails.build()
    .from("noreply@example.com")
    .to("user@example.com")
    .subject("Order Confirmation")
    .html("<h1>Thanks for your order!</h1>")
    .cc("manager@example.com")
    .reply_to("support@example.com")
    .attachment("invoice.pdf", "<base64-content>")
    .send()
    .await?;
# Ok(())
# }
```

### Manage Domains

```rust,no_run
# async fn example() -> usesend::ApiResult<()> {
let client = usesend::UseSend::new("us_api_key");

// Add a domain
let domain = client.domains.create("example.com", "us-east-1").await?;

// List all domains
let domains = client.domains.list().await?;

// Verify DNS records
let status = client.domains.verify(&domain.id).await?;
# Ok(())
# }
```

### Contacts & Contact Books

```rust,no_run
# async fn example() -> usesend::ApiResult<()> {
let client = usesend::UseSend::new("us_api_key");

// Create a contact book
let book = client.contact_books.create("Newsletter Subscribers").await?;

// Add a contact
let contact = client.contacts.build(&book.id)
    .email("alice@example.com")
    .first_name("Alice")
    .last_name("Smith")
    .subscribed(true)
    .create()
    .await?;
# Ok(())
# }
```

### Campaigns

```rust,no_run
# async fn example() -> usesend::ApiResult<()> {
let client = usesend::UseSend::new("us_api_key");

let campaign = client.campaigns.build()
    .name("Weekly Digest")
    .from("newsletter@example.com")
    .subject("This Week's Highlights")
    .contact_book_id("book_id")
    .html("<h1>Weekly Digest</h1>")
    .send_now(true)
    .create()
    .await?;
# Ok(())
# }
```

### Self-hosted Instance

```rust,no_run
let client = usesend::UseSend::with_base_url(
    "us_api_key",
    "https://send.internal.company.com",
);
```

### Environment Variable

```rust,no_run
// Reads USESEND_API_KEY from environment
let client = usesend::UseSend::from_env();
```

## Feature Flags

| Flag | Default | Description |
|------|---------|-------------|
| `native-tls` | ✅ | Platform-native TLS (OpenSSL / Schannel / Secure Transport) |
| `rustls-tls` | ❌ | Pure-Rust TLS via [rustls](https://github.com/rustls/rustls) — no system dependency |

```toml
[dependencies]
usesend = { version = "0.1", default-features = false, features = ["rustls-tls"] }
```

## Crate Structure

| Crate | Description |
|-------|-------------|
| [`usesend`](https://crates.io/crates/usesend) | High-level SDK with builder pattern — **use this** |
| [`usesend-api`](https://crates.io/crates/usesend-api) | Low-level typed HTTP client and request/response models |

Most users should depend on **`usesend`**. Use `usesend-api` directly only if you need raw access to the API types or want to build your own abstraction layer.

## Minimum Supported Rust Version

The MSRV is **1.85** (Rust edition 2024).

## Contributing

Contributions are welcome! Please feel free to open an issue or submit a pull request.

## License

[MIT](LICENSE) © 2024-2026 AprilNEA
