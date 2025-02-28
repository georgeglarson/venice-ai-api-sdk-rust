use std::env;
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
    // Get API key from environment variable
    let api_key = env::var("VENICE_API_KEY").expect("VENICE_API_KEY not set");
    
    // Create a unified client
    let client = Client::new(&api_key)?;
    
    // List available models
    let (models_response, _) = client.list_models().await?;
    println!("Available models:");
    for model in &models_response.data {
        if model.supports_chat_completions {
            println!("- {} (owned by: {})", model.id, model.owned_by);
        }
    }
    println!();
    
    // Create a chat completion request
    let request = ChatCompletionBuilder::new("llama-3.3-70b")
        .add_system("You are a helpful assistant with expertise in Rust programming.")
        .add_user("Explain how to use async/await in Rust with a simple example.")
        .max_tokens(1000)
        .temperature(0.7)
        .build();
    
    // Send the request
    println!("Sending chat completion request...");
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
    
    Ok(())
}