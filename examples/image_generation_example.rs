use std::error::Error;
use venice_ai_api_sdk_rust::{
    image::{ImageClient, ImageGenerateBuilder},
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
    
    // First, let's list available image styles
    println!("Fetching available image styles...");
    let styles_response = image_client.list_styles().await?;
    
    println!("\nAvailable image styles:");
    for style in &styles_response.styles {
        println!("- {}", style);
    }
    
    // Pick a style to use (or default to "3D Model" if none are available)
    let style_preset = styles_response.styles.first()
        .map(|s| s.clone())
        .unwrap_or_else(|| "3D Model".to_string());
    
    println!("\nUsing style preset: {}", style_preset);
    
    // Build an image generation request
    let request = ImageGenerateBuilder::new(
        "fluently-xl", // Using the model from documentation
        "A serene mountain lake at sunset with reflections in the water"
    )
    .style_preset(style_preset)
    .width(1024)
    .height(1024)
    .steps(30)
    .cfg_scale(7.5)
    .seed(12345)
    .build();
    
    // Send the request
    println!("\nGenerating image...");
    let response = image_client.generate_image(request).await?;
    
    // Print the response
    println!("\nImage generated:");
    if let Some(created) = response.created {
        println!("Created timestamp: {}", created);
    }
    
    for (i, image) in response.data.iter().enumerate() {
        println!("Image {}:", i + 1);
        
        if let Some(url) = &image.url {
            println!("  URL: {}", url);
        }
        
        if let Some(seed) = image.seed {
            println!("  Seed: {}", seed);
        }
        
        if let Some(revised_prompt) = &image.revised_prompt {
            println!("  Revised prompt: {}", revised_prompt);
        }
    }
    
    println!("\nImage generation completed successfully!");
    
    Ok(())
}