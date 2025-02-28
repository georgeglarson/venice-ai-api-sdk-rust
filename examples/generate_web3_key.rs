use std::env;
use std::error::Error;
use venice_ai_api_sdk_rust::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Get API key from environment variable
    let api_key = env::var("VENICE_API_KEY").expect("VENICE_API_KEY not set");
    
    // Create a client
    let client = Client::new(&api_key)?;
    
    println!("Venice API SDK Test - Generate Web3 Key");
    println!("Client created successfully with API key: {}", api_key);
    
    Ok(())
}