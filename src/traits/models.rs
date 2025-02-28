use async_trait::async_trait;
use serde::Deserialize;

use crate::error::{RateLimitInfo, VeniceResult};
use crate::pagination::{PaginationParams, Paginator};
use crate::models::list::ListModelsRequest;

/// Information about a model
#[derive(Debug, Deserialize, Clone)]
pub struct Model {
    /// The model identifier
    pub id: String,
    /// The type of object, always "model"
    pub object: String,
    /// The owner of the model
    pub owned_by: String,
    /// Maximum context size for this model
    #[serde(default)]
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
}

/// Response from models API
#[derive(Debug, Deserialize)]
pub struct ListModelsResponse {
    /// Array of model information
    pub data: Vec<Model>,
}

/// Response from model traits API
#[derive(Debug, Deserialize)]
pub struct ModelTraitsResponse {
    /// The model identifier
    pub model: String,
    /// The traits supported by the model
    pub traits: Vec<String>,
}

/// Models API trait
#[async_trait]
pub trait ModelsApi {
    /// List available models
    async fn list_models(&self) -> VeniceResult<(ListModelsResponse, RateLimitInfo)>;
    
    /// List available models with pagination parameters
    async fn list_models_with_params(
        &self,
        request: ListModelsRequest,
    ) -> VeniceResult<(crate::models::list::ListModelsResponse, RateLimitInfo)>;
    
    /// Create a paginator for listing models
    fn list_models_paginator(&self, params: PaginationParams) -> impl Paginator<crate::models::list::Model>;
    
    /// Get the traits supported by a model
    async fn get_model_traits(&self, model_id: &str) -> VeniceResult<(ModelTraitsResponse, RateLimitInfo)>;
    
    /// Internal method to get model traits from the API
    async fn get_model_traits_internal(
        &self,
        request: Option<crate::models::traits::ModelTraitsRequest>,
    ) -> VeniceResult<(crate::models::traits::ModelTraitsResponse, RateLimitInfo)>;
    
    /// Check if a model is compatible with a feature
    async fn is_model_compatible(&self, model_id: &str, feature: &str) -> VeniceResult<bool>;
}
