# Venice AI API SDK Examples

This directory contains examples demonstrating how to use the Venice AI API SDK.

## Common Examples

These examples demonstrate general usage of the SDK with the unified client architecture:

- [Unified Client Demo](common/unified_client_demo.rs) - Demonstrates how to use the unified client architecture with trait extensions for different API categories
- [API Key Management](common/api_key_management.rs) - Demonstrates how to manage API keys (list, create, delete)

## Models API Examples

These examples demonstrate how to use the Models API:

- [List Models](models/list_models.rs) - Lists all available models
- [Model Traits](models/model_traits.rs) - Demonstrates how to get model traits and check compatibility
- [Model Compatibility](models/model_compatibility.rs) - Checks model compatibility with different features

## Chat API Examples

These examples demonstrate how to use the Chat API:

- [Chat Completion](chat/chat_completion.rs) - Demonstrates how to create a chat completion
- [Model Feature Suffix](chat/model_feature_suffix.rs) - Demonstrates how to use model feature suffixes

## Image API Examples

These examples demonstrate how to use the Image API:

- [Generate Image](image/generate_image.rs) - Demonstrates how to generate an image
- [Upscale Image](image/upscale_image.rs) - Demonstrates how to upscale an image
- [List Styles](image/list_styles.rs) - Lists all available image styles
- [Response Parsing](image/response_parsing.rs) - Shows how to parse image generation responses

## API Keys API Examples

These examples demonstrate how to use the API Keys API:

- [List API Keys](api_keys/list_api_keys.rs) - Lists all API keys
- [Create API Key](api_keys/create_api_key.rs) - Creates a new API key
- [Delete API Key](api_keys/delete_api_key.rs) - Deletes an API key
- [Generate Web3 Key](api_keys/generate_web3_key.rs) - Generates a Web3 key

## Legacy Examples

These examples use the deprecated client architecture and are kept for backward compatibility:

- [Simple Test](simple_test.rs) - A simple test using the deprecated VerySimpleClient and ChatClient
- [Basic Test](basic_test.rs) - A basic test using the deprecated ModelsClient and ChatClient

## Running Examples

To run an example, use the following command:

```bash
cargo run --example <example_name> --features examples
```

For example, to run the unified client demo:

```bash
cargo run --example unified_client_demo --features examples
```

For examples in subdirectories, use the path with slashes replaced by underscores:

```bash
cargo run --example models_list_models --features examples
```

## Environment Variables

Most examples require the `VENICE_API_KEY` environment variable to be set. You can set this in a `.env` file in the root of the project:

```
VENICE_API_KEY=your_api_key_here
```

Or you can set it in your environment:

```bash
export VENICE_API_KEY=your_api_key_here