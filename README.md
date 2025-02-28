# Venice.ai API SDK for Rust

[![Rust](https://img.shields.io/badge/rust-stable-brightgreen.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

A comprehensive, high-performance Rust SDK for the Venice.ai AI platform. This official SDK provides type-safe, fully-featured bindings to all Venice.ai API endpoints, including LLM chat completions, image generation, and API key management.

> **Venice.ai** is a modern AI platform offering powerful Large Language Models (LLMs) like Llama for text generation and AI image models for creative visual content.

**Keywords**: Rust AI SDK, Venice.ai API, LLM API, Llama API, AI Image Generation, API Key Management, CORS Debugging, Rust Async API Client

## Features

- **AI Text Generation**: Access Venice.ai's powerful LLMs like Llama 3.2 through the Chat completions API (with streaming support)
- **AI Image Generation**: Create stunning images with text prompts using models like Fluently-XL
- **Image Styles**: Explore and utilize dozens of artistic styles and presets for image generation
- **Image Upscaling**: Enhance image resolution with AI-powered upscaling technology
- **Models Discovery**: Programmatically list and select from available AI models
- **API Key Management**: Create, list, and delete API keys with customized rate limits
- **Robust Error Handling**: Comprehensive error types with detailed messages and recovery options
- **Developer-Friendly Design**: Type-safe interfaces with intuitive builder patterns for request construction
- **Async Support**: Built on Tokio runtime for high-performance asynchronous operations
- **Full API Coverage**: Complete support for all Venice.ai API endpoints and features
- **Unified Client Architecture**: Single client interface with trait-based extensions for all API categories
- **SRP-Based Architecture**: Clean separation of concerns with dedicated API implementations and services

## Installation

Add the SDK to your Cargo.toml:

```toml
[dependencies]
venice-ai-api-sdk-rust = "0.2.0"
tokio = { version = "1.0", features = ["full"] }
```

## Quick Start

### Authentication

The SDK requires an API key from Venice.ai. There are two recommended ways to provide your API key:

#### Option 1: Using Environment Variables (Recommended for security)

Store your API key in environment variables and load it in your code:

```rust
// Get API key from environment variable
let api_key = std::env::var("VENICE_API_KEY").expect("VENICE_API_KEY not set");

// Create a unified client with your API key
let client = Client::new(api_key)?;
```

#### Option 2: Using .env File (Recommended for development)

For local development, you can use a `.env` file to store your API key. First, add the `dotenv` crate to your dependencies:

```toml
[dependencies]
dotenv = "0.15.0"
```

Create a `.env` file in your project root:

```
VENICE_API_KEY=your_actual_api_key_here
```

Then load it in your code:

```rust
// Load environment variables from .env file
dotenv::dotenv().ok();

// Get API key from environment variable
let api_key = std::env::var("VENICE_API_KEY").expect("VENICE_API_KEY not set");

// Create a unified client with your API key
let client = Client::new(api_key)?;
```

**Important:** Make sure to add `.env` to your `.gitignore` file to prevent accidentally committing your API key to version control.

### Unified Client Architecture

The SDK now uses a unified client architecture with trait-based extensions for each API category:

```rust
use venice_ai_api_sdk_rust::{
    Client,
    traits::{
        models::ModelsApi,
        chat::ChatApi,
        image::ImageApi,
        api_keys::ApiKeysApi,
    },
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a single unified client
    let client = Client::new("your-api-key")?;
    
    // Use the client with different API traits
    
    // Models API
    let (models, _) = client.list_models().await?;
    
    // Chat API
    let request = ChatCompletionBuilder::new("llama-3.3-70b")
        .add_system("You are a helpful assistant.")
        .add_user("Tell me about AI")
        .build();
    let (response, _) = client.create_chat_completion(request).await?;
    
    // Image API
    let (styles, _) = client.list_styles().await?;
    
    // API Keys API
    let (keys, _) = client.list_api_keys().await?;
    
    Ok(())
}
```

### Models API

Query available AI models:

```rust
use venice_ai_api_sdk_rust::{Client, traits::models::ModelsApi};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a client
    let client = Client::new("your-api-key")?;
    
    // List available models
    let (models, rate_limit) = client.list_models().await?;
    
    // Print model information
    for model in models.data {
        println!("Model: {} (owned by: {})", model.id, model.owned_by);
        
        if model.supports_chat_completions {
            println!("  Supports chat completions: Yes");
        }
        
        if model.supports_image_generation {
            println!("  Supports image generation: Yes");
        }
        
        if let Some(context_size) = model.context_size {
            println!("  Context size: {}", context_size);
        }
    }
    
    // Print rate limit information
    println!("Rate limit info: {}/{} requests remaining", 
        rate_limit.remaining_requests.unwrap_or(0),
        rate_limit.limit_requests.unwrap_or(0)
    );
    
    Ok(())
}
```

### Chat Completions API

Generate text responses via chat:

```rust
use venice_ai_api_sdk_rust::{
    Client,
    traits::chat::{ChatApi, ChatCompletionBuilder}
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a client
    let client = Client::new("your-api-key")?;
    
    // Build a chat completion request
    let request = ChatCompletionBuilder::new("llama-3.2-3b")
        .add_system("You are a helpful assistant.")
        .add_user("Tell me about AI")
        .max_tokens(100)
        .temperature(0.7)
        .build();
    
    // Send the request
    let (response, rate_limit) = client.create_chat_completion(request).await?;
    
    // Print the response
    println!("AI response: {}", response.choices[0].message.content);
    
    // Print usage information
    if let Some(usage) = response.usage {
        println!("Prompt tokens: {}", usage.prompt_tokens);
        println!("Completion tokens: {}", usage.completion_tokens);
        println!("Total tokens: {}", usage.total_tokens);
    }
    
    // Print rate limit information
    println!("Rate limit info: {}/{} tokens remaining",
        rate_limit.remaining_tokens.unwrap_or(0),
        rate_limit.limit_tokens.unwrap_or(0)
    );
    
    Ok(())
}
```

### Streaming Chat Completions

For a better user experience with real-time responses, use streaming chat completions:

```rust
use std::io::Write;
use futures::StreamExt;
use venice_ai_api_sdk_rust::{
    Client,
    traits::chat::{ChatApi, ChatCompletionBuilder}
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a client
    let client = Client::new("your-api-key")?;
    
    // Build a chat completion request with streaming enabled
    let request = ChatCompletionBuilder::new("llama-3.3-70b")
        .add_system("You are a helpful assistant.")
        .add_user("Tell me about AI")
        .max_tokens(1000)
        .temperature(0.7)
        .stream(true) // Enable streaming
        .build();
    
    // Send the streaming request
    let (stream, rate_limit) = client.create_streaming_chat_completion(request).await?;
    
    // Process the stream
    println!("Response (streaming):");
    
    // Consume the stream
    let mut stream = Box::pin(stream);
    while let Some(chunk_result) = stream.next().await {
        match chunk_result {
            Ok(chunk) => {
                // Process each choice in the chunk
                for choice in &chunk.choices {
                    if let Some(content) = &choice.delta.content {
                        print!("{}", content);
                        std::io::stdout().flush()?;
                    }
                }
            }
            Err(e) => {
                eprintln!("Error processing chunk: {}", e);
                break;
            }
        }
    }
    
    println!("\n\nStreaming complete!");
    
    Ok(())
}
```

You can also use the builder's convenience method for streaming:

```rust
// Using the builder's stream_with_client method
let (stream, _) = ChatCompletionBuilder::new("llama-3.3-70b")
    .add_system("You are a helpful assistant.")
    .add_user("Write a short poem about Rust programming language.")
    .max_tokens(500)
    .temperature(0.8)
    // No need to call .stream(true) as stream_with_client sets it automatically
    .stream_with_client(&client)
    .await?;
```

### Image Generation API

Generate images from text prompts:

```rust
use venice_ai_api_sdk_rust::{
    Client, 
    traits::image::{ImageApi, ImageGenerateBuilder}
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a client
    let client = Client::new("your-api-key")?;
    
    // Build an image generation request
    let request = ImageGenerateBuilder::new(
        "fluently-xl",
        "A beautiful sunset over a mountain range"
    )
    .style_preset("3D Model")
    .width(1024)
    .height(1024)
    .steps(30)
    .cfg_scale(7.5)
    .build();
    
    // Send the request
    let (response, _) = client.generate_image(request).await?;
    
    // Print the image URL or base64 data
    if let Some(image) = response.data.first() {
        if let Some(url) = &image.url {
            println!("Generated image URL: {}", url);
        } else if let Some(b64) = &image.b64_json {
            println!("Generated image data: {} (base64, first 20 chars)", &b64[..20]);
        }
    }
    
    Ok(())
}
```

### API Key Management

Create, list, and delete API keys:

```rust
use venice_ai_api_sdk_rust::{
    Client,
    traits::api_keys::{ApiKeysApi, CreateApiKeyRequest}
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a client
    let client = Client::new("your-api-key")?;
    
    // List existing API keys
    let (keys, _) = client.list_api_keys().await?;
    for key in keys.data {
        println!("Key: {} ({})", key.name, key.id);
    }
    
    // Create a new API key
    let request = CreateApiKeyRequest {
        name: "My New API Key".to_string(),
    };
    
    let (create_response, _) = client.create_api_key(request).await?;
    
    // IMPORTANT: Save this key, it's only shown once
    println!("New API Key: {}", create_response.secret);
    
    // Delete an API key
    let (delete_response, _) = client.delete_api_key(&create_response.key.id).await?;
    
    if delete_response.deleted {
        println!("Successfully deleted API key");
    }
    
    Ok(())
}
```

### Webhook Verification

Verify webhook signatures from Venice.ai:

```rust
use venice_ai_api_sdk_rust::{
    webhooks::{verify_webhook_signature, get_webhook_headers},
    services::webhook::WebhookService,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Option 1: Using the webhook verification function
    let webhook_secret = std::env::var("VENICE_WEBHOOK_SECRET")?;
    let payload = b"{\"event\":\"model.created\",\"data\":{...}}";
    let signature = "abcdef1234567890..."; // From X-Venice-Signature header
    let timestamp = "1234567890"; // From X-Venice-Timestamp header
    
    if verify_webhook_signature(payload, signature, timestamp, &webhook_secret)? {
        println!("Webhook signature verified!");
        // Process the webhook
    } else {
        println!("Invalid webhook signature!");
    }
    
    // Option 2: Using the WebhookService directly
    let webhook_service = WebhookService::new();
    
    match webhook_service.verify_signature(signature, timestamp, payload, &webhook_secret) {
        Ok(()) => {
            println!("Webhook signature verified!");
            // Process the webhook
        },
        Err(e) => {
            println!("Invalid webhook signature: {}", e);
        }
    }
    
    Ok(())
}
```

## Error Handling

The SDK provides comprehensive error handling through the `VeniceError` enum:

```rust
match result {
    Ok((data, rate_limit)) => {
        // Handle successful response
        println!("Rate limit: {}/{} requests remaining", 
            rate_limit.remaining_requests.unwrap_or(0),
            rate_limit.limit_requests.unwrap_or(0)
        );
    },
    Err(err) => match err {
        VeniceError::ApiError { status, code, message } => {
            // Handle API-specific errors
            println!("API Error: {} - {}", code, message);
        },
        VeniceError::HttpError(err) => {
            // Handle HTTP errors
            println!("HTTP Error: {}", err);
        },
        VeniceError::RateLimitExceeded(msg) => {
            // Handle rate limiting
            println!("Rate limit exceeded: {}", msg);
        },
        VeniceError::ParseError(msg) => {
            // Handle parsing errors
            println!("Parse error: {}", msg);
        },
        _ => {
            // Handle other errors
        }
    }
}
```

## Examples

The repository includes comprehensive examples for all Venice.ai API features in the `examples/` directory:

### Core Examples:
- `unified_client.rs` - Example using the new unified client architecture
- `working_example.rs` - Basic example with models and chat (legacy)
- `image_generation_example.rs` - Example of generating images (legacy)
- `image_upscale_example.rs` - Example of upscaling images (legacy)
- `api_keys_example.rs` - Example of API key management (legacy)
- `dotenv_example.rs` - Example of loading API key from .env file (legacy)

### Organized Examples by Category:
- **Chat API**:
  - `examples/chat/chat_completion.rs` - Basic chat completion
  - `examples/chat/streaming_chat_completion.rs` - Streaming chat completion
  - `examples/chat/builder_streaming.rs` - Using the builder's streaming convenience method
  - `examples/chat/model_feature_suffix.rs` - Using model feature suffixes
- **Models API**:
  - `examples/models/list_models.rs` - Listing available models
  - `examples/models/model_traits.rs` - Working with model traits
  - `examples/models/model_compatibility.rs` - Model compatibility features
- **Image API**:
  - `examples/image/generate_image.rs` - Generating images
  - `examples/image/upscale_image.rs` - Upscaling images
  - `examples/image/list_styles.rs` - Listing available styles
- **API Keys API**:
  - `examples/api_keys/list_api_keys.rs` - Listing API keys
  - `examples/api_keys/create_api_key.rs` - Creating API keys
  - `examples/api_keys/delete_api_key.rs` - Deleting API keys
  - `examples/api_keys/generate_web3_key.rs` - Generating Web3 keys

### Running Examples

Run any example with:

```bash
cargo run --example unified_client --features examples
```

## Architecture

The SDK follows a clean architecture based on the Single Responsibility Principle (SRP):

### API Layer

Each API category has its own implementation class that handles the specific API endpoints:

- `ChatApiImpl` - Handles chat completions API
- `ModelsApiImpl` - Handles models listing and features
- `ImageApiImpl` - Handles image generation and upscaling
- `ApiKeysApiImpl` - Handles API key management

### Services Layer

Utility services provide reusable functionality:

- `WebhookService` - Handles webhook signature verification

### Client Layer

The unified `Client` class delegates API calls to the appropriate implementation:

```rust
// The client internally uses the API implementations
let client = Client::new("your-api-key")?;

// When you call a method on the client, it delegates to the appropriate API implementation
let (models, _) = client.list_models().await?; // Delegates to ModelsApiImpl
let (response, _) = client.create_chat_completion(request).await?; // Delegates to ChatApiImpl
```

You can also use the API implementations directly if you prefer:

```rust
use venice_ai_api_sdk_rust::{
    http::{HttpClientConfig, new_shared_http_client},
    api::{ChatApiImpl, ModelsApiImpl},
    traits::{chat::ChatApi, models::ModelsApi},
};

// Create an HTTP client
let config = HttpClientConfig {
    api_key: "your-api-key".to_string(),
    base_url: "https://api.venice.ai".to_string(),
    custom_headers: reqwest::header::HeaderMap::new(),
    timeout_secs: None,
};
let http_client = new_shared_http_client(config)?;

// Create API implementations
let chat_api = ChatApiImpl::new(http_client.clone());
let models_api = ModelsApiImpl::new(http_client);

// Use the API implementations directly
let (models, _) = models_api.list_models().await?;
let (response, _) = chat_api.create_chat_completion(request).await?;
```

## Migration from v0.1.0 to v0.2.0

Version 0.2.0 introduces a unified client architecture with trait-based extensions. Here's how to migrate:

### Before (v0.1.0):

```rust
use venice_ai_api_sdk_rust::{
    models::ModelsClient,
    chat::ChatClient,
    image::ImageClient,
    api_keys::ApiKeysClient,
};

// Create separate clients for each API
let models_client = ModelsClient::new(&api_key)?;
let chat_client = ChatClient::new(&api_key)?;
let image_client = ImageClient::new(&api_key)?;
let api_keys_client = ApiKeysClient::new(&api_key)?;

// Use the clients
let models = models_client.list_models().await?;
let response = chat_client.create_chat_completion(request).await?;
```

### After (v0.2.0):

```rust
use venice_ai_api_sdk_rust::{
    Client,
    traits::{
        models::ModelsApi,
        chat::ChatApi,
        image::ImageApi,
        api_keys::ApiKeysApi,
    },
};

// Create a single unified client
let client = Client::new(&api_key)?;

// Use the client with different API traits
let (models, _) = client.list_models().await?;
let (response, _) = client.create_chat_completion(request).await?;
```

The legacy clients are still available but marked as deprecated.

## License

This SDK is available under the MIT License.