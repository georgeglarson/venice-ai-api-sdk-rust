//! # Venice.ai API SDK for Rust
//!
//! This crate provides a Rust SDK for the Venice.ai API.
//!
//! Venice.ai is an AI platform that provides models for text generation and image generation.
//! This SDK provides a type-safe way to interact with the Venice.ai API.
//!
//! ## Features
//!
//! - Chat completions API for text generation
//! - Image generation API
//! - Image styles API
//! - Image upscaling API
//! - Models listing API
//! - API key management
//! - Simple, type-safe interface
//!
//! ## Example
//!
//! ```rust,no_run
//! use venice_ai_api_sdk_rust::{
//!     Client,
//!     traits::chat::{ChatApi, ChatCompletionBuilder},
//!     traits::models::ModelsApi,
//! };
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create a client with your API key
//!     let api_key = "your-api-key";
//!     let client = Client::new(api_key)?;
//!     
//!     // List available models
//!     let (models, _) = client.list_models().await?;
//!     
//!     println!("Available models:");
//!     for model in models.data {
//!         println!("- {}", model.id);
//!     }
//!     
//!     // Build a chat completion request
//!     let request = ChatCompletionBuilder::new("llama-3.2-3b")
//!         .add_system("You are a helpful assistant.")
//!         .add_user("Tell me about AI")
//!         .max_tokens(100)
//!         .temperature(0.7)
//!         .build();
//!     
//!     // Send the request
//!     let (response, _) = client.create_chat_completion(request).await?;
//!     
//!     // Print the response
//!     println!("AI response: {}", response.choices[0].message.content);
//!     
//!     Ok(())
//! }
//! ```

// Internal modules
mod error;
mod config;
mod http;
mod client;
mod utils;
#[macro_use]
mod macros;
// TODO: Fix middleware module
// mod middleware;
mod pagination;
mod retry;
mod rate_limit;
mod api;
mod services;

// Public modules
pub mod traits;
pub mod models;
pub mod chat;
pub mod image;
pub mod api_keys;
pub mod webhooks;

// Public exports
pub use error::{VeniceError, VeniceResult, RateLimitInfo};
pub use config::{ClientConfig, DEFAULT_BASE_URL};
pub use client::{Client, SharedClient, new_shared_client};
pub use http::{HttpClient, HttpClientConfig, HttpResult, SharedHttpClient, new_shared_http_client};
// TODO: Fix middleware module
// pub use middleware::{
//     Middleware, MiddlewareChain, Request, Method, Next,
//     RateLimiterMiddleware, RetryMiddleware,
// };
pub use pagination::{
    PaginatedResponse, PaginationParams, Paginator,
    PaginationInfo, create_paginator, create_async_paginator,
};
pub use retry::{RetryConfig, with_retry};
pub use rate_limit::{RateLimiter, RateLimiterConfig, new_shared_rate_limiter, new_shared_rate_limiter_with_config};
pub use api::{ChatApiImpl, ImageApiImpl, ModelsApiImpl, ApiKeysApiImpl};
pub use services::webhook::WebhookService;

// Re-export utility modules
pub mod util {
    //! Utility functions for working with the Venice AI API
    
    pub use crate::utils::serialization;
    pub use crate::utils::validation;
}
