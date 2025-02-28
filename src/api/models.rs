//! Models API implementation
//!
//! This module provides an implementation of the models API.

use async_trait::async_trait;

use crate::error::{RateLimitInfo, VeniceResult};
use crate::http::SharedHttpClient;
use crate::models::list::{ListModelsRequest, ListModelsResponse as ModelsListResponse};
use crate::models::traits::{ModelTraitsRequest, ModelTraitsResponse as ModelsTraitsResponse};
use crate::pagination::{PaginationParams, Paginator};
use crate::traits::models::{ListModelsResponse, ModelTraitsResponse, ModelsApi};

/// Implementation of the models API
#[derive(Debug, Clone)]
pub struct ModelsApiImpl {
    /// The HTTP client to use for requests
    http_client: SharedHttpClient,
}

impl ModelsApiImpl {
    /// Create a new models API implementation
    pub fn new(http_client: SharedHttpClient) -> Self {
        Self { http_client }
    }
}

#[async_trait]
impl ModelsApi for ModelsApiImpl {
    async fn list_models(&self) -> VeniceResult<(ListModelsResponse, RateLimitInfo)> {
        let (response, rate_limit) = self.list_models_with_params(ListModelsRequest::default()).await?;
        
        // Convert from models::list::ListModelsResponse to traits::models::ListModelsResponse
        let models_response = ListModelsResponse {
            data: response.data.into_iter().map(|m| crate::traits::models::Model {
                id: m.id,
                object: m.object,
                owned_by: m.owned_by,
                context_size: m.context_size,
                supports_streaming: m.supports_streaming,
                supports_image_generation: m.supports_image_generation,
                supports_chat_completions: m.supports_chat_completions,
            }).collect(),
        };
        
        Ok((models_response, rate_limit))
    }
    
    async fn list_models_with_params(
        &self,
        request: ListModelsRequest,
    ) -> VeniceResult<(ModelsListResponse, RateLimitInfo)> {
        self.http_client.get_with_query("models", &request).await
    }
    
    fn list_models_paginator(&self, params: PaginationParams) -> impl Paginator<crate::models::list::Model> {
        let http_client = self.http_client.clone();
        
        // Create an async function that fetches a page
        let fetch_page = move |params: PaginationParams| {
            let http_client = http_client.clone();
            async move {
                let request = ListModelsRequest { pagination: params };
                http_client.get_with_query::<_, ModelsListResponse>("models", &request).await
            }
        };
        
        crate::create_async_paginator(fetch_page, params)
    }
    
    async fn get_model_traits(&self, model_id: &str) -> VeniceResult<(ModelTraitsResponse, RateLimitInfo)> {
        // Create a request to get traits for a specific model
        let request = Some(ModelTraitsRequest {
            model: Some(model_id.to_string()),
        });
        
        // Get the traits from the API
        let (response, rate_limit) = self.get_model_traits_internal(request).await?;
        
        // Convert from models::traits::ModelTraitsResponse to traits::models::ModelTraitsResponse
        let traits = response.data.iter()
            .filter(|t| t.models.contains(&model_id.to_string()))
            .map(|t| t.id.clone())
            .collect();
        
        let traits_response = ModelTraitsResponse {
            model: model_id.to_string(),
            traits,
        };
        
        Ok((traits_response, rate_limit))
    }
    
    async fn get_model_traits_internal(
        &self,
        request: Option<ModelTraitsRequest>,
    ) -> VeniceResult<(ModelsTraitsResponse, RateLimitInfo)> {
        match request {
            Some(req) => {
                if let Some(model) = &req.model {
                    let url = format!("models/{}/traits", model);
                    self.http_client.get(&url).await
                } else {
                    self.http_client.get("models/traits").await
                }
            },
            None => self.http_client.get("models/traits").await,
        }
    }
    
    async fn is_model_compatible(&self, model_id: &str, feature: &str) -> VeniceResult<bool> {
        // Create a request to get traits for a specific model
        let request = Some(ModelTraitsRequest {
            model: Some(model_id.to_string()),
        });
        
        // Get the traits from the API
        let (response, _) = self.get_model_traits_internal(request).await?;
        
        // Check if any of the traits match the feature
        for trait_info in &response.data {
            if trait_info.id == feature && trait_info.models.contains(&model_id.to_string()) {
                return Ok(true);
            }
        }
        
        Ok(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::http::{HttpClientConfig, new_shared_http_client};
    
    #[tokio::test]
    async fn test_list_models() {
        // Create a mock HTTP client
        let config = HttpClientConfig {
            api_key: "test_api_key".to_string(),
            base_url: "https://api.venice.ai".to_string(),
            custom_headers: reqwest::header::HeaderMap::new(),
            timeout_secs: None,
        };
        let http_client = new_shared_http_client(config).unwrap();
        
        // Create the models API implementation
        let models_api = ModelsApiImpl::new(http_client);
        
        // TODO: Mock the HTTP client to return a response
        // For now, we'll just check that the method exists and has the right signature
        let _: ModelsApiImpl = models_api;
    }
    
    #[tokio::test]
    async fn test_get_model_traits() {
        // Create a mock HTTP client
        let config = HttpClientConfig {
            api_key: "test_api_key".to_string(),
            base_url: "https://api.venice.ai".to_string(),
            custom_headers: reqwest::header::HeaderMap::new(),
            timeout_secs: None,
        };
        let http_client = new_shared_http_client(config).unwrap();
        
        // Create the models API implementation
        let models_api = ModelsApiImpl::new(http_client);
        
        // TODO: Mock the HTTP client to return a response
        // For now, we'll just check that the method exists and has the right signature
        let _: ModelsApiImpl = models_api;
    }
}