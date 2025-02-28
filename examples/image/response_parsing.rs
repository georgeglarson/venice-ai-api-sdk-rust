use std::error::Error;
use venice_ai_api_sdk_rust::traits::image::ImageGenerateResponse;

fn main() -> Result<(), Box<dyn Error>> {
    // Sample JSON response from the Venice API
    let sample_response = r#"{
        "id": "test-image-id",
        "images": ["base64data1", "base64data2"],
        "request": {
            "model": "fluently-xl",
            "prompt": "A test prompt",
            "width": 512,
            "height": 512,
            "steps": 20,
            "seed": 123456
        },
        "timing": {
            "total_ms": 2500.0
        }
    }"#;
    
    // Parse the sample response
    let response: ImageGenerateResponse = serde_json::from_str(sample_response)?;
    
    // Verify fields were parsed correctly
    println!("Parsing test successful!");
    println!("ID: {}", response.id);
    println!("Number of images: {}", response.images.len());
    
    if let Some(req) = &response.request {
        println!("Model: {}", req.model);
        println!("Prompt: {}", req.prompt);
    }
    
    // Print the data field (which is the same as images in the new architecture)
    println!("\nData field check:");
    println!("data.len(): {}", response.data.len());
    
    // Demonstrate how to access the first image
    if let Some(first_image) = response.data.first() {
        println!("\nFirst image details:");
        if let Some(url) = &first_image.url {
            println!("URL: {}", url);
        }
        if let Some(b64) = &first_image.b64_json {
            println!("Base64 data available: {} bytes", b64.len());
        }
        if let Some(seed) = first_image.seed {
            println!("Seed: {}", seed);
        }
    }
    
    Ok(())
}