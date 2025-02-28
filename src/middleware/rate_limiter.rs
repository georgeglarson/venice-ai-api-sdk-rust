//! Rate limiter middleware for the Venice AI API
//!
//! This module provides a middleware implementation for rate limiting
//! requests to the Venice AI API.

use std::sync::Arc;

use crate::error::{RateLimitInfo, VeniceError};
use crate::middleware::{Middleware, Next, Request};
use crate::rate_limit::RateLimiter;

/// Middleware for rate limiting requests to the Venice AI API
#[derive(Clone)]
pub struct RateLimiterMiddleware {
    /// The rate limiter to use
    rate_limiter: Arc<RateLimiter>,
}

impl RateLimiterMiddleware {
    /// Create a new rate limiter middleware with the given rate limiter
    pub fn new(rate_limiter: Arc<RateLimiter>) -> Self {
        Self { rate_limiter }
    }
}

#[async_trait::async_trait]
impl<B: Send + 'static> Middleware<B> for RateLimiterMiddleware {
    async fn process<T: Send + 'static>(&self, request: Request<B>, next: Next<'_, T, B>) -> crate::http::HttpResult<T> {
        // Check rate limits before proceeding
        self.rate_limiter.acquire().await?;
        
        // Call the next middleware or the actual request
        let result = next.run(request).await;
        
        // Update rate limit info from response
        if let Ok((_, ref rate_limit_info)) = result {
            self.rate_limiter.update_from_response(rate_limit_info);
        }
        
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::middleware::Method;
    use crate::rate_limit::RateLimiterConfig;
    
    #[tokio::test]
    async fn test_rate_limiter_middleware() {
        // Create a rate limiter with auto_wait disabled
        let config = RateLimiterConfig {
            auto_wait: false,
            max_wait_time: 60,
        };
        let rate_limiter = Arc::new(RateLimiter::with_config(config));
        
        // Create the middleware
        let middleware = RateLimiterMiddleware::new(rate_limiter.clone());
        
        // Create a request
        let request = Request {
            endpoint: "/test".to_string(),
            method: Method::Get,
            body: None,
        };
        
        // Create a handler that returns a successful response
        let handler = |_req| async {
            Ok((42, RateLimitInfo {
                limit_requests: Some(100),
                remaining_requests: Some(99),
                reset_requests: Some(3600),
                limit_tokens: Some(1000),
                remaining_tokens: Some(999),
                reset_tokens: Some(3600),
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
        
        // Verify that the rate limiter was updated
        assert_eq!(rate_limiter.max_requests.load(std::sync::atomic::Ordering::Relaxed), 100);
        assert_eq!(rate_limiter.remaining_requests.load(std::sync::atomic::Ordering::Relaxed), 99);
    }
    
    #[tokio::test]
    async fn test_rate_limiter_middleware_rate_limited() {
        // Create a rate limiter with auto_wait disabled
        let config = RateLimiterConfig {
            auto_wait: false,
            max_wait_time: 60,
        };
        let rate_limiter = Arc::new(RateLimiter::with_config(config));
        
        // Set the remaining requests to 0 to simulate being rate limited
        rate_limiter.remaining_requests.store(0, std::sync::atomic::Ordering::Relaxed);
        
        // Create the middleware
        let middleware = RateLimiterMiddleware::new(rate_limiter);
        
        // Create a request
        let request = Request {
            endpoint: "/test".to_string(),
            method: Method::Get,
            body: None,
        };
        
        // Create a handler that should not be called
        let handler = |_req| async {
            panic!("Handler should not be called when rate limited");
        };
        
        // Process the request
        let next = Next::new(handler);
        let result = middleware.process(request, next).await;
        
        // Verify that the request was rejected due to rate limiting
        assert!(result.is_err());
        match result {
            Err(VeniceError::RateLimitExceeded(_)) => (),
            _ => panic!("Expected RateLimitExceeded error"),
        }
    }
}
