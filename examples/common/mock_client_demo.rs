use std::env;
use std::error::Error;
use venice_ai_api_sdk_rust::{
    Client,
    traits::chat::ChatCompletionBuilder,
};

// This is a mock example that doesn't make actual API calls
// It's useful for testing the SDK structure without a real API key
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("=== Venice AI API SDK Mock Client Demo ===");
    println!("This example demonstrates the SDK structure without making actual API calls.\n");
    
    // Create a client with a mock API key
    let api_key = env::var("VENICE_API_KEY").unwrap_or_else(|_| "mock_api_key".to_string());
    println!("Using API key: {}", api_key);
    
    // Create a client
    match Client::new(&api_key) {
        Ok(_) => {
            println!("✓ Successfully created client");
        },
        Err(e) => {
            println!("✗ Failed to create client: {}", e);
            return Ok(());
        }
    };
    
    // Demonstrate the models API structure
    println!("\n=== Models API ===");
    println!("In a real scenario, client.list_models() would return a list of available models");
    println!("Example model data structure:");
    println!("```");
    println!("Model {{");
    println!("  id: \"llama-3.3-70b\",");
    println!("  owned_by: \"venice\",");
    println!("  supports_chat_completions: true,");
    println!("  supports_image_generation: false,");
    println!("  context_size: Some(128000),");
    println!("}}");
    println!("```");
    
    // Demonstrate the chat API structure
    println!("\n=== Chat API ===");
    println!("In a real scenario, you would create a chat completion request like this:");
    
    let chat_request = ChatCompletionBuilder::new("llama-3.3-70b")
        .add_system("You are a helpful assistant.")
        .add_user("Hello, how are you?")
        .max_tokens(100)
        .temperature(0.7)
        .build();
    
    println!("Chat request structure:");
    println!("- Model: {}", chat_request.model);
    println!("- Messages: {} message(s)", chat_request.messages.len());
    println!("- Max tokens: {}", chat_request.max_tokens.unwrap_or(100));
    println!("- Temperature: {}", chat_request.temperature.unwrap_or(0.7));
    
    println!("\n=== Demo Complete ===");
    println!("This mock demo shows the SDK structure without making actual API calls.");
    println!("To use the real API, provide a valid Venice API key.");
    
    Ok(())
}