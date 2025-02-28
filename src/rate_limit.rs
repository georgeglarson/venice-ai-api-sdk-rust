//! Rate limit handling for Venice.ai API
//!
//! This module provides functionality for handling rate limits when making requests to the Venice.ai API.
//! It includes a rate limiter that can track rate limit information and automatically wait when limits are reached.

use std::sync::atomic::{AtomicU32, AtomicI64, Ordering};
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::time::sleep;

use crate::error::{RateLimitInfo, VeniceError, VeniceResult};

/// Configuration for the rate limiter
#[derive(Debug, Clone)]
pub struct RateLimiterConfig {
    /// Whether to automatically wait when rate limits are reached
    pub auto_wait: bool,
    
    /// Maximum time to wait for rate limits to reset (in seconds)
    pub max_wait_time: u64,
}

impl Default for RateLimiterConfig {
    fn default() -> Self {
        Self {
            auto_wait: true,
            max_wait_time: 60, // Default to waiting up to 60 seconds
        }
    }
}

/// Rate limiter for managing API rate limits
///
/// The rate limiter tracks the current rate limit status and can automatically
/// wait until rate limits reset if configured to do so.
#[derive(Debug)]
pub struct RateLimiter {
    /// Maximum requests per minute
    pub max_requests: AtomicU32,
    
    /// Current remaining requests
    pub remaining_requests: AtomicU32,
    
    /// Unix timestamp when the request limit will reset
    pub reset_time_requests: AtomicI64,
    
    /// Maximum tokens per minute
    pub max_tokens: AtomicU32,
    
    /// Current remaining tokens
    pub remaining_tokens: AtomicU32,
    
    /// Unix timestamp when the token limit will reset
    pub reset_time_tokens: AtomicI64,
    
    /// Configuration for the rate limiter
    pub config: RateLimiterConfig,
}

impl Clone for RateLimiter {
    fn clone(&self) -> Self {
        Self {
            max_requests: AtomicU32::new(self.max_requests.load(Ordering::Relaxed)),
            remaining_requests: AtomicU32::new(self.remaining_requests.load(Ordering::Relaxed)),
            reset_time_requests: AtomicI64::new(self.reset_time_requests.load(Ordering::Relaxed)),
            max_tokens: AtomicU32::new(self.max_tokens.load(Ordering::Relaxed)),
            remaining_tokens: AtomicU32::new(self.remaining_tokens.load(Ordering::Relaxed)),
            reset_time_tokens: AtomicI64::new(self.reset_time_tokens.load(Ordering::Relaxed)),
            config: self.config.clone(),
        }
    }
}

impl RateLimiter {
    /// Creates a new rate limiter with default configuration
    pub fn new() -> Self {
        Self::with_config(RateLimiterConfig::default())
    }
    
    /// Creates a new rate limiter with the specified configuration
    pub fn with_config(config: RateLimiterConfig) -> Self {
        Self {
            max_requests: AtomicU32::new(0),
            remaining_requests: AtomicU32::new(1), // Initialize to 1 to avoid being rate limited initially
            reset_time_requests: AtomicI64::new(0),
            max_tokens: AtomicU32::new(0),
            remaining_tokens: AtomicU32::new(1), // Initialize to 1 to avoid being rate limited initially
            reset_time_tokens: AtomicI64::new(0),
            config,
        }
    }
    
    /// Updates the rate limiter with information from a response
    pub fn update_from_response(&self, rate_limit_info: &RateLimitInfo) {
        if let Some(limit) = rate_limit_info.limit_requests {
            self.max_requests.store(limit, Ordering::Relaxed);
        }
        
        if let Some(remaining) = rate_limit_info.remaining_requests {
            self.remaining_requests.store(remaining, Ordering::Relaxed);
        }
        
        if let Some(reset) = rate_limit_info.reset_requests {
            self.reset_time_requests.store(reset as i64, Ordering::Relaxed);
        }
        
        if let Some(limit) = rate_limit_info.limit_tokens {
            self.max_tokens.store(limit, Ordering::Relaxed);
        }
        
        if let Some(remaining) = rate_limit_info.remaining_tokens {
            self.remaining_tokens.store(remaining, Ordering::Relaxed);
        }
        
        if let Some(reset) = rate_limit_info.reset_tokens {
            // Convert reset_tokens (seconds until reset) to a Unix timestamp
            if let Ok(now) = SystemTime::now().duration_since(UNIX_EPOCH) {
                let reset_time = now.as_secs() + reset;
                self.reset_time_tokens.store(reset_time as i64, Ordering::Relaxed);
            }
        }
    }
    
    /// Checks if the rate limit is currently exceeded
    pub fn is_rate_limited(&self) -> bool {
        self.remaining_requests.load(Ordering::Relaxed) == 0 || 
        self.remaining_tokens.load(Ordering::Relaxed) == 0
    }
    
    /// Gets the time until the rate limit resets (in seconds)
    pub fn time_until_reset(&self) -> Option<u64> {
        let now = match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(duration) => duration.as_secs() as i64,
            Err(_) => return None,
        };
        
        let reset_time_requests = self.reset_time_requests.load(Ordering::Relaxed);
        let reset_time_tokens = self.reset_time_tokens.load(Ordering::Relaxed);
        
        // Find the earliest reset time that is in the future
        let mut earliest_reset = None;
        
        if reset_time_requests > now {
            earliest_reset = Some((reset_time_requests - now) as u64);
        }
        
        if reset_time_tokens > now {
            let tokens_reset = (reset_time_tokens - now) as u64;
            earliest_reset = match earliest_reset {
                Some(time) => Some(time.min(tokens_reset)),
                None => Some(tokens_reset),
            };
        }
        
        earliest_reset
    }
    
    /// Acquires permission to make a request, waiting if necessary
    ///
    /// If the rate limit is exceeded and auto_wait is enabled, this function will
    /// wait until the rate limit resets before returning.
    ///
    /// # Returns
    ///
    /// * `Ok(())` if permission is granted
    /// * `Err(VeniceError::RateLimitExceeded)` if the rate limit is exceeded and auto_wait is disabled
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use venice_ai_api_sdk_rust::rate_limit::RateLimiter;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let rate_limiter = RateLimiter::new();
    ///     
    ///     // Acquire permission to make a request
    ///     rate_limiter.acquire().await?;
    ///     
    ///     // Make the request
    ///     // ...
    ///     
    ///     Ok(())
    /// }
    /// ```
    pub async fn acquire(&self) -> VeniceResult<()> {
        if !self.is_rate_limited() {
            return Ok(());
        }
        
        if !self.config.auto_wait {
            return Err(VeniceError::RateLimitExceeded(
                "Rate limit exceeded. Consider enabling auto_wait or implementing backoff.".to_string()
            ));
        }
        
        if let Some(wait_time) = self.time_until_reset() {
            let wait_time = wait_time.min(self.config.max_wait_time);
            
            if wait_time > 0 {
                log::info!("Rate limit exceeded. Waiting for {} seconds...", wait_time);
                sleep(Duration::from_secs(wait_time)).await;
            }
            
            Ok(())
        } else {
            Err(VeniceError::RateLimitExceeded(
                "Rate limit exceeded and reset time is unknown.".to_string()
            ))
        }
    }
}

/// Creates a shared rate limiter that can be used across multiple clients
pub fn new_shared_rate_limiter() -> Arc<RateLimiter> {
    Arc::new(RateLimiter::new())
}

/// Creates a shared rate limiter with the specified configuration
pub fn new_shared_rate_limiter_with_config(config: RateLimiterConfig) -> Arc<RateLimiter> {
    Arc::new(RateLimiter::with_config(config))
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_rate_limiter_update() {
        let rate_limiter = RateLimiter::new();
        
        let rate_limit_info = RateLimitInfo {
            limit_requests: Some(100),
            remaining_requests: Some(50),
            reset_requests: Some(1614556800),
            limit_tokens: Some(1000),
            remaining_tokens: Some(500),
            reset_tokens: Some(60),
            balance_vcu: None,
            balance_usd: None,
        };
        
        rate_limiter.update_from_response(&rate_limit_info);
        
        assert_eq!(rate_limiter.max_requests.load(Ordering::Relaxed), 100);
        assert_eq!(rate_limiter.remaining_requests.load(Ordering::Relaxed), 50);
        assert_eq!(rate_limiter.reset_time_requests.load(Ordering::Relaxed), 1614556800);
        assert_eq!(rate_limiter.max_tokens.load(Ordering::Relaxed), 1000);
        assert_eq!(rate_limiter.remaining_tokens.load(Ordering::Relaxed), 500);
        // We can't easily test reset_time_tokens as it depends on the current time
    }
    
    #[test]
    fn test_is_rate_limited() {
        let rate_limiter = RateLimiter::new();
        
        // Not rate limited initially
        assert!(!rate_limiter.is_rate_limited());
        
        // Rate limited when remaining_requests is 0
        rate_limiter.remaining_requests.store(0, Ordering::Relaxed);
        assert!(rate_limiter.is_rate_limited());
        
        // Not rate limited when remaining_requests is positive
        rate_limiter.remaining_requests.store(10, Ordering::Relaxed);
        assert!(!rate_limiter.is_rate_limited());
        
        // Rate limited when remaining_tokens is 0
        rate_limiter.remaining_tokens.store(0, Ordering::Relaxed);
        assert!(rate_limiter.is_rate_limited());
        
        // Not rate limited when both are positive
        rate_limiter.remaining_requests.store(10, Ordering::Relaxed);
        rate_limiter.remaining_tokens.store(10, Ordering::Relaxed);
        assert!(!rate_limiter.is_rate_limited());
    }
}