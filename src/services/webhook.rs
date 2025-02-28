//! Webhook verification service
//!
//! This module provides a service for verifying webhook signatures.

use hmac::{Hmac, Mac};
use sha2::Sha256;

use crate::error::VeniceError;

/// Service for verifying webhook signatures
#[derive(Debug, Clone)]
pub struct WebhookService;

impl WebhookService {
    /// Create a new webhook service
    pub fn new() -> Self {
        Self
    }
    
    /// Verify a webhook signature
    ///
    /// # Arguments
    ///
    /// * `signature` - The signature from the webhook request
    /// * `timestamp` - The timestamp from the webhook request
    /// * `body` - The raw body of the webhook request
    /// * `secret` - The webhook secret
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the signature is valid
    /// * `Err(VeniceError)` if the signature is invalid
    pub fn verify_signature(
        &self,
        signature: &str,
        timestamp: &str,
        body: &[u8],
        secret: &str,
    ) -> Result<(), VeniceError> {
        // Create the message to verify
        let message = format!("{}:{}", timestamp, String::from_utf8_lossy(body));
        
        // Create the HMAC
        let mut mac = Hmac::<Sha256>::new_from_slice(secret.as_bytes())
            .map_err(|_| VeniceError::InvalidWebhookSignature("Invalid secret".to_string()))?;
        
        // Update the HMAC with the message
        mac.update(message.as_bytes());
        
        // Get the HMAC result
        let result = mac.finalize().into_bytes();
        
        // Convert the result to a hex string
        let computed_signature = hex::encode(result);
        
        // Compare the signatures using constant-time comparison
        if self.constant_time_compare(signature, &computed_signature) {
            Ok(())
        } else {
            Err(VeniceError::InvalidWebhookSignature(
                "Signature mismatch".to_string(),
            ))
        }
    }
    
    /// Compare two strings in constant time
    ///
    /// This function compares two strings in constant time to prevent timing attacks.
    /// It returns true if the strings are equal, false otherwise.
    fn constant_time_compare(&self, a: &str, b: &str) -> bool {
        if a.len() != b.len() {
            return false;
        }
        
        let a_bytes = a.as_bytes();
        let b_bytes = b.as_bytes();
        
        let mut result = 0;
        for i in 0..a.len() {
            result |= a_bytes[i] ^ b_bytes[i];
        }
        
        result == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_constant_time_compare() {
        let service = WebhookService::new();
        
        // Equal strings
        assert!(service.constant_time_compare("hello", "hello"));
        
        // Different strings of same length
        assert!(!service.constant_time_compare("hello", "world"));
        
        // Different strings of different length
        assert!(!service.constant_time_compare("hello", "hello world"));
    }
    
    #[test]
    fn test_verify_signature_valid() {
        let service = WebhookService::new();
        
        // Test data
        let secret = "test_secret";
        let timestamp = "1234567890";
        let body = b"{\"test\":\"data\"}";
        
        // Create a valid signature
        let message = format!("{}:{}", timestamp, String::from_utf8_lossy(body));
        let mut mac = Hmac::<Sha256>::new_from_slice(secret.as_bytes()).unwrap();
        mac.update(message.as_bytes());
        let result = mac.finalize().into_bytes();
        let signature = hex::encode(result);
        
        // Verify the signature
        let result = service.verify_signature(&signature, timestamp, body, secret);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_verify_signature_invalid() {
        let service = WebhookService::new();
        
        // Test data
        let secret = "test_secret";
        let timestamp = "1234567890";
        let body = b"{\"test\":\"data\"}";
        let signature = "invalid_signature";
        
        // Verify the signature
        let result = service.verify_signature(signature, timestamp, body, secret);
        assert!(result.is_err());
    }
}