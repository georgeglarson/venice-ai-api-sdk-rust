use std::env;
use std::error::Error;
use venice_ai_api_sdk_rust::{
    Client,
    traits::api_keys::ApiKeysApi,
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
    
    // List API keys
    println!("Fetching API keys...");
    let (keys_response, rate_limit) = client.list_api_keys().await?;
    
    println!("\nYour API keys:");
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
    
    // Print rate limit information
    println!("Rate limit information:");
    println!("Requests: {}/{}", 
        rate_limit.remaining_requests.unwrap_or(0),
        rate_limit.limit_requests.unwrap_or(0)
    );
    
    Ok(())
}