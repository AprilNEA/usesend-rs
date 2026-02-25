use std::future::Future;
use std::ops::Range;
use std::time::Duration;

use crate::error::{ApiError, ApiResult};

/// Options for retrying rate-limited requests.
#[derive(Debug, Clone)]
pub struct RetryOptions {
    /// Maximum number of retry attempts (default: 3).
    pub max_retries: u32,
    /// Base delay in milliseconds between retries (default: 1000).
    pub base_delay_ms: u64,
    /// Random jitter range in milliseconds added to delay (default: 0..30).
    pub jitter_range_ms: Range<u64>,
}

impl Default for RetryOptions {
    fn default() -> Self {
        Self {
            max_retries: 3,
            base_delay_ms: 1000,
            jitter_range_ms: 0..30,
        }
    }
}

/// Retry a future-producing closure on `ApiError::RateLimit`.
///
/// If the API returns a `ratelimit-reset` header, that value (in seconds) is
/// used as the sleep duration. Otherwise, `opts.base_delay_ms` is used.
pub async fn send_with_retry<F, Fut, T>(f: F, opts: &RetryOptions) -> ApiResult<T>
where
    F: Fn() -> Fut,
    Fut: Future<Output = ApiResult<T>>,
{
    let mut retries_left = opts.max_retries;
    loop {
        match f().await {
            Ok(v) => return Ok(v),
            Err(ApiError::RateLimit { retry_after }) if retries_left > 0 => {
                retries_left -= 1;
                let base = retry_after.map(|s| s * 1000).unwrap_or(opts.base_delay_ms);
                let jitter = if opts.jitter_range_ms.is_empty() {
                    0
                } else {
                    opts.jitter_range_ms.start
                        + (rand_u64() % (opts.jitter_range_ms.end - opts.jitter_range_ms.start))
                };
                tokio::time::sleep(Duration::from_millis(base + jitter)).await;
            }
            Err(e) => return Err(e),
        }
    }
}

/// Simple non-cryptographic random u64 using std.
fn rand_u64() -> u64 {
    use std::collections::hash_map::RandomState;
    use std::hash::{BuildHasher, Hasher};
    RandomState::new().build_hasher().finish()
}
