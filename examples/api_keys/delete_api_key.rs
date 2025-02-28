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
    
    // Get the API key ID to delete from command line arguments
    let args: Vec<String> = env::args().collect();
    let key_id = if args.len() > 1 {
        args[1].clone()
    } else {
        // If no key ID is provided, list available keys
        println!("No API key ID provided. Listing available keys...");
        let (keys_response, _) = client.list_api_keys().await?;
        
        println!("\nAvailable API keys:");
        for (i, key) in keys_response.data.iter().enumerate() {
            println!("{}. {} ({})", i + 1, key.name, key.id);
        }
        
        println!("\nUsage: cargo run --example delete_api_key --features examples -- <key_id>");
        return Ok(());
    };
    
    // Confirm deletion
    println!("Are you sure you want to delete API key {}? (y/N)", key_id);
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    
    if input.trim().to_lowercase() != "y" {
        println!("Deletion cancelled.");
        return Ok(());
    }
    
    // Delete the API key
    println!("Deleting API key {}...", key_id);
    let (delete_response, rate_limit) = client.delete_api_key(&key_id).await?;
    
    // Print the result
    if delete_response.deleted {
        println!("API key deleted successfully!");
    } else {
        println!("Failed to delete API key.");
    }
    
    // Print rate limit information
    println!("\nRate limit information:");
    println!("Requests: {}/{}", 
        rate_limit.remaining_requests.unwrap_or(0),
        rate_limit.limit_requests.unwrap_or(0)
    );
    
    Ok(())
}