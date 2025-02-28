use std::error::Error;
use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE, ACCEPT},
    Client as ReqwestClient,
};
use serde_json::Value;

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
    
    if api_key == "your_api_key_here" {
        println!("Error: You're using the placeholder API key.");
        println!("Please replace 'your_api_key_here' in the .env file with your actual Venice API key.");
        return Ok(());
    }
    
    println!("Creating request with headers...");
    
    // Create custom headers
    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", api_key))
            .expect("Failed to create Authorization header"),
    );
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
    
    // Add a custom header for demonstration
    headers.insert(
        "X-Debug-Info",
        HeaderValue::from_static("debug_headers_example"),
    );
    
    // Print request headers
    println!("\nRequest Headers:");
    for (name, value) in headers.iter() {
        // Mask the API key for security
        if name == "authorization" {
            println!("  {}: Bearer ***API_KEY_MASKED***", name);
        } else {
            println!("  {}: {}", name, value.to_str().unwrap_or("Invalid header value"));
        }
    }
    
    // Create a client with the headers
    let client = ReqwestClient::builder()
        .default_headers(headers)
        .build()?;
    
    // Make a request to the models endpoint
    println!("\nSending request to Venice.ai API...");
    let response = client
        .get("https://api.venice.ai/api/v1/models")
        .send()
        .await?;
    
    // Print response status and headers
    println!("\nResponse Status: {}", response.status());
    println!("\nResponse Headers:");
    for (name, value) in response.headers().iter() {
        println!("  {}: {}", name, value.to_str().unwrap_or("Invalid header value"));
    }
    
    // Extract and display CORS configuration
    println!("\nCORS Configuration:");
    let cors_headers = [
        "access-control-allow-origin",
        "access-control-allow-methods",
        "access-control-allow-headers",
        "access-control-allow-credentials",
        "access-control-max-age",
        "access-control-expose-headers",
    ];
    
    let mut found_cors_headers = false;
    for header in &cors_headers {
        if let Some(value) = response.headers().get(*header) {
            found_cors_headers = true;
            println!("  {}: {}", header, value.to_str().unwrap_or("Invalid header value"));
        }
    }
    
    if found_cors_headers {
        println!("\nCORS Explanation:");
        println!("  - access-control-allow-origin: Specifies which origins can access the resource");
        println!("    * Value '*' means any origin is allowed");
        println!("  - access-control-allow-methods: HTTP methods allowed when accessing the resource");
        println!("  - access-control-allow-headers: Headers that can be used in the actual request");
        println!("  - access-control-allow-credentials: Whether request can include user credentials");
        println!("  - access-control-max-age: How long preflight request results can be cached");
        println!("  - access-control-expose-headers: Headers that browsers are allowed to access");
    } else {
        println!("  No CORS headers found in the response");
    }
    
    // Parse and print the response body
    let json: Value = response.json().await?;
    
    println!("\nResponse Body (first model only):");
    if let Some(data) = json.get("data").and_then(|d| d.as_array()) {
        if let Some(first_model) = data.first() {
            println!("  {}", serde_json::to_string_pretty(first_model)?);
        } else {
            println!("  No models found");
        }
    } else {
        println!("  Unexpected response format");
    }
    
    println!("\nFull headers debug test completed successfully!");
    
    Ok(())
}