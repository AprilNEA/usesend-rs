use governor::clock::DefaultClock;
use governor::middleware::NoOpMiddleware;
use governor::state::{InMemoryState, NotKeyed};
use governor::{Jitter, Quota, RateLimiter};
use reqwest::{Client, RequestBuilder, Response, StatusCode};
use serde::de::DeserializeOwned;
use std::num::NonZeroU32;
use std::sync::Arc;
use std::time::Duration;

use crate::error::{ApiError, ApiResult};
use crate::types::ErrorResponse;

const DEFAULT_BASE_URL: &str = "https://app.usesend.com/api";
const DEFAULT_RATE_LIMIT: u32 = 9;

type DirectRateLimiter = RateLimiter<NotKeyed, InMemoryState, DefaultClock, NoOpMiddleware>;

/// Shared configuration for all API services.
pub struct Config {
    pub(crate) client: Client,
    pub(crate) base_url: String,
    pub(crate) api_key: String,
    pub(crate) limiter: Arc<DirectRateLimiter>,
}

impl std::fmt::Debug for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Config")
            .field("base_url", &self.base_url)
            .field("api_key", &"[REDACTED]")
            .finish()
    }
}

impl Config {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            client: Client::new(),
            base_url: DEFAULT_BASE_URL.to_string(),
            api_key: api_key.into(),
            limiter: Arc::new(Self::build_limiter(DEFAULT_RATE_LIMIT)),
        }
    }

    pub fn with_base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = base_url.into();
        self
    }

    pub fn with_client(mut self, client: Client) -> Self {
        self.client = client;
        self
    }

    /// Set the maximum number of requests per second (default: 9).
    pub fn with_rate_limit(mut self, per_second: u32) -> Self {
        self.limiter = Arc::new(Self::build_limiter(per_second));
        self
    }

    fn build_limiter(per_second: u32) -> DirectRateLimiter {
        let quota = Quota::with_period(Duration::from_millis(1100))
            .expect("valid quota")
            .allow_burst(NonZeroU32::new(per_second).expect("rate limit must be > 0"));
        RateLimiter::direct(quota)
    }

    pub(crate) fn url(&self, path: &str) -> String {
        format!("{}{}", self.base_url, path)
    }

    pub(crate) fn auth(&self, req: RequestBuilder) -> RequestBuilder {
        req.bearer_auth(&self.api_key)
    }

    pub(crate) async fn send_and_parse<T: DeserializeOwned>(
        &self,
        req: RequestBuilder,
    ) -> ApiResult<T> {
        let jitter = Jitter::new(Duration::from_millis(10), Duration::from_millis(50));
        self.limiter.until_ready_with_jitter(jitter).await;

        let resp: Response = req.send().await?;
        handle_response(resp).await
    }
}

pub(crate) async fn handle_response<T: DeserializeOwned>(resp: Response) -> ApiResult<T> {
    let status = resp.status();
    if status.is_success() {
        let body = resp.text().await?;
        serde_json::from_str::<T>(&body).map_err(|source| ApiError::Deserialize {
            status,
            body,
            source,
        })
    } else if status == StatusCode::CONFLICT {
        let text = resp.text().await.unwrap_or_default();
        Err(ApiError::Conflict { message: text })
    } else if status == StatusCode::TOO_MANY_REQUESTS {
        let retry_after = resp
            .headers()
            .get("ratelimit-reset")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.parse::<u64>().ok());
        Err(ApiError::RateLimit { retry_after })
    } else {
        let text = resp.text().await.unwrap_or_default();
        match serde_json::from_str::<ErrorResponse>(&text) {
            Ok(body) => Err(ApiError::Api { status, body }),
            Err(_) => Err(ApiError::Unexpected { status, text }),
        }
    }
}

/// Wraps `Arc<Config>` for sharing across services.
#[derive(Debug, Clone)]
pub(crate) struct SharedConfig(pub(crate) Arc<Config>);

impl SharedConfig {
    pub(crate) fn new(config: Config) -> Self {
        Self(Arc::new(config))
    }
}

impl std::ops::Deref for SharedConfig {
    type Target = Config;
    fn deref(&self) -> &Config {
        &self.0
    }
}
