use std::env;
use std::error::Error;
use venice_ai_api_sdk_rust::{
    Client,
    traits::api_keys::{ApiKeysApi, CreateApiKeyRequest},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load environment variables from .env file
    if let Err(e) = dotenv::dotenv() {
        println!("Warning: Could not load .env file: {}", e);
    }
    
    // Get API key from environment variable
    let api_key = env::var("VENICE_API_KEY").expect("VENICE_API_KEY not set");
    
    // Create a client with the unified architecture
    let client = Client::new(&api_key)?;
    
    // Create a request to create a new API key
    let request = CreateApiKeyRequest {
        name: "SDK Example Key".to_string(),
    };
    
    // Send the request
    println!("Creating a new API key...");
    let (create_response, rate_limit) = client.create_api_key(request).await?;
    
    // Print the new API key information
    println!("\nAPI key created successfully!");
    println!("Key details:");
    println!("- Name: {}", create_response.key.name);
    println!("- ID: {}", create_response.key.id);
    println!("- Prefix: {}", create_response.key.prefix);
    println!("- Created: {}", create_response.key.created_at);
    
    // IMPORTANT: The full API key is only returned once
    println!("\n⚠️ IMPORTANT: Save this API key, it will not be shown again!");
    println!("API Key: {}", create_response.secret);
    
    // Print rate limit information
    println!("\nRate limit information:");
    println!("Requests: {}/{}", 
        rate_limit.remaining_requests.unwrap_or(0),
        rate_limit.limit_requests.unwrap_or(0)
    );
    
    // Provide instructions for deleting the key
    println!("\nTo delete this key, run:");
    println!("cargo run --example delete_api_key --features examples -- {}", create_response.key.id);
    
    Ok(())
}