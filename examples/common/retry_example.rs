use std::env;
use std::error::Error;
use venice_ai_api_sdk_rust::{
    Client, RetryConfig,
    chat::ChatCompletionRequestBuilder,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Get API key from environment variable
    let api_key = env::var("VENICE_API_KEY").expect("VENICE_API_KEY environment variable not set");
    
    println!("Example: Using Retry Logic");
    println!("-------------------------");
    
    // Create a retry configuration
    let retry_config = RetryConfig::new()
        .max_retries(3)
        .initial_delay_ms(500)
        .max_delay_ms(5000)
        .backoff_factor(2.0)
        .add_jitter(true);
    
    println!("Retry configuration:");
    println!("  Max retries: {}", retry_config.max_retries);
    println!("  Initial delay: {}ms", retry_config.initial_delay_ms);
    println!("  Max delay: {}ms", retry_config.max_delay_ms);
    println!("  Backoff factor: {}", retry_config.backoff_factor);
    println!("  Add jitter: {}", retry_config.add_jitter);
    println!();
    
    // Create a client with retry configuration
    let client = Client::new(api_key)?
        .with_retry_config(retry_config);
    
    println!("Creating a chat completion with retry logic...");
    
    // Create a request
    let request = ChatCompletionRequestBuilder::new("llama-3.3-70b")
        .add_system_message("You are a helpful assistant.")
        .add_user_message("What are the benefits of using retry logic in API clients?")
        .with_max_tokens(200)
        .with_temperature(0.7)
        .build();
    
    // Send the request with retry logic
    let (response, _) = client.create_chat_completion(request).await?;
    
    println!("Response received successfully!");
    println!("Response: {}", response.choices[0].message.content);
    
    // You can also use the builder pattern to create a client with retries
    println!("\nCreating a client with the builder pattern...");
    let _client = Client::builder()
        .api_key(env::var("VENICE_API_KEY").expect("VENICE_API_KEY environment variable not set"))
        .with_retries()  // Use default retry configuration
        .build()?;
    
    println!("Client created with default retry configuration!");
    
    Ok(())
}