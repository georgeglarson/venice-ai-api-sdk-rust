//! Example demonstrating webhook verification for Venice.ai API
//!
//! This example shows how to verify webhook signatures from Venice.ai.
//! It simulates receiving a webhook request and verifying its signature.

use std::collections::HashMap;
use venice_ai_api_sdk_rust::webhooks::{verify_webhook_signature, get_webhook_signature};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // In a real application, this would be your webhook secret from Venice.ai
    // You should store this securely (e.g., in an environment variable)
    let webhook_secret = "your_webhook_secret";
    
    // Simulate receiving a webhook request
    // In a real application, this would be the raw request body from the webhook
    let webhook_payload = r#"{"event":"model.created","data":{"id":"model-123","name":"New Model"}}"#;
    
    // Simulate the headers from the webhook request
    // In a real application, these would come from the HTTP request headers
    let mut headers = HashMap::new();
    
    // Create a valid signature for demonstration purposes
    // In a real application, this would be provided by Venice.ai in the request headers
    let mut mac = hmac::Hmac::<sha2::Sha256>::new_from_slice(webhook_secret.as_bytes())?;
    hmac::Mac::update(&mut mac, webhook_payload.as_bytes());
    let signature = hex::encode(mac.finalize().into_bytes());
    
    // Add the signature to the headers
    headers.insert("x-venice-signature".to_string(), signature.clone());
    
    println!("Simulated webhook payload: {}", webhook_payload);
    println!("Generated signature: {}", signature);
    
    // Extract the signature from headers
    let extracted_signature = get_webhook_signature(&headers)
        .expect("Signature should be present in headers");
    
    println!("Extracted signature from headers: {}", extracted_signature);
    
    // Verify the webhook signature
    let is_valid = verify_webhook_signature(
        webhook_payload.as_bytes(),
        &extracted_signature,
        webhook_secret,
    )?;
    
    if is_valid {
        println!("✅ Webhook signature verified successfully!");
        
        // Process the webhook payload
        // In a real application, you would parse the JSON and handle the event
        let event: serde_json::Value = serde_json::from_str(webhook_payload)?;
        println!("Event type: {}", event["event"]);
        println!("Model ID: {}", event["data"]["id"]);
        println!("Model name: {}", event["data"]["name"]);
    } else {
        println!("❌ Invalid webhook signature!");
    }
    
    // Demonstrate an invalid signature
    println!("\nTesting with an invalid signature:");
    let invalid_signature = "invalid_signature";
    
    let is_valid = verify_webhook_signature(
        webhook_payload.as_bytes(),
        invalid_signature,
        webhook_secret,
    )?;
    
    if is_valid {
        println!("✅ Webhook signature verified successfully!");
    } else {
        println!("❌ Invalid webhook signature!");
    }
    
    Ok(())
}