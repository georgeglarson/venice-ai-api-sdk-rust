//! HTTP client for Venice.ai API
//!
//! This module provides a dedicated HTTP client layer that only handles HTTP communication.

use reqwest::Client as ReqwestClient;
use serde::{de::DeserializeOwned, Serialize};
use std::sync::Arc;

use crate::error::{RateLimitInfo, VeniceError, VeniceResult};
use crate::http::response_processor;
use crate::http::url;

/// Configuration for the HTTP client
#[derive(Debug, Clone)]
pub struct HttpClientConfig {
    /// API key for authentication
    pub api_key: String,
    /// Base URL for the API
    pub base_url: String,
    /// Custom headers to include in all requests
    pub custom_headers: reqwest::header::HeaderMap,
    /// Timeout in seconds
    pub timeout_secs: Option<u64>,
}

/// Result type for HTTP operations
pub type HttpResult<T> = VeniceResult<(T, RateLimitInfo)>;

/// HTTP client for Venice.ai API
///
/// This client handles the low-level HTTP communication with the API,
/// including authentication, request building, and response processing.
#[derive(Debug, Clone)]
pub struct HttpClient {
    /// The underlying HTTP client
    client: ReqwestClient,
    /// The client configuration
    config: HttpClientConfig,
}

impl HttpClient {
    /// Create a new HTTP client with the given configuration
    pub fn new(config: HttpClientConfig) -> VeniceResult<Self> {
        let mut headers = reqwest::header::HeaderMap::new();
        
        // Add API key header
        let auth_value = format!("Bearer {}", config.api_key);
        let auth_header = reqwest::header::HeaderValue::from_str(&auth_value)
            .map_err(|e| VeniceError::InvalidInput(format!("Invalid API key: {}", e)))?;
        headers.insert(reqwest::header::AUTHORIZATION, auth_header);
        
        // Add custom headers
        for (key, value) in config.custom_headers.iter() {
            headers.insert(key.clone(), value.clone());
        }
        
        // Build the client
        let mut client_builder = ReqwestClient::builder()
            .default_headers(headers);
        
        // Add timeout if specified
        if let Some(timeout_secs) = config.timeout_secs {
            client_builder = client_builder.timeout(std::time::Duration::from_secs(timeout_secs));
        }
        
        let client = client_builder.build()
            .map_err(|e| VeniceError::InvalidInput(format!("Failed to create HTTP client: {}", e)))?;
        
        Ok(Self {
            client,
            config,
        })
    }
    
    /// Get the client configuration
    pub fn config(&self) -> &HttpClientConfig {
        &self.config
    }
    
    /// Send a GET request to the API
    pub async fn get<T: DeserializeOwned>(&self, endpoint: &str) -> HttpResult<T> {
        let url = url::build_url(&self.config.base_url, endpoint)?;
        
        let response = self.client.get(url)
            .send()
            .await
            .map_err(VeniceError::HttpError)?;
        
        response_processor::process_response(response).await
    }
    
    /// Send a GET request with query parameters to the API
    pub async fn get_with_query<Q: Serialize, T: DeserializeOwned>(
        &self,
        endpoint: &str,
        query: &Q,
    ) -> HttpResult<T> {
        let url = url::build_url(&self.config.base_url, endpoint)?;
        
        let response = self.client.get(url)
            .query(query)
            .send()
            .await
            .map_err(VeniceError::HttpError)?;
        
        response_processor::process_response(response).await
    }
    
    /// Send a POST request to the API
    pub async fn post<S: Serialize, T: DeserializeOwned>(
        &self,
        endpoint: &str,
        body: &S,
    ) -> HttpResult<T> {
        let url = url::build_url(&self.config.base_url, endpoint)?;
        
        let response = self.client.post(url)
            .json(body)
            .send()
            .await
            .map_err(VeniceError::HttpError)?;
        
        response_processor::process_response(response).await
    }
    
    /// Send a DELETE request to the API
    pub async fn delete<T: DeserializeOwned>(&self, endpoint: &str) -> HttpResult<T> {
        let url = url::build_url(&self.config.base_url, endpoint)?;
        
        let response = self.client.delete(url)
            .send()
            .await
            .map_err(VeniceError::HttpError)?;
        
        response_processor::process_response(response).await
    }
    
    /// Send a multipart POST request to the API
    pub async fn post_multipart<T: DeserializeOwned>(
        &self,
        endpoint: &str,
        form: reqwest::multipart::Form,
    ) -> HttpResult<T> {
        let url = url::build_url(&self.config.base_url, endpoint)?;
        
        let response = self.client.post(url)
            .multipart(form)
            .send()
            .await
            .map_err(VeniceError::HttpError)?;
        
        response_processor::process_response(response).await
    }
    
    /// Send a multipart POST request to the API and get a binary response
    pub async fn post_multipart_binary(
        &self,
        endpoint: &str,
        form: reqwest::multipart::Form,
    ) -> VeniceResult<(Vec<u8>, String, RateLimitInfo)> {
        let url = url::build_url(&self.config.base_url, endpoint)?;
        
        let response = self.client.post(url)
            .multipart(form)
            .send()
            .await
            .map_err(VeniceError::HttpError)?;
        
        response_processor::process_binary_response(response).await
    }
    
    /// Send a POST request to the API and get a streaming response
    pub async fn post_streaming<S: Serialize, T: DeserializeOwned + 'static + Send>(
        &self,
        endpoint: &str,
        body: &S,
    ) -> VeniceResult<(crate::traits::chat::ChatCompletionStream, RateLimitInfo)> {
        let url = url::build_url(&self.config.base_url, endpoint)?;
        
        let response = self.client.post(url)
            .json(body)
            .send()
            .await
            .map_err(VeniceError::HttpError)?;
        
        response_processor::process_streaming_response(response).await
    }
}

/// A shared HTTP client that can be cloned cheaply
pub type SharedHttpClient = Arc<HttpClient>;

/// Create a new shared HTTP client with the given configuration
pub fn new_shared_http_client(config: HttpClientConfig) -> VeniceResult<SharedHttpClient> {
    Ok(Arc::new(HttpClient::new(config)?))
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_http_client_creation() {
        let config = HttpClientConfig {
            api_key: "test_api_key".to_string(),
            base_url: "https://api.venice.ai".to_string(),
            custom_headers: reqwest::header::HeaderMap::new(),
            timeout_secs: None,
        };
        
        let client = HttpClient::new(config.clone()).unwrap();
        
        assert_eq!(client.config().api_key, "test_api_key");
        assert_eq!(client.config().base_url, "https://api.venice.ai");
    }
    
    #[test]
    fn test_http_client_with_custom_headers() {
        let mut custom_headers = reqwest::header::HeaderMap::new();
        custom_headers.insert(
            reqwest::header::HeaderName::from_static("x-custom-header"),
            reqwest::header::HeaderValue::from_static("custom-value"),
        );
        
        let config = HttpClientConfig {
            api_key: "test_api_key".to_string(),
            base_url: "https://api.venice.ai".to_string(),
            custom_headers,
            timeout_secs: None,
        };
        
        let client = HttpClient::new(config).unwrap();
        
        // We can't easily test that the headers are actually sent,
        // but we can at least verify that the client was created successfully
        assert_eq!(client.config().api_key, "test_api_key");
    }
    
    #[test]
    fn test_http_client_with_timeout() {
        let config = HttpClientConfig {
            api_key: "test_api_key".to_string(),
            base_url: "https://api.venice.ai".to_string(),
            custom_headers: reqwest::header::HeaderMap::new(),
            timeout_secs: Some(30),
        };
        
        let client = HttpClient::new(config).unwrap();
        
        assert_eq!(client.config().timeout_secs, Some(30));
    }
}