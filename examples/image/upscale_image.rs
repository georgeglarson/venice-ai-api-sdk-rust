use std::env;
use std::error::Error;
use std::fs;
use std::path::Path;
use venice_ai_api_sdk_rust::{
    Client,
    traits::image::{ImageApi, ImageUpscaleBuilder},
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
    
    // You can upscale an image from a URL
    let image_url = "https://example.com/image.jpg";
    println!("Upscaling image from URL: {}", image_url);
    
    // Create an upscale request with URL
    let request = ImageUpscaleBuilder::with_url(
        "upscale-xl",
        image_url
    )
    .scale(4)  // 4x upscaling
    .build();
    
    // Send the request
    println!("\nSending upscale request...");
    match client.upscale_image(request).await {
        Ok(response) => {
            println!("Image upscaled successfully!");
            println!("Received {} bytes of {} data", 
                response.image_data.len(), 
                response.mime_type
            );
            
            // Save the upscaled image to a file
            let output_path = "upscaled_image.png";
            fs::write(Path::new(output_path), &response.image_data)?;
            println!("Saved upscaled image to: {}", output_path);
            
            // For backward compatibility, the response also includes base64 data
            if let Some(upscaled_data) = response.data.first() {
                if let Some(b64) = &upscaled_data.b64_json {
                    println!("Base64 data available: {} bytes", b64.len());
                }
            }
        },
        Err(e) => {
            println!("Error upscaling image: {}", e);
            println!("Note: This example requires a valid image URL to work.");
            println!("You can also upscale an image from base64 data:");
            
            // Example of upscaling from base64 data
            println!("\nAlternative: Upscaling from base64 data");
            println!("(This is just a demonstration, not actually sending a request)");
            
            let _request = ImageUpscaleBuilder::with_data(
                "upscale-xl",
                "base64_encoded_image_data_here"
            )
            .scale(2)  // 2x upscaling
            .build();
            
            println!("To use this approach, you would need to:");
            println!("1. Read an image file into memory");
            println!("2. Convert it to base64");
            println!("3. Pass it to the with_data method");
        }
    }
    
    Ok(())
}