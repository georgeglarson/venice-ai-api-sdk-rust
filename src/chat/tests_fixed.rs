#[cfg(test)]
mod tests {
    use crate::{
        Client,
        traits::chat::{ChatCompletionBuilder, ChatCompletionChunk},
        error::VeniceError,
    };
    use futures::StreamExt;
    use mockito::{mock, server};

    #[tokio::test]
    async fn test_streaming_chat_completion() {
        // Set up a mock server
        let mock_server = mock("POST", "/chat/completions")
            .with_status(200)
            .with_header("content-type", "text/event-stream")
            .with_body(
                "data: {\"id\":\"chatcmpl-123\",\"object\":\"chat.completion.chunk\",\"created\":1677652288,\"model\":\"llama-3.3-70b\",\"choices\":[{\"index\":0,\"delta\":{\"role\":\"assistant\",\"content\":\"Hello\"},\"finish_reason\":null}]}\n\n\
                 data: {\"id\":\"chatcmpl-123\",\"object\":\"chat.completion.chunk\",\"created\":1677652288,\"model\":\"llama-3.3-70b\",\"choices\":[{\"index\":0,\"delta\":{\"content\":\" world\"},\"finish_reason\":null}]}\n\n\
                 data: {\"id\":\"chatcmpl-123\",\"object\":\"chat.completion.chunk\",\"created\":1677652288,\"model\":\"llama-3.3-70b\",\"choices\":[{\"index\":0,\"delta\":{\"content\":\"!\"},\"finish_reason\":\"stop\"}]}\n\n\
                 data: [DONE]\n\n"
            )
            .create();

        // Create a client that points to our mock server
        let client = Client::builder()
            .api_key("test_api_key")
            .base_url(&server().url())
            .build()
            .unwrap();

        // Create a streaming chat completion request
        let request = ChatCompletionBuilder::new("llama-3.3-70b")
            .add_user("Hello")
            .stream(true)
            .build();

        // Send the streaming request using the ChatApi trait
        let (stream, _) = client.create_streaming_chat_completion(request.into()).await.unwrap();

        // Collect all chunks from the stream
        let chunks: Vec<ChatCompletionChunk> = stream
            .map(|result| result.unwrap())
            .collect::<Vec<_>>()
            .await;

        // Verify we got the expected number of chunks
        assert_eq!(chunks.len(), 3);

        // Verify the content of each chunk
        assert_eq!(chunks[0].choices[0].delta.role.as_ref().unwrap(), &crate::traits::chat::ChatRole::Assistant);
        assert_eq!(chunks[0].choices[0].delta.content.as_ref().unwrap(), "Hello");
        
        assert_eq!(chunks[1].choices[0].delta.role, None);
        assert_eq!(chunks[1].choices[0].delta.content.as_ref().unwrap(), " world");
        
        assert_eq!(chunks[2].choices[0].delta.role, None);
        assert_eq!(chunks[2].choices[0].delta.content.as_ref().unwrap(), "!");
        assert_eq!(chunks[2].choices[0].finish_reason.as_ref().unwrap(), "stop");

        // Verify the mock was called
        mock_server.assert();
    }

    #[tokio::test]
    async fn test_streaming_chat_completion_error() {
        // Set up a mock server that returns an error
        let mock_server = mock("POST", "/chat/completions")
            .with_status(401)
            .with_header("content-type", "application/json")
            .with_body("{\"error\":\"Authentication failed\"}")
            .create();

        // Create a client that points to our mock server
        let client = Client::builder()
            .api_key("invalid_api_key")
            .base_url(&server().url())
            .build()
            .unwrap();

        // Create a streaming chat completion request
        let request = ChatCompletionBuilder::new("llama-3.3-70b")
            .add_user("Hello")
            .stream(true)
            .build();

        // Send the streaming request and expect an error
        let result = client.create_streaming_chat_completion(request.into()).await;
        
        // Verify we got the expected error
        assert!(result.is_err());
        if let Err(VeniceError::ApiError { code, message, .. }) = result {
            assert_eq!(code, "api_error");
            assert_eq!(message, "Authentication failed");
        } else {
            panic!("Expected ApiError");
        }

        // Verify the mock was called
        mock_server.assert();
    }

    #[tokio::test]
    async fn test_streaming_builder_convenience_method() {
        // Set up a mock server
        let mock_server = mock("POST", "/chat/completions")
            .with_status(200)
            .with_header("content-type", "text/event-stream")
            .with_body(
                "data: {\"id\":\"chatcmpl-123\",\"object\":\"chat.completion.chunk\",\"created\":1677652288,\"model\":\"llama-3.3-70b\",\"choices\":[{\"index\":0,\"delta\":{\"role\":\"assistant\",\"content\":\"Hello\"},\"finish_reason\":null}]}\n\n\
                 data: [DONE]\n\n"
            )
            .create();

        // Create a client that points to our mock server
        let client = Client::builder()
            .api_key("test_api_key")
            .base_url(&server().url())
            .build()
            .unwrap();

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
        assert_eq!(chunks[0].choices[0].delta.role.as_ref().unwrap(), &crate::traits::chat::ChatRole::Assistant);
        assert_eq!(chunks[0].choices[0].delta.content.as_ref().unwrap(), "Hello");

        // Verify the mock was called
        mock_server.assert();
    }
}