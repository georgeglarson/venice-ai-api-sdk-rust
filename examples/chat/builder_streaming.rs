use std::env;
use std::error::Error;
use futures::StreamExt;
use venice_ai_api_sdk_rust::{
    Client,
    traits::chat::{ChatCompletionBuilder},
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
    
    // Find a chat model to use
    let chat_model = models_response.data.iter()
        .find(|m| m.supports_chat_completions)
        .map(|m| m.id.clone())
        .unwrap_or_else(|| "llama-3.3-70b".to_string());
    
    println!("\nUsing chat model: {}", chat_model);
    
    // Create a chat completion request and stream it directly using the builder
    println!("\nSending streaming chat completion request...\n");
    
    // Use the builder's stream_with_client method
    let (stream, rate_limit) = ChatCompletionBuilder::new(chat_model)
        .add_system("You are a helpful assistant with expertise in Rust programming.")
        .add_user("Write a short poem about Rust programming language.")
        .max_tokens(500)
        .temperature(0.8)
        // No need to call .stream(true) as stream_with_client sets it automatically
        .stream_with_client(&client)
        .await?;
    
    // Process the stream
    println!("Response (streaming):");
    
    // Consume the stream
    let mut stream = Box::pin(stream);
    while let Some(chunk_result) = stream.next().await {
        match chunk_result {
            Ok(chunk) => {
                // Process each choice in the chunk
                for choice in &chunk.choices {
                    if let Some(content) = &choice.delta.content {
                        print!("{}", content);
                        std::io::Write::flush(&mut std::io::stdout())?;
                    }
                }
            }
            Err(e) => {
                eprintln!("Error processing chunk: {}", e);
                break;
            }
        }
    }
    
    println!("\n\nStreaming complete!");
    
    // Print rate limit information
    println!("\nRate limit information:");
    println!("Requests: {}/{}", 
        rate_limit.remaining_requests.unwrap_or(0),
        rate_limit.limit_requests.unwrap_or(0)
    );
    
    Ok(())
}