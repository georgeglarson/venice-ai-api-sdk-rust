use std::error::Error;
use venice_ai_api_sdk_rust::api_keys::ApiKeysClient;

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
    
    println!("Testing API Keys Endpoint");
    println!("========================");
    
    // Create an API keys client (with updated implementation)
    println!("Creating ApiKeysClient...");
    let api_keys_client = ApiKeysClient::new(api_key)?;
    
    // List existing API keys
    println!("Fetching existing API keys...");
    let result = api_keys_client.list_api_keys().await;
    
    match result {
        Ok(keys) => {
            println!("\nExisting API keys:");
            if keys.data.is_empty() {
                println!("No API keys found.");
            } else {
                for key in &keys.data {
                    println!("- {} ({})", key.name, key.id);
                    println!("  Created: {}", key.created_at);
                    
                    if let Some(expires_at) = &key.expires_at {
                        println!("  Expires: {}", expires_at);
                    }
                    
                    println!("  Last chars: {}", key.last_chars);
                    
                    if let Some(last_used_at) = &key.last_used_at {
                        println!("  Last used: {}", last_used_at);
                    }
                    
                    if let Some(api_key_type) = &key.api_key_type {
                        println!("  Type: {}", api_key_type);
                    }
                    
                    if let Some(revoked) = key.revoked {
                        println!("  Revoked: {}", revoked);
                    }
                    
                    if let Some(usage) = &key.usage {
                        println!("  Usage:");
                        
                        if let Some(requests) = &usage.requests {
                            if let (Some(used), Some(limit)) = (requests.used, requests.limit) {
                                println!("    Requests: {}/{}", used, limit);
                            }
                        }
                        
                        if let Some(tokens) = &usage.tokens {
                            if let (Some(used), Some(limit)) = (tokens.used, tokens.limit) {
                                println!("    Tokens: {}/{}", used, limit);
                            }
                        }
                    }
                    
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
            }
        }
        Err(e) => {
            println!("\nError fetching API keys: {}", e);
        }
    }
    
    println!("API Keys endpoint test completed!");
    
    Ok(())
}