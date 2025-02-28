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
    
    // Get the list of models
    println!("Fetching available models...");
    let (models_response, _) = client.list_models().await?;
    
    println!("\nAvailable models:");
    for model in &models_response.data {
        println!("- {} (owned by: {})", model.id, model.owned_by);
    }
    
    // Send a simple chat message
    println!("\nSending a simple chat message...");
    let request = ChatCompletionBuilder::new("llama-3.2-3b")
        .add_user("What is Venice.ai?")
        .max_tokens(100)
        .build();
        
    let (response, _) = client.create_chat_completion(request).await?;
    
    println!("\nResponse:");
    println!("{}", response.choices[0].message.content);
    
    Ok(())
}