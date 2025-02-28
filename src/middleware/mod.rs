//! Middleware for the Venice AI API SDK
//!
//! This module provides middleware components for cross-cutting concerns
//! such as rate limiting, retries, and logging.

mod rate_limiter;
mod retry;

pub use rate_limiter::RateLimiterMiddleware;
pub use retry::RetryMiddleware;

use std::future::Future;
use std::pin::Pin;

use crate::http::HttpResult;

/// A request to be processed by middleware
#[derive(Debug, Clone)]
pub struct Request<B = ()> {
    /// The endpoint to send the request to
    pub endpoint: String,
    /// The HTTP method to use
    pub method: Method,
    /// The request body
    pub body: Option<B>,
}

/// HTTP methods supported by the middleware
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Method {
    /// GET request
    Get,
    /// POST request
    Post,
    /// DELETE request
    Delete,
}

/// A function that processes the next middleware in the chain
pub struct Next<'a, T, B = ()> {
    /// The next middleware function to call
    pub(crate) next: Box<dyn FnOnce(Request<B>) -> Pin<Box<dyn Future<Output = HttpResult<T>> + Send>> + Send + 'a>,
}

impl<'a, T, B> Next<'a, T, B> {
    /// Create a new Next with the given function
    pub fn new<F, Fut>(f: F) -> Self
    where
        F: FnOnce(Request<B>) -> Fut + Send + 'a,
        Fut: Future<Output = HttpResult<T>> + Send + 'static,
    {
        Self {
            next: Box::new(move |req| Box::pin(f(req))),
        }
    }
    
    /// Run the next middleware function
    pub async fn run(self, req: Request<B>) -> HttpResult<T> {
        (self.next)(req).await
    }
}

/// Middleware trait for processing requests
#[async_trait::async_trait]
pub trait Middleware<B = ()>: Send + Sync {
    /// Process a request, potentially modifying it before passing it to the next middleware
    async fn process<T: Send + 'static>(&self, request: Request<B>, next: Next<'_, T, B>) -> HttpResult<T>;
}

/// A chain of middleware components
#[derive(Clone)]
pub struct MiddlewareChain<B = ()> {
    /// The middleware components in the chain
    middlewares: Vec<Box<dyn Middleware<B> + Send + Sync + 'static>>,
}

impl<B: Send + 'static> MiddlewareChain<B> {
    /// Create a new empty middleware chain
    pub fn new() -> Self {
        Self {
            middlewares: Vec::new(),
        }
    }
    
    /// Add a middleware to the chain
    pub fn add<M: Middleware<B> + Send + Sync + 'static>(mut self, middleware: M) -> Self {
        self.middlewares.push(Box::new(middleware));
        self
    }
    
    /// Process a request through the middleware chain
    pub async fn process<T, F, Fut>(&self, request: Request<B>, handler: F) -> HttpResult<T>
    where
        T: Send + 'static,
        F: FnOnce(Request<B>) -> Fut + Send + Clone + 'static,
        Fut: Future<Output = HttpResult<T>> + Send + 'static,
    {
        let middlewares = self.middlewares.iter().rev().cloned().collect::<Vec<_>>();
        
        // If there are no middlewares, just call the handler directly
        if middlewares.is_empty() {
            return handler(request).await;
        }
        
        // Create a chain of nested Next functions
        let mut next = Next::new(handler);
        
        for middleware in middlewares {
            let prev_next = next;
            let middleware_clone = middleware.clone();
            
            next = Next::new(move |req| {
                let middleware = middleware_clone.clone();
                let prev_next = prev_next.clone();
                
                async move {
                    middleware.process(req, prev_next).await
                }
            });
        }
        
        // Process the request through the middleware chain
        next.run(request).await
    }
}

impl<B> Default for MiddlewareChain<B> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use crate::error::RateLimitInfo;
    
    #[derive(Clone)]
    struct TestMiddleware {
        name: &'static str,
        counter: Arc<AtomicUsize>,
    }
    
    #[async_trait::async_trait]
    impl Middleware for TestMiddleware {
        async fn process<T: Send + 'static>(&self, request: Request, next: Next<'_, T>) -> HttpResult<T> {
            println!("Before middleware: {}", self.name);
            self.counter.fetch_add(1, Ordering::SeqCst);
            
            let result = next.run(request).await;
            
            println!("After middleware: {}", self.name);
            
            result
        }
    }
    
    #[tokio::test]
    async fn test_middleware_chain() {
        let counter1 = Arc::new(AtomicUsize::new(0));
        let counter2 = Arc::new(AtomicUsize::new(0));
        
        let middleware1 = TestMiddleware {
            name: "middleware1",
            counter: counter1.clone(),
        };
        
        let middleware2 = TestMiddleware {
            name: "middleware2",
            counter: counter2.clone(),
        };
        
        let chain = MiddlewareChain::new()
            .add(middleware1)
            .add(middleware2);
        
        let request = Request {
            endpoint: "/test".to_string(),
            method: Method::Get,
            body: None,
        };
        
        let handler = |_req| async {
            println!("Handler called");
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
        
        let result = chain.process(request, handler).await;
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap().0, 42);
        assert_eq!(counter1.load(Ordering::SeqCst), 1);
        assert_eq!(counter2.load(Ordering::SeqCst), 1);
    }
}