use std::env;
use std::error::Error;
use venice_ai_api_sdk_rust::{
    Client,
    traits::{
        models::ModelsApi,
        chat::{ChatApi, ChatCompletionBuilder},
        image::{ImageApi},
        api_keys::ApiKeysApi,
    },
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Get API key from environment variable
    let api_key = env::var("VENICE_API_KEY").expect("VENICE_API_KEY not set");
    
    // Create a single unified client
    let client = Client::new(api_key)?;
    
    println!("Venice API SDK - Unified Client Example");
    println!("=======================================\n");
    
    // === Models API ===
    println!("1. Listing available models...");
    let (models_response, rate_limit) = client.list_models().await?;
    
    println!("Available models:");
    for model in &models_response.data {
        println!("- {} (owned by: {})", model.id, model.owned_by);
        println!("  Supports: chat={}, images={}, streaming={}",
            model.supports_chat_completions,
            model.supports_image_generation,
            model.supports_streaming
        );
    }
    println!("Rate limit info: {}/{} requests remaining", 
        rate_limit.remaining_requests.unwrap_or(0),
        rate_limit.limit_requests.unwrap_or(0)
    );
    println!();
    
    // === Chat API ===
    println!("2. Creating a chat completion...");
    
    // Find a chat model
    let chat_model = models_response.data.iter()
        .find(|m| m.supports_chat_completions)
        .map(|m| m.id.clone())
        .unwrap_or_else(|| "llama-3.3-70b".to_string());
    
    println!("Using chat model: {}", chat_model);
    
    // Create a chat completion request
    let request = ChatCompletionBuilder::new(chat_model)
        .add_system("You are a helpful assistant with expertise in Rust programming.")
        .add_user("Explain how to use async/await in Rust with a simple example.")
        .max_tokens(1000)
        .temperature(0.7)
        .build();
    
    // Send the request
    let (response, rate_limit) = client.create_chat_completion(request).await?;
    
    // Print the response
    println!("\nChat response:");
    println!("{}", response.choices[0].message.content);
    
    // Print usage information
    if let Some(usage) = response.usage {
        println!("\nToken usage:");
        println!("Prompt tokens: {}", usage.prompt_tokens);
        println!("Completion tokens: {}", usage.completion_tokens);
        println!("Total tokens: {}", usage.total_tokens);
    }
    
    println!("Rate limit info: {}/{} tokens remaining", 
        rate_limit.remaining_tokens.unwrap_or(0),
        rate_limit.limit_tokens.unwrap_or(0)
    );
    println!();
    
    // === Image API ===
    println!("3. Listing available image styles...");
    let (styles_response, _) = client.list_styles().await?;
    
    println!("Available styles:");
    for style in &styles_response.styles {
        println!("- {}", style);
    }
    println!();
    
    // === API Keys API ===
    println!("4. Listing API keys...");
    let (keys_response, _) = client.list_api_keys().await?;
    
    println!("API keys:");
    for key in &keys_response.data {
        println!("- {} ({}): prefix={}, active={}", 
            key.id, 
            key.name, 
            key.prefix,
            key.active
        );
    }
    
    println!("\nAll operations completed successfully!");
    Ok(())
}