use std::env;
use std::error::Error;
use venice_ai_api_sdk_rust::{
    Client,
    traits::image::{ImageApi, ImageGenerateBuilder},
    traits::models::ModelsApi,
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
    
    // List available models to find one that supports image generation
    println!("Fetching available models...");
    let (models_response, _) = client.list_models().await?;
    
    println!("\nAvailable image generation models:");
    for model in &models_response.data {
        if model.supports_image_generation {
            println!("- {} (owned by: {})", model.id, model.owned_by);
        }
    }
    
    // Find an image generation model to use
    let image_model = models_response.data.iter()
        .find(|m| m.supports_image_generation)
        .map(|m| m.id.clone())
        .unwrap_or_else(|| "fluently-xl".to_string());
    
    println!("\nUsing image model: {}", image_model);
    
    // List available styles
    println!("\nFetching available styles...");
    let (styles_response, _) = client.list_styles().await?;
    
    println!("\nAvailable styles:");
    for (i, style) in styles_response.styles.iter().enumerate().take(5) {
        println!("- {}", style);
    }
    if styles_response.styles.len() > 5 {
        println!("... and {} more", styles_response.styles.len() - 5);
    }
    
    // Choose a style
    let style = styles_response.styles.first()
        .cloned()
        .unwrap_or_else(|| "3D Model".to_string());
    
    // Create an image generation request
    let request = ImageGenerateBuilder::new(
        image_model,
        "A beautiful sunset over a mountain range with a lake in the foreground"
    )
    .negative_prompt("clouds, people, text, watermark")
    .style_preset(&style)
    .width(1024)
    .height(1024)
    .steps(30)
    .cfg_scale(7.5)
    .seed(12345)  // Use a specific seed for reproducible results
    .safe_mode(false)
    .build();
    
    // Send the request
    println!("\nGenerating image...");
    let (response, rate_limit) = client.generate_image(request).await?;
    
    // Print information about the generated images
    println!("\nGenerated {} image(s)", response.images.len());
    
    for (i, image_data) in response.data.iter().enumerate() {
        println!("\nImage {}:", i + 1);
        
        if let Some(url) = &image_data.url {
            println!("URL: {}", url);
        }
        
        if let Some(b64) = &image_data.b64_json {
            println!("Base64 data: {} (first 20 chars)", &b64[..20.min(b64.len())]);
        }
        
        if let Some(seed) = image_data.seed {
            println!("Seed: {}", seed);
        }
        
        if let Some(revised_prompt) = &image_data.revised_prompt {
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