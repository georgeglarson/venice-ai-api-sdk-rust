use std::env;
use std::error::Error;
use venice_ai_api_sdk_rust::{Client, traits::models::ModelsApi};

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
    
    // List available models
    println!("Fetching available models...");
    let (models_response, rate_limit) = client.list_models().await?;
    
    println!("\nAvailable models:");
    for model in &models_response.data {
        println!("- {} (owned by: {})", model.id, model.owned_by);
        
        if model.supports_chat_completions {
            println!("  Supports chat completions: Yes");
        }
        
        if model.supports_image_generation {
            println!("  Supports image generation: Yes");
        }
        
        if let Some(context_size) = model.context_size {
            println!("  Context size: {}", context_size);
        }
    }
    
    // Print rate limit information
    println!("\nRate limit information:");
    println!("Requests: {}/{}", 
        rate_limit.remaining_requests.unwrap_or(0),
        rate_limit.limit_requests.unwrap_or(0)
    );
    println!("Tokens: {}/{}", 
        rate_limit.remaining_tokens.unwrap_or(0),
        rate_limit.limit_tokens.unwrap_or(0)
    );
    
    Ok(())
}