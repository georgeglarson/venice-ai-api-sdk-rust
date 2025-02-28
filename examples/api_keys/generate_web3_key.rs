use std::env;
use std::error::Error;
use venice_ai_api_sdk_rust::{
    Client,
    traits::api_keys::{ApiKeysApi, GenerateWeb3KeyRequest},
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
    
    // Get the Ethereum address from command line arguments
    let args: Vec<String> = env::args().collect();
    let eth_address = if args.len() > 1 {
        args[1].clone()
    } else {
        println!("No Ethereum address provided.");
        println!("Usage: cargo run --example generate_web3_key --features examples -- <ethereum_address>");
        return Ok(());
    };
    
    // Create a request to generate a Web3 key
    let request = GenerateWeb3KeyRequest {
        address: eth_address.clone(),
    };
    
    // Send the request
    println!("Generating Web3 key for address {}...", eth_address);
    let (response, rate_limit) = client.generate_web3_key(request).await?;
    
    // Print the result
    println!("\nWeb3 key generated successfully!");
    println!("Key details:");
    println!("- Message to sign: {}", response.message);
    println!("- Expires at: {}", response.expires_at);
    
    println!("\nInstructions:");
    println!("1. Sign this message with your Ethereum wallet");
    println!("2. Use the signature to authenticate with the Venice API");
    
    // Print rate limit information
    println!("\nRate limit information:");
    println!("Requests: {}/{}", 
        rate_limit.remaining_requests.unwrap_or(0),
        rate_limit.limit_requests.unwrap_or(0)
    );
    
    Ok(())
}