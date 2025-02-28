//! Retry middleware for the Venice AI API
//!
//! This module provides a middleware implementation for retrying
//! failed requests to the Venice AI API.

use std::time::Duration;
use rand::Rng;

use crate::error::VeniceError;
use crate::middleware::{Middleware, Next, Request};
use crate::retry::RetryConfig;

/// Middleware for retrying failed requests to the Venice AI API
#[derive(Clone)]
pub struct RetryMiddleware {
    /// The retry configuration to use
    config: RetryConfig,
}

impl RetryMiddleware {
    /// Create a new retry middleware with the given configuration
    pub fn new(config: RetryConfig) -> Self {
        Self { config }
    }
    
    /// Calculate the delay for a retry attempt
    fn calculate_delay(&self, attempt: u32) -> Duration {
        let base_delay = self.config.initial_delay_ms as f64 * self.config.backoff_factor.powi(attempt as i32 - 1);
        let max_delay = self.config.max_delay_ms as f64;
        let delay = base_delay.min(max_delay);
        
        let delay = if self.config.add_jitter {
            // Add jitter to avoid thundering herd problem
            let jitter = rand::thread_rng().gen_range(0.0..0.5);
            delay * (1.0 + jitter)
        } else {
            delay
        };
        
        Duration::from_millis(delay as u64)
    }
    
    /// Check if an error is retryable
    fn is_retryable(&self, error: &VeniceError) -> bool {
        match error {
            // Network errors are retryable
            VeniceError::HttpError(_) => true,
            
            // Rate limit errors are retryable if auto_wait is disabled
            VeniceError::RateLimitExceeded(_) => true,
            
            // API errors with 5xx status codes are retryable
            VeniceError::ApiError { status, .. } => {
                status.as_u16() >= 500 && status.as_u16() < 600
            },
            
            // Other errors are not retryable
            _ => false,
        }
    }
}

#[async_trait::async_trait]
impl<B: Send + Clone + 'static> Middleware<B> for RetryMiddleware {
    async fn process<T: Send + 'static>(&self, request: Request<B>, next: Next<'_, T, B>) -> crate::http::HttpResult<T> {
        let mut attempt = 0;
        let max_retries = self.config.max_retries;
        
        loop {
            attempt += 1;
            
            // Clone the request for each attempt
            let request_clone = Request {
                endpoint: request.endpoint.clone(),
                method: request.method,
                body: request.body.clone(),
            };
            
            // Call the next middleware or the actual request
            let result = next.run(request_clone).await;
            
            // If the request succeeded or we've reached the maximum number of retries, return the result
            if result.is_ok() || attempt > max_retries {
                return result;
            }
            
            // Check if the error is retryable
            let error = result.unwrap_err();
            if !self.is_retryable(&error) {
                return Err(error);
            }
            
            // Calculate the delay for this retry attempt
            let delay = self.calculate_delay(attempt);
            
            // Log the retry
            log::info!("Retrying request to {} after error: {}. Attempt {}/{}, waiting for {:?}",
                request.endpoint, error, attempt, max_retries, delay);
            
            // Wait before retrying
            tokio::time::sleep(delay).await;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::middleware::Method;
    use crate::error::RateLimitInfo;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};
    
    #[tokio::test]
    async fn test_retry_middleware_success() {
        // Create the middleware with default configuration
        let middleware = RetryMiddleware::new(RetryConfig::default());
        
        // Create a request
        let request = Request {
            endpoint: "/test".to_string(),
            method: Method::Get,
            body: None,
        };
        
        // Create a handler that returns a successful response
        let handler = |_req| async {
            Ok((42, RateLimitInfo {
                limit_requests: None,
                remaining_requests: None,
                reset_requests: None,
                limit_tokens: None,
                remaining_tokens: None,
                reset_tokens: None,
                balance_vcu: None,
                balance_usd: None,
            }))
        };
        
        // Process the request
        let next = Next::new(handler);
        let result = middleware.process(request, next).await;
        
        // Verify the result
        assert!(result.is_ok());
        assert_eq!(result.unwrap().0, 42);
    }
    
    #[tokio::test]
    async fn test_retry_middleware_retry_success() {
        // Create the middleware with a configuration that allows 3 retries
        let config = RetryConfig::new()
            .max_retries(3)
            .initial_delay_ms(1)
            .max_delay_ms(10)
            .backoff_factor(1.0)
            .add_jitter(false);
        let middleware = RetryMiddleware::new(config);
        
        // Create a counter to track the number of attempts
        let attempts = Arc::new(AtomicUsize::new(0));
        
        // Create a request
        let request = Request {
            endpoint: "/test".to_string(),
            method: Method::Get,
            body: None,
        };
        
        // Create a handler that fails on the first two attempts but succeeds on the third
        let attempts_clone = attempts.clone();
        let handler = move |_req| {
            let attempts = attempts_clone.clone();
            async move {
                let attempt = attempts.fetch_add(1, Ordering::SeqCst) + 1;
                
                if attempt <= 2 {
                    Err(VeniceError::HttpError(reqwest::Error::from(std::io::Error::new(
                        std::io::ErrorKind::ConnectionReset,
                        "Connection reset",
                    ))))
                } else {
                    Ok((42, RateLimitInfo {
                        limit_requests: None,
                        remaining_requests: None,
                        reset_requests: None,
                        limit_tokens: None,
                        remaining_tokens: None,
                        reset_tokens: None,
                        balance_vcu: None,
                        balance_usd: None,
                    }))
                }
            }
        };
        
        // Process the request
        let next = Next::new(handler);
        let result = middleware.process(request, next).await;
        
        // Verify the result
        assert!(result.is_ok());
        assert_eq!(result.unwrap().0, 42);
        assert_eq!(attempts.load(Ordering::SeqCst), 3);
    }
    
    #[tokio::test]
    async fn test_retry_middleware_max_retries_exceeded() {
        // Create the middleware with a configuration that allows 2 retries
        let config = RetryConfig::new()
            .max_retries(2)
            .initial_delay_ms(1)
            .max_delay_ms(10)
            .backoff_factor(1.0)
            .add_jitter(false);
        let middleware = RetryMiddleware::new(config);
        
        // Create a counter to track the number of attempts
        let attempts = Arc::new(AtomicUsize::new(0));
        
        // Create a request
        let request = Request {
            endpoint: "/test".to_string(),
            method: Method::Get,
            body: None,
        };
        
        // Create a handler that always fails
        let attempts_clone = attempts.clone();
        let handler = move |_req| {
            let attempts = attempts_clone.clone();
            async move {
                let _ = attempts.fetch_add(1, Ordering::SeqCst) + 1;
                
                Err(VeniceError::HttpError(reqwest::Error::from(std::io::Error::new(
                    std::io::ErrorKind::ConnectionReset,
                    "Connection reset",
                ))))
            }
        };
        
        // Process the request
        let next = Next::new(handler);
        let result = middleware.process(request, next).await;
        
        // Verify the result
        assert!(result.is_err());
        assert_eq!(attempts.load(Ordering::SeqCst), 3); // Initial attempt + 2 retries
    }
    
    #[tokio::test]
    async fn test_retry_middleware_non_retryable_error() {
        // Create the middleware with default configuration
        let middleware = RetryMiddleware::new(RetryConfig::default());
        
        // Create a counter to track the number of attempts
        let attempts = Arc::new(AtomicUsize::new(0));
        
        // Create a request
        let request = Request {
            endpoint: "/test".to_string(),
            method: Method::Get,
            body: None,
        };
        
        // Create a handler that returns a non-retryable error
        let attempts_clone = attempts.clone();
        let handler = move |_req| {
            let attempts = attempts_clone.clone();
            async move {
                let _ = attempts.fetch_add(1, Ordering::SeqCst) + 1;
                
                Err(VeniceError::InvalidInput("Invalid input".to_string()))
            }
        };
        
        // Process the request
        let next = Next::new(handler);
        let result = middleware.process(request, next).await;
        
        // Verify the result
        assert!(result.is_err());
        assert_eq!(attempts.load(Ordering::SeqCst), 1); // Only the initial attempt, no retries
    }
}