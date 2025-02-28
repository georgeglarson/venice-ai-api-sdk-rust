//! Example demonstrating rate limit handling for Venice.ai API
//!
//! This example shows how to use the rate limiter to automatically handle rate limits
//! when making requests to the Venice.ai API.

use std::env;
use venice_ai_api_sdk_rust::{
    Client,
    traits::chat::{ChatApi, ChatCompletionBuilder},
    rate_limit::{RateLimiter, RateLimiterConfig},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load API key from environment variable
    let api_key = env::var("VENICE_API_KEY")
        .expect("VENICE_API_KEY environment variable not set");
    
    // Create a client with rate limiting enabled
    let client = Client::builder()
        .api_key(api_key)
        .with_rate_limiting() // Enable rate limiting with default configuration
        .build()?;
    
    println!("Making multiple requests with automatic rate limiting...");
    
    // Make multiple requests in a loop
    for i in 1..=5 {
        println!("\nRequest #{}", i);
        
        // Build a chat completion request
        let request = ChatCompletionBuilder::new("llama-3.1-8b")
            .add_system("You are a helpful assistant.")
            .add_user("Give me a one-sentence fact about rate limiting.")
            .max_tokens(50)
            .build();
        
        // Send the request - the rate limiter will automatically wait if needed
        let (response, rate_limit_info) = client.create_chat_completion(request).await?;
        
        // Print the response
        println!("Response: {}", response.choices[0].message.content.trim());
        
        // Print rate limit information
        println!("Rate limit info:");
        println!("  Requests: {}/{}", 
            rate_limit_info.remaining_requests.unwrap_or(0),
            rate_limit_info.limit_requests.unwrap_or(0));
        println!("  Tokens: {}/{}", 
            rate_limit_info.remaining_tokens.unwrap_or(0),
            rate_limit_info.limit_tokens.unwrap_or(0));
        
        if let Some(reset) = rate_limit_info.reset_requests {
            println!("  Reset time: {} seconds", reset);
        }
    }
    
    println!("\n--- Custom Rate Limiter Configuration ---");
    
    // Create a client with custom rate limiting configuration
    let custom_config = RateLimiterConfig {
        auto_wait: true,
        max_wait_time: 30, // Maximum wait time of 30 seconds
    };
    
    let client = Client::builder()
        .api_key(env::var("VENICE_API_KEY").unwrap())
        .with_rate_limiting_config(custom_config)
        .build()?;
    
    println!("Client created with custom rate limiter configuration");
    println!("- auto_wait: true");
    println!("- max_wait_time: 30 seconds");
    
    // Make a request with the custom rate limiter
    let request = ChatCompletionBuilder::new("llama-3.1-8b")
        .add_system("You are a helpful assistant.")
        .add_user("Explain rate limiting in one sentence.")
        .max_tokens(50)
        .build();
    
    let (response, _) = client.create_chat_completion(request).await?;
    println!("\nResponse with custom rate limiter: {}", response.choices[0].message.content.trim());
    
    Ok(())
}