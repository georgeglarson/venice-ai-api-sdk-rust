use std::env;
use std::error::Error;
use std::time::Instant;
use futures::StreamExt;
use venice_ai_api_sdk_rust::{
    Client,
    traits::chat::{ChatApi, ChatCompletionBuilder, ChatRole},
    traits::models::ModelsApi,
    VeniceError,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load environment variables from .env file
    if let Err(e) = dotenv::dotenv() {
        println!("Warning: Could not load .env file: {}", e);
    }
    
    // Get API key from environment variable
    let api_key = match env::var("VENICE_API_KEY") {
        Ok(key) => key,
        Err(_) => {
            eprintln!("Error: VENICE_API_KEY environment variable not set");
            eprintln!("Please set it in your .env file or environment");
            return Ok(());
        }
    };
    
    // Create a client with the unified architecture
    let client = Client::new(&api_key)?;
    
    // List available models to find one that supports chat
    println!("Fetching available models...");
    let models_result = client.list_models().await;
    
    let chat_model = match models_result {
        Ok((models_response, _)) => {
            println!("\nAvailable chat models:");
            for model in &models_response.data {
                if model.supports_chat_completions {
                    println!("- {} (owned by: {})", model.id, model.owned_by);
                }
            }
            
            // Find a chat model to use
            models_response.data.iter()
                .find(|m| m.supports_chat_completions)
                .map(|m| m.id.clone())
                .unwrap_or_else(|| "llama-3.3-70b".to_string())
        },
        Err(e) => {
            eprintln!("Warning: Could not fetch models: {}", e);
            eprintln!("Defaulting to llama-3.3-70b");
            "llama-3.3-70b".to_string()
        }
    };
    
    println!("\nUsing chat model: {}", chat_model);
    
    // Create a conversation with multiple messages
    println!("\nCreating a multi-turn conversation...");
    let request = ChatCompletionBuilder::new(&chat_model)
        .add_system("You are a helpful assistant with expertise in Rust programming. Keep your answers concise.")
        .add_user("What are the key features of Rust?")
        .add_message(venice_ai_api_sdk_rust::traits::chat::ChatMessage {
            role: ChatRole::Assistant,
            content: "Rust's key features include memory safety without garbage collection, zero-cost abstractions, fearless concurrency, and a rich type system with pattern matching.".to_string(),
            name: None,
        })
        .add_user("Give me a simple example of using the Result type.")
        .max_tokens(500)
        .temperature(0.7)
        .stream(true) // Enable streaming
        .build();
    
    // Send the streaming request
    println!("\nSending streaming chat completion request...\n");
    println!("Response (streaming):");
    
    // Measure time
    let start_time = Instant::now();
    
    // Track the full content for token counting
    let mut full_content = String::new();
    
    // Try the streaming request
    match client.create_streaming_chat_completion(request).await {
        Ok((stream, rate_limit)) => {
            // Consume the stream
            let mut stream = Box::pin(stream);
            while let Some(chunk_result) = stream.next().await {
                match chunk_result {
                    Ok(chunk) => {
                        // Process each choice in the chunk
                        for choice in &chunk.choices {
                            // Handle role if present (usually only in the first chunk)
                            if let Some(role) = &choice.delta.role {
                                print!("\n{:?}: ", role);
                                std::io::Write::flush(&mut std::io::stdout())?;
                            }
                            
                            // Handle content if present
                            if let Some(content) = &choice.delta.content {
                                print!("{}", content);
                                std::io::Write::flush(&mut std::io::stdout())?;
                                full_content.push_str(content);
                            }
                            
                            // Check if we're done
                            if let Some(reason) = &choice.finish_reason {
                                if reason != "null" {
                                    println!("\n\nFinish reason: {}", reason);
                                }
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("\nError processing chunk: {}", e);
                        break;
                    }
                }
            }
            
            let elapsed = start_time.elapsed();
            println!("\n\nStreaming complete in {:.2} seconds!", elapsed.as_secs_f64());
            
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
            
            // Print approximate token count (rough estimate)
            let approx_tokens = full_content.split_whitespace().count();
            println!("\nApproximate completion tokens: {}", approx_tokens);
            println!("Approximate tokens per second: {:.2}", approx_tokens as f64 / elapsed.as_secs_f64());
        },
        Err(e) => {
            eprintln!("\nError creating streaming chat completion: {}", e);
            
            // Provide more specific error handling
            match e {
                VeniceError::ApiError { status, code, message } => {
                    eprintln!("API Error (Status {}): {} - {}", status, code, message);
                },
                VeniceError::RateLimitExceeded(msg) => {
                    eprintln!("Rate limit exceeded: {}", msg);
                    eprintln!("Please wait before making more requests.");
                },
                VeniceError::HttpError(e) => {
                    eprintln!("HTTP Error: {}", e);
                    eprintln!("Please check your internet connection.");
                },
                _ => {
                    eprintln!("Unexpected error: {}", e);
                }
            }
        }
    }
    
    // Now demonstrate the builder's stream_with_client convenience method
    println!("\n\n=== Using the builder's stream_with_client method ===\n");
    
    // Measure time
    let start_time = Instant::now();
    
    // Use the builder's stream_with_client method
    match ChatCompletionBuilder::new(&chat_model)
        .add_system("You are a helpful assistant. Be very concise.")
        .add_user("Write a haiku about Rust programming.")
        .max_tokens(100)
        .temperature(0.8)
        // No need to call .stream(true) as stream_with_client sets it automatically
        .stream_with_client(&client)
        .await
    {
        Ok((stream, rate_limit)) => {
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
                        eprintln!("\nError processing chunk: {}", e);
                        break;
                    }
                }
            }
            
            let elapsed = start_time.elapsed();
            println!("\n\nStreaming complete in {:.2} seconds!", elapsed.as_secs_f64());
            
            // Print rate limit information
            println!("\nRate limit information:");
            println!("Requests: {}/{}", 
                rate_limit.remaining_requests.unwrap_or(0),
                rate_limit.limit_requests.unwrap_or(0)
            );
        },
        Err(e) => {
            eprintln!("\nError with stream_with_client: {}", e);
        }
    }
    
    Ok(())
}