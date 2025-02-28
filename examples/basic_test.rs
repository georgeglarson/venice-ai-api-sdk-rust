use std::error::Error;
use venice_ai_api_sdk_rust::{
    Client,
    traits::{
        models::ModelsApi,
        chat::{ChatApi, ChatCompletionBuilder},
    },
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
    
    // Create a unified client
    let client = Client::new(&api_key)?;
    
    // List available models
    println!("Fetching available models...");
    let (models_response, _) = client.list_models().await?;
    
    println!("\nAvailable models:");
    for model in &models_response.data {
        if model.supports_chat_completions {
            println!("- {} (owned by: {})", model.id, model.owned_by);
        }
    }
    
    // Create a simple chat completion request with a small model
    let request = ChatCompletionBuilder::new("llama-3.2-3b") // Using a smaller model for quick testing
        .add_system("You are a helpful assistant that responds with brief answers.")
        .add_user("What is Venice.ai?")
        .max_tokens(100)
        .temperature(0.7)
        .build();
    
    // Send the request
    println!("\nSending a simple chat completion request...");
    let (response, rate_limit) = client.create_chat_completion(request).await?;
    
    // Print the response
    println!("\nResponse from Venice.ai:");
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
    
    Ok(())
}