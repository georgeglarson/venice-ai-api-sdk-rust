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
    let (models_response, _) = client.list_models().await?;
    
    // Define features to check for compatibility
    let features = vec!["chat", "streaming", "vision", "image-generation"];
    
    println!("\nModel Compatibility Matrix:");
    println!("{:<20} {}", "Model", features.join(" | "));
    println!("{}", "-".repeat(20 + features.len() * 10));
    
    // Check compatibility for each model
    for model in &models_response.data {
        let mut compatibility_row = format!("{:<20}", model.id);
        
        for feature in &features {
            let is_compatible = client.is_model_compatible(&model.id, feature).await?;
            let marker = if is_compatible { "✓" } else { "✗" };
            compatibility_row.push_str(&format!(" {:<8} |", marker));
        }
        
        println!("{}", compatibility_row);
    }
    
    Ok(())
}