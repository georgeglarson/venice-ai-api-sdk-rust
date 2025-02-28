use std::env;
use std::error::Error;
use venice_ai_api_sdk_rust::{
    Client,
    traits::api_keys::{ApiKeysApi, CreateApiKeyRequest},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Get API key from environment variable
    let api_key = env::var("VENICE_API_KEY").expect("VENICE_API_KEY not set");
    
    // Create a unified client
    let client = Client::new(&api_key)?;
    
    // List existing API keys
    println!("Listing existing API keys:");
    let (keys_response, _) = client.list_api_keys().await?;
    for key in &keys_response.data {
        println!("- {} ({})", key.name, key.id);
        println!("  Created: {}", key.created_at);
        println!("  Prefix: {}", key.prefix);
        println!("  Last used: {}", key.last_used_at.unwrap_or_default());
        println!("  Active: {}", key.active);
        println!();
    }
    
    // Create a new API key
    println!("Creating a new API key...");
    let request = CreateApiKeyRequest {
        name: "SDK Example Key".to_string(),
    };
    
    let (create_response, _) = client.create_api_key(request).await?;
    
    println!("New API Key created:");
    println!("- Name: {}", create_response.key.name);
    println!("- ID: {}", create_response.key.id);
    println!("- Created: {}", create_response.key.created_at);
    println!("- Key: {}", create_response.secret);
    println!("  (IMPORTANT: Save this key now as it won't be shown again)");
    
    // Delete the API key (commented out for safety in this example)
    // Uncomment these lines if you want to delete the key
    /*
    println!("\nDeleting the created API key...");
    let (delete_response, _) = client.delete_api_key(&create_response.key.id).await?;
    
    if delete_response.deleted {
        println!("Successfully deleted API key: {}", delete_response.id);
    } else {
        println!("Failed to delete API key");
    }
    */
    
    Ok(())
}