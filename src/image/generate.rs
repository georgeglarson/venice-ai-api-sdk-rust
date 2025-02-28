use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{
    client::Client,
    error::{RateLimitInfo, VeniceResult},
};

/// The endpoint for image generation
const IMAGE_GENERATE_ENDPOINT: &str = "image/generate";

/// Request for image generation
#[derive(Debug, Clone, Serialize)]
pub struct ImageGenerateRequest {
    /// ID of the model to use
    pub model: String,
    /// The prompt to generate images for
    pub prompt: String,
    /// Negative prompt (what not to include in the image)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative_prompt: Option<String>,
    /// Style preset for the generated image
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style_preset: Option<String>,
    /// Height of the generated image in pixels
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u32>,
    /// Width of the generated image in pixels
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,
    /// Number of diffusion steps
    #[serde(skip_serializing_if = "Option::is_none")]
    pub steps: Option<u32>,
    /// Guidance scale (how closely to follow the prompt)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cfg_scale: Option<f32>,
    /// Random seed for reproducible results
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<u64>,
    /// Strength of LoRA adaptation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lora_strength: Option<u32>,
    /// Enable safe mode to filter unsafe content
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safe_mode: Option<bool>,
    /// Return the image as binary data instead of URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_binary: Option<bool>,
    /// Remove the watermark from the generated image
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hide_watermark: Option<bool>,
    /// Additional custom parameters
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// Response from image generation API
#[derive(Debug, Clone, Deserialize)]
pub struct ImageGenerateResponse {
    /// The ID of the image generation request
    pub id: String,
    /// Array of generated image data (base64 encoded)
    pub images: Vec<String>,
    /// Request details that were used for generation
    #[serde(default)]
    pub request: Option<ImageGenerateRequestDetails>,
    /// Timing information about the request
    #[serde(default)]
    pub timing: Option<ImageGenerateTiming>,
    
    // For backward compatibility with code expecting the old format
    #[serde(skip)]
    pub created: u64,
    #[serde(skip)]
    pub data: Vec<ImageData>,
}

/// Request details returned in the response
#[derive(Debug, Clone, Deserialize)]
pub struct ImageGenerateRequestDetails {
    /// The model used for generation
    pub model: String,
    /// The prompt used for generation
    pub prompt: String,
    /// Width of the generated image
    #[serde(default)]
    pub width: Option<u32>,
    /// Height of the generated image
    #[serde(default)]
    pub height: Option<u32>,
    /// Steps used for diffusion
    #[serde(default)]
    pub steps: Option<u32>,
    /// Seed used for generation
    #[serde(default)]
    pub seed: Option<u64>,
    /// Additional fields (captured as a map)
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// Timing information from the API response
#[derive(Debug, Clone, Deserialize)]
pub struct ImageGenerateTiming {
    /// Total processing time in milliseconds
    #[serde(default)]
    pub total_ms: Option<f64>,
    /// Additional timing details
    #[serde(flatten)]
    pub details: HashMap<String, serde_json::Value>,
}

/// Data for a generated image (for backward compatibility)
#[derive(Debug, Clone, Deserialize)]
pub struct ImageData {
    /// URL to the generated image
    #[serde(default)]
    pub url: Option<String>,
    /// Base64 encoded image data (if return_binary is true)
    #[serde(default)]
    pub b64_json: Option<String>,
    /// Revised prompt that was used for generation
    #[serde(default)]
    pub revised_prompt: Option<String>,
    /// Seed that was used for generation
    #[serde(default)]
    pub seed: Option<u64>,
}

impl Default for ImageGenerateRequest {
    fn default() -> Self {
        Self {
            model: "fluently-xl".to_string(),
            prompt: String::new(),
            negative_prompt: None,
            style_preset: None,
            height: None,
            width: None,
            steps: None,
            cfg_scale: None,
            seed: None,
            lora_strength: None,
            safe_mode: None,
            return_binary: None,
            hide_watermark: None,
            extra: HashMap::new(),
        }
    }
}

/// Builder for image generation requests
#[derive(Debug, Clone)]
pub struct ImageGenerateRequestBuilder {
    request: ImageGenerateRequest,
}

impl ImageGenerateRequestBuilder {
    /// Create a new image generation request builder
    pub fn new(model: impl Into<String>, prompt: impl Into<String>) -> Self {
        Self {
            request: ImageGenerateRequest {
                model: model.into(),
                prompt: prompt.into(),
                ..Default::default()
            },
        }
    }

    /// Set the negative prompt
    pub fn with_negative_prompt(mut self, negative_prompt: impl Into<String>) -> Self {
        self.request.negative_prompt = Some(negative_prompt.into());
        self
    }

    /// Set the style preset
    pub fn with_style_preset(mut self, style_preset: impl Into<String>) -> Self {
        self.request.style_preset = Some(style_preset.into());
        self
    }

    /// Set the image height
    pub fn with_height(mut self, height: u32) -> Self {
        self.request.height = Some(height);
        self
    }

    /// Set the image width
    pub fn with_width(mut self, width: u32) -> Self {
        self.request.width = Some(width);
        self
    }

    /// Set the diffusion steps
    pub fn with_steps(mut self, steps: u32) -> Self {
        self.request.steps = Some(steps);
        self
    }

    /// Set the guidance scale
    pub fn with_cfg_scale(mut self, cfg_scale: f32) -> Self {
        self.request.cfg_scale = Some(cfg_scale);
        self
    }

    /// Set the random seed
    pub fn with_seed(mut self, seed: u64) -> Self {
        self.request.seed = Some(seed);
        self
    }

    /// Set the LoRA strength
    pub fn with_lora_strength(mut self, lora_strength: u32) -> Self {
        self.request.lora_strength = Some(lora_strength);
        self
    }

    /// Enable or disable safe mode
    pub fn with_safe_mode(mut self, safe_mode: bool) -> Self {
        self.request.safe_mode = Some(safe_mode);
        self
    }

    /// Enable or disable binary return format
    pub fn with_return_binary(mut self, return_binary: bool) -> Self {
        self.request.return_binary = Some(return_binary);
        self
    }

    /// Enable or disable watermark
    pub fn with_hide_watermark(mut self, hide_watermark: bool) -> Self {
        self.request.hide_watermark = Some(hide_watermark);
        self
    }

    /// Add a custom parameter to the request
    pub fn with_extra(mut self, key: impl Into<String>, value: impl Into<serde_json::Value>) -> Self {
        self.request.extra.insert(key.into(), value.into());
        self
    }

    /// Build the image generation request
    pub fn build(self) -> ImageGenerateRequest {
        self.request
    }
}

impl Client {
    /// Generate images
    ///
    /// # Examples
    ///
    /// ```
    /// use venice_ai_api_sdk_rust::{
    ///     Client,
    ///     image::ImageGenerateRequestBuilder,
    /// };
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::new("your-api-key")?;
    ///     
    ///     let request = ImageGenerateRequestBuilder::new(
    ///         "fluently-xl",
    ///         "A beautiful sunset over mountains",
    ///     )
    ///     .with_width(1024)
    ///     .with_height(1024)
    ///     .with_steps(30)
    ///     .build();
    ///     
    ///     let (response, _) = client.generate_image(request).await?;
    ///     
    ///     if let Some(image) = &response.data.first() {
    ///         println!("Image URL: {}", image.url.as_ref().unwrap_or(&"No URL".to_string()));
    ///     }
    ///     
    ///     Ok(())
    /// }
    /// ```
    pub async fn generate_image(
        &self,
        request: ImageGenerateRequest,
    ) -> VeniceResult<(ImageGenerateResponse, RateLimitInfo)> {
        let (mut response, rate_limit_info): (ImageGenerateResponse, RateLimitInfo) = self.post(IMAGE_GENERATE_ENDPOINT, &request).await?;
        
        // For backward compatibility, populate the old fields from the new response format
        response.created = chrono::Utc::now().timestamp() as u64;
        
        // Convert images array to the old data format
        response.data = response.images.iter().enumerate().map(|(_i, img_data)| {
            ImageData {
                // We don't have URLs in the new format, just base64 data
                url: None,
                b64_json: Some(img_data.clone()),
                revised_prompt: None,
                seed: response.request.as_ref().and_then(|req| req.seed),
            }
        }).collect();
        
        Ok((response, rate_limit_info))
    }
}

/// Helper function to generate images
///
/// # Examples
///
/// ```
/// use venice_ai_api_sdk_rust::image::{
///     generate_image,
///     ImageGenerateRequestBuilder,
/// };
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let request = ImageGenerateRequestBuilder::new(
///         "fluently-xl",
///         "A beautiful sunset over mountains",
///     )
///     .with_width(1024)
///     .with_height(1024)
///     .with_steps(30)
///     .build();
///     
///     let (response, _) = generate_image("your-api-key", request).await?;
///     
///     if let Some(image) = &response.data.first() {
///         println!("Image URL: {}", image.url.as_ref().unwrap_or(&"No URL".to_string()));
///     }
///     
///     Ok(())
/// }
/// ```
pub async fn generate_image(
    api_key: impl Into<String>,
    request: ImageGenerateRequest,
) -> VeniceResult<(ImageGenerateResponse, RateLimitInfo)> {
    let client = Client::new(api_key)?;
    client.generate_image(request).await
}