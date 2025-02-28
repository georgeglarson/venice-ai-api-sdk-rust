use std::env;
use std::error::Error;
use venice_ai_api_sdk_rust::{
    Client,
    traits::image::{ImageApi, ImageGenerateBuilder},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Get API key from environment variable
    let api_key = env::var("VENICE_API_KEY").expect("VENICE_API_KEY not set");
    
    // Create a unified client
    let client = Client::new(&api_key)?;
    
    // List available image styles
    let (styles_response, _) = client.list_styles().await?;
    println!("Available image styles:");
    for style in &styles_response.styles {
        println!("- {}", style);
    }
    println!();
    
    // Create an image generation request
    let request = ImageGenerateBuilder::new(
        "fluently-xl",
        "A stunning sunset over a serene mountain lake, with vibrant colors reflecting in the water",
    )
    .negative_prompt("clouds, people, text, watermark")
    .style_preset("3D Model")  // Use one of the available style presets
    .width(1024)
    .height(1024)
    .steps(30)
    .cfg_scale(7.5)
    .seed(12345)  // Use a specific seed for reproducible results
    .safe_mode(false)
    .build();
    
    // Send the request
    println!("Generating image...");
    let (response, rate_limit) = client.generate_image(request).await?;
    
    // Print the response
    println!("\nImage(s) generated:");
    for (i, image) in response.data.iter().enumerate() {
        println!("Image {}:", i + 1);
        if let Some(url) = &image.url {
            println!("URL: {}", url);
        }
        if let Some(b64) = &image.b64_json {
            println!("Base64 data available (length: {})", b64.len());
        }
        if let Some(seed) = image.seed {
            println!("Seed used: {}", seed);
        }
        if let Some(revised_prompt) = &image.revised_prompt {
            println!("Revised prompt: {}", revised_prompt);
        }
    }
    
    // Print rate limit information
    println!("\nRate limit information:");
    println!("Requests: {}/{}",
        rate_limit.remaining_requests.unwrap_or(0),
        rate_limit.limit_requests.unwrap_or(0)
    );
    
    Ok(())
}