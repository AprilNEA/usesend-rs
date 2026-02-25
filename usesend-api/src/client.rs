use crate::config::{Config, SharedConfig};
use crate::services::*;

/// Low-level API client for the useSend email service.
///
/// Uses a sub-service pattern — access each API resource via its field:
///
/// ```no_run
/// # async fn example() -> usesend_api::ApiResult<()> {
/// let client = usesend_api::UseSendApiClient::new("us_api_key");
/// let domains = client.domains.list().await?;
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct UseSendApiClient {
    pub domains: DomainsSvc,
    pub emails: EmailsSvc,
    pub contact_books: ContactBooksSvc,
    pub contacts: ContactsSvc,
    pub campaigns: CampaignsSvc,
}

impl UseSendApiClient {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self::from_config(Config::new(api_key))
    }

    pub fn with_base_url(api_key: impl Into<String>, base_url: impl Into<String>) -> Self {
        Self::from_config(Config::new(api_key).with_base_url(base_url))
    }

    pub fn with_client(api_key: impl Into<String>, client: reqwest::Client) -> Self {
        Self::from_config(Config::new(api_key).with_client(client))
    }

    pub fn from_config(config: Config) -> Self {
        let shared = SharedConfig::new(config);
        Self {
            domains: DomainsSvc(shared.clone()),
            emails: EmailsSvc(shared.clone()),
            contact_books: ContactBooksSvc(shared.clone()),
            contacts: ContactsSvc(shared.clone()),
            campaigns: CampaignsSvc(shared),
        }
    }
}
