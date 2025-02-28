use std::env;
use std::error::Error;
use venice_ai_api_sdk_rust::{
    Client,
    PaginationParams, Paginator,
    api_keys,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Get API key from environment variable
    let api_key = env::var("VENICE_API_KEY").expect("VENICE_API_KEY environment variable not set");
    
    // Create a client
    let client = Client::new(api_key.clone())?;
    
    println!("Example 1: Getting all API keys using the paginator");
    println!("------------------------------------------------");
    
    // Create a paginator with default parameters
    let params = PaginationParams::new().limit(5);
    let mut paginator = client.list_api_keys_paginator(params);
    
    // Get all API keys
    let all_keys = paginator.all_pages().await?;
    println!("Found {} API keys in total\n", all_keys.len());
    
    println!("Example 2: Iterating through pages manually");
    println!("------------------------------------------");
    
    // Create a new paginator
    let params = PaginationParams::new().limit(5);
    let mut paginator = client.list_api_keys_paginator(params);
    
    // Iterate through pages
    let mut page_num = 1;
    while let Some(page) = paginator.next_page().await? {
        println!("Page {}: Got {} API keys", page_num, page.data.len());
        
        for (i, key) in page.data.iter().enumerate() {
            let display_name = key.name.as_deref()
                .or_else(|| if !key.last_chars.is_empty() { Some(key.last_chars.as_str()) } else { None })
                .unwrap_or("API Key");
            println!("  {}. {} ({})", i + 1, display_name, key.id);
        }
        
        println!("  Has more: {}", page.has_more);
        println!();
        
        page_num += 1;
    }
    
    println!("Example 3: Using list_api_keys_with_params directly");
    println!("-----------------------------------------------");
    
    // Get the first page with 5 API keys
    let request = api_keys::ListApiKeysRequest::new().limit(5);
    let (keys, _) = client.list_api_keys_with_params(request).await?;
    
    println!("First page: {} API keys", keys.data.len());
    
    for (i, key) in keys.data.iter().enumerate() {
        let display_name = key.name.as_deref()
            .or_else(|| if !key.last_chars.is_empty() { Some(key.last_chars.as_str()) } else { None })
            .unwrap_or("API Key");
        println!("  {}. {} ({})", i + 1, display_name, key.id);
    }
    
    println!("\nExample 4: Using the helper function");
    println!("----------------------------------");
    
    // Create a paginator using the helper function
    let params = PaginationParams::new().limit(5);
    let mut paginator = api_keys::list_api_keys_paginator(api_key, params)?;
    
    // Get all API keys
    let all_keys = paginator.all_pages().await?;
    println!("Found {} API keys in total", all_keys.len());
    
    Ok(())
}