//! HTTP utilities for the Venice AI API SDK

mod client;
mod client_factory;
mod response_processor;
mod url;

pub use client::{HttpClient, HttpClientConfig, HttpResult, SharedHttpClient, new_shared_http_client};
pub use client_factory::create_client;
pub use response_processor::{process_response, process_binary_response, process_streaming_response};
pub use url::build_url;