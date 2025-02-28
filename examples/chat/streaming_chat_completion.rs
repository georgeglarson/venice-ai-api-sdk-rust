use futures::StreamExt;
use std::env;
use std::error::Error;
use venice_ai_api_sdk_rust::{
    Client,
    chat::ChatCompletionRequestBuilder,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Get API key from environment variable
    let api_key = env::var("VENICE_API_KEY").expect("VENICE_API_KEY environment variable not set");
    
    // Create a client
    let client = Client::new(api_key)?;
    
    println!("Example: Streaming Chat Completion");
    println!("----------------------------------");
    
    // Create a request with streaming enabled
    let request = ChatCompletionRequestBuilder::new("llama-3.3-70b")
        .add_system_message("You are a helpful assistant.")
        .add_user_message("Explain quantum computing in 5 simple sentences.")
        .with_max_tokens(200)
        .with_temperature(0.7)
        .with_streaming(true)
        .build();
    
    // Get a streaming response
    let (mut stream, _) = client.create_streaming_chat_completion(request).await?;
    
    println!("Response (streaming):");
    
    // Process each chunk as it arrives
    let mut full_content = String::new();
    while let Some(chunk_result) = stream.next().await {
        match chunk_result {
            Ok(chunk) => {
                for choice in &chunk.choices {
                    if let Some(content) = &choice.delta.content {
                        print!("{}", content);
                        std::io::Write::flush(&mut std::io::stdout())?;
                        full_content.push_str(content);
                    }
                }
            }
            Err(err) => eprintln!("Error: {}", err),
        }
    }
    
    println!("\n\nFull response collected from stream:");
    println!("{}", full_content);
    
    Ok(())
}