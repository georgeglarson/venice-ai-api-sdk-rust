use std::env;
use futures::StreamExt;
use venice_ai_api_sdk_rust::{
    Client,
    traits::chat::{ChatApi, ChatCompletionBuilder, ChatCompletionChunk},
    VeniceError,
};

// Helper function to get a client with the API key from environment
fn get_client() -> Option<Client> {
    match env::var("VENICE_API_KEY") {
        Ok(api_key) => {
            match Client::new(&api_key) {
                Ok(client) => Some(client),
                Err(e) => {
                    eprintln!("Failed to create client: {}", e);
                    None
                }
            }
        },
        Err(_) => {
            eprintln!("VENICE_API_KEY environment variable not set, skipping integration tests");
            None
        }
    }
}

// Only run these tests when an API key is available
#[tokio::test]
async fn test_streaming_chat_completion() {
    let client = match get_client() {
        Some(client) => client,
        None => return,
    };

    // Create a streaming chat completion request
    let request = ChatCompletionBuilder::new("llama-3.3-70b")
        .add_system("You are a helpful assistant. Keep responses very short.")
        .add_user("Say hello world")
        .max_tokens(10)
        .stream(true)
        .build();

    // Send the streaming request
    let result = client.create_streaming_chat_completion(request).await;
    assert!(result.is_ok(), "Failed to create streaming chat completion");

    let (stream, rate_limit) = result.unwrap();
    
    // Verify rate limit info is present
    assert!(rate_limit.limit_requests.is_some(), "Rate limit info missing");
    
    // Collect all chunks from the stream
    let chunks: Vec<ChatCompletionChunk> = stream
        .map(|result| {
            assert!(result.is_ok(), "Error in stream: {:?}", result);
            result.unwrap()
        })
        .collect::<Vec<_>>()
        .await;

    // Verify we got at least one chunk
    assert!(!chunks.is_empty(), "No chunks received");
    
    // Only verify chunk content if we have chunks
    if !chunks.is_empty() {
        // Verify the first chunk has the assistant role or content
        let first_chunk = &chunks[0];
        
        // Make sure the first chunk has choices
        if !first_chunk.choices.is_empty() {
            // Check if any chunk has content
            let has_content = chunks.iter()
                .filter(|chunk| !chunk.choices.is_empty())
                .any(|chunk| {
                    chunk.choices[0].delta.content.is_some() &&
                    !chunk.choices[0].delta.content.as_ref().unwrap().is_empty()
                });
            
            // Either role or content should be present
            if !first_chunk.choices.is_empty() {
                let has_role = first_chunk.choices[0].delta.role.is_some();
                assert!(has_role || has_content, "Neither role nor content present in chunks");
            }
        }
        
        // Verify the model field is present
        assert!(!first_chunk.model.is_empty(), "Model field is empty");
    }
    
    println!("Received {} chunks from streaming API", chunks.len());
}

#[tokio::test]
async fn test_streaming_builder_convenience_method() {
    let client = match get_client() {
        Some(client) => client,
        None => return,
    };

    // Use the builder's convenience method
    let result = ChatCompletionBuilder::new("llama-3.3-70b")
        .add_system("You are a helpful assistant. Keep responses very short.")
        .add_user("Write a haiku about Rust")
        .max_tokens(20)
        .stream_with_client(&client)
        .await;
    
    assert!(result.is_ok(), "Failed to use stream_with_client");

    let (stream, rate_limit) = result.unwrap();
    
    // Verify rate limit info is present
    assert!(rate_limit.limit_requests.is_some(), "Rate limit info missing");
    
    // Collect all chunks from the stream
    let chunks: Vec<ChatCompletionChunk> = stream
        .map(|result| {
            assert!(result.is_ok(), "Error in stream: {:?}", result);
            result.unwrap()
        })
        .collect::<Vec<_>>()
        .await;

    // Verify we got at least one chunk
    assert!(!chunks.is_empty(), "No chunks received");
    
    // Print the full response for debugging
    if !chunks.is_empty() {
        // Make sure we have choices and they have content
        let full_response = chunks.iter()
            .filter(|chunk| !chunk.choices.is_empty())
            .filter_map(|chunk| {
                if chunk.choices.is_empty() {
                    None
                } else {
                    chunk.choices[0].delta.content.as_ref().map(|s| s.as_str())
                }
            })
            .collect::<Vec<&str>>()
            .join("");
        
        println!("Full response: {}", full_response);
    }
    
    println!("Received {} chunks from streaming API", chunks.len());
}

#[tokio::test]
async fn test_streaming_error_handling() {
    // Create a client with an invalid API key
    let client = Client::new("invalid_api_key").unwrap();

    // Create a streaming chat completion request
    let request = ChatCompletionBuilder::new("llama-3.3-70b")
        .add_user("Hello")
        .stream(true)
        .build();

    // Send the streaming request and expect an error
    let result = client.create_streaming_chat_completion(request).await;
    
    // Verify we got the expected error
    assert!(result.is_err(), "Expected error but got success");
    
    if let Err(VeniceError::ApiError { code, message, .. }) = result {
        assert_eq!(code, "api_error", "Unexpected error code: {}", code);
        assert!(message.contains("Authentication failed") || message.contains("auth"),
                "Unexpected error message: {}", message);
    } else {
        panic!("Expected ApiError, got another error type");
    }
}