use std::error::Error;
use std::time::Duration;
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
    
    if api_key == "your_api_key_here" {
        println!("Error: You're using the placeholder API key.");
        println!("Please replace 'your_api_key_here' in the .env file with your actual Venice API key.");
        return Ok(());
    }
    
    // Create a models client
    let models_client = match ModelsClient::new(&api_key) {
        Ok(client) => client,
        Err(e) => {
            println!("Error creating client: {}", e);
            return Ok(());
        }
    };
    
    // List available models with timeout
    println!("Connecting to Venice.ai API...");
    
    // Use tokio timeout to prevent hanging indefinitely
    let models_response = match tokio::time::timeout(
        Duration::from_secs(10),
        models_client.list_models()
    ).await {
        Ok(result) => match result {
            Ok(response) => response,
            Err(e) => {
                println!("API Error: {}", e);
                println!("This could be due to an invalid API key or network issues.");
                return Ok(());
            }
        },
        Err(_) => {
            println!("Error: Request timed out after 10 seconds.");
            println!("This could be due to network issues or an invalid API key.");
            return Ok(());
        }
    };
    
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
    
    println!("\nAPI key loaded successfully from .env file!");
    println!("Note: If you're seeing this message without any models listed, your API key may be invalid.");
    println!("Please check your .env file and make sure you've replaced the placeholder with a valid API key.");
    
    Ok(())
}