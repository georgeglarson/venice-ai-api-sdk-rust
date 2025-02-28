use std::fmt;
use thiserror::Error;

/// Represents all possible errors that can occur when using the Venice.ai API SDK
#[derive(Error, Debug)]
pub enum VeniceError {
    /// Error returned by the Venice.ai API
    #[error("API error: {code} - {message}")]
    ApiError {
        /// HTTP status code
        status: reqwest::StatusCode,
        /// Error code returned by the API
        code: String,
        /// Error message returned by the API
        message: String,
    },

    /// Error occurred while sending the request or receiving the response
    #[error("HTTP error: {0}")]
    HttpError(#[from] reqwest::Error),

    /// Error occurred while parsing the response
    #[error("Failed to parse response: {0}")]
    ParseError(String),

    /// Error occurred due to invalid input
    #[error("Invalid input: {0}")]
    InvalidInput(String),

    /// Rate limit exceeded
    #[error("Rate limit exceeded: {0}")]
    RateLimitExceeded(String),

    /// Authentication failed
    #[error("Authentication failed: {0}")]
    AuthenticationFailed(String),
    
    /// Invalid webhook signature
    #[error("Invalid webhook signature: {0}")]
    InvalidWebhookSignature(String),

    /// Error occurred due to an unknown cause
    #[error("Unknown error: {0}")]
    Unknown(String),
}

/// Represents the rate limit information returned in the response headers
#[derive(Debug, Clone)]
pub struct RateLimitInfo {
    /// Total requests limit
    pub limit_requests: Option<u32>,
    /// Remaining requests
    pub remaining_requests: Option<u32>,
    /// Unix timestamp when the rate limit will reset
    pub reset_requests: Option<u64>,
    /// Total token limit
    pub limit_tokens: Option<u32>,
    /// Remaining tokens
    pub remaining_tokens: Option<u32>,
    /// Duration in seconds until the token rate limit resets
    pub reset_tokens: Option<u64>,
    /// User's VCU balance
    pub balance_vcu: Option<f64>,
    /// User's USD balance
    pub balance_usd: Option<f64>,
}

impl fmt::Display for RateLimitInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Rate Limit Info: {}/{} requests, {}/{} tokens",
            self.remaining_requests.unwrap_or(0),
            self.limit_requests.unwrap_or(0),
            self.remaining_tokens.unwrap_or(0),
            self.limit_tokens.unwrap_or(0)
        )
    }
}

impl RateLimitInfo {
    /// Extract rate limit information from response headers
    pub fn from_headers(headers: &reqwest::header::HeaderMap) -> Self {
        fn parse_header<T: std::str::FromStr>(
            headers: &reqwest::header::HeaderMap,
            name: &str,
        ) -> Option<T> {
            headers
                .get(name)
                .and_then(|value| value.to_str().ok())
                .and_then(|value| value.parse::<T>().ok())
        }

        RateLimitInfo {
            limit_requests: parse_header(headers, "x-ratelimit-limit-requests"),
            remaining_requests: parse_header(headers, "x-ratelimit-remaining-requests"),
            reset_requests: parse_header(headers, "x-ratelimit-reset-requests"),
            limit_tokens: parse_header(headers, "x-ratelimit-limit-tokens"),
            remaining_tokens: parse_header(headers, "x-ratelimit-remaining-tokens"),
            reset_tokens: parse_header(headers, "x-ratelimit-reset-tokens"),
            balance_vcu: parse_header(headers, "x-venice-balance-vcu"),
            balance_usd: parse_header(headers, "x-venice-balance-usd"),
        }
    }

    /// Check if rate limit is exceeded
    pub fn is_rate_limited(&self) -> bool {
        self.remaining_requests.map_or(false, |r| r == 0) || self.remaining_tokens.map_or(false, |t| t == 0)
    }
}

/// Result type for Venice API operations
pub type VeniceResult<T> = Result<T, VeniceError>;