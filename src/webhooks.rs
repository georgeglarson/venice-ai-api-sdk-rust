//! Webhook verification utilities for Venice.ai API
//!
//! This module provides functions for verifying webhook signatures from Venice.ai.
//! Webhooks are HTTP callbacks that are triggered when certain events occur in the Venice.ai system.
//! To ensure that webhook requests are genuinely from Venice.ai, they include a signature that can be verified.

use crate::error::{VeniceError, VeniceResult};
use crate::services::webhook::WebhookService;

/// Verifies a webhook signature from Venice.ai
///
/// This function verifies that a webhook request is genuinely from Venice.ai by checking
/// the signature against the payload using the shared webhook secret.
///
/// # Arguments
///
/// * `payload` - The raw webhook payload (request body)
/// * `signature` - The signature provided in the request headers (typically in the `X-Venice-Signature` header)
/// * `timestamp` - The timestamp provided in the request headers (typically in the `X-Venice-Timestamp` header)
/// * `secret` - The webhook secret shared between your application and Venice.ai
///
/// # Returns
///
/// * `Ok(true)` if the signature is valid
/// * `Ok(false)` if the signature is invalid
/// * `Err(VeniceError)` if an error occurs during verification
///
/// # Example
///
/// ```rust,no_run
/// use venice_ai_api_sdk_rust::webhooks::verify_webhook_signature;
///
/// async fn handle_webhook(
///     body: Vec<u8>,
///     signature: String,
///     timestamp: String,
/// ) -> Result<(), Box<dyn std::error::Error>> {
///     let webhook_secret = std::env::var("VENICE_WEBHOOK_SECRET")?;
///
///     if verify_webhook_signature(&body, &signature, &timestamp, &webhook_secret)? {
///         println!("Webhook signature verified!");
///         // Process the webhook
///     } else {
///         println!("Invalid webhook signature!");
///     }
///
///     Ok(())
/// }
/// ```
pub fn verify_webhook_signature(
    payload: &[u8],
    signature: &str,
    timestamp: &str,
    secret: &str,
) -> VeniceResult<bool> {
    let webhook_service = WebhookService::new();
    
    match webhook_service.verify_signature(signature, timestamp, payload, secret) {
        Ok(()) => Ok(true),
        Err(VeniceError::InvalidWebhookSignature(_)) => Ok(false),
        Err(e) => Err(e),
    }
}

/// Extracts the Venice.ai webhook signature and timestamp from headers
///
/// This function extracts the webhook signature and timestamp from the request headers.
/// The signature is typically provided in the `X-Venice-Signature` header.
/// The timestamp is typically provided in the `X-Venice-Timestamp` header.
///
/// # Arguments
///
/// * `headers` - The request headers
///
/// # Returns
///
/// * `(Option<String>, Option<String>)` - The signature and timestamp, if present
///
/// # Example
///
/// ```rust,no_run
/// use venice_ai_api_sdk_rust::webhooks::get_webhook_headers;
/// use std::collections::HashMap;
///
/// fn extract_headers(headers: &HashMap<String, String>) -> (Option<String>, Option<String>) {
///     get_webhook_headers(headers)
/// }
/// ```
pub fn get_webhook_headers<T>(headers: &T) -> (Option<String>, Option<String>)
where
    T: Headers,
{
    let signature = headers.get("x-venice-signature");
    let timestamp = headers.get("x-venice-timestamp");
    
    (signature, timestamp)
}

/// Trait for accessing headers from different HTTP implementations
///
/// This trait allows the webhook functions to work with different HTTP implementations
/// by providing a common interface for accessing headers.
pub trait Headers {
    /// Gets a header value by name
    fn get(&self, name: &str) -> Option<String>;
}

// Implement Headers for common HTTP header types
impl Headers for std::collections::HashMap<String, String> {
    fn get(&self, name: &str) -> Option<String> {
        self.get(&name.to_lowercase()).cloned()
    }
}

impl Headers for reqwest::header::HeaderMap {
    fn get(&self, name: &str) -> Option<String> {
        self.get(name)
            .and_then(|value| value.to_str().ok())
            .map(|s| s.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hmac::{Hmac, Mac};
    use sha2::Sha256;
    
    type HmacSha256 = Hmac<Sha256>;
    
    #[test]
    fn test_verify_webhook_signature_valid() {
        let secret = "test_secret";
        let payload = b"test_payload";
        let timestamp = "1234567890";
        
        // Create a valid signature
        let message = format!("{}:{}", timestamp, String::from_utf8_lossy(payload));
        let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).unwrap();
        mac.update(message.as_bytes());
        let signature = hex::encode(mac.finalize().into_bytes());
        
        assert!(verify_webhook_signature(payload, &signature, timestamp, secret).unwrap());
    }
    
    #[test]
    fn test_verify_webhook_signature_invalid() {
        let secret = "test_secret";
        let payload = b"test_payload";
        let timestamp = "1234567890";
        let invalid_signature = "invalid_signature";
        
        assert!(!verify_webhook_signature(payload, invalid_signature, timestamp, secret).unwrap());
    }
    
    #[test]
    fn test_get_webhook_headers() {
        let mut headers = std::collections::HashMap::new();
        headers.insert("x-venice-signature".to_string(), "test_signature".to_string());
        headers.insert("x-venice-timestamp".to_string(), "1234567890".to_string());
        
        let (signature, timestamp) = get_webhook_headers(&headers);
        assert_eq!(signature, Some("test_signature".to_string()));
        assert_eq!(timestamp, Some("1234567890".to_string()));
    }
}