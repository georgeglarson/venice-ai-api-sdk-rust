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
    
    // List existing API keys
    println!("Fetching existing API keys...");
    let (keys_response, _) = client.list_api_keys().await?;
    
    println!("\nExisting API keys:");
    if keys_response.data.is_empty() {
        println!("No API keys found.");
    } else {
        for (i, key) in keys_response.data.iter().enumerate() {
            println!("{}. {} ({})", i + 1, key.name, key.id);
            println!("   Prefix: {}", key.prefix);
            println!("   Created: {}", key.created_at);
            println!("   Last used: {}", key.last_used_at.unwrap_or_default());
            println!("   Active: {}", if key.active { "Yes" } else { "No" });
            println!();
        }
    }
    
    // Create a new API key
    println!("Creating a new API key...");
    let request = CreateApiKeyRequest {
        name: "SDK Example Key".to_string(),
    };
    
    let (create_response, _) = client.create_api_key(request).await?;
    
    println!("\nAPI key created successfully!");
    println!("Key details:");
    println!("- Name: {}", create_response.key.name);
    println!("- ID: {}", create_response.key.id);
    println!("- Created: {}", create_response.key.created_at);
    
    // IMPORTANT: The full API key is only returned once
    println!("\n⚠️ IMPORTANT: Save this API key, it will not be shown again!");
    println!("API Key: {}", create_response.secret);
    
    // Delete the API key we just created
    println!("\nDeleting the API key we just created...");
    let (delete_response, _) = client.delete_api_key(&create_response.key.id).await?;
    
    if delete_response.deleted {
        println!("API key deleted successfully!");
    } else {
        println!("Failed to delete API key.");
    }
    
    // List API keys again to confirm deletion
    println!("\nFetching API keys again to confirm deletion...");
    let (updated_keys_response, rate_limit) = client.list_api_keys().await?;
    
    println!("\nCurrent API keys:");
    for (i, key) in updated_keys_response.data.iter().enumerate() {
        println!("{}. {} ({})", i + 1, key.name, key.id);
    }
    
    // Print rate limit information
    println!("\nRate limit information:");
    println!("Requests: {}/{}", 
        rate_limit.remaining_requests.unwrap_or(0),
        rate_limit.limit_requests.unwrap_or(0)
    );
    
    Ok(())
}