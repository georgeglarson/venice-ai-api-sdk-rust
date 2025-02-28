use serde::{Deserialize, Serialize};

use crate::{
    client::Client,
    error::{RateLimitInfo, VeniceResult},
};

/// The endpoint for image styles
const IMAGE_STYLES_ENDPOINT: &str = "image/styles";

/// Request parameters for listing image styles
#[derive(Debug, Serialize, Default)]
pub struct ListImageStylesRequest {
    // This struct is currently empty but may include future parameters
}

/// Response from the image styles API
#[derive(Debug, Deserialize)]
pub struct ListImageStylesResponse {
    /// Array of available style presets
    pub data: Vec<ImageStyle>,
}

/// Information about an image style preset
#[derive(Debug, Deserialize, Clone)]
pub struct ImageStyle {
    /// The style preset identifier
    pub id: String,
    /// Display name for the style
    pub name: String,
    /// Description of the style
    #[serde(default)]
    pub description: Option<String>,
    /// Sample prompt for the style
    #[serde(default)]
    pub sample_prompt: Option<String>,
    /// Sample image URL for the style
    #[serde(default)]
    pub sample_image_url: Option<String>,
    /// Models that support this style
    #[serde(default)]
    pub supported_models: Vec<String>,
}

impl Client {
    /// List available image style presets
    ///
    /// # Examples
    ///
    /// ```
    /// use venice_ai_api_sdk_rust::Client;
    /// 
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::new("your-api-key")?;
    ///     let (styles, _) = client.list_image_styles().await?;
    ///     
    ///     for style in styles.data {
    ///         println!("Style: {} ({})", style.name, style.id);
    ///     }
    ///     
    ///     Ok(())
    /// }
    /// ```
    pub async fn list_image_styles(&self) -> VeniceResult<(ListImageStylesResponse, RateLimitInfo)> {
        self.get(IMAGE_STYLES_ENDPOINT).await
    }
}

/// Helper function to list image styles
///
/// # Examples
///
/// ```
/// use venice_ai_api_sdk_rust::image::list_image_styles;
/// 
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let (styles, _) = list_image_styles("your-api-key").await?;
///     
///     for style in styles.data {
///         println!("Style: {} ({})", style.name, style.id);
///     }
///     
///     Ok(())
/// }
/// ```
pub async fn list_image_styles(
    api_key: impl Into<String>,
) -> VeniceResult<(ListImageStylesResponse, RateLimitInfo)> {
    let client = Client::new(api_key)?;
    client.list_image_styles().await
}