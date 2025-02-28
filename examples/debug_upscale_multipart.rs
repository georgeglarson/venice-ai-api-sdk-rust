use std::error::Error;
use reqwest::{Client, multipart, header::{HeaderMap, HeaderValue, AUTHORIZATION}};

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
    
    println!("Setting up HTTP client...");
    
    // Create headers with authorization
    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", api_key))?
    );
    
    // Create a reqwest client
    let client = Client::builder()
        .default_headers(headers)
        .build()?;
    
    // First, generate a simple image to get base64 data
    println!("First generating an image to get base64 data...");
    let gen_url = "https://api.venice.ai/api/v1/image/generate";
    
    // Simple image generation request
    let gen_payload = serde_json::json!({
        "model": "fluently-xl",
        "prompt": "A simple circle",
        "width": 256,
        "height": 256,
        "steps": 10
    });
    
    // Send the generation request
    let gen_response = client.post(gen_url)
        .json(&gen_payload)
        .send()
        .await?;
    
    if !gen_response.status().is_success() {
        return Err(format!("Image generation failed: {}", gen_response.status()).into());
    }
    
    // Parse the generation response
    let gen_json: serde_json::Value = gen_response.json().await?;
    
    // Extract the first image's base64 data
    let base64_data = match gen_json.get("images").and_then(|imgs| imgs.as_array()).and_then(|arr| arr.first()) {
        Some(img) => img.as_str().unwrap_or("").to_string(),
        None => return Err("No image data found in generation response".into())
    };
    
    println!("Got base64 image data of length: {} characters", base64_data.len());
    
    // Now try the upscale with multipart/form-data
    println!("\nNow trying upscale with multipart/form-data...");
    let upscale_url = "https://api.venice.ai/api/v1/image/upscale";
    
    // Convert base64 to binary
    let image_bytes = base64::decode(&base64_data)?;
    println!("Converted base64 to {} bytes of binary data", image_bytes.len());
    
    // Create multipart form
    let form = multipart::Form::new()
        .text("model", "upscale-xl")
        .text("scale", "2")  // API requires either 2 or 4 as scale value
        .part("image", multipart::Part::bytes(image_bytes)
            .file_name("image.png")
            .mime_str("image/png")?);
    
    println!("Sending upscale request with multipart form...");
    let upscale_response = client.post(upscale_url)
        .multipart(form)
        .send()
        .await?;
    
    println!("Response status: {}", upscale_response.status());
    println!("Response headers:");
    for (name, value) in upscale_response.headers() {
        println!("  {}: {}", name, value.to_str().unwrap_or("(invalid)"));
    }
    
    // Get the response body as text
    let body = upscale_response.text().await?;
    println!("\nResponse body preview (first 1000 chars):");
    println!("{}", &body.chars().take(1000).collect::<String>());
    
    println!("\nDebug completed.");
    
    Ok(())
}