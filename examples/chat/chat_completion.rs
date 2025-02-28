use std::env;
use std::error::Error;
use venice_ai_api_sdk_rust::{
    Client,
    traits::chat::{ChatApi, ChatCompletionBuilder},
    traits::models::ModelsApi,
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
    
    // List available models to find one that supports chat
    println!("Fetching available models...");
    let (models_response, _) = client.list_models().await?;
    
    println!("\nAvailable chat models:");
    for model in &models_response.data {
        if model.supports_chat_completions {
            println!("- {} (owned by: {})", model.id, model.owned_by);
        }
    }
    
    // Find a chat model to use
    let chat_model = models_response.data.iter()
        .find(|m| m.supports_chat_completions)
        .map(|m| m.id.clone())
        .unwrap_or_else(|| "llama-3.3-70b".to_string());
    
    println!("\nUsing chat model: {}", chat_model);
    
    // Create a chat completion request
    let request = ChatCompletionBuilder::new(chat_model)
        .add_system("You are a helpful assistant with expertise in Rust programming.")
        .add_user("Explain how to use async/await in Rust with a simple example.")
        .max_tokens(1000)
        .temperature(0.7)
        .build();
    
    // Send the request
    println!("\nSending chat completion request...");
    let (response, rate_limit) = client.create_chat_completion(request).await?;
    
    // Print the response
    println!("\nResponse:");
    println!("{}", response.choices[0].message.content);
    
    // Print usage information
    if let Some(usage) = response.usage {
        println!("\nToken usage:");
        println!("Prompt tokens: {}", usage.prompt_tokens);
        println!("Completion tokens: {}", usage.completion_tokens);
        println!("Total tokens: {}", usage.total_tokens);
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