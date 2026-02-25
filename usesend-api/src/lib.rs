//! Low-level API client for the [useSend](https://usesend.com) email service.
//!
//! This crate provides typed request/response models and a raw HTTP client
//! for every useSend API endpoint. For a higher-level, more ergonomic interface,
//! see the [`usesend`](https://crates.io/crates/usesend) crate.
//!
//! # Quick Start
//!
//! ```no_run
//! use usesend_api::UseSendApiClient;
//!
//! # async fn example() -> usesend_api::ApiResult<()> {
//! let client = UseSendApiClient::new("us_api_key");
//! let domains = client.domains.list().await?;
//! println!("Found {} domains", domains.len());
//! # Ok(())
//! # }
//! ```

pub mod types;
pub mod services;

mod client;
mod config;
mod error;
mod retry;

pub use client::UseSendApiClient;
pub use config::Config;
pub use error::{ApiError, ApiResult};
pub use retry::{RetryOptions, send_with_retry};
pub use services::{CampaignsSvc, ContactBooksSvc, ContactsSvc, DomainsSvc, EmailsSvc};
