use async_trait::async_trait;
use futures::stream;

use crate::error::{RateLimitInfo, VeniceError, VeniceResult};
use crate::traits::chat::{
    ChatApi, ChatCompletionRequest, ChatCompletionResponse, ChatCompletionStream,
    ChatCompletionChoice, ChatMessage, ChatRole, ChatCompletionChunk, ChatCompletionChunkChoice,
    ChatCompletionChunkDelta,
};

/// A test implementation of the ChatApi trait that returns predefined responses
/// without making actual HTTP requests.
pub struct TestChatClient {
    /// Predefined response for create_chat_completion
    pub chat_completion_response: Option<ChatCompletionResponse>,
    /// Predefined error for create_chat_completion
    pub chat_completion_error: Option<ErrorConfig>,
    /// Predefined chunks for create_streaming_chat_completion
    pub streaming_chunks: Vec<ChatCompletionChunk>,
    /// Predefined error for create_streaming_chat_completion
    pub streaming_error: Option<ErrorConfig>,
    /// Rate limit info to return with responses
    pub rate_limit_info: RateLimitInfo,
}

/// Configuration for creating error responses
pub struct ErrorConfig {
    /// HTTP status code
    pub status: reqwest::StatusCode,
    /// Error code
    pub code: String,
    /// Error message
    pub message: String,
}

impl TestChatClient {
    /// Create a new TestChatClient with default values
    pub fn new() -> Self {
        Self {
            chat_completion_response: None,
            chat_completion_error: None,
            streaming_chunks: Vec::new(),
            streaming_error: None,
            rate_limit_info: RateLimitInfo {
                limit_requests: Some(1000),
                remaining_requests: Some(999),
                reset_requests: Some(3600),
                limit_tokens: Some(10000),
                remaining_tokens: Some(9999),
                reset_tokens: Some(3600),
                balance_vcu: Some(100.0),
                balance_usd: Some(10.0),
            },
        }
    }

    /// Set the predefined response for create_chat_completion
    pub fn with_chat_completion_response(mut self, response: ChatCompletionResponse) -> Self {
        self.chat_completion_response = Some(response);
        self
    }

    /// Set the predefined error for create_chat_completion
    pub fn with_chat_completion_error(mut self, error: ErrorConfig) -> Self {
        self.chat_completion_error = Some(error);
        self
    }

    /// Set the predefined chunks for create_streaming_chat_completion
    pub fn with_streaming_chunks(mut self, chunks: Vec<ChatCompletionChunk>) -> Self {
        self.streaming_chunks = chunks;
        self
    }

    /// Set the predefined error for create_streaming_chat_completion
    pub fn with_streaming_error(mut self, error: ErrorConfig) -> Self {
        self.streaming_error = Some(error);
        self
    }

    /// Set the rate limit info to return with responses
    pub fn with_rate_limit_info(mut self, rate_limit_info: RateLimitInfo) -> Self {
        self.rate_limit_info = rate_limit_info;
        self
    }

    /// Create a default success response based on a request
    pub fn default_success_response(request: &ChatCompletionRequest) -> ChatCompletionResponse {
        ChatCompletionResponse {
            id: "chatcmpl-123".to_string(),
            object: "chat.completion".to_string(),
            created: 1677652288,
            model: request.model.clone(),
            choices: vec![ChatCompletionChoice {
                message: ChatMessage {
                    role: ChatRole::Assistant,
                    content: "This is a test response".to_string(),
                    name: None,
                },
                finish_reason: Some("stop".to_string()),
                index: 0,
            }],
            usage: None,
        }
    }

    /// Create default streaming chunks based on a request
    pub fn default_streaming_chunks(request: &ChatCompletionRequest) -> Vec<ChatCompletionChunk> {
        vec![
            ChatCompletionChunk {
                id: "chatcmpl-123".to_string(),
                object: "chat.completion.chunk".to_string(),
                created: 1677652288,
                model: request.model.clone(),
                choices: vec![ChatCompletionChunkChoice {
                    index: 0,
                    delta: ChatCompletionChunkDelta {
                        role: Some(ChatRole::Assistant),
                        content: Some("This ".to_string()),
                    },
                    finish_reason: None,
                }],
            },
            ChatCompletionChunk {
                id: "chatcmpl-123".to_string(),
                object: "chat.completion.chunk".to_string(),
                created: 1677652288,
                model: request.model.clone(),
                choices: vec![ChatCompletionChunkChoice {
                    index: 0,
                    delta: ChatCompletionChunkDelta {
                        role: None,
                        content: Some("is ".to_string()),
                    },
                    finish_reason: None,
                }],
            },
            ChatCompletionChunk {
                id: "chatcmpl-123".to_string(),
                object: "chat.completion.chunk".to_string(),
                created: 1677652288,
                model: request.model.clone(),
                choices: vec![ChatCompletionChunkChoice {
                    index: 0,
                    delta: ChatCompletionChunkDelta {
                        role: None,
                        content: Some("a ".to_string()),
                    },
                    finish_reason: None,
                }],
            },
            ChatCompletionChunk {
                id: "chatcmpl-123".to_string(),
                object: "chat.completion.chunk".to_string(),
                created: 1677652288,
                model: request.model.clone(),
                choices: vec![ChatCompletionChunkChoice {
                    index: 0,
                    delta: ChatCompletionChunkDelta {
                        role: None,
                        content: Some("test ".to_string()),
                    },
                    finish_reason: None,
                }],
            },
            ChatCompletionChunk {
                id: "chatcmpl-123".to_string(),
                object: "chat.completion.chunk".to_string(),
                created: 1677652288,
                model: request.model.clone(),
                choices: vec![ChatCompletionChunkChoice {
                    index: 0,
                    delta: ChatCompletionChunkDelta {
                        role: None,
                        content: Some("response".to_string()),
                    },
                    finish_reason: Some("stop".to_string()),
                }],
            },
        ]
    }
}

#[async_trait]
impl ChatApi for TestChatClient {
    async fn create_chat_completion(
        &self,
        request: ChatCompletionRequest,
    ) -> VeniceResult<(ChatCompletionResponse, RateLimitInfo)> {
        // Return predefined error if set
        if let Some(error_config) = &self.chat_completion_error {
            return Err(VeniceError::ApiError {
                status: error_config.status,
                code: error_config.code.clone(),
                message: error_config.message.clone(),
            });
        }

        // Return predefined response if set, otherwise create a default response
        let response = self.chat_completion_response.clone()
            .unwrap_or_else(|| Self::default_success_response(&request));

        Ok((response, self.rate_limit_info.clone()))
    }

    async fn create_streaming_chat_completion(
        &self,
        request: ChatCompletionRequest,
    ) -> VeniceResult<(ChatCompletionStream, RateLimitInfo)> {
        // Return predefined error if set
        if let Some(error_config) = &self.streaming_error {
            return Err(VeniceError::ApiError {
                status: error_config.status,
                code: error_config.code.clone(),
                message: error_config.message.clone(),
            });
        }

        // Use predefined chunks if set, otherwise create default chunks
        let chunks = if !self.streaming_chunks.is_empty() {
            self.streaming_chunks.clone()
        } else {
            Self::default_streaming_chunks(&request)
        };

        // Create a stream from the chunks
        let stream = stream::iter(chunks.into_iter().map(Ok));
        let boxed_stream: ChatCompletionStream = Box::pin(stream);

        Ok((boxed_stream, self.rate_limit_info.clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::StreamExt;
    use reqwest::StatusCode;

    #[tokio::test]
    async fn test_create_chat_completion() {
        // Create a test client with a predefined response
        let response = ChatCompletionResponse {
            id: "test-id".to_string(),
            object: "chat.completion".to_string(),
            created: 1234567890,
            model: "test-model".to_string(),
            choices: vec![ChatCompletionChoice {
                message: ChatMessage {
                    role: ChatRole::Assistant,
                    content: "Hello, world!".to_string(),
                    name: None,
                },
                finish_reason: Some("stop".to_string()),
                index: 0,
            }],
            usage: None,
        };

        let client = TestChatClient::new().with_chat_completion_response(response.clone());

        // Create a request
        let request = ChatCompletionRequest {
            model: "test-model".to_string(),
            messages: vec![ChatMessage {
                role: ChatRole::User,
                content: "Hello".to_string(),
                name: None,
            }],
            max_tokens: None,
            temperature: None,
            stream: None,
        };

        // Send the request
        let (result, _) = client.create_chat_completion(request).await.unwrap();

        // Verify the response
        assert_eq!(result.id, "test-id");
        assert_eq!(result.choices[0].message.content, "Hello, world!");
    }

    #[tokio::test]
    async fn test_create_streaming_chat_completion() {
        // Create a test client with predefined chunks
        let chunks = vec![
            ChatCompletionChunk {
                id: "test-id".to_string(),
                object: "chat.completion.chunk".to_string(),
                created: 1234567890,
                model: "test-model".to_string(),
                choices: vec![ChatCompletionChunkChoice {
                    index: 0,
                    delta: ChatCompletionChunkDelta {
                        role: Some(ChatRole::Assistant),
                        content: Some("Hello".to_string()),
                    },
                    finish_reason: None,
                }],
            },
            ChatCompletionChunk {
                id: "test-id".to_string(),
                object: "chat.completion.chunk".to_string(),
                created: 1234567890,
                model: "test-model".to_string(),
                choices: vec![ChatCompletionChunkChoice {
                    index: 0,
                    delta: ChatCompletionChunkDelta {
                        role: None,
                        content: Some(", world!".to_string()),
                    },
                    finish_reason: Some("stop".to_string()),
                }],
            },
        ];

        let client = TestChatClient::new().with_streaming_chunks(chunks.clone());

        // Create a request
        let request = ChatCompletionRequest {
            model: "test-model".to_string(),
            messages: vec![ChatMessage {
                role: ChatRole::User,
                content: "Hello".to_string(),
                name: None,
            }],
            max_tokens: None,
            temperature: None,
            stream: Some(true),
        };

        // Send the request
        let (stream, _) = client.create_streaming_chat_completion(request).await.unwrap();

        // Collect all chunks from the stream
        let result: Vec<ChatCompletionChunk> = stream
            .map(|result| result.unwrap())
            .collect::<Vec<_>>()
            .await;

        // Verify the chunks
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].choices[0].delta.content.as_ref().unwrap(), "Hello");
        assert_eq!(result[1].choices[0].delta.content.as_ref().unwrap(), ", world!");
    }

    #[tokio::test]
    async fn test_create_chat_completion_error() {
        // Create a test client with a predefined error
        let error_config = ErrorConfig {
            status: StatusCode::UNAUTHORIZED,
            code: "test_error".to_string(),
            message: "Test error".to_string(),
        };

        let client = TestChatClient::new().with_chat_completion_error(error_config);

        // Create a request
        let request = ChatCompletionRequest {
            model: "test-model".to_string(),
            messages: vec![ChatMessage {
                role: ChatRole::User,
                content: "Hello".to_string(),
                name: None,
            }],
            max_tokens: None,
            temperature: None,
            stream: None,
        };

        // Send the request and expect an error
        let result = client.create_chat_completion(request).await;
        assert!(result.is_err());

        // Verify the error
        match result.unwrap_err() {
            VeniceError::ApiError { code, message, .. } => {
                assert_eq!(code, "test_error");
                assert_eq!(message, "Test error");
            }
            _ => panic!("Expected ApiError"),
        }
    }

    #[tokio::test]
    async fn test_create_streaming_chat_completion_error() {
        // Create a test client with a predefined error
        let error_config = ErrorConfig {
            status: StatusCode::UNAUTHORIZED,
            code: "test_error".to_string(),
            message: "Test error".to_string(),
        };

        let client = TestChatClient::new().with_streaming_error(error_config);

        // Create a request
        let request = ChatCompletionRequest {
            model: "test-model".to_string(),
            messages: vec![ChatMessage {
                role: ChatRole::User,
                content: "Hello".to_string(),
                name: None,
            }],
            max_tokens: None,
            temperature: None,
            stream: Some(true),
        };

        // Send the request and expect an error
        let result = client.create_streaming_chat_completion(request).await;
        assert!(result.is_err());

        // Verify the error - use a pattern match on the Result directly
        if let Err(VeniceError::ApiError { code, message, .. }) = result {
            assert_eq!(code, "test_error");
            assert_eq!(message, "Test error");
        } else {
            panic!("Expected ApiError");
        }
    }
}