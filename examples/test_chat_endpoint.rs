use std::error::Error;
use venice_ai_api_sdk_rust::chat::{ChatClient, ChatCompletionBuilder};

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
    
    println!("Testing Chat Completions API Endpoint");
    println!("====================================");
    
    // Create a chat client
    println!("Creating ChatClient...");
    let chat_client = ChatClient::new(api_key)?;
    
    // Build a simple chat completion request
    println!("Building chat completion request...");
    let request = ChatCompletionBuilder::new("llama-3.2-3b") // Using a smaller model for faster results
        .add_system("You are a helpful assistant. Keep your answers brief and to the point.")
        .add_user("What's the capital of France?")
        .max_tokens(50)  // Keeping it short for testing
        .temperature(0.7)
        .build();
    
    // Send the request
    println!("Sending chat completion request...");
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
    
    println!("\nChat API test completed successfully!");
    
    Ok(())
}