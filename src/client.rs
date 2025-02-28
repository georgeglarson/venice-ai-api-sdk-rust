use futures::Stream;
use reqwest::Client as ReqwestClient;
use serde::{de::DeserializeOwned, Serialize};
use std::pin::Pin;
use std::sync::Arc;

use crate::api::{ApiKeysApiImpl, ChatApiImpl, ImageApiImpl, ModelsApiImpl};
use crate::config::ClientConfig;
use crate::error::{RateLimitInfo, VeniceError, VeniceResult};
use crate::http::{self, HttpClientConfig, new_shared_http_client};
use crate::rate_limit::{RateLimiter, RateLimiterConfig};
use crate::retry::{RetryConfig, with_retry};

/// The main client for the Venice.ai API
#[derive(Debug, Clone)]
pub struct Client {
    /// The underlying HTTP client
    client: ReqwestClient,
    /// The client configuration
    config: ClientConfig,
    /// Retry configuration
    retry_config: Option<RetryConfig>,
    /// Rate limiter for managing API rate limits
    rate_limiter: Option<Arc<RateLimiter>>,
    /// Chat API implementation
    chat_api: ChatApiImpl,
    /// Models API implementation
    models_api: ModelsApiImpl,
    /// Image API implementation
    image_api: ImageApiImpl,
    /// API Keys API implementation
    api_keys_api: ApiKeysApiImpl,
}

impl Client {
    /// Create a new client with the given API key
    pub fn new(api_key: impl Into<String>) -> VeniceResult<Self> {
        Self::with_config(ClientConfig::new(api_key))
    }

    /// Create a new client builder
    pub fn builder() -> ClientBuilder {
        ClientBuilder::new()
    }

    /// Create a new client with the given configuration
    pub fn with_config(config: ClientConfig) -> VeniceResult<Self> {
        let client = http::create_client(&config)?;
        
        // Create the HTTP client for the API implementations
        let http_client_config = HttpClientConfig {
            api_key: config.api_key.clone(),
            base_url: config.base_url.clone(),
            custom_headers: config.custom_headers.clone(),
            timeout_secs: config.timeout_secs,
        };
        let http_client = new_shared_http_client(http_client_config)?;
        
        // Create the API implementations
        let chat_api = ChatApiImpl::new(http_client.clone());
        let models_api = ModelsApiImpl::new(http_client.clone());
        let image_api = ImageApiImpl::new(http_client.clone());
        let api_keys_api = ApiKeysApiImpl::new(http_client);
        
        Ok(Self {
            client,
            config,
            retry_config: None,
            rate_limiter: None,
            chat_api,
            models_api,
            image_api,
            api_keys_api,
        })
    }

    /// Get the client configuration
    pub fn config(&self) -> &ClientConfig {
        &self.config
    }
    
    /// Set the retry configuration
    pub fn with_retry_config(mut self, retry_config: RetryConfig) -> Self {
        self.retry_config = Some(retry_config);
        self
    }
    
    /// Enable retries with default configuration
    pub fn with_retries(self) -> Self {
        self.with_retry_config(RetryConfig::default())
    }
    
    /// Get the retry configuration
    pub fn retry_config(&self) -> Option<&RetryConfig> {
        self.retry_config.as_ref()
    }
    
    /// Set the rate limiter
    pub fn with_rate_limiter(mut self, rate_limiter: Arc<RateLimiter>) -> Self {
        self.rate_limiter = Some(rate_limiter);
        self
    }
    
    /// Enable rate limiting with default configuration
    pub fn with_rate_limiting(self) -> Self {
        let rate_limiter = Arc::new(RateLimiter::new());
        self.with_rate_limiter(rate_limiter)
    }
    
    /// Enable rate limiting with custom configuration
    pub fn with_rate_limiting_config(self, config: RateLimiterConfig) -> Self {
        let rate_limiter = Arc::new(RateLimiter::with_config(config));
        self.with_rate_limiter(rate_limiter)
    }
    
    /// Get the rate limiter
    pub fn rate_limiter(&self) -> Option<&Arc<RateLimiter>> {
        self.rate_limiter.as_ref()
    }

    /// Send a GET request to the API
    pub async fn get<T: DeserializeOwned>(
        &self,
        endpoint: &str,
    ) -> VeniceResult<(T, RateLimitInfo)> {
        // Check rate limits before making the request
        if let Some(rate_limiter) = &self.rate_limiter {
            rate_limiter.acquire().await?;
        }
        
        let url = http::build_url(&self.config.base_url, endpoint)?;
        
        let result = if let Some(retry_config) = &self.retry_config {
            with_retry(|| async {
                let response = self.client.get(url.clone()).send().await.map_err(VeniceError::HttpError)?;
                http::process_response(response).await
            }, retry_config).await
        } else {
            let response = self.client.get(url).send().await.map_err(VeniceError::HttpError)?;
            http::process_response(response).await
        };
        
        // Update rate limit information
        if let Ok((_, ref rate_limit_info)) = result {
            if let Some(rate_limiter) = &self.rate_limiter {
                rate_limiter.update_from_response(rate_limit_info);
            }
        }
        
        result
    }
    
    /// Send a GET request with query parameters to the API
    pub async fn get_with_query<Q: Serialize, T: DeserializeOwned>(
        &self,
        endpoint: &str,
        query: &Q,
    ) -> VeniceResult<(T, RateLimitInfo)> {
        // Check rate limits before making the request
        if let Some(rate_limiter) = &self.rate_limiter {
            rate_limiter.acquire().await?;
        }
        
        let url = http::build_url(&self.config.base_url, endpoint)?;
        
        let result = if let Some(retry_config) = &self.retry_config {
            // For retries, we need to clone the query parameters
            // Since we can't easily clone Q, we'll rebuild the request each time
            let endpoint = endpoint.to_string();
            
            with_retry(|| async {
                let url = http::build_url(&self.config.base_url, &endpoint)?;
                
                // For each retry, we'll use the original query
                let response = self.client
                    .get(url)
                    .query(query)
                    .send()
                    .await
                    .map_err(VeniceError::HttpError)?;
                
                http::process_response(response).await
            }, retry_config).await
        } else {
            let response = self.client
                .get(url)
                .query(query)
                .send()
                .await
                .map_err(VeniceError::HttpError)?;
            
            http::process_response(response).await
        };
        
        // Update rate limit information
        if let Ok((_, ref rate_limit_info)) = result {
            if let Some(rate_limiter) = &self.rate_limiter {
                rate_limiter.update_from_response(rate_limit_info);
            }
        }
        
        result
    }

    /// Send a POST request to the API
    pub async fn post<S: Serialize, T: DeserializeOwned>(
        &self,
        endpoint: &str,
        body: &S,
    ) -> VeniceResult<(T, RateLimitInfo)> {
        // Check rate limits before making the request
        if let Some(rate_limiter) = &self.rate_limiter {
            rate_limiter.acquire().await?;
        }
        
        let url = http::build_url(&self.config.base_url, endpoint)?;
        
        let result = if let Some(retry_config) = &self.retry_config {
            // For retries, we need to clone the body
            // Since we can't easily clone S, we'll rebuild the request each time
            let endpoint = endpoint.to_string();
            
            with_retry(|| async {
                let url = http::build_url(&self.config.base_url, &endpoint)?;
                
                // For each retry, we'll use the original body
                let response = self
                    .client
                    .post(url)
                    .json(body)
                    .send()
                    .await
                    .map_err(VeniceError::HttpError)?;
                
                http::process_response(response).await
            }, retry_config).await
        } else {
            let response = self
                .client
                .post(url)
                .json(body)
                .send()
                .await
                .map_err(VeniceError::HttpError)?;
            
            http::process_response(response).await
        };
        
        // Update rate limit information
        if let Ok((_, ref rate_limit_info)) = result {
            if let Some(rate_limiter) = &self.rate_limiter {
                rate_limiter.update_from_response(rate_limit_info);
            }
        }
        
        result
    }

    /// Send a DELETE request to the API
    pub async fn delete<T: DeserializeOwned>(
        &self,
        endpoint: &str,
    ) -> VeniceResult<(T, RateLimitInfo)> {
        // Check rate limits before making the request
        if let Some(rate_limiter) = &self.rate_limiter {
            rate_limiter.acquire().await?;
        }
        
        let url = http::build_url(&self.config.base_url, endpoint)?;
        
        let result = if let Some(retry_config) = &self.retry_config {
            with_retry(|| async {
                let response = self
                    .client
                    .delete(url.clone())
                    .send()
                    .await
                    .map_err(VeniceError::HttpError)?;
                
                http::process_response(response).await
            }, retry_config).await
        } else {
            let response = self
                .client
                .delete(url)
                .send()
                .await
                .map_err(VeniceError::HttpError)?;
            
            http::process_response(response).await
        };
        
        // Update rate limit information
        if let Ok((_, ref rate_limit_info)) = result {
            if let Some(rate_limiter) = &self.rate_limiter {
                rate_limiter.update_from_response(rate_limit_info);
            }
        }
        
        result
    }

    /// Send a multipart POST request to the API
    pub async fn post_multipart<T: DeserializeOwned>(
        &self,
        endpoint: &str,
        form: reqwest::multipart::Form,
    ) -> VeniceResult<(T, RateLimitInfo)> {
        // Check rate limits before making the request
        if let Some(rate_limiter) = &self.rate_limiter {
            rate_limiter.acquire().await?;
        }
        
        let url = http::build_url(&self.config.base_url, endpoint)?;
        
        // Multipart forms can't be easily cloned for retries
        // For now, we don't support retries for multipart requests
        let response = self
            .client
            .post(url)
            .multipart(form)
            .send()
            .await
            .map_err(VeniceError::HttpError)?;
        
        let result = http::process_response(response).await;
        
        // Update rate limit information
        if let Ok((_, ref rate_limit_info)) = result {
            if let Some(rate_limiter) = &self.rate_limiter {
                rate_limiter.update_from_response(rate_limit_info);
            }
        }
        
        result
    }

    /// Send a multipart POST request to the API and get a binary response
    pub async fn post_multipart_binary(
        &self,
        endpoint: &str,
        form: reqwest::multipart::Form,
    ) -> VeniceResult<(Vec<u8>, String, RateLimitInfo)> {
        // Check rate limits before making the request
        if let Some(rate_limiter) = &self.rate_limiter {
            rate_limiter.acquire().await?;
        }
        
        let url = http::build_url(&self.config.base_url, endpoint)?;
        
        // Multipart forms can't be easily cloned for retries
        // For now, we don't support retries for multipart requests
        let response = self
            .client
            .post(url)
            .multipart(form)
            .send()
            .await
            .map_err(VeniceError::HttpError)?;
        
        let result = http::process_binary_response(response).await;
        
        // Update rate limit information
        if let Ok((_, _, ref rate_limit_info)) = result {
            if let Some(rate_limiter) = &self.rate_limiter {
                rate_limiter.update_from_response(rate_limit_info);
            }
        }
        
        result
    }
    
    /// Send a POST request to the API and get a streaming response
    pub async fn post_streaming<S: Serialize, T: DeserializeOwned + 'static + Send>(
        &self,
        endpoint: &str,
        body: &S,
    ) -> VeniceResult<(Pin<Box<dyn Stream<Item = VeniceResult<T>> + Send>>, RateLimitInfo)> {
        // Check rate limits before making the request
        if let Some(rate_limiter) = &self.rate_limiter {
            rate_limiter.acquire().await?;
        }
        
        let url = http::build_url(&self.config.base_url, endpoint)?;
        
        let result = if let Some(retry_config) = &self.retry_config {
            // For retries, we need to clone the body
            // Since we can't easily clone S, we'll rebuild the request each time
            let endpoint = endpoint.to_string();
            
            with_retry(|| async {
                let url = http::build_url(&self.config.base_url, &endpoint)?;
                
                // For each retry, we'll use the original body
                let response = self
                    .client
                    .post(url)
                    .json(body)
                    .send()
                    .await
                    .map_err(VeniceError::HttpError)?;
                
                http::process_streaming_response(response).await
            }, retry_config).await
        } else {
            let response = self
                .client
                .post(url)
                .json(body)
                .send()
                .await
                .map_err(VeniceError::HttpError)?;
            
            http::process_streaming_response(response).await
        };
        
        // Update rate limit information
        if let Ok((_, ref rate_limit_info)) = result {
            if let Some(rate_limiter) = &self.rate_limiter {
                rate_limiter.update_from_response(rate_limit_info);
            }
        }
        
        result
    }
}

/// A shared client that can be cloned cheaply
pub type SharedClient = Arc<Client>;

/// A builder for creating a client
#[derive(Debug, Clone)]
pub struct ClientBuilder {
    api_key: Option<String>,
    base_url: Option<String>,
    retry_config: Option<RetryConfig>,
    rate_limiter: Option<Arc<RateLimiter>>,
}

impl ClientBuilder {
    /// Create a new client builder
    pub fn new() -> Self {
        Self {
            api_key: None,
            base_url: None,
            retry_config: None,
            rate_limiter: None,
        }
    }

    /// Set the API key
    pub fn api_key(mut self, api_key: impl Into<String>) -> Self {
        self.api_key = Some(api_key.into());
        self
    }

    /// Set the base URL
    pub fn base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = Some(base_url.into());
        self
    }
    
    /// Set the retry configuration
    pub fn retry_config(mut self, retry_config: RetryConfig) -> Self {
        self.retry_config = Some(retry_config);
        self
    }
    
    /// Enable retries with default configuration
    pub fn with_retries(mut self) -> Self {
        self.retry_config = Some(RetryConfig::default());
        self
    }
    
    /// Set the rate limiter
    pub fn rate_limiter(mut self, rate_limiter: Arc<RateLimiter>) -> Self {
        self.rate_limiter = Some(rate_limiter);
        self
    }
    
    /// Enable rate limiting with default configuration
    pub fn with_rate_limiting(self) -> Self {
        let rate_limiter = Arc::new(RateLimiter::new());
        self.rate_limiter(rate_limiter)
    }
    
    /// Enable rate limiting with custom configuration
    pub fn with_rate_limiting_config(self, config: RateLimiterConfig) -> Self {
        let rate_limiter = Arc::new(RateLimiter::with_config(config));
        self.rate_limiter(rate_limiter)
    }

    /// Build the client
    pub fn build(self) -> VeniceResult<Client> {
        let api_key = self.api_key.ok_or_else(|| VeniceError::InvalidInput("API key is required".to_string()))?;
        let base_url = self.base_url.unwrap_or_else(|| crate::config::DEFAULT_BASE_URL.to_string());
        
        let config = ClientConfig {
            api_key,
            base_url,
            custom_headers: reqwest::header::HeaderMap::new(),
            timeout_secs: None,
        };
        
        let mut client = Client::with_config(config)?;
        
        if let Some(retry_config) = self.retry_config {
            client = client.with_retry_config(retry_config);
        }
        
        if let Some(rate_limiter) = self.rate_limiter {
            client = client.with_rate_limiter(rate_limiter);
        }
        
        Ok(client)
    }
}

/// Create a new shared client with the given API key
pub fn new_shared_client(api_key: impl Into<String>) -> VeniceResult<SharedClient> {
    Ok(Arc::new(Client::new(api_key)?))
}

// Implement the ChatApi trait for Client by delegating to the chat_api
#[async_trait::async_trait]
impl crate::traits::chat::ChatApi for Client {
    async fn create_chat_completion(
        &self,
        request: crate::traits::chat::ChatCompletionRequest,
    ) -> VeniceResult<(crate::traits::chat::ChatCompletionResponse, RateLimitInfo)> {
        self.chat_api.create_chat_completion(request).await
    }
    
    async fn create_streaming_chat_completion(
        &self,
        request: crate::traits::chat::ChatCompletionRequest,
    ) -> VeniceResult<(crate::traits::chat::ChatCompletionStream, RateLimitInfo)> {
        self.chat_api.create_streaming_chat_completion(request).await
    }
}

// Implement the ModelsApi trait for Client by delegating to the models_api
#[async_trait::async_trait]
impl crate::traits::models::ModelsApi for Client {
    async fn list_models(&self) -> VeniceResult<(crate::traits::models::ListModelsResponse, RateLimitInfo)> {
        self.models_api.list_models().await
    }
    
    async fn list_models_with_params(
        &self,
        request: crate::models::list::ListModelsRequest,
    ) -> VeniceResult<(crate::models::list::ListModelsResponse, RateLimitInfo)> {
        self.models_api.list_models_with_params(request).await
    }
    
    fn list_models_paginator(&self, params: crate::pagination::PaginationParams) -> impl crate::pagination::Paginator<crate::models::list::Model> {
        self.models_api.list_models_paginator(params)
    }
    
    async fn get_model_traits(&self, model_id: &str) -> VeniceResult<(crate::traits::models::ModelTraitsResponse, RateLimitInfo)> {
        self.models_api.get_model_traits(model_id).await
    }
    
    async fn get_model_traits_internal(
        &self,
        request: Option<crate::models::traits::ModelTraitsRequest>,
    ) -> VeniceResult<(crate::models::traits::ModelTraitsResponse, RateLimitInfo)> {
        self.models_api.get_model_traits_internal(request).await
    }
    
    async fn is_model_compatible(&self, model_id: &str, feature: &str) -> VeniceResult<bool> {
        self.models_api.is_model_compatible(model_id, feature).await
    }
}

// Implement the ImageApi trait for Client by delegating to the image_api
#[async_trait::async_trait]
impl crate::traits::image::ImageApi for Client {
    async fn generate_image(
        &self,
        request: crate::traits::image::ImageGenerateRequest,
    ) -> VeniceResult<(crate::traits::image::ImageGenerateResponse, RateLimitInfo)> {
        self.image_api.generate_image(request).await
    }
    
    async fn upscale_image(
        &self,
        request: crate::traits::image::ImageUpscaleRequest,
    ) -> VeniceResult<crate::traits::image::ImageUpscaleResponse> {
        self.image_api.upscale_image(request).await
    }
    
    async fn list_styles(&self) -> VeniceResult<(crate::traits::image::ListImageStylesResponse, RateLimitInfo)> {
        self.image_api.list_styles().await
    }
}

// Additional image API methods not part of the ImageApi trait
impl Client {
    /// Get models that are compatible with image generation
    pub async fn get_compatible_models(&self) -> VeniceResult<(Vec<crate::models::list::Model>, RateLimitInfo)> {
        self.image_api.get_compatible_models().await
    }
}

// Implement the ApiKeysApi trait for Client by delegating to the api_keys_api
#[async_trait::async_trait]
impl crate::traits::api_keys::ApiKeysApi for Client {
    async fn list_api_keys(&self) -> VeniceResult<(crate::api_keys::list::ListApiKeysResponse, RateLimitInfo)> {
        self.api_keys_api.list_api_keys().await
    }
    
    async fn list_api_keys_with_params(
        &self,
        request: crate::api_keys::list::ListApiKeysRequest,
    ) -> VeniceResult<(crate::api_keys::list::ListApiKeysResponse, RateLimitInfo)> {
        self.api_keys_api.list_api_keys_with_params(request).await
    }
    
    fn list_api_keys_paginator(&self, params: crate::pagination::PaginationParams) -> impl crate::pagination::Paginator<crate::api_keys::list::ApiKey> {
        self.api_keys_api.list_api_keys_paginator(params)
    }
    
    async fn create_api_key(
        &self,
        request: crate::traits::api_keys::CreateApiKeyRequest,
    ) -> VeniceResult<(crate::traits::api_keys::CreateApiKeyResponse, RateLimitInfo)> {
        self.api_keys_api.create_api_key(request).await
    }
    
    async fn delete_api_key(&self, key_id: &str) -> VeniceResult<(crate::traits::api_keys::DeleteApiKeyResponse, RateLimitInfo)> {
        self.api_keys_api.delete_api_key(key_id).await
    }
    
    async fn generate_web3_key(
        &self,
        request: crate::traits::api_keys::GenerateWeb3KeyRequest,
    ) -> VeniceResult<(crate::traits::api_keys::GenerateWeb3KeyResponse, RateLimitInfo)> {
        self.api_keys_api.generate_web3_key(request).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let client = Client::new("test_api_key").unwrap();
        assert_eq!(client.config.api_key, "test_api_key");
        assert_eq!(client.config.base_url, crate::config::DEFAULT_BASE_URL);
    }

    #[test]
    fn test_client_with_custom_base_url() {
        let client = Client::builder()
            .api_key("test_api_key")
            .base_url("https://custom.api.example.com")
            .build()
            .unwrap();
        
        assert_eq!(client.config.api_key, "test_api_key");
        assert_eq!(client.config.base_url, "https://custom.api.example.com");
    }
    
    #[test]
    fn test_client_with_retry_config() {
        let client = Client::builder()
            .api_key("test_api_key")
            .with_retries()
            .build()
            .unwrap();
        
        assert!(client.retry_config().is_some());
        let retry_config = client.retry_config().unwrap();
        assert_eq!(retry_config.max_retries, 3);
    }
    
    #[test]
    fn test_client_with_custom_retry_config() {
        let retry_config = RetryConfig::new()
            .max_retries(5)
            .initial_delay_ms(1000)
            .max_delay_ms(20000)
            .backoff_factor(3.0)
            .add_jitter(false);
            
        let client = Client::builder()
            .api_key("test_api_key")
            .retry_config(retry_config.clone())
            .build()
            .unwrap();
        
        assert!(client.retry_config().is_some());
        let client_retry_config = client.retry_config().unwrap();
        assert_eq!(client_retry_config.max_retries, 5);
        assert_eq!(client_retry_config.initial_delay_ms, 1000);
        assert_eq!(client_retry_config.max_delay_ms, 20000);
        assert_eq!(client_retry_config.backoff_factor, 3.0);
        assert_eq!(client_retry_config.add_jitter, false);
    }
    
    #[test]
    fn test_client_with_rate_limiter() {
        let client = Client::builder()
            .api_key("test_api_key")
            .with_rate_limiting()
            .build()
            .unwrap();
        
        assert!(client.rate_limiter().is_some());
    }
    
    #[test]
    fn test_client_with_custom_rate_limiter_config() {
        let rate_limiter_config = RateLimiterConfig {
            auto_wait: false,
            max_wait_time: 30,
        };
        
        let client = Client::builder()
            .api_key("test_api_key")
            .with_rate_limiting_config(rate_limiter_config)
            .build()
            .unwrap();
        
        assert!(client.rate_limiter().is_some());
    }
}
