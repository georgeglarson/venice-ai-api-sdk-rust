use serde::{Deserialize, Serialize};

use crate::{
    client::Client,
    error::{RateLimitInfo, VeniceResult},
};

/// The endpoint for retrieving model feature suffixes
const FEATURE_SUFFIX_ENDPOINT: &str = "chat/model_feature_suffix";

/// Request parameters for retrieving model feature suffixes
#[derive(Debug, Serialize, Default)]
pub struct ModelFeatureSuffixRequest {
    /// Optional model ID to filter suffixes for a specific model
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
}

/// Information about a model feature suffix
#[derive(Debug, Deserialize, Clone)]
pub struct ModelFeatureSuffix {
    /// The suffix identifier
    pub id: String,
    /// Description of what the suffix does
    pub description: String,
    /// Example of how to use the suffix
    pub example: String,
    /// Models that support this suffix
    #[serde(default)]
    pub supported_models: Vec<String>,
}

/// Response from model feature suffix API
#[derive(Debug, Deserialize)]
pub struct ModelFeatureSuffixResponse {
    /// Array of feature suffixes
    pub data: Vec<ModelFeatureSuffix>,
    /// Type of object
    pub object: String,
}

impl Client {
    /// Get model feature suffixes
    ///
    /// # Examples
    ///
    /// ```
    /// use venice_ai_api_sdk_rust::{Client, chat::ModelFeatureSuffixRequest};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::new("your-api-key")?;
    ///
    ///     // Get all model feature suffixes
    ///     let (suffixes, _) = client.get_model_feature_suffixes(None).await?;
    ///
    ///     for suffix in suffixes.data {
    ///         println!("Suffix: {}", suffix.id);
    ///         println!("Description: {}", suffix.description);
    ///         println!("Example: {}", suffix.example);
    ///         println!();
    ///     }
    ///
    ///     // Get suffixes for a specific model
    ///     let request = ModelFeatureSuffixRequest {
    ///         model: Some("llama-3.2-70b".to_string()),
    ///     };
    ///
    ///     let (model_suffixes, _) = client.get_model_feature_suffixes(Some(request)).await?;
    ///
    ///     println!("Suffixes for llama-3.2-70b:");
    ///     for suffix in model_suffixes.data {
    ///         println!("- {}", suffix.id);
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_model_feature_suffixes(
        &self,
        request: Option<ModelFeatureSuffixRequest>,
    ) -> VeniceResult<(ModelFeatureSuffixResponse, RateLimitInfo)> {
        match request {
            Some(req) => {
                // Build query parameters
                let mut query_params = Vec::new();
                if let Some(model) = req.model {
                    query_params.push(("model", model));
                }
                
                // Append query parameters to endpoint
                if query_params.is_empty() {
                    self.get(FEATURE_SUFFIX_ENDPOINT).await
                } else {
                    let endpoint = format!(
                        "{}?{}",
                        FEATURE_SUFFIX_ENDPOINT,
                        serde_urlencoded::to_string(query_params).unwrap_or_default()
                    );
                    self.get(&endpoint).await
                }
            }
            None => self.get(FEATURE_SUFFIX_ENDPOINT).await,
        }
    }
}

/// Helper function to get model feature suffixes
///
/// # Examples
///
/// ```
/// use venice_ai_api_sdk_rust::chat::{get_model_feature_suffixes, ModelFeatureSuffixRequest};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     // Get all model feature suffixes
///     let (suffixes, _) = get_model_feature_suffixes("your-api-key", None).await?;
///
///     for suffix in suffixes.data {
///         println!("Suffix: {}", suffix.id);
///     }
///
///     Ok(())
/// }
/// ```
pub async fn get_model_feature_suffixes(
    api_key: impl Into<String>,
    request: Option<ModelFeatureSuffixRequest>,
) -> VeniceResult<(ModelFeatureSuffixResponse, RateLimitInfo)> {
    let client = Client::new(api_key)?;
    client.get_model_feature_suffixes(request).await
}