use serde::{Deserialize, Serialize};

use crate::{
    client::Client,
    error::{RateLimitInfo, VeniceResult},
    pagination::{PaginationInfo, PaginationParams, Paginator},
};

/// The endpoint for listing models
const MODELS_ENDPOINT: &str = "models";

/// Request parameters for listing models
#[derive(Debug, Serialize, Default)]
pub struct ListModelsRequest {
    /// Pagination parameters
    #[serde(flatten)]
    pub pagination: PaginationParams,
}

impl ListModelsRequest {
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

/// Response from the models API
#[derive(Debug, Deserialize)]
pub struct ListModelsResponse {
    /// Array of model information
    pub data: Vec<Model>,
    /// Object type
    pub object: String,
    /// Whether there are more items available
    #[serde(default)]
    pub has_more: bool,
    /// The cursor to use for the next page, if any
    #[serde(default)]
    pub next_cursor: Option<String>,
}

impl PaginationInfo<Model> for ListModelsResponse {
    fn get_data(&self) -> Vec<Model> {
        self.data.clone()
    }
    
    fn has_more(&self) -> bool {
        self.has_more
    }
    
    fn next_cursor(&self) -> Option<String> {
        self.next_cursor.clone()
    }
}

/// Information about a model
#[derive(Debug, Deserialize, Clone)]
pub struct Model {
    /// The model identifier
    pub id: String,
    /// The type of object, always "model"
    pub object: String,
    /// The owner of the model
    pub owned_by: String,
    /// The maximum number of tokens allowed for this model
    #[serde(default)]
    pub max_tokens: Option<u32>,
    /// The maximum context size for this model
    pub context_size: Option<u32>,
    /// Whether the model supports streaming
    #[serde(default)]
    pub supports_streaming: bool,
    /// Whether the model supports image generation
    #[serde(default)]
    pub supports_image_generation: bool,
    /// Whether the model supports chat completions
    #[serde(default)]
    pub supports_chat_completions: bool,
    /// Whether the model supports function calling
    #[serde(default)]
    pub supports_function_calling: bool,
    /// Array of model permissions
    #[serde(default)]
    pub permissions: Vec<ModelPermission>,
    /// Model pricing information
    pub pricing: Option<ModelPricing>,
}

/// Model permission information
#[derive(Debug, Deserialize, Clone)]
pub struct ModelPermission {
    /// The type of object
    pub object: String,
    /// The ID of this permission
    pub id: String,
    /// Whether this permission allows creating engine
    pub allow_create_engine: bool,
    /// Whether this permission allows sampling
    pub allow_sampling: bool,
    /// Whether this permission allows logprobs
    pub allow_logprobs: bool,
    /// Whether this permission allows search indices
    pub allow_search_indices: bool,
    /// Whether this permission allows view
    pub allow_view: bool,
    /// Whether this permission allows fine tuning
    pub allow_fine_tuning: bool,
    /// The organization ID this permission applies to
    pub organization: String,
    /// The group this permission applies to
    pub group: Option<String>,
    /// Whether this permission is blocking
    pub is_blocking: bool,
}

/// Model pricing information
#[derive(Debug, Deserialize, Clone)]
pub struct ModelPricing {
    /// Cost per 1K tokens for input/prompt
    pub prompt: Option<f64>,
    /// Cost per 1K tokens for output/completion
    pub completion: Option<f64>,
}

impl Client {
    /// List available models
    ///
    /// # Examples
    ///
    /// ```
    /// use venice_ai_api_sdk_rust::Client;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::new("your-api-key")?;
    ///     let (models, _) = client.list_models().await?;
    ///
    ///     for model in models.data {
    ///         println!("Model: {} (owned by: {})", model.id, model.owned_by);
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn list_models(&self) -> VeniceResult<(ListModelsResponse, RateLimitInfo)> {
        self.list_models_with_params(ListModelsRequest::default()).await
    }
    
    /// List available models with pagination parameters
    ///
    /// # Examples
    ///
    /// ```
    /// use venice_ai_api_sdk_rust::{Client, models::ListModelsRequest};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::new("your-api-key")?;
    ///
    ///     // Get the first page with 10 models
    ///     let request = ListModelsRequest::new().limit(10);
    ///     let (models, _) = client.list_models_with_params(request).await?;
    ///
    ///     for model in models.data {
    ///         println!("Model: {} (owned by: {})", model.id, model.owned_by);
    ///     }
    ///
    ///     // If there are more models, get the next page
    ///     if models.has_more {
    ///         if let Some(cursor) = models.next_cursor {
    ///             let next_request = ListModelsRequest::new().limit(10).cursor(cursor);
    ///             let (next_models, _) = client.list_models_with_params(next_request).await?;
    ///
    ///             for model in next_models.data {
    ///                 println!("Model: {} (owned by: {})", model.id, model.owned_by);
    ///             }
    ///         }
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn list_models_with_params(
        &self,
        request: ListModelsRequest,
    ) -> VeniceResult<(ListModelsResponse, RateLimitInfo)> {
        self.get_with_query(MODELS_ENDPOINT, &request).await
    }
    
    /// Create a paginator for listing models
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
    ///     // Create a paginator with 10 models per page
    ///     let params = PaginationParams::new().limit(10);
    ///     let mut paginator = client.list_models_paginator(params);
    ///
    ///     // Get all models
    ///     let all_models = paginator.all_pages().await?;
    ///     println!("Found {} models", all_models.len());
    ///
    ///     // Or iterate through pages
    ///     let mut paginator = client.list_models_paginator(params);
    ///     while let Some(page) = paginator.next_page().await? {
    ///         println!("Got page with {} models", page.data.len());
    ///
    ///         for model in page.data {
    ///             println!("Model: {} (owned by: {})", model.id, model.owned_by);
    ///         }
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn list_models_paginator(&self, params: PaginationParams) -> impl Paginator<Model> {
        let client = std::sync::Arc::new(self.clone());
        
        // Create an async function that fetches a page
        let fetch_page = move |params: PaginationParams| {
            let client = client.clone();
            async move {
                let request = ListModelsRequest { pagination: params };
                client.list_models_with_params(request).await
            }
        };
        
        crate::create_async_paginator(fetch_page, params)
    }
}

/// Helper function to list models
///
/// # Examples
///
/// ```
/// use venice_ai_api_sdk_rust::models::list_models;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let (models, _) = list_models("your-api-key").await?;
///
///     for model in models.data {
///         println!("Model: {}", model.id);
///     }
///
///     Ok(())
/// }
/// ```
pub async fn list_models(
    api_key: impl Into<String>,
) -> VeniceResult<(ListModelsResponse, RateLimitInfo)> {
    let client = Client::new(api_key)?;
    client.list_models().await
}

/// Helper function to list models with pagination parameters
pub async fn list_models_with_params(
    api_key: impl Into<String>,
    request: ListModelsRequest,
) -> VeniceResult<(ListModelsResponse, RateLimitInfo)> {
    let client = Client::new(api_key)?;
    client.list_models_with_params(request).await
}

/// Helper function to create a paginator for listing models
pub fn list_models_paginator(
    api_key: impl Into<String>,
    params: PaginationParams,
) -> VeniceResult<impl Paginator<Model>> {
    let client = Client::new(api_key)?;
    Ok(client.list_models_paginator(params))
}