use crate::types::ErrorResponse;

pub type ApiResult<T> = Result<T, ApiError>;

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("API error ({status}): {}", body.error)]
    Api {
        status: reqwest::StatusCode,
        body: ErrorResponse,
    },

    #[error("Conflict (idempotency): {message}")]
    Conflict { message: String },

    #[error("Rate limited (retry after {retry_after:?}s)")]
    RateLimit { retry_after: Option<u64> },

    #[error("Unexpected status {status}: {text}")]
    Unexpected {
        status: reqwest::StatusCode,
        text: String,
    },

    #[error("Failed to deserialize response (status {status}): {source}")]
    Deserialize {
        status: reqwest::StatusCode,
        body: String,
        source: serde_json::Error,
    },
}
