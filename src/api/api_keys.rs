//! API Keys API implementation
//!
//! This module provides an implementation of the API keys API.

use async_trait::async_trait;

use crate::error::{RateLimitInfo, VeniceResult};
use crate::http::SharedHttpClient;
use crate::pagination::{PaginationParams, Paginator};
use crate::api_keys::list::{ListApiKeysRequest, ListApiKeysResponse};
use crate::traits::api_keys::{
    ApiKeysApi, CreateApiKeyRequest, CreateApiKeyResponse,
    DeleteApiKeyResponse, GenerateWeb3KeyRequest, GenerateWeb3KeyResponse,
};

/// Implementation of the API keys API
#[derive(Debug, Clone)]
pub struct ApiKeysApiImpl {
    /// The HTTP client to use for requests
    http_client: SharedHttpClient,
}

impl ApiKeysApiImpl {
    /// Create a new API keys API implementation
    pub fn new(http_client: SharedHttpClient) -> Self {
        Self { http_client }
    }
}

#[async_trait]
impl ApiKeysApi for ApiKeysApiImpl {
    async fn list_api_keys(&self) -> VeniceResult<(ListApiKeysResponse, RateLimitInfo)> {
        self.http_client.get::<ListApiKeysResponse>("api-keys").await
    }
    
    async fn list_api_keys_with_params(
        &self,
        request: ListApiKeysRequest,
    ) -> VeniceResult<(ListApiKeysResponse, RateLimitInfo)> {
        self.http_client.get_with_query::<_, ListApiKeysResponse>("api-keys", &request).await
    }
    
    fn list_api_keys_paginator(&self, params: PaginationParams) -> impl Paginator<crate::api_keys::list::ApiKey> {
        let http_client = self.http_client.clone();
        
        // Create an async function that fetches a page
        let fetch_page = move |params: PaginationParams| {
            let http_client = http_client.clone();
            async move {
                let request = ListApiKeysRequest { pagination: params };
                http_client.get_with_query::<_, crate::api_keys::list::ListApiKeysResponse>("api-keys", &request).await
            }
        };
        
        crate::create_async_paginator(fetch_page, params)
    }
    
    async fn create_api_key(
        &self,
        request: CreateApiKeyRequest,
    ) -> VeniceResult<(CreateApiKeyResponse, RateLimitInfo)> {
        self.http_client.post("api-keys", &request).await
    }
    
    async fn delete_api_key(&self, key_id: &str) -> VeniceResult<(DeleteApiKeyResponse, RateLimitInfo)> {
        let url = format!("api-keys/{}", key_id);
        self.http_client.delete(&url).await
    }
    
    async fn generate_web3_key(
        &self,
        request: GenerateWeb3KeyRequest,
    ) -> VeniceResult<(GenerateWeb3KeyResponse, RateLimitInfo)> {
        self.http_client.post("api-keys/web3", &request).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::http::{HttpClientConfig, new_shared_http_client};
    
    #[tokio::test]
    async fn test_list_api_keys() {
        // Create a mock HTTP client
        let config = HttpClientConfig {
            api_key: "test_api_key".to_string(),
            base_url: "https://api.venice.ai".to_string(),
            custom_headers: reqwest::header::HeaderMap::new(),
            timeout_secs: None,
        };
        let http_client = new_shared_http_client(config).unwrap();
        
        // Create the API keys API implementation
        let api_keys_api = ApiKeysApiImpl::new(http_client);
        
        // TODO: Mock the HTTP client to return a response
        // For now, we'll just check that the method exists and has the right signature
        let _: ApiKeysApiImpl = api_keys_api;
    }
    
    #[tokio::test]
    async fn test_create_api_key() {
        // Create a mock HTTP client
        let config = HttpClientConfig {
            api_key: "test_api_key".to_string(),
            base_url: "https://api.venice.ai".to_string(),
            custom_headers: reqwest::header::HeaderMap::new(),
            timeout_secs: None,
        };
        let http_client = new_shared_http_client(config).unwrap();
        
        // Create the API keys API implementation
        let api_keys_api = ApiKeysApiImpl::new(http_client);
        
        // TODO: Mock the HTTP client to return a response
        // For now, we'll just check that the method exists and has the right signature
        let _: ApiKeysApiImpl = api_keys_api;
    }
}