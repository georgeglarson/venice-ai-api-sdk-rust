use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use crate::error::{VeniceError, VeniceResult};

/// Default base URL for the Venice.ai API
pub const DEFAULT_BASE_URL: &str = "https://api.venice.ai/api/v1";

/// Configuration for the Venice.ai API client
#[derive(Debug, Clone)]
pub struct ClientConfig {
    /// Base URL for the API
    pub base_url: String,
    /// API key for authentication
    pub api_key: String,
    /// Custom headers to include in all requests
    pub custom_headers: HeaderMap,
    /// Timeout in seconds for requests
    pub timeout_secs: Option<u64>,
}

impl ClientConfig {
    /// Create a new configuration with the given API key
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            base_url: DEFAULT_BASE_URL.to_string(),
            api_key: api_key.into(),
            custom_headers: HeaderMap::new(),
            timeout_secs: None,
        }
    }

    /// Set a custom base URL
    pub fn with_base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = base_url.into();
        self
    }

    /// Set a timeout in seconds
    pub fn with_timeout(mut self, timeout_secs: u64) -> Self {
        self.timeout_secs = Some(timeout_secs);
        self
    }

    /// Add a custom header
    pub fn with_header(mut self, name: &str, value: &str) -> VeniceResult<Self> {
        let header_name = reqwest::header::HeaderName::from_bytes(name.as_bytes())
            .map_err(|_| VeniceError::InvalidInput(format!("Invalid header name: {}", name)))?;
        
        let header_value = HeaderValue::from_str(value)
            .map_err(|_| VeniceError::InvalidInput(format!("Invalid header value: {}", value)))?;
        
        self.custom_headers.insert(header_name, header_value);
        Ok(self)
    }

    /// Create default headers for requests
    pub fn create_default_headers(&self) -> VeniceResult<HeaderMap> {
        let mut headers = HeaderMap::new();
        
        // Add authorization header
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", self.api_key)).map_err(|_| {
                VeniceError::InvalidInput("Invalid API key format".to_string())
            })?,
        );
        
        // Add content type header
        headers.insert(
            CONTENT_TYPE,
            HeaderValue::from_static("application/json"),
        );
        
        // Add accept header
        headers.insert(
            ACCEPT,
            HeaderValue::from_static("application/json"),
        );
        
        // Add custom headers
        for (name, value) in self.custom_headers.iter() {
            headers.insert(name.clone(), value.clone());
        }
        
        Ok(headers)
    }
}