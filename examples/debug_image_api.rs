use std::error::Error;
use reqwest::{Client, header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE}};

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
    headers.insert(
        CONTENT_TYPE, 
        HeaderValue::from_static("application/json")
    );
    
    // Create a reqwest client
    let client = Client::builder()
        .default_headers(headers)
        .build()?;
    
    // API endpoint
    let url = "https://api.venice.ai/api/v1/image/generate";
    
    println!("Sending image generation request to: {}", url);
    
    // Request payload
    let payload = serde_json::json!({
        "model": "fluently-xl",
        "prompt": "A serene mountain lake at sunset with reflections in the water",
        "width": 512,
        "height": 512,
        "steps": 20
    });
    
    // Send the request
    let response = client.post(url)
        .json(&payload)
        .send()
        .await?;
    
    // Get the status code
    let status = response.status();
    println!("Response status: {}", status);
    
    // Get the headers
    println!("Response headers:");
    for (name, value) in response.headers() {
        println!("  {}: {}", name, value.to_str().unwrap_or("(invalid)"));
    }
    
    // Get the response body as text
    let body = response.text().await?;
    
    // Print first 1000 chars of the response body
    println!("\nResponse body preview (first 1000 chars):");
    println!("{}", &body.chars().take(1000).collect::<String>());
    
    println!("\nTotal response length: {} characters", body.len());
    
    // Try to parse as JSON to check structure
    println!("\nAttempting to parse response as JSON...");
    match serde_json::from_str::<serde_json::Value>(&body) {
        Ok(json) => {
            println!("Successfully parsed as JSON.");
            println!("Top-level keys:");
            if let Some(obj) = json.as_object() {
                for key in obj.keys() {
                    println!("  {}", key);
                }
                
                // Check if there's a data field and what its structure is
                if let Some(data) = obj.get("data") {
                    println!("\nData field structure:");
                    if data.is_array() {
                        println!("  Is an array with {} items", data.as_array().unwrap().len());
                    } else if data.is_object() {
                        println!("  Is an object with keys: {}", 
                            data.as_object().unwrap().keys().map(|k| k.to_string())
                                .collect::<Vec<_>>().join(", "));
                    } else {
                        println!("  Is a non-object, non-array type: {}", 
                                 if data.is_null() { "null" } 
                                 else if data.is_string() { "string" }
                                 else if data.is_number() { "number" }
                                 else if data.is_boolean() { "boolean" }
                                 else { "unknown" });
                    }
                } else {
                    println!("\nNo 'data' field found in the response!");
                }
            } else {
                println!("Response is not a JSON object!");
            }
        },
        Err(e) => {
            println!("Failed to parse as JSON: {}", e);
            
            // Get line and column of the error
            let line = e.line();
            let column = e.column();
            println!("Error at line {}, column {}", line, column);
            
            // Show the problematic part
            let lines: Vec<&str> = body.lines().collect();
            if line <= lines.len() {
                let problematic_line = lines[line - 1];
                println!("Problematic line: {}", problematic_line);
            }
        }
    }
    
    println!("\nDebug completed.");
    
    Ok(())
}