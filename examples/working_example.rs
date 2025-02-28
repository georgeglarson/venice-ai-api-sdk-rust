use std::error::Error;
use venice_ai_api_sdk_rust::{
    chat::{ChatClient, ChatCompletionBuilder},
    models::ModelsClient,
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
    
    // List available models
    println!("Connecting to Venice.ai API...");
    
    let models_client = ModelsClient::new(api_key)?;
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
    }
    
    // Find a suitable model for chat completions
    let chat_model = models.data.iter()
        .find(|m| m.supports_chat_completions && m.id.contains("llama"))
        .map(|m| m.id.clone())
        .unwrap_or_else(|| "llama-3.2-3b".to_string());
    
    println!("\nUsing model: {}", chat_model);
    
    // Create a chat client
    let chat_client = ChatClient::new(api_key)?;
    
    // Build a chat completion request
    let request = ChatCompletionBuilder::new(chat_model)
        .add_system("You are a helpful assistant. Keep your answers brief and to the point.")
        .add_user("What is Venice.ai and what makes it special?")
        .max_tokens(100)
        .temperature(0.7)
        .build();
    
    // Send the request
    println!("\nSending chat completion request...");
    let response = chat_client.create_chat_completion(request).await?;
    
    // Print the response
    println!("\nAI Response:");
    println!("{}", response.choices[0].message.content);
    
    // Print usage information if available
    if let Some(usage) = response.usage {
        println!("\nToken usage:");
        println!("Prompt tokens: {}", usage.prompt_tokens);
        println!("Completion tokens: {}", usage.completion_tokens);
        println!("Total tokens: {}", usage.total_tokens);
    }
    
    Ok(())
}