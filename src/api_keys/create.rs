use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{
    client::Client,
    error::{RateLimitInfo, VeniceResult},
};

/// The endpoint for creating API keys
const API_KEYS_ENDPOINT: &str = "api_keys";

/// Request for creating an API key
#[derive(Debug, Clone, Serialize)]
pub struct CreateApiKeyRequest {
    /// Name of the API key
    pub name: String,
    /// Rate limit configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limits: Option<CreateApiKeyRateLimits>,
    /// Additional custom parameters
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// Rate limit configuration for creating an API key
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateApiKeyRateLimits {
    /// Requests per minute limit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requests_per_minute: Option<u32>,
    /// Requests per day limit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requests_per_day: Option<u32>,
    /// Tokens per minute limit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tokens_per_minute: Option<u32>,
}

/// Response from creating an API key
#[derive(Debug, Deserialize)]
pub struct CreateApiKeyResponse {
    /// The created API key object
    pub data: CreatedApiKey,
    /// Type of object
    pub object: String,
}

/// Information about a created API key
#[derive(Debug, Deserialize, Clone)]
pub struct CreatedApiKey {
    /// The API key identifier
    pub id: String,
    /// The type of object
    pub object: String,
    /// The name of the API key
    pub name: String,
    /// When the API key was created
    pub created: u64,
    /// The full API key value (only returned on creation)
    pub key: String,
    /// Rate limit information for the key
    #[serde(default)]
    pub rate_limits: Option<CreateApiKeyRateLimits>,
}

impl Default for CreateApiKeyRequest {
    fn default() -> Self {
        Self {
            name: String::new(),
            rate_limits: None,
            extra: HashMap::new(),
        }
    }
}

/// Builder for API key creation requests
#[derive(Debug, Clone)]
pub struct CreateApiKeyRequestBuilder {
    request: CreateApiKeyRequest,
}

impl CreateApiKeyRequestBuilder {
    /// Create a new API key creation request builder
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            request: CreateApiKeyRequest {
                name: name.into(),
                ..Default::default()
            },
        }
    }

    /// Set rate limits for the API key
    pub fn with_rate_limits(mut self, rate_limits: CreateApiKeyRateLimits) -> Self {
        self.request.rate_limits = Some(rate_limits);
        self
    }

    /// Set requests per minute limit
    pub fn with_requests_per_minute(mut self, requests_per_minute: u32) -> Self {
        let rate_limits = self.request.rate_limits.get_or_insert(CreateApiKeyRateLimits {
            requests_per_minute: None,
            requests_per_day: None,
            tokens_per_minute: None,
        });
        rate_limits.requests_per_minute = Some(requests_per_minute);
        self
    }

    /// Set requests per day limit
    pub fn with_requests_per_day(mut self, requests_per_day: u32) -> Self {
        let rate_limits = self.request.rate_limits.get_or_insert(CreateApiKeyRateLimits {
            requests_per_minute: None,
            requests_per_day: None,
            tokens_per_minute: None,
        });
        rate_limits.requests_per_day = Some(requests_per_day);
        self
    }

    /// Set tokens per minute limit
    pub fn with_tokens_per_minute(mut self, tokens_per_minute: u32) -> Self {
        let rate_limits = self.request.rate_limits.get_or_insert(CreateApiKeyRateLimits {
            requests_per_minute: None,
            requests_per_day: None,
            tokens_per_minute: None,
        });
        rate_limits.tokens_per_minute = Some(tokens_per_minute);
        self
    }

    /// Add a custom parameter to the request
    pub fn with_extra(mut self, key: impl Into<String>, value: impl Into<serde_json::Value>) -> Self {
        self.request.extra.insert(key.into(), value.into());
        self
    }

    /// Build the API key creation request
    pub fn build(self) -> CreateApiKeyRequest {
        self.request
    }
}

impl Client {
    /// Create a new API key
    ///
    /// # Examples
    ///
    /// ```
    /// use venice_ai_api_sdk_rust::{
    ///     Client,
    ///     api_keys::CreateApiKeyRequestBuilder,
    /// };
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::new("your-api-key")?;
    ///     
    ///     let request = CreateApiKeyRequestBuilder::new("My New API Key")
    ///         .with_requests_per_minute(100)
    ///         .with_tokens_per_minute(10000)
    ///         .build();
    ///     
    ///     let (response, _) = client.create_api_key(request).await?;
    ///     
    ///     println!("Created API Key: {} ({})", response.data.name, response.data.key);
    ///     
    ///     Ok(())
    /// }
    /// ```
    pub async fn create_api_key(
        &self,
        request: CreateApiKeyRequest,
    ) -> VeniceResult<(CreateApiKeyResponse, RateLimitInfo)> {
        self.post(API_KEYS_ENDPOINT, &request).await
    }
}

/// Helper function to create an API key
///
/// # Examples
///
/// ```
/// use venice_ai_api_sdk_rust::api_keys::{
///     create_api_key,
///     CreateApiKeyRequestBuilder,
/// };
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let request = CreateApiKeyRequestBuilder::new("My New API Key")
///         .with_requests_per_minute(100)
///         .with_tokens_per_minute(10000)
///         .build();
///     
///     let (response, _) = create_api_key("your-api-key", request).await?;
///     
///     println!("Created API Key: {} ({})", response.data.name, response.data.key);
///     
///     Ok(())
/// }
/// ```
pub async fn create_api_key(
    api_key: impl Into<String>,
    request: CreateApiKeyRequest,
) -> VeniceResult<(CreateApiKeyResponse, RateLimitInfo)> {
    let client = Client::new(api_key)?;
    client.create_api_key(request).await
}