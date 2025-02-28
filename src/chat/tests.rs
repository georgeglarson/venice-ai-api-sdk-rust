#[cfg(test)]
mod tests {
    use crate::{
        traits::chat::{ChatApi, ChatCompletionBuilder, ChatCompletionChunk, ChatRole},
        error::VeniceError,
        chat::test_client::{TestChatClient, ErrorConfig},
    };
    use futures::StreamExt;
    use reqwest::StatusCode;

    #[tokio::test]
    async fn test_streaming_chat_completion() {
        // Create a test client with default responses
        let client = TestChatClient::new();

        // Create a streaming chat completion request
        let request = ChatCompletionBuilder::new("llama-3.3-70b")
            .add_user("Hello")
            .stream(true)
            .build();

        // Send the streaming request using the ChatApi trait
        let (stream, _) = client.create_streaming_chat_completion(request).await.unwrap();

        // Collect all chunks from the stream
        let chunks: Vec<ChatCompletionChunk> = stream
            .map(|result| result.unwrap())
            .collect::<Vec<_>>()
            .await;

        // Verify we got the expected number of chunks (default is 5)
        assert_eq!(chunks.len(), 5);

        // Verify the content of the first chunk
        assert_eq!(chunks[0].choices[0].delta.role.as_ref().unwrap(), &ChatRole::Assistant);
        assert_eq!(chunks[0].choices[0].delta.content.as_ref().unwrap(), "This ");
        
        // Verify the content of the last chunk
        assert_eq!(chunks[4].choices[0].delta.role, None);
        assert_eq!(chunks[4].choices[0].delta.content.as_ref().unwrap(), "response");
        assert_eq!(chunks[4].choices[0].finish_reason.as_ref().unwrap(), "stop");
    }

    #[tokio::test]
    async fn test_streaming_chat_completion_error() {
        // Create a test client with a predefined error
        let error_config = ErrorConfig {
            status: StatusCode::UNAUTHORIZED,
            code: "api_error".to_string(),
            message: "Authentication failed".to_string(),
        };
        let client = TestChatClient::new().with_streaming_error(error_config);

        // Create a streaming chat completion request
        let request = ChatCompletionBuilder::new("llama-3.3-70b")
            .add_user("Hello")
            .stream(true)
            .build();

        // Send the streaming request and expect an error
        let result = client.create_streaming_chat_completion(request).await;
        
        // Verify we got the expected error
        assert!(result.is_err());
        if let Err(VeniceError::ApiError { code, message, .. }) = result {
            assert_eq!(code, "api_error");
            assert_eq!(message, "Authentication failed");
        } else {
            panic!("Expected ApiError");
        }
    }

    #[tokio::test]
    async fn test_streaming_builder_convenience_method() {
        // Create a test client with custom chunks
        let chunks = vec![
            TestChatClient::default_streaming_chunks(&ChatCompletionBuilder::new("llama-3.3-70b").build())[0].clone(),
        ];
        let client = TestChatClient::new().with_streaming_chunks(chunks);

        // Use the builder's convenience method
        let (stream, _) = ChatCompletionBuilder::new("llama-3.3-70b")
            .add_user("Hello")
            .stream_with_client(&client)
            .await
            .unwrap();

        // Collect all chunks from the stream
        let chunks: Vec<ChatCompletionChunk> = stream
            .map(|result| result.unwrap())
            .collect::<Vec<_>>()
            .await;

        // Verify we got the expected number of chunks
        assert_eq!(chunks.len(), 1);

        // Verify the content of the chunk
        assert_eq!(chunks[0].choices[0].delta.role.as_ref().unwrap(), &ChatRole::Assistant);
        assert_eq!(chunks[0].choices[0].delta.content.as_ref().unwrap(), "This ");
    }
}