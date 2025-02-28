use std::env;
use std::error::Error;
use venice_ai_api_sdk_rust::{
    Client,
    traits::{
        models::ModelsApi,
        chat::{ChatApi, ChatCompletionBuilder},
        image::{ImageApi, ImageGenerateBuilder},
        api_keys::ApiKeysApi,
    },
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load environment variables from .env file
    if let Err(e) = dotenv::dotenv() {
        println!("Warning: Could not load .env file: {}", e);
    }
    
    // Get API key from environment variable
    let api_key = env::var("VENICE_API_KEY").expect("VENICE_API_KEY not set");
    
    // Create a single unified client
    let client = Client::new(&api_key)?;
    
    println!("=== Venice AI API SDK Unified Client Demo ===");
    println!("This example demonstrates how to use the unified client architecture");
    println!("with trait extensions for different API categories.\n");
    
    // === Models API ===
    println!("=== Models API ===");
    println!("Fetching available models...");
    let (models_response, _) = client.list_models().await?;
    
    println!("Found {} models", models_response.data.len());
    println!("First few models:");
    for model in models_response.data.iter().take(3) {
        println!("- {} (owned by: {})", model.id, model.owned_by);
    }
    println!();
    
    // Find models for different capabilities
    let chat_model = models_response.data.iter()
        .find(|m| m.supports_chat_completions)
        .map(|m| m.id.clone())
        .unwrap_or_else(|| "llama-3.3-70b".to_string());
        
    let image_model = models_response.data.iter()
        .find(|m| m.supports_image_generation)
        .map(|m| m.id.clone())
        .unwrap_or_else(|| "fluently-xl".to_string());
    
    // === Chat API ===
    println!("=== Chat API ===");
    println!("Using chat model: {}", chat_model);
    
    let chat_request = ChatCompletionBuilder::new(chat_model)
        .add_user("Write a haiku about Rust programming.")
        .max_tokens(100)
        .build();
    
    println!("Sending chat completion request...");
    match client.create_chat_completion(chat_request).await {
        Ok((response, _)) => {
            println!("Response:");
            println!("{}", response.choices[0].message.content);
            println!();
        },
        Err(e) => {
            println!("Error: {}", e);
            println!();
        }
    }
    
    // === Image API ===
    println!("=== Image API ===");
    println!("Using image model: {}", image_model);
    
    // List available styles
    println!("Fetching available styles...");
    match client.list_styles().await {
        Ok((styles_response, _)) => {
            println!("Found {} styles", styles_response.styles.len());
            println!("First few styles:");
            for style in styles_response.styles.iter().take(3) {
                println!("- {}", style);
            }
            println!();
            
            // Choose a style
            let style = styles_response.styles.first()
                .cloned()
                .unwrap_or_else(|| "3D Model".to_string());
                
            // Create an image generation request
            let image_request = ImageGenerateBuilder::new(
                image_model,
                "A Rust crab mascot coding on a laptop"
            )
            .style_preset(&style)
            .build();
            
            println!("Generating image (this would actually send a request)...");
            println!("Request details:");
            println!("- Model: {}", image_model);
            println!("- Prompt: A Rust crab mascot coding on a laptop");
            println!("- Style: {}", style);
            println!();
        },
        Err(e) => {
            println!("Error fetching styles: {}", e);
            println!();
        }
    }
    
    // === API Keys API ===
    println!("=== API Keys API ===");
    println!("Fetching API keys...");
    
    match client.list_api_keys().await {
        Ok((keys_response, rate_limit)) => {
            println!("Found {} API keys", keys_response.data.len());
            
            // Print rate limit information
            println!("\nRate limit information:");
            println!("Requests: {}/{}", 
                rate_limit.remaining_requests.unwrap_or(0),
                rate_limit.limit_requests.unwrap_or(0)
            );
        },
        Err(e) => {
            println!("Error fetching API keys: {}", e);
        }
    }
    
    println!("\n=== Demo Complete ===");
    println!("The unified client architecture allows you to use a single client");
    println!("instance with trait extensions for different API categories.");
    
    Ok(())
}