use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::error::{RateLimitInfo, VeniceResult};

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
    pub created: Option<u64>,
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
    pub seed: Option<i64>,
}

/// Timing information from the API response
#[derive(Debug, Clone, Deserialize)]
pub struct ImageGenerateTiming {
    /// Total processing time in milliseconds
    #[serde(default)]
    pub total_ms: Option<f64>,
}

/// Data for a generated image
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
    pub seed: Option<i64>,
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

/// Response from image styles API
#[derive(Debug, Deserialize)]
pub struct ListImageStylesResponse {
    /// Array of available style presets or a single style name
    #[serde(rename = "data")]
    pub styles: Vec<String>,
}

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
}

/// Response from image upscaling API
#[derive(Debug, Clone)]
pub struct ImageUpscaleResponse {
    /// Raw binary data of the upscaled image
    pub image_data: Vec<u8>,
    /// MIME type of the image (usually image/png)
    pub mime_type: String,
    
    // For backward compatibility
    pub created: Option<u64>,
    pub data: Vec<UpscaledImageData>,
}

/// Data for an upscaled image (for backward compatibility)
#[derive(Debug, Clone, Deserialize)]
pub struct UpscaledImageData {
    /// URL to the upscaled image
    #[serde(default)]
    pub url: Option<String>,
    /// Base64 encoded image data (if return_binary is true)
    #[serde(default)]
    pub b64_json: Option<String>,
}

/// Image API trait
#[async_trait]
pub trait ImageApi {
    /// Generate images
    async fn generate_image(
        &self,
        request: ImageGenerateRequest,
    ) -> VeniceResult<(ImageGenerateResponse, RateLimitInfo)>;
    
    /// List available image styles
    async fn list_styles(&self) -> VeniceResult<(ListImageStylesResponse, RateLimitInfo)>;
    
    /// Upscale an image
    async fn upscale_image(
        &self,
        request: ImageUpscaleRequest,
    ) -> VeniceResult<ImageUpscaleResponse>;
}


/// Builder for image generation requests
#[derive(Debug, Clone)]
pub struct ImageGenerateBuilder {
    request: ImageGenerateRequest,
}

impl ImageGenerateBuilder {
    /// Create a new image generation request builder
    pub fn new(model: impl Into<String>, prompt: impl Into<String>) -> Self {
        Self {
            request: ImageGenerateRequest {
                model: model.into(),
                prompt: prompt.into(),
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
            },
        }
    }

    /// Set the negative prompt
    pub fn negative_prompt(mut self, value: impl Into<String>) -> Self {
        self.request.negative_prompt = Some(value.into());
        self
    }

    /// Set the style preset
    pub fn style_preset(mut self, value: impl Into<String>) -> Self {
        self.request.style_preset = Some(value.into());
        self
    }

    /// Set the image height
    pub fn height(mut self, value: u32) -> Self {
        self.request.height = Some(value);
        self
    }

    /// Set the image width
    pub fn width(mut self, value: u32) -> Self {
        self.request.width = Some(value);
        self
    }

    /// Set the diffusion steps
    pub fn steps(mut self, value: u32) -> Self {
        self.request.steps = Some(value);
        self
    }

    /// Set the guidance scale
    pub fn cfg_scale(mut self, value: f32) -> Self {
        self.request.cfg_scale = Some(value);
        self
    }

    /// Set the random seed
    pub fn seed(mut self, value: u64) -> Self {
        self.request.seed = Some(value);
        self
    }

    /// Set the LoRA strength
    pub fn lora_strength(mut self, value: u32) -> Self {
        self.request.lora_strength = Some(value);
        self
    }

    /// Enable or disable safe mode
    pub fn safe_mode(mut self, value: bool) -> Self {
        self.request.safe_mode = Some(value);
        self
    }

    /// Enable or disable binary return format
    pub fn return_binary(mut self, value: bool) -> Self {
        self.request.return_binary = Some(value);
        self
    }

    /// Enable or disable watermark
    pub fn hide_watermark(mut self, value: bool) -> Self {
        self.request.hide_watermark = Some(value);
        self
    }

    /// Build the image generation request
    pub fn build(self) -> ImageGenerateRequest {
        self.request
    }
}

/// Builder for image upscaling requests
#[derive(Debug, Clone)]
pub struct ImageUpscaleBuilder {
    request: ImageUpscaleRequest,
}

impl ImageUpscaleBuilder {
    /// Create a new image upscaling request builder with image URL
    pub fn with_url(model: impl Into<String>, image_url: impl Into<String>) -> Self {
        Self {
            request: ImageUpscaleRequest {
                model: model.into(),
                image_url: Some(image_url.into()),
                image_data: None,
                scale: None,
                return_binary: None,
            },
        }
    }

    /// Create a new image upscaling request builder with image data
    pub fn with_data(model: impl Into<String>, image_data: impl Into<String>) -> Self {
        Self {
            request: ImageUpscaleRequest {
                model: model.into(),
                image_url: None,
                image_data: Some(image_data.into()),
                scale: None,
                return_binary: None,
            },
        }
    }

    /// Set the scale factor
    pub fn scale(mut self, value: u32) -> Self {
        self.request.scale = Some(value);
        self
    }

    /// Enable or disable binary return format
    pub fn return_binary(mut self, value: bool) -> Self {
        self.request.return_binary = Some(value);
        self
    }

    /// Build the image upscaling request
    pub fn build(self) -> ImageUpscaleRequest {
        self.request
    }
}