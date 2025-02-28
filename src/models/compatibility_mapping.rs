use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{
    client::Client,
    error::{RateLimitInfo, VeniceResult},
};

/// The endpoint for retrieving model compatibility mapping
const COMPATIBILITY_ENDPOINT: &str = "models/compatibility_mapping";

/// Request parameters for retrieving model compatibility mapping
#[derive(Debug, Serialize, Default)]
pub struct CompatibilityMappingRequest {
    /// Optional source model ID to filter compatibility for a specific model
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_model: Option<String>,
}

/// Information about model compatibility
#[derive(Debug, Deserialize, Clone)]
pub struct ModelCompatibility {
    /// The source model ID
    pub source_model: String,
    /// Map of target models to compatibility scores (0.0 to 1.0)
    pub compatibility: HashMap<String, f32>,
    /// Additional notes about compatibility
    #[serde(default)]
    pub notes: Option<String>,
}

/// Response from model compatibility mapping API
#[derive(Debug, Deserialize)]
pub struct CompatibilityMappingResponse {
    /// Array of model compatibility information
    pub data: Vec<ModelCompatibility>,
    /// Type of object
    pub object: String,
}

impl Client {
    /// Get model compatibility mapping
    ///
    /// # Examples
    ///
    /// ```
    /// use venice_ai_api_sdk_rust::{Client, models::CompatibilityMappingRequest};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::new("your-api-key")?;
    ///
    ///     // Get all model compatibility mappings
    ///     let (mappings, _) = client.get_compatibility_mapping(None).await?;
    ///
    ///     for mapping in mappings.data {
    ///         println!("Source model: {}", mapping.source_model);
    ///         for (target, score) in mapping.compatibility {
    ///             println!("  - {} compatibility: {:.2}", target, score);
    ///         }
    ///     }
    ///
    ///     // Get compatibility for a specific model
    ///     let request = CompatibilityMappingRequest {
    ///         source_model: Some("llama-3.2-70b".to_string()),
    ///     };
    ///
    ///     let (model_mapping, _) = client.get_compatibility_mapping(Some(request)).await?;
    ///
    ///     println!("Compatibility for llama-3.2-70b:");
    ///     for mapping in model_mapping.data {
    ///         for (target, score) in mapping.compatibility {
    ///             println!("  - {} compatibility: {:.2}", target, score);
    ///         }
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_compatibility_mapping(
        &self,
        request: Option<CompatibilityMappingRequest>,
    ) -> VeniceResult<(CompatibilityMappingResponse, RateLimitInfo)> {
        match request {
            Some(req) => {
                // Build query parameters
                let mut query_params = Vec::new();
                if let Some(source_model) = req.source_model {
                    query_params.push(("source_model", source_model));
                }
                
                // Append query parameters to endpoint
                if query_params.is_empty() {
                    self.get(COMPATIBILITY_ENDPOINT).await
                } else {
                    let endpoint = format!(
                        "{}?{}",
                        COMPATIBILITY_ENDPOINT,
                        serde_urlencoded::to_string(query_params).unwrap_or_default()
                    );
                    self.get(&endpoint).await
                }
            }
            None => self.get(COMPATIBILITY_ENDPOINT).await,
        }
    }
}

/// Helper function to get model compatibility mapping
///
/// # Examples
///
/// ```
/// use venice_ai_api_sdk_rust::models::{get_compatibility_mapping, CompatibilityMappingRequest};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     // Get all model compatibility mappings
///     let (mappings, _) = get_compatibility_mapping("your-api-key", None).await?;
///
///     for mapping in mappings.data {
///         println!("Source model: {}", mapping.source_model);
///     }
///
///     Ok(())
/// }
/// ```
pub async fn get_compatibility_mapping(
    api_key: impl Into<String>,
    request: Option<CompatibilityMappingRequest>,
) -> VeniceResult<(CompatibilityMappingResponse, RateLimitInfo)> {
    let client = Client::new(api_key)?;
    client.get_compatibility_mapping(request).await
}