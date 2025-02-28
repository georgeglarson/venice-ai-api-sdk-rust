use std::env;
use std::error::Error;
use venice_ai_api_sdk_rust::{
    Client,
    traits::image::ImageApi,
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
    
    // List available image styles
    println!("Fetching available image styles...");
    let (styles_response, rate_limit) = client.list_styles().await?;
    
    println!("\nAvailable image styles:");
    for (i, style) in styles_response.styles.iter().enumerate() {
        println!("{}. {}", i + 1, style);
    }
    
    // Print rate limit information
    println!("\nRate limit information:");
    println!("Requests: {}/{}", 
        rate_limit.remaining_requests.unwrap_or(0),
        rate_limit.limit_requests.unwrap_or(0)
    );
    
    // Provide usage examples
    println!("\nUsage example:");
    println!("When generating images, you can use these styles with the style_preset parameter:");
    println!("```rust");
    println!("let request = ImageGenerateBuilder::new(");
    println!("    \"fluently-xl\",");
    println!("    \"A beautiful landscape\"");
    println!(")");
    println!(".style_preset(\"{}\")", 
        styles_response.styles.first().unwrap_or(&"3D Model".to_string())
    );
    println!(".build();");
    println!("```");
    
    Ok(())
}