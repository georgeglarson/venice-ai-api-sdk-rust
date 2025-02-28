use std::env;
use std::error::Error;
use venice_ai_api_sdk_rust::{
    Client,
    traits::chat::{ChatApi, ChatCompletionBuilder},
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
    
    // Define a base model and features to test
    let base_model = "llama-3.3-70b";
    let features = vec!["", "vision", "instruct"];
    
    println!("Testing model feature suffixes with base model: {}", base_model);
    
    for feature in features {
        // Construct model name with feature suffix
        let model_name = if feature.is_empty() {
            base_model.to_string()
        } else {
            format!("{}-{}", base_model, feature)
        };
        
        println!("\nTesting model: {}", model_name);
        
        // Create a simple request
        let request = ChatCompletionBuilder::new(model_name.clone())
            .add_user("What can you do?")
            .max_tokens(100)
            .build();
        
        // Send the request
        match client.create_chat_completion(request).await {
            Ok((response, _)) => {
                println!("✓ Success! Model responded:");
                println!("  {}", response.choices[0].message.content
                    .lines()
                    .next()
                    .unwrap_or_default()
                    .chars()
                    .take(80)
                    .collect::<String>()
                    + "..."
                );
            },
            Err(e) => {
                println!("✗ Error: {}", e);
            }
        }
    }
    
    Ok(())
}