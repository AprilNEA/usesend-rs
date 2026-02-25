//! Ergonomic Rust SDK for the [useSend](https://usesend.com) email service.
//!
//! This crate wraps [`usesend_api`] with a user-friendly, builder-based API.
//!
//! # Quick Start
//!
//! ```no_run
//! # async fn example() -> usesend::ApiResult<()> {
//! let client = usesend::UseSend::new("us_api_key");
//!
//! // Send an email using the builder
//! let resp = client.emails.build()
//!     .from("hello@example.com")
//!     .to("user@example.com")
//!     .subject("Hello!")
//!     .html("<h1>Hi</h1>")
//!     .send()
//!     .await?;
//!
//! println!("Sent: {}", resp.email_id);
//! # Ok(())
//! # }
//! ```
//!
//! # Environment Variable
//!
//! You can use `UseSend::from_env()` or `UseSend::default()` which reads the
//! `USESEND_API_KEY` environment variable.

mod campaigns;
mod client;
mod contact_books;
mod contacts;
mod domains;
mod emails;

pub use campaigns::Campaigns;
pub use client::UseSend;
pub use contact_books::ContactBooks;
pub use contacts::Contacts;
pub use domains::Domains;
pub use emails::{EmailBuilder, Emails};

pub use usesend_api::types;
pub use usesend_api::{ApiError, ApiResult};
