# Venice.ai API SDK for Rust

[![Rust](https://img.shields.io/badge/rust-stable-brightgreen.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Crates.io](https://img.shields.io/crates/v/venice-ai-api-sdk-rust.svg)](https://crates.io/crates/venice-ai-api-sdk-rust)
[![Documentation](https://docs.rs/venice-ai-api-sdk-rust/badge.svg)](https://docs.rs/venice-ai-api-sdk-rust)

A comprehensive, high-performance Rust SDK for the Venice.ai AI platform. This official SDK provides type-safe, fully-featured bindings to all Venice.ai API endpoints, including LLM chat completions, image generation, and API key management.

> **Venice.ai** is a modern AI platform offering powerful Large Language Models (LLMs) like Llama 3.3 for text generation and AI image models for creative visual content.

## 🚀 Why Choose Venice.ai Rust SDK?

- **High Performance**: Built with Rust's zero-cost abstractions and async runtime for maximum efficiency
- **Type Safety**: Leverage Rust's strong type system to prevent runtime errors
- **Modern Architecture**: Clean, trait-based design following the Single Responsibility Principle
- **Complete API Coverage**: Access all Venice.ai capabilities through a unified interface
- **Developer Experience**: Intuitive builder patterns and comprehensive documentation
- **Production Ready**: Robust error handling, rate limiting, and retry mechanisms

**Keywords**: Rust AI SDK, Venice.ai API, LLM API, Llama 3.3 API, AI Image Generation, Rust Machine Learning, Generative AI, Rust Async API Client, AI Text Generation, AI Chat Completions, Streaming AI Responses, Rust AI Integration, Enterprise AI SDK, AI Development Tools, Rust AI Framework, Llama Integration, AI API Wrapper, Rust AI Client, AI Model Access, Rust Generative AI

## 🌟 Features

### AI Text Generation
- **Chat Completions API**: Access Venice.ai's powerful LLMs like Llama 3.3 with full parameter control
- **Streaming Support**: Real-time token streaming for responsive AI applications
- **Context Management**: Easily build and manage conversation contexts
- **Parameter Tuning**: Fine-tune temperature, top_p, frequency penalty and more
- **Token Counting**: Built-in utilities for token usage tracking and optimization

### AI Image Generation
- **Text-to-Image**: Create stunning images from text prompts using models like Fluently-XL
- **Style Presets**: 50+ artistic styles and presets for diverse creative outputs
- **Resolution Control**: Generate images at various resolutions up to 4K
- **Parameter Customization**: Adjust steps, CFG scale, and other generation parameters
- **Image Upscaling**: Enhance image resolution with AI-powered upscaling technology

### Platform Features
- **Models Discovery**: Programmatically list and select from available AI models
- **API Key Management**: Create, list, and delete API keys with customized rate limits
- **Webhook Integration**: Verify and process webhook events from Venice.ai
- **Rate Limit Handling**: Automatic tracking and management of API rate limits
- **Pagination Support**: Efficiently navigate large result sets with built-in pagination

### Developer Experience
- **Unified Client**: Single client interface with trait-based extensions for all API categories
- **Builder Patterns**: Intuitive request builders with method chaining
- **Comprehensive Error Handling**: Detailed error types with recovery suggestions
- **Full Type Safety**: Rust's type system ensures correct API usage at compile time
- **Async/Await Support**: Built on Tokio runtime for high-performance asynchronous operations

## 📦 Installation

Add the SDK to your Cargo.toml:

```toml
[dependencies]
venice-ai-api-sdk-rust = "1.0.0"
tokio = { version = "1.0", features = ["full"] }
```

For local development with environment variables:

```toml
[dependencies]
venice-ai-api-sdk-rust = "1.0.0"
tokio = { version = "1.0", features = ["full"] }
dotenv = "0.15.0"
```

## 🚀 Quick Start

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

For local development, you can use a `.env` file to store your API key:

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

## 💬 Chat Completions API

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
    let request = ChatCompletionBuilder::new("llama-3.3-70b")
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

For a better user experience with real-time responses:

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

## 🖼️ Image Generation API

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

### Available Image Styles

Venice.ai offers a wide range of artistic styles for image generation:

```rust
// List available image styles
let (styles, _) = client.list_styles().await?;

for style in styles.data {
    println!("Style: {}", style.name);
    if let Some(description) = style.description {
        println!("  Description: {}", description);
    }
}
```

### Image Upscaling

Enhance the resolution of existing images:

```rust
use venice_ai_api_sdk_rust::{
    Client, 
    traits::image::{ImageApi, ImageUpscaleBuilder}
};
use std::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a client
    let client = Client::new("your-api-key")?;
    
    // Read image file as bytes
    let image_bytes = fs::read("input_image.jpg")?;
    
    // Build an upscale request
    let request = ImageUpscaleBuilder::new(image_bytes)
        .scale(2) // Double the resolution
        .build();
    
    // Send the request
    let (response, _) = client.upscale_image(request).await?;
    
    // Save the upscaled image
    if let Some(image) = response.data.first() {
        if let Some(b64) = &image.b64_json {
            let decoded = base64::decode(b64)?;
            fs::write("upscaled_image.jpg", decoded)?;
            println!("Upscaled image saved to upscaled_image.jpg");
        }
    }
    
    Ok(())
}
```

## 🔍 Models API

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

### Model Compatibility

The SDK provides utilities for working with model compatibility:

```rust
use venice_ai_api_sdk_rust::{
    Client,
    models::compatibility_mapping::get_compatible_models,
    traits::models::ModelsApi
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a client
    let client = Client::new("your-api-key")?;
    
    // Get all available models
    let (models_response, _) = client.list_models().await?;
    
    // Find models compatible with a specific model
    let compatible_models = get_compatible_models("llama-3.3-70b", &models_response.data);
    
    println!("Models compatible with llama-3.3-70b:");
    for model in compatible_models {
        println!("- {}", model.id);
    }
    
    Ok(())
}
```

## 🔑 API Key Management

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

### Pagination Support

For accounts with many API keys, use pagination:

```rust
use venice_ai_api_sdk_rust::{
    Client,
    traits::api_keys::ApiKeysApi,
    pagination::PaginationParams
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a client
    let client = Client::new("your-api-key")?;
    
    // Create pagination parameters
    let pagination = PaginationParams {
        limit: Some(10),
        after: None,
        before: None,
    };
    
    // List API keys with pagination
    let (keys, _) = client.list_api_keys_with_pagination(pagination).await?;
    
    println!("API Keys (Page 1):");
    for key in &keys.data {
        println!("- {} ({})", key.name, key.id);
    }
    
    // Get the next page if available
    if let Some(next_cursor) = keys.next_cursor {
        let next_pagination = PaginationParams {
            limit: Some(10),
            after: Some(next_cursor),
            before: None,
        };
        
        let (next_keys, _) = client.list_api_keys_with_pagination(next_pagination).await?;
        
        println!("\nAPI Keys (Page 2):");
        for key in &next_keys.data {
            println!("- {} ({})", key.name, key.id);
        }
    }
    
    Ok(())
}
```

## 🔔 Webhook Verification

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

## ⚠️ Error Handling

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

## 🔄 Retry and Rate Limiting

The SDK includes built-in support for retry logic and rate limit handling:

```rust
use venice_ai_api_sdk_rust::{
    Client,
    retry::{RetryConfig, RetryStrategy},
    rate_limit::RateLimitConfig,
    config::ClientConfig,
    traits::chat::ChatApi,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Configure retry logic
    let retry_config = RetryConfig {
        max_retries: 3,
        initial_backoff_ms: 1000,
        max_backoff_ms: 10000,
        strategy: RetryStrategy::ExponentialBackoff,
    };
    
    // Configure rate limiting
    let rate_limit_config = RateLimitConfig {
        max_requests_per_minute: Some(60),
        max_tokens_per_minute: Some(10000),
    };
    
    // Create a client config
    let config = ClientConfig {
        api_key: "your-api-key".to_string(),
        base_url: None, // Uses default
        timeout_secs: Some(30),
        retry_config: Some(retry_config),
        rate_limit_config: Some(rate_limit_config),
        custom_headers: None,
    };
    
    // Create a client with the config
    let client = Client::with_config(config)?;
    
    // Use the client as normal
    // The SDK will automatically handle retries and rate limiting
    
    Ok(())
}
```

## 📚 Examples

The repository includes comprehensive examples for all Venice.ai API features in the `examples/` directory:

### Organized Examples by Category:
- **Chat API**:
  - `examples/chat/chat_completion.rs` - Basic chat completion
  - `examples/chat/streaming_chat_completion.rs` - Streaming chat completion
  - `examples/chat/builder_streaming.rs` - Using the builder's streaming convenience method
  - `examples/chat/model_feature_suffix.rs` - Using model feature suffixes
  - `examples/chat/advanced_streaming.rs` - Advanced streaming techniques
- **Models API**:
  - `examples/models/list_models.rs` - Listing available models
  - `examples/models/model_traits.rs` - Working with model traits
  - `examples/models/model_compatibility.rs` - Model compatibility features
  - `examples/models/paginated_models.rs` - Paginated model listing
- **Image API**:
  - `examples/image/generate_image.rs` - Generating images
  - `examples/image/upscale_image.rs` - Upscaling images
  - `examples/image/list_styles.rs` - Listing available styles
  - `examples/image/response_parsing.rs` - Parsing image responses
- **API Keys API**:
  - `examples/api_keys/list_api_keys.rs` - Listing API keys
  - `examples/api_keys/create_api_key.rs` - Creating API keys
  - `examples/api_keys/delete_api_key.rs` - Deleting API keys
  - `examples/api_keys/generate_web3_key.rs` - Generating Web3 keys
  - `examples/api_keys/paginated_api_keys.rs` - Paginated API key listing
- **Common Utilities**:
  - `examples/common/api_key_management.rs` - API key management
  - `examples/common/mock_client_demo.rs` - Using mock clients for testing
  - `examples/common/rate_limit_example.rs` - Rate limit handling
  - `examples/common/retry_example.rs` - Retry logic
  - `examples/common/unified_client_demo.rs` - Unified client usage
  - `examples/common/webhook_verification_example.rs` - Webhook verification

### Running Examples

Run any example with:

```bash
cargo run --example chat_completion
```

## 🏗️ Architecture

The SDK follows a clean architecture based on the Single Responsibility Principle (SRP):

### Unified Client Architecture

The SDK uses a unified client architecture with trait-based extensions for each API category:

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

### API Layer

Each API category has its own implementation class that handles the specific API endpoints:

- `ChatApiImpl` - Handles chat completions API
- `ModelsApiImpl` - Handles models listing and features
- `ImageApiImpl` - Handles image generation and upscaling
- `ApiKeysApiImpl` - Handles API key management

### Services Layer

Utility services provide reusable functionality:

- `WebhookService` - Handles webhook signature verification

## 🔧 Advanced Configuration

### Custom HTTP Client Configuration

```rust
use venice_ai_api_sdk_rust::{
    Client,
    config::ClientConfig,
};
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create custom headers
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static("MyApp/1.0"));
    
    // Create a client config
    let config = ClientConfig {
        api_key: "your-api-key".to_string(),
        base_url: Some("https://api.venice.ai".to_string()),
        timeout_secs: Some(60),
        retry_config: None,
        rate_limit_config: None,
        custom_headers: Some(headers),
    };
    
    // Create a client with the config
    let client = Client::with_config(config)?;
    
    Ok(())
}
```

### Mock Client for Testing

```rust
use venice_ai_api_sdk_rust::{
    Client,
    chat::test_client::MockChatClient,
    traits::chat::{ChatApi, ChatCompletionBuilder},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a mock client for testing
    let mock_client = MockChatClient::new();
    
    // Build a chat completion request
    let request = ChatCompletionBuilder::new("llama-3.3-70b")
        .add_system("You are a helpful assistant.")
        .add_user("Tell me about AI")
        .build();
    
    // Use the mock client
    let (response, _) = mock_client.create_chat_completion(request).await?;
    
    // The mock client returns predefined responses
    println!("Mock response: {}", response.choices[0].message.content);
    
    Ok(())
}
```

## 📈 Performance Optimization

### Token Usage Optimization

```rust
use venice_ai_api_sdk_rust::{
    Client,
    traits::chat::{ChatApi, ChatCompletionBuilder},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new("your-api-key")?;
    
    // Optimize token usage by:
    // 1. Using a smaller model when appropriate
    // 2. Setting a reasonable max_tokens value
    // 3. Keeping system messages concise
    // 4. Using function calling for structured outputs
    
    let request = ChatCompletionBuilder::new("llama-3.3-8b") // Smaller model
        .add_system("You are a helpful assistant. Be concise.") // Concise system message
        .add_user("Summarize the key features of Rust in bullet points")
        .max_tokens(150) // Limit response size
        .temperature(0.3) // Lower temperature for more focused responses
        .build();
    
    let (response, _) = client.create_chat_completion(request).await?;
    
    // Check token usage
    if let Some(usage) = response.usage {
        println!("Total tokens used: {}", usage.total_tokens);
        println!("Cost efficiency: {:.2}%", 
            (usage.completion_tokens as f64 / usage.total_tokens as f64) * 100.0);
    }
    
    Ok(())
}
```

## 🔗 Related Resources

- [Venice.ai Platform Documentation](https://docs.venice.ai)
- [Venice.ai API Reference](https://api.venice.ai/docs)
- [Rust Programming Language](https://www.rust-lang.org/)
- [Tokio Async Runtime](https://tokio.rs/)

## 📄 License

This SDK is available under the MIT License.

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request