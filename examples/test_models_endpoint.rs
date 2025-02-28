use std::error::Error;
use venice_ai_api_sdk_rust::models::ModelsClient;

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
    
    println!("Testing Models API Endpoint");
    println!("===========================");
    
    // Create a models client
    println!("Creating ModelsClient...");
    let models_client = ModelsClient::new(api_key)?;
    
    // List available models
    println!("Fetching available models...");
    let models = models_client.list_models().await?;
    
    println!("\nAvailable models:");
    for model in &models.data {
        println!("- {} (owned by: {})", model.id, model.owned_by);
        
        if model.supports_chat_completions {
            println!("  Supports chat completions: Yes");
        }
        
        if let Some(context_size) = model.context_size {
            println!("  Context size: {}", context_size);
        }
        
        println!();
    }
    
    println!("Models API test completed successfully!");
    
    Ok(())
}