use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::error::{RateLimitInfo, VeniceResult};
use crate::pagination::{PaginationParams, Paginator};
use crate::api_keys::list::{ApiKey, ListApiKeysRequest, ListApiKeysResponse};

/// Request to create a new API key
#[derive(Debug, Clone, Serialize)]
pub struct CreateApiKeyRequest {
    /// The name of the API key
    pub name: String,
}

/// Response from creating a new API key
#[derive(Debug, Clone, Deserialize)]
pub struct CreateApiKeyResponse {
    /// The created API key
    pub key: ApiKey,
    /// The full API key value (only returned once)
    pub secret: String,
}

/// Response from deleting an API key
#[derive(Debug, Clone, Deserialize)]
pub struct DeleteApiKeyResponse {
    /// Whether the deletion was successful
    pub deleted: bool,
    /// The ID of the deleted API key
    pub id: String,
}

/// Request to generate a Web3 key
#[derive(Debug, Clone, Serialize)]
pub struct GenerateWeb3KeyRequest {
    /// The Ethereum address to generate a key for
    pub address: String,
    /// The signature of the message
    pub signature: String,
}

/// Response from generating a Web3 key
#[derive(Debug, Clone, Deserialize)]
pub struct GenerateWeb3KeyResponse {
    /// The generated API key
    pub key: String,
}

/// API Keys API trait
#[async_trait]
pub trait ApiKeysApi {
    /// List all API keys
    async fn list_api_keys(&self) -> VeniceResult<(ListApiKeysResponse, RateLimitInfo)>;
    
    /// List API keys with pagination parameters
    async fn list_api_keys_with_params(
        &self,
        request: ListApiKeysRequest,
    ) -> VeniceResult<(ListApiKeysResponse, RateLimitInfo)>;
    
    /// Create a paginator for listing API keys
    fn list_api_keys_paginator(&self, params: PaginationParams) -> impl Paginator<ApiKey>;
    
    /// Create a new API key
    async fn create_api_key(
        &self,
        request: CreateApiKeyRequest,
    ) -> VeniceResult<(CreateApiKeyResponse, RateLimitInfo)>;
    
    /// Delete an API key
    async fn delete_api_key(&self, key_id: &str) -> VeniceResult<(DeleteApiKeyResponse, RateLimitInfo)>;
    
    /// Generate a Web3 key
    async fn generate_web3_key(
        &self,
        request: GenerateWeb3KeyRequest,
    ) -> VeniceResult<(GenerateWeb3KeyResponse, RateLimitInfo)>;
}
