use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::{
    client::Client,
    error::{RateLimitInfo, VeniceResult},
    pagination::{PaginationInfo, PaginationParams, Paginator},
};

/// The endpoint for listing API keys
const API_KEYS_ENDPOINT: &str = "api_keys";

/// Request parameters for listing API keys
#[derive(Debug, Serialize, Default)]
pub struct ListApiKeysRequest {
    /// Pagination parameters
    #[serde(flatten)]
    pub pagination: PaginationParams,
}

impl ListApiKeysRequest {
    /// Create a new request with default parameters
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Set the maximum number of items to return
    pub fn limit(mut self, limit: u32) -> Self {
        self.pagination = self.pagination.limit(limit);
        self
    }
    
    /// Set the cursor for pagination
    pub fn cursor(mut self, cursor: impl Into<String>) -> Self {
        self.pagination = self.pagination.cursor(cursor);
        self
    }
}

/// Response from the API keys list endpoint
#[derive(Debug, Deserialize)]
pub struct ListApiKeysResponse {
    /// Array of API key information
    pub data: Vec<ApiKey>,
    /// Type of object
    #[serde(default)]
    pub object: Option<String>,
    /// Whether there are more items available
    #[serde(default)]
    pub has_more: bool,
    /// The cursor to use for the next page, if any
    #[serde(default)]
    pub next_cursor: Option<String>,
}

impl PaginationInfo<ApiKey> for ListApiKeysResponse {
    fn get_data(&self) -> Vec<ApiKey> {
        self.data.clone()
    }
    
    fn has_more(&self) -> bool {
        self.has_more
    }
    
    fn next_cursor(&self) -> Option<String> {
        self.next_cursor.clone()
    }
}

/// Information about an API key
#[derive(Debug, Deserialize, Clone)]
pub struct ApiKey {
    /// The API key identifier
    pub id: String,
    /// The type of object, always "api_key"
    #[serde(default)]
    pub object: Option<String>,
    /// The name of the API key
    #[serde(default)]
    pub name: Option<String>,
    /// When the API key was created
    #[serde(default)]
    pub created: u64,
    /// Last characters of the API key
    #[serde(default)]
    pub last_chars: String,
    /// Whether the key has been revoked
    #[serde(default)]
    pub revoked: bool,
    /// Rate limit information for the key
    #[serde(default)]
    pub rate_limits: Option<ApiKeyRateLimits>,
}

/// Rate limit information for an API key
#[derive(Debug, Deserialize, Clone)]
pub struct ApiKeyRateLimits {
    /// Requests per minute limit
    pub requests_per_minute: Option<u32>,
    /// Requests per day limit
    pub requests_per_day: Option<u32>,
    /// Tokens per minute limit
    pub tokens_per_minute: Option<u32>,
}

impl Client {
    /// List API keys
    ///
    /// # Examples
    ///
    /// ```
    /// use venice_ai_api_sdk_rust::Client;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::new("your-api-key")?;
    ///     let (keys, _) = client.list_api_keys().await?;
    ///
    ///     for key in keys.data {
    ///         println!("API Key: {} ({})", key.name, key.id);
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn list_api_keys(&self) -> VeniceResult<(ListApiKeysResponse, RateLimitInfo)> {
        self.list_api_keys_with_params(ListApiKeysRequest::default()).await
    }
    
    /// List API keys with pagination parameters
    ///
    /// # Examples
    ///
    /// ```
    /// use venice_ai_api_sdk_rust::{Client, api_keys::ListApiKeysRequest};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::new("your-api-key")?;
    ///
    ///     // Get the first page with 5 API keys
    ///     let request = ListApiKeysRequest::new().limit(5);
    ///     let (keys, _) = client.list_api_keys_with_params(request).await?;
    ///
    ///     for key in keys.data {
    ///         println!("API Key: {} ({})", key.name, key.id);
    ///     }
    ///
    ///     // If there are more keys, get the next page
    ///     if keys.has_more {
    ///         if let Some(cursor) = keys.next_cursor {
    ///             let next_request = ListApiKeysRequest::new().limit(5).cursor(cursor);
    ///             let (next_keys, _) = client.list_api_keys_with_params(next_request).await?;
    ///
    ///             for key in next_keys.data {
    ///                 println!("API Key: {} ({})", key.name, key.id);
    ///             }
    ///         }
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn list_api_keys_with_params(
        &self,
        request: ListApiKeysRequest,
    ) -> VeniceResult<(ListApiKeysResponse, RateLimitInfo)> {
        self.get_with_query(API_KEYS_ENDPOINT, &request).await
    }
    
    /// Create a paginator for listing API keys
    ///
    /// # Examples
    ///
    /// ```
    /// use venice_ai_api_sdk_rust::{Client, pagination::{PaginationParams, Paginator}};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::new("your-api-key")?;
    ///
    ///     // Create a paginator with 5 API keys per page
    ///     let params = PaginationParams::new().limit(5);
    ///     let mut paginator = client.list_api_keys_paginator(params);
    ///
    ///     // Get all API keys
    ///     let all_keys = paginator.all_pages().await?;
    ///     println!("Found {} API keys", all_keys.len());
    ///
    ///     // Or iterate through pages
    ///     let mut paginator = client.list_api_keys_paginator(params);
    ///     while let Some(page) = paginator.next_page().await? {
    ///         println!("Got page with {} API keys", page.data.len());
    ///
    ///         for key in page.data {
    ///             println!("API Key: {} ({})", key.name, key.id);
    ///         }
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn list_api_keys_paginator(&self, params: PaginationParams) -> impl Paginator<ApiKey> {
        let client = Arc::new(self.clone());
        
        // Create an async function that fetches a page
        let fetch_page = move |params: PaginationParams| {
            let client = client.clone();
            async move {
                let request = ListApiKeysRequest { pagination: params };
                client.list_api_keys_with_params(request).await
            }
        };
        
        crate::create_async_paginator(fetch_page, params)
    }
}

/// Helper function to list API keys
///
/// # Examples
///
/// ```
/// use venice_ai_api_sdk_rust::api_keys::list_api_keys;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let (keys, _) = list_api_keys("your-api-key").await?;
///
///     for key in keys.data {
///         println!("API Key: {} ({})", key.name, key.id);
///     }
///
///     Ok(())
/// }
/// ```
pub async fn list_api_keys(
    api_key: impl Into<String>,
) -> VeniceResult<(ListApiKeysResponse, RateLimitInfo)> {
    let client = Client::new(api_key)?;
    client.list_api_keys().await
}

/// Helper function to list API keys with pagination parameters
///
/// # Examples
///
/// ```
/// use venice_ai_api_sdk_rust::api_keys::{list_api_keys_with_params, ListApiKeysRequest};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     // Get the first page with 5 API keys
///     let request = ListApiKeysRequest::new().limit(5);
///     let (keys, _) = list_api_keys_with_params("your-api-key", request).await?;
///
///     for key in keys.data {
///         println!("API Key: {} ({})", key.name, key.id);
///     }
///
///     Ok(())
/// }
/// ```
pub async fn list_api_keys_with_params(
    api_key: impl Into<String>,
    request: ListApiKeysRequest,
) -> VeniceResult<(ListApiKeysResponse, RateLimitInfo)> {
    let client = Client::new(api_key)?;
    client.list_api_keys_with_params(request).await
}

/// Helper function to create a paginator for listing API keys
///
/// # Examples
///
/// ```
/// use venice_ai_api_sdk_rust::{
///     api_keys::list_api_keys_paginator,
///     pagination::{PaginationParams, Paginator},
/// };
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     // Create a paginator with 5 API keys per page
///     let params = PaginationParams::new().limit(5);
///     let mut paginator = list_api_keys_paginator("your-api-key", params)?;
///
///     // Get all API keys
///     let all_keys = paginator.all_pages().await?;
///     println!("Found {} API keys", all_keys.len());
///
///     Ok(())
/// }
/// ```
pub fn list_api_keys_paginator(
    api_key: impl Into<String>,
    params: PaginationParams,
) -> VeniceResult<impl Paginator<ApiKey>> {
    let client = Client::new(api_key)?;
    Ok(client.list_api_keys_paginator(params))
}