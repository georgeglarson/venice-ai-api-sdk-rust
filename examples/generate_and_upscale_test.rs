use std::error::Error;
use venice_ai_api_sdk_rust::{
    image::{ImageClient, ImageGenerateBuilder, ImageUpscaleBuilder},
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
    
    // STEP 1: Generate an image
    println!("\n=== STEP 1: Generating an image ===");
    
    // Build an image generation request
    let gen_request = ImageGenerateBuilder::new(
        "fluently-xl", // Using a model from documentation
        "A simple sketch of a cat" // Simpler prompt for faster generation
    )
    .width(256)  // Using smaller dimensions for faster generation
    .height(256)
    .steps(10)   // Fewer steps for faster generation
    .build();
    
    // Send the request
    println!("Generating image...");
    let gen_response = image_client.generate_image(gen_request).await?;
    
    // Check if we have any generated images
    if gen_response.data.is_empty() {
        return Err("No images were generated".into());
    }
    
    println!("Image generated successfully!");
    
    // In the new API format, we get base64 image data directly, not URLs
    // Let's check if we have any image data to work with
    if gen_response.images.is_empty() {
        return Err("No images were generated".into());
    }
    
    // Get the base64 data from the first image
    let image_data = &gen_response.images[0];
    println!("Got base64 image data of length: {} characters", image_data.len());
    
    // STEP 2: Upscale the generated image
    println!("\n=== STEP 2: Upscaling the generated image ===");
    
    // Build an image upscaling request using the image data from generation
    let upscale_request = ImageUpscaleBuilder::with_data(
        "upscale-xl", // Model from documentation
        image_data.clone()
    )
    .scale(2) // Must be either 2 or 4 for the API
    .build();
    
    // Send the upscale request
    println!("Upscaling image...");
    let upscale_response = image_client.upscale_image(upscale_request).await?;
    
    // Print the upscale response
    println!("Image upscaled successfully!");
    println!("Received {} bytes of upscaled image data", upscale_response.image_data.len());
    println!("MIME type: {}", upscale_response.mime_type);
    
    // In a real application, you could save the image to a file
    // For this test, we just confirm we received the data
    
    println!("\nImage generation and upscaling test completed successfully!");
    
    Ok(())
}