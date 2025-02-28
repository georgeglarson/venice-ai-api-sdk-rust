use std::error::Error;
use venice_ai_api_sdk_rust::{
    image::{ImageClient, ImageUpscaleBuilder},
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
    
    // Create an image client
    println!("Connecting to Venice.ai API...");
    let image_client = ImageClient::new(api_key)?;
    
    // For this example, we need an image URL to upscale
    // This is a sample image URL (replace with a real one for actual testing)
    let image_url = "https://example.com/sample-image.jpg";
    
    // Build an image upscaling request
    let request = ImageUpscaleBuilder::with_url(
        "upscale-xl", // Model from documentation
        image_url
    )
    .scale(4) // 4x upscaling
    .build();
    
    // Send the request
    println!("\nUpscaling image...");
    println!("Image URL: {}", image_url);
    
    let response = image_client.upscale_image(request).await?;
    
    // Print the response
    println!("\nImage upscaled:");
    if let Some(created) = response.created {
        println!("Created timestamp: {}", created);
    }
    
    for (i, image) in response.data.iter().enumerate() {
        println!("Upscaled image {}:", i + 1);
        
        if let Some(url) = &image.url {
            println!("  URL: {}", url);
        }
    }
    
    println!("\nImage upscaling completed successfully!");
    
    Ok(())
}