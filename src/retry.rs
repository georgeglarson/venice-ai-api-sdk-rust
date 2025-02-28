use std::time::Duration;
use tokio::time::sleep;

use crate::error::{VeniceError, VeniceResult};

/// Configuration for retry behavior
#[derive(Debug, Clone)]
pub struct RetryConfig {
    /// Maximum number of retry attempts
    pub max_retries: u32,
    /// Initial delay between retries in milliseconds
    pub initial_delay_ms: u64,
    /// Maximum delay between retries in milliseconds
    pub max_delay_ms: u64,
    /// Multiplier for exponential backoff
    pub backoff_factor: f64,
    /// Whether to add jitter to the delay
    pub add_jitter: bool,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            initial_delay_ms: 500,
            max_delay_ms: 10000,
            backoff_factor: 2.0,
            add_jitter: true,
        }
    }
}

impl RetryConfig {
    /// Create a new retry configuration
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the maximum number of retry attempts
    pub fn max_retries(mut self, max_retries: u32) -> Self {
        self.max_retries = max_retries;
        self
    }

    /// Set the initial delay between retries in milliseconds
    pub fn initial_delay_ms(mut self, initial_delay_ms: u64) -> Self {
        self.initial_delay_ms = initial_delay_ms;
        self
    }

    /// Set the maximum delay between retries in milliseconds
    pub fn max_delay_ms(mut self, max_delay_ms: u64) -> Self {
        self.max_delay_ms = max_delay_ms;
        self
    }

    /// Set the backoff factor for exponential backoff
    pub fn backoff_factor(mut self, backoff_factor: f64) -> Self {
        self.backoff_factor = backoff_factor;
        self
    }

    /// Set whether to add jitter to the delay
    pub fn add_jitter(mut self, add_jitter: bool) -> Self {
        self.add_jitter = add_jitter;
        self
    }

    /// Calculate the delay for a given retry attempt
    pub fn calculate_delay(&self, attempt: u32) -> Duration {
        let base_delay = (self.initial_delay_ms as f64 * self.backoff_factor.powi(attempt as i32)) as u64;
        let delay = base_delay.min(self.max_delay_ms);
        
        if self.add_jitter {
            // Add jitter by multiplying by a random value between 0.5 and 1.5
            let jitter = 0.5 + rand::random::<f64>();
            Duration::from_millis((delay as f64 * jitter) as u64)
        } else {
            Duration::from_millis(delay)
        }
    }
}

/// Determines if an error is retryable
pub fn is_retryable_error(error: &VeniceError) -> bool {
    match error {
        // Network errors are generally retryable
        VeniceError::HttpError(_) => true,
        
        // Rate limit errors are retryable
        VeniceError::RateLimitExceeded(_) => true,
        
        // Server errors (5xx) are retryable
        VeniceError::ApiError { status, .. } => status.as_u16() >= 500 && status.as_u16() < 600,
        
        // Other errors are not retryable
        _ => false,
    }
}

/// Execute a function with retry logic
pub async fn with_retry<T, F, Fut>(
    f: F,
    config: &RetryConfig,
) -> VeniceResult<T>
where
    F: Fn() -> Fut,
    Fut: std::future::Future<Output = VeniceResult<T>>,
{
    let mut attempt = 0;
    
    loop {
        match f().await {
            Ok(result) => return Ok(result),
            Err(error) => {
                attempt += 1;
                
                if attempt > config.max_retries || !is_retryable_error(&error) {
                    return Err(error);
                }
                
                let delay = config.calculate_delay(attempt);
                log::debug!(
                    "Request failed with error: {}. Retrying in {:?} (attempt {}/{})",
                    error,
                    delay,
                    attempt,
                    config.max_retries
                );
                
                sleep(delay).await;
            }
        }
    }
}