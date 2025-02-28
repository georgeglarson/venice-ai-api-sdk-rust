use std::env;
use std::error::Error;
use venice_ai_api_sdk_rust::{
    Client,
    PaginationParams, Paginator,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load environment variables from .env file
    if let Err(e) = dotenv::dotenv() {
        println!("Warning: Could not load .env file: {}", e);
    }
    
    // Get API key from environment variable
    let api_key = env::var("VENICE_API_KEY").expect("VENICE_API_KEY not set");
    
    // Create a client
    let client = Client::new(&api_key)?;
    
    // Example 1: Using the paginator to get all models
    println!("Example 1: Getting all models using the paginator");
    println!("------------------------------------------------");
    
    // Create a paginator with 5 models per page
    let params = PaginationParams::new().limit(5);
    let mut paginator = client.list_models_paginator(params);
    
    // Get all models
    let all_models = paginator.all_pages().await?;
    println!("Found {} models in total\n", all_models.len());
    
    // Example 2: Iterating through pages manually
    println!("Example 2: Iterating through pages manually");
    println!("------------------------------------------");
    
    // Create a new paginator with 3 models per page
    let params = PaginationParams::new().limit(3);
    let mut paginator = client.list_models_paginator(params);
    
    // Iterate through pages
    let mut page_num = 1;
    while let Some(page) = paginator.next_page().await? {
        println!("Page {}: Got {} models", page_num, page.data.len());
        
        for (i, model) in page.data.iter().enumerate() {
            println!("  {}. {} (owned by: {})", i + 1, model.id, model.owned_by);
        }
        
        println!("  Has more: {}", page.has_more);
        if let Some(cursor) = &page.next_cursor {
            println!("  Next cursor: {}", cursor);
        }
        
        println!();
        page_num += 1;
    }
    
    // Example 3: Using the list_models_with_params method directly
    println!("Example 3: Using list_models_with_params directly");
    println!("-----------------------------------------------");
    
    // Get the first page with 2 models
    let request = venice_ai_api_sdk_rust::models::list::ListModelsRequest::new().limit(2);
    let (models, _) = client.list_models_with_params(request).await?;
    
    println!("First page: {} models", models.data.len());
    for (i, model) in models.data.iter().enumerate() {
        println!("  {}. {} (owned by: {})", i + 1, model.id, model.owned_by);
    }
    
    // If there are more models, get the next page
    if models.has_more {
        if let Some(cursor) = models.next_cursor {
            println!("\nGetting next page using cursor: {}", cursor);
            
            let next_request = venice_ai_api_sdk_rust::models::list::ListModelsRequest::new()
                .limit(2)
                .cursor(cursor);
            let (next_models, _) = client.list_models_with_params(next_request).await?;
            
            println!("Second page: {} models", next_models.data.len());
            for (i, model) in next_models.data.iter().enumerate() {
                println!("  {}. {} (owned by: {})", i + 1, model.id, model.owned_by);
            }
        }
    }
    
    Ok(())
}