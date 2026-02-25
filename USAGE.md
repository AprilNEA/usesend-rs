# Usage

> For full API documentation, see [docs.rs/usesend](https://docs.rs/usesend).

## Send an Email

```rust,no_run
# async fn example() -> usesend::ApiResult<()> {
use usesend::types::email::SendEmailRequest;

let client = usesend::UseSend::new("us_api_key");

let email = SendEmailRequest::builder()
    .from("noreply@example.com")
    .to("user@example.com")
    .subject("Order Confirmation")
    .html("<h1>Thanks for your order!</h1>")
    .cc("manager@example.com")
    .reply_to("support@example.com")
    .build();

client.emails.send(&email).await?;
# Ok(())
# }
```

## Manage Domains

```rust,no_run
# async fn example() -> usesend::ApiResult<()> {
use usesend::types::domain::CreateDomainRequest;

let client = usesend::UseSend::new("us_api_key");

// Add a domain
let req = CreateDomainRequest::builder()
    .name("example.com")
    .region("us-east-1")
    .build();
let domain = client.domains.create(&req).await?;

// List all domains
let domains = client.domains.list().await?;

// Verify DNS records
let status = client.domains.verify(&domain.id).await?;
# Ok(())
# }
```

## Contacts & Contact Books

```rust,no_run
# async fn example() -> usesend::ApiResult<()> {
use usesend::types::contact::CreateContactRequest;
use usesend::types::contact_book::CreateContactBookRequest;

let client = usesend::UseSend::new("us_api_key");

// Create a contact book
let book_req = CreateContactBookRequest::builder()
    .name("Newsletter Subscribers")
    .build();
let book = client.contact_books.create(&book_req).await?;

// Add a contact
let contact_req = CreateContactRequest::builder()
    .email("alice@example.com")
    .first_name("Alice")
    .last_name("Smith")
    .subscribed(true)
    .build();
let contact = client.contacts.create(&book.id, &contact_req).await?;
# Ok(())
# }
```

## Campaigns

```rust,no_run
# async fn example() -> usesend::ApiResult<()> {
use usesend::types::campaign::CreateCampaignRequest;

let client = usesend::UseSend::new("us_api_key");

let req = CreateCampaignRequest::builder()
    .name("Weekly Digest")
    .from("newsletter@example.com")
    .subject("This Week's Highlights")
    .contact_book_id("book_id")
    .html("<h1>Weekly Digest</h1>")
    .send_now(true)
    .build();

let campaign = client.campaigns.create(&req).await?;
# Ok(())
# }
```

## Self-hosted Instance

```rust,no_run
let client = usesend::UseSend::with_base_url(
    "us_api_key",
    "https://send.internal.company.com",
);
```

## Environment Variable

```rust,no_run
// Reads USESEND_API_KEY from environment
let client = usesend::UseSend::from_env();
```

## Feature Flags

| Flag | Default | Description |
|------|---------|-------------|
| `native-tls` | ❌ | Platform-native TLS (OpenSSL / Schannel / Secure Transport) |
| `rustls-tls` | ✅ | Pure-Rust TLS via [rustls](https://github.com/rustls/rustls) — no system dependency |

```toml
[dependencies]
usesend = { version = "0.1", default-features = false, features = ["native-tls"] }
```
