use std::error::Error;
use venice_ai_api_sdk_rust::VerySimpleClient;

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
    
    // Create the very simple client
    let client = VerySimpleClient::new(api_key)?;
    
    // List available models
    println!("Fetching available models...");
    let models_json = client.list_models().await?;
    
    // Print the result
    println!("\nModels available on Venice.ai:");
    println!("{}", models_json);
    
    Ok(())
}