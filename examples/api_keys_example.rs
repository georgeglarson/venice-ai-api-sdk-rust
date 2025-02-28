use std::error::Error;
use venice_ai_api_sdk_rust::{
    api_keys::{ApiKeysClient, CreateApiKeyBuilder},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load environment variables from .env file
    if let Err(e) = dotenv::dotenv() {
        println!("Warning: Could not load .env file: {}", e);
    }
    
    // Get API key from environment variable
    let api_key = match std::env::var("VENICE_API_KEY") {
        Ok(key) => key,
        Err(e) => {
            println!("Error: VENICE_API_KEY not set in .env file: {}", e);
            println!("Please make sure you've created a .env file with your API key.");
            return Ok(());
        }
    };
    
    // Create an API keys client
    println!("Connecting to Venice.ai API...");
    let api_keys_client = ApiKeysClient::new(api_key)?;
    
    // List existing API keys
    println!("Fetching existing API keys...");
    let keys = api_keys_client.list_api_keys().await?;
    
    println!("\nExisting API keys:");
    for key in &keys.data {
        println!("- {} ({})", key.name, key.id);
        println!("  Created: {}", format_timestamp(key.created));
        println!("  Last chars: {}", key.last_chars);
        println!("  Revoked: {}", key.revoked);
        
        if let Some(rate_limits) = &key.rate_limits {
            println!("  Rate Limits:");
            if let Some(rpm) = rate_limits.requests_per_minute {
                println!("    Requests per minute: {}", rpm);
            }
            if let Some(rpd) = rate_limits.requests_per_day {
                println!("    Requests per day: {}", rpd);
            }
            if let Some(tpm) = rate_limits.tokens_per_minute {
                println!("    Tokens per minute: {}", tpm);
            }
        }
        
        println!();
    }
    
    // Create a new API key (commented out to prevent actual creation in example)
    /*
    println!("Creating a new API key...");
    let request = CreateApiKeyBuilder::new("SDK Example Key")
        .with_requests_per_minute(100)
        .with_tokens_per_minute(10000)
        .build();
    
    let create_response = api_keys_client.create_api_key(request).await?;
    
    println!("\nNew API Key created:");
    println!("- Name: {}", create_response.data.name);
    println!("- ID: {}", create_response.data.id);
    println!("- Created: {}", format_timestamp(create_response.data.created));
    println!("- Key: {}", create_response.data.key);
    println!("  (IMPORTANT: Save this key now as it won't be shown again)");
    
    // Delete the created API key
    println!("\nDeleting the created API key...");
    let delete_response = api_keys_client.delete_api_key(&create_response.data.id).await?;
    
    if delete_response.deleted {
        println!("Successfully deleted API key: {}", delete_response.id);
    } else {
        println!("Failed to delete API key");
    }
    */
    
    println!("API key management example completed!");
    
    Ok(())
}

// Helper function to format Unix timestamp
fn format_timestamp(timestamp: u64) -> String {
    let datetime = chrono::DateTime::<chrono::Utc>::from_timestamp(timestamp as i64, 0)
        .expect("Invalid timestamp");
    datetime.format("%Y-%m-%d %H:%M:%S UTC").to_string()
}