use usesend_api::UseSendApiClient;

use crate::{Campaigns, ContactBooks, Contacts, Domains, Emails};

/// High-level, ergonomic client for the useSend API.
///
/// ```no_run
/// # async fn example() -> usesend::ApiResult<()> {
/// let client = usesend::UseSend::new("us_api_key");
///
/// // Send an email
/// let resp = client.emails.build()
///     .from("hello@example.com")
///     .to("user@example.com")
///     .subject("Hello!")
///     .html("<h1>Hi</h1>")
///     .send()
///     .await?;
///
/// // List domains
/// let domains = client.domains.list().await?;
/// # Ok(())
/// # }
/// ```
#[derive(Clone)]
pub struct UseSend {
    pub domains: Domains,
    pub emails: Emails,
    pub contact_books: ContactBooks,
    pub contacts: Contacts,
    pub campaigns: Campaigns,
}

impl UseSend {
    /// Create a new client with the given API key.
    pub fn new(api_key: impl Into<String>) -> Self {
        Self::from_api_client(UseSendApiClient::new(api_key))
    }

    /// Create a client with a custom base URL (e.g., self-hosted).
    pub fn with_base_url(api_key: impl Into<String>, base_url: impl Into<String>) -> Self {
        Self::from_api_client(UseSendApiClient::with_base_url(api_key, base_url))
    }

    /// Create a client with a custom reqwest::Client.
    pub fn with_client(api_key: impl Into<String>, client: reqwest::Client) -> Self {
        Self::from_api_client(UseSendApiClient::with_client(api_key, client))
    }

    /// Create from a fully configured low-level API client.
    pub fn from_api_client(api: UseSendApiClient) -> Self {
        Self {
            domains: Domains(api.domains),
            emails: Emails(api.emails),
            contact_books: ContactBooks(api.contact_books),
            contacts: Contacts(api.contacts),
            campaigns: Campaigns(api.campaigns),
        }
    }

    /// Create from the `USESEND_API_KEY` environment variable.
    ///
    /// # Panics
    /// Panics if `USESEND_API_KEY` is not set.
    pub fn from_env() -> Self {
        let key = std::env::var("USESEND_API_KEY")
            .expect("USESEND_API_KEY environment variable not set");
        Self::new(key)
    }
}

impl Default for UseSend {
    fn default() -> Self {
        Self::from_env()
    }
}
