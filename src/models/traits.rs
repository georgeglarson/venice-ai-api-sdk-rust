use serde::{Deserialize, Serialize};

use crate::{
    client::Client,
    error::{RateLimitInfo, VeniceResult},
};

/// The endpoint for retrieving model traits
const TRAITS_ENDPOINT: &str = "models/traits";

/// Request parameters for retrieving model traits
#[derive(Debug, Serialize, Default)]
pub struct ModelTraitsRequest {
    /// Optional model ID to filter traits for a specific model
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
}

/// Information about a model trait
#[derive(Debug, Deserialize, Clone)]
pub struct ModelTrait {
    /// The trait identifier
    pub id: String,
    /// Display name for the trait
    pub name: String,
    /// Description of what the trait means
    pub description: String,
    /// Category of the trait (e.g., "performance", "capability")
    #[serde(default)]
    pub category: Option<String>,
    /// Models that have this trait
    #[serde(default)]
    pub models: Vec<String>,
}

/// Response from model traits API
#[derive(Debug, Deserialize)]
pub struct ModelTraitsResponse {
    /// Array of model traits
    pub data: Vec<ModelTrait>,
    /// Type of object
    pub object: String,
}

impl Client {
    /// Get model traits
    ///
    /// # Examples
    ///
    /// ```
    /// use venice_ai_api_sdk_rust::{Client, models::ModelTraitsRequest};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::new("your-api-key")?;
    ///
    ///     // Get all model traits
    ///     let (traits, _) = client.get_model_traits(None).await?;
    ///
    ///     for trait_info in traits.data {
    ///         println!("Trait: {} - {}", trait_info.name, trait_info.description);
    ///     }
    ///
    ///     // Get traits for a specific model
    ///     let request = ModelTraitsRequest {
    ///         model: Some("llama-3.2-70b".to_string()),
    ///     };
    ///
    ///     let (model_traits, _) = client.get_model_traits(Some(request)).await?;
    ///
    ///     println!("Traits for llama-3.2-70b:");
    ///     for trait_info in model_traits.data {
    ///         println!("- {}", trait_info.name);
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_model_traits(
        &self,
        request: Option<ModelTraitsRequest>,
    ) -> VeniceResult<(ModelTraitsResponse, RateLimitInfo)> {
        match request {
            Some(req) => {
                // Build query parameters
                let mut query_params = Vec::new();
                if let Some(model) = req.model {
                    query_params.push(("model", model));
                }
                
                // Append query parameters to endpoint
                if query_params.is_empty() {
                    self.get(TRAITS_ENDPOINT).await
                } else {
                    let endpoint = format!(
                        "{}?{}",
                        TRAITS_ENDPOINT,
                        serde_urlencoded::to_string(query_params).unwrap_or_default()
                    );
                    self.get(&endpoint).await
                }
            }
            None => self.get(TRAITS_ENDPOINT).await,
        }
    }
}

/// Helper function to get model traits
///
/// # Examples
///
/// ```
/// use venice_ai_api_sdk_rust::models::{get_model_traits, ModelTraitsRequest};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     // Get all model traits
///     let (traits, _) = get_model_traits("your-api-key", None).await?;
///
///     for trait_info in traits.data {
///         println!("Trait: {}", trait_info.name);
///     }
///
///     Ok(())
/// }
/// ```
pub async fn get_model_traits(
    api_key: impl Into<String>,
    request: Option<ModelTraitsRequest>,
) -> VeniceResult<(ModelTraitsResponse, RateLimitInfo)> {
    let client = Client::new(api_key)?;
    client.get_model_traits(request).await
}