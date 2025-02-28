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
    
    // List available models first to get a model ID
    println!("Fetching available models...");
    let (models_response, _) = client.list_models().await?;
    
    // Find a model that supports chat completions
    let model_id = models_response.data.iter()
        .find(|m| m.supports_chat_completions)
        .map(|m| m.id.clone())
        .unwrap_or_else(|| "llama-3.3-70b".to_string());
    
    println!("\nFetching traits for model: {}", model_id);
    let (traits_response, rate_limit) = client.get_model_traits(&model_id).await?;
    
    println!("\nModel traits:");
    for trait_name in &traits_response.traits {
        println!("- {}", trait_name);
    }
    
    // Check if the model is compatible with specific features
    let features_to_check = ["chat", "streaming", "vision"];
    
    println!("\nFeature compatibility:");
    for feature in features_to_check {
        let is_compatible = client.is_model_compatible(&model_id, feature).await?;
        println!("- {}: {}", feature, if is_compatible { "Yes" } else { "No" });
    }
    
    // Print rate limit information
    println!("\nRate limit information:");
    println!("Requests: {}/{}", 
        rate_limit.remaining_requests.unwrap_or(0),
        rate_limit.limit_requests.unwrap_or(0)
    );
    
    Ok(())
}