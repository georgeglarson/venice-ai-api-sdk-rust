//! Image API implementation
//!
//! This module provides an implementation of the image API.

use async_trait::async_trait;

use crate::error::{RateLimitInfo, VeniceResult};
use crate::http::SharedHttpClient;
use crate::models::list::Model;
use crate::traits::image::{
    ImageApi, ImageGenerateRequest, ImageGenerateResponse,
    ImageUpscaleRequest, ImageUpscaleResponse, ListImageStylesResponse,
};

/// Implementation of the image API
#[derive(Debug, Clone)]
pub struct ImageApiImpl {
    /// The HTTP client to use for requests
    http_client: SharedHttpClient,
}

impl ImageApiImpl {
    /// Create a new image API implementation
    pub fn new(http_client: SharedHttpClient) -> Self {
        Self { http_client }
    }
}

#[async_trait]
impl ImageApi for ImageApiImpl {
    async fn generate_image(
        &self,
        request: ImageGenerateRequest,
    ) -> VeniceResult<(ImageGenerateResponse, RateLimitInfo)> {
        let (mut result, rate_limit_info): (ImageGenerateResponse, RateLimitInfo) = self.http_client.post("image/generations", &request).await?;
        
        // Populate backward compatibility fields
        result.created = Some(chrono::Utc::now().timestamp() as u64);
        
        // Convert images array to the old data format
        result.data = result.images.iter().enumerate().map(|(_i, img_data)| {
            crate::traits::image::ImageData {
                // We don't have URLs in the new format, just base64 data
                url: None,
                b64_json: Some(img_data.clone()),
                revised_prompt: None,
                seed: result.request.as_ref().and_then(|req| req.seed),
            }
        }).collect();
        
        Ok((result, rate_limit_info))
    }
    
    async fn list_styles(&self) -> VeniceResult<(ListImageStylesResponse, RateLimitInfo)> {
        self.http_client.get("image/styles").await
    }
    
    async fn upscale_image(
        &self,
        request: ImageUpscaleRequest,
    ) -> VeniceResult<ImageUpscaleResponse> {
        // The API requires multipart/form-data for upscaling
        let mut form = reqwest::multipart::Form::new()
            .text("model", request.model.clone());
        
        // Scale must be either 2 or 4
        let scale = request.scale.unwrap_or(2);
        if scale != 2 && scale != 4 {
            return Err(crate::error::VeniceError::InvalidInput(
                "Scale must be either 2 or 4".to_string()
            ));
        }
        form = form.text("scale", scale.to_string());
        
        // Add the image data - either from URL or base64
        if let Some(image_url) = &request.image_url {
            // If URL provided, add it as text
            form = form.text("image_url", image_url.clone());
        } else if let Some(image_data) = &request.image_data {
            // If base64 provided, convert to binary and add as part
            let binary_data = match base64::decode(image_data) {
                Ok(data) => data,
                Err(e) => return Err(crate::error::VeniceError::InvalidInput(
                    format!("Invalid base64 data: {}", e)
                )),
            };
            
            let part = reqwest::multipart::Part::bytes(binary_data)
                .file_name("image.png")
                .mime_str("image/png")
                .map_err(|e| crate::error::VeniceError::InvalidInput(format!("Invalid mime type: {}", e)))?;
            
            form = form.part("image", part);
        } else {
            return Err(crate::error::VeniceError::InvalidInput(
                "Either image_url or image_data must be provided".to_string()
            ));
        }
        
        // Send the multipart request
        let (binary_data, mime_type, _) = self.http_client.post_multipart_binary("image/upscale", form).await?;
        
        // Create response with binary data
        let mut result = ImageUpscaleResponse {
            image_data: binary_data,
            mime_type,
            created: Some(chrono::Utc::now().timestamp() as u64),
            data: Vec::new(),
        };
        
        // For backward compatibility, encode the binary data back to base64
        let b64_data = base64::encode(&result.image_data);
        result.data.push(crate::traits::image::UpscaledImageData {
            url: None,
            b64_json: Some(b64_data),
        });
        
        Ok(result)
    }
}

// Additional methods not part of the ImageApi trait
impl ImageApiImpl {
    /// Get models that are compatible with image generation
    pub async fn get_compatible_models(&self) -> VeniceResult<(Vec<Model>, RateLimitInfo)> {
        self.http_client.get("models?supports_image_generation=true").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::http::{HttpClientConfig, new_shared_http_client};
    
    #[tokio::test]
    async fn test_generate_image() {
        // Create a mock HTTP client
        let config = HttpClientConfig {
            api_key: "test_api_key".to_string(),
            base_url: "https://api.venice.ai".to_string(),
            custom_headers: reqwest::header::HeaderMap::new(),
            timeout_secs: None,
        };
        let http_client = new_shared_http_client(config).unwrap();
        
        // Create the image API implementation
        let image_api = ImageApiImpl::new(http_client);
        
        // TODO: Mock the HTTP client to return a response
        // For now, we'll just check that the method exists and has the right signature
        let _: ImageApiImpl = image_api;
    }
    
    #[tokio::test]
    async fn test_upscale_image() {
        // Create a mock HTTP client
        let config = HttpClientConfig {
            api_key: "test_api_key".to_string(),
            base_url: "https://api.venice.ai".to_string(),
            custom_headers: reqwest::header::HeaderMap::new(),
            timeout_secs: None,
        };
        let http_client = new_shared_http_client(config).unwrap();
        
        // Create the image API implementation
        let image_api = ImageApiImpl::new(http_client);
        
        // TODO: Mock the HTTP client to return a response
        // For now, we'll just check that the method exists and has the right signature
        let _: ImageApiImpl = image_api;
    }
}