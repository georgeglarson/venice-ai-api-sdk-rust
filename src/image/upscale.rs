use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{
    client::Client,
    error::{RateLimitInfo, VeniceResult},
};

/// The endpoint for image upscaling
const IMAGE_UPSCALE_ENDPOINT: &str = "image/upscale";

/// Request for image upscaling
#[derive(Debug, Clone, Serialize)]
pub struct ImageUpscaleRequest {
    /// ID of the model to use
    pub model: String,
    /// URL of the image to upscale
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    /// Base64 encoded image data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_data: Option<String>,
    /// Scale factor for upscaling
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<u32>,
    /// Return the image as binary data instead of URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_binary: Option<bool>,
    /// Additional custom parameters
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// Response from image upscaling API
#[derive(Debug, Clone, Deserialize)]
pub struct ImageUpscaleResponse {
    /// Created timestamp
    pub created: u64,
    /// List of upscaled images
    pub data: Vec<UpscaledImageData>,
}

/// Data for an upscaled image
#[derive(Debug, Clone, Deserialize)]
pub struct UpscaledImageData {
    /// URL to the upscaled image
    #[serde(default)]
    pub url: Option<String>,
    /// Base64 encoded image data (if return_binary is true)
    #[serde(default)]
    pub b64_json: Option<String>,
}

impl Default for ImageUpscaleRequest {
    fn default() -> Self {
        Self {
            model: "upscale-xl".to_string(),
            image_url: None,
            image_data: None,
            scale: None,
            return_binary: None,
            extra: HashMap::new(),
        }
    }
}

/// Builder for image upscaling requests
#[derive(Debug, Clone)]
pub struct ImageUpscaleRequestBuilder {
    request: ImageUpscaleRequest,
}

impl ImageUpscaleRequestBuilder {
    /// Create a new image upscaling request builder with image URL
    pub fn with_url(model: impl Into<String>, image_url: impl Into<String>) -> Self {
        Self {
            request: ImageUpscaleRequest {
                model: model.into(),
                image_url: Some(image_url.into()),
                ..Default::default()
            },
        }
    }

    /// Create a new image upscaling request builder with image data
    pub fn with_data(model: impl Into<String>, image_data: impl Into<String>) -> Self {
        Self {
            request: ImageUpscaleRequest {
                model: model.into(),
                image_data: Some(image_data.into()),
                ..Default::default()
            },
        }
    }

    /// Set the scale factor
    pub fn with_scale(mut self, scale: u32) -> Self {
        self.request.scale = Some(scale);
        self
    }

    /// Enable or disable binary return format
    pub fn with_return_binary(mut self, return_binary: bool) -> Self {
        self.request.return_binary = Some(return_binary);
        self
    }

    /// Add a custom parameter to the request
    pub fn with_extra(mut self, key: impl Into<String>, value: impl Into<serde_json::Value>) -> Self {
        self.request.extra.insert(key.into(), value.into());
        self
    }

    /// Build the image upscaling request
    pub fn build(self) -> ImageUpscaleRequest {
        self.request
    }
}

impl Client {
    /// Upscale an image
    ///
    /// # Examples
    ///
    /// ```
    /// use venice_ai_api_sdk_rust::{
    ///     Client,
    ///     image::ImageUpscaleRequestBuilder,
    /// };
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::new("your-api-key")?;
    ///     
    ///     let request = ImageUpscaleRequestBuilder::with_url(
    ///         "upscale-xl",
    ///         "https://example.com/image.jpg",
    ///     )
    ///     .with_scale(4)
    ///     .build();
    ///     
    ///     let (response, _) = client.upscale_image(request).await?;
    ///     
    ///     if let Some(image) = &response.data.first() {
    ///         println!("Upscaled Image URL: {}", image.url.as_ref().unwrap_or(&"No URL".to_string()));
    ///     }
    ///     
    ///     Ok(())
    /// }
    /// ```
    pub async fn upscale_image(
        &self,
        request: ImageUpscaleRequest,
    ) -> VeniceResult<(ImageUpscaleResponse, RateLimitInfo)> {
        self.post(IMAGE_UPSCALE_ENDPOINT, &request).await
    }
}

/// Helper function to upscale an image
///
/// # Examples
///
/// ```
/// use venice_ai_api_sdk_rust::image::{
///     upscale_image,
///     ImageUpscaleRequestBuilder,
/// };
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let request = ImageUpscaleRequestBuilder::with_url(
///         "upscale-xl",
///         "https://example.com/image.jpg",
///     )
///     .with_scale(4)
///     .build();
///     
///     let (response, _) = upscale_image("your-api-key", request).await?;
///     
///     if let Some(image) = &response.data.first() {
///         println!("Upscaled Image URL: {}", image.url.as_ref().unwrap_or(&"No URL".to_string()));
///     }
///     
///     Ok(())
/// }
/// ```
pub async fn upscale_image(
    api_key: impl Into<String>,
    request: ImageUpscaleRequest,
) -> VeniceResult<(ImageUpscaleResponse, RateLimitInfo)> {
    let client = Client::new(api_key)?;
    client.upscale_image(request).await
}