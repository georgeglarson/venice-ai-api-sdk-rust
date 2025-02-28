use async_trait::async_trait;
use futures::Stream;
use serde::{Deserialize, Serialize};
use std::pin::Pin;

use crate::error::{RateLimitInfo, VeniceResult};

/// Chat message roles
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ChatRole {
    /// System message
    System,
    /// User message
    User,
    /// Assistant message
    Assistant,
    /// Function message
    Function,
}

/// A chat message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    /// The role of the message author
    pub role: ChatRole,
    /// The content of the message
    pub content: String,
    /// Name of the message author
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Request for chat completions
#[derive(Debug, Clone, Serialize)]
pub struct ChatCompletionRequest {
    /// ID of the model to use
    pub model: String,
    /// The messages to generate chat completions for
    pub messages: Vec<ChatMessage>,
    /// Maximum number of tokens to generate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
    /// Sampling temperature between 0 and 2
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    /// Whether to stream the results
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
}

/// A chat completion choice
#[derive(Debug, Clone, Deserialize)]
pub struct ChatCompletionChoice {
    /// The completion message
    pub message: ChatMessage,
    /// The reason the completion stopped
    pub finish_reason: Option<String>,
    /// The index of the choice
    pub index: u32,
}

/// Usage information for a chat completion request
#[derive(Debug, Clone, Deserialize)]
pub struct ChatCompletionUsage {
    /// The number of prompt tokens used
    pub prompt_tokens: u32,
    /// The number of completion tokens used
    pub completion_tokens: u32,
    /// The total number of tokens used
    pub total_tokens: u32,
}

/// Response from the chat completions API
#[derive(Debug, Clone, Deserialize)]
pub struct ChatCompletionResponse {
    /// The ID of the chat completion
    pub id: String,
    /// The type of the object, always "chat.completion"
    pub object: String,
    /// The timestamp of when the chat completion was created
    pub created: u64,
    /// The model used for the chat completion
    pub model: String,
    /// The chat completion choices
    pub choices: Vec<ChatCompletionChoice>,
    /// The usage information for the request
    pub usage: Option<ChatCompletionUsage>,
}

/// A streaming chat completion chunk
#[derive(Debug, Clone, Deserialize)]
pub struct ChatCompletionChunk {
    /// The ID of the chat completion
    pub id: String,
    /// The type of the object, always "chat.completion.chunk"
    pub object: String,
    /// The timestamp of when the chat completion chunk was created
    pub created: u64,
    /// The model used for the chat completion
    pub model: String,
    /// The chat completion chunk choices
    pub choices: Vec<ChatCompletionChunkChoice>,
}

/// A streaming chat completion chunk choice
#[derive(Debug, Clone, Deserialize)]
pub struct ChatCompletionChunkChoice {
    /// The index of the choice
    pub index: u32,
    /// The delta content for this chunk
    pub delta: ChatCompletionChunkDelta,
    /// The reason the completion stopped, if applicable
    pub finish_reason: Option<String>,
}

/// The delta content for a streaming chat completion chunk
#[derive(Debug, Clone, Deserialize)]
pub struct ChatCompletionChunkDelta {
    /// The role of the message author, if present in this chunk
    pub role: Option<ChatRole>,
    /// The content of the message, if present in this chunk
    pub content: Option<String>,
}

/// Type alias for a stream of chat completion chunks
pub type ChatCompletionStream = Pin<Box<dyn Stream<Item = VeniceResult<ChatCompletionChunk>> + Send>>;

/// Chat API trait
#[async_trait]
pub trait ChatApi {
    /// Create a chat completion
    async fn create_chat_completion(
        &self,
        request: ChatCompletionRequest,
    ) -> VeniceResult<(ChatCompletionResponse, RateLimitInfo)>;
    
    /// Create a streaming chat completion
    ///
    /// Returns a stream of chat completion chunks that can be consumed as they arrive.
    /// This is more efficient for large responses and provides a better user experience
    /// for real-time applications.
    async fn create_streaming_chat_completion(
        &self,
        request: ChatCompletionRequest,
    ) -> VeniceResult<(ChatCompletionStream, RateLimitInfo)>;
}


/// Helper functions to create chat messages
impl ChatMessage {
    /// Create a new system message
    pub fn system(content: impl Into<String>) -> Self {
        Self {
            role: ChatRole::System,
            content: content.into(),
            name: None,
        }
    }

    /// Create a new user message
    pub fn user(content: impl Into<String>) -> Self {
        Self {
            role: ChatRole::User,
            content: content.into(),
            name: None,
        }
    }

    /// Create a new assistant message
    pub fn assistant(content: impl Into<String>) -> Self {
        Self {
            role: ChatRole::Assistant,
            content: content.into(),
            name: None,
        }
    }
}

/// Builder for chat completion requests
#[derive(Debug, Clone)]
pub struct ChatCompletionBuilder {
    request: ChatCompletionRequest,
}

impl ChatCompletionBuilder {
    /// Create a new chat completion request builder
    pub fn new(model: impl Into<String>) -> Self {
        Self {
            request: ChatCompletionRequest {
                model: model.into(),
                messages: Vec::new(),
                max_tokens: None,
                temperature: None,
                stream: None,
            },
        }
    }

    /// Add a message to the request
    pub fn add_message(mut self, message: ChatMessage) -> Self {
        self.request.messages.push(message);
        self
    }

    /// Add a system message to the request
    pub fn add_system(self, content: impl Into<String>) -> Self {
        self.add_message(ChatMessage::system(content))
    }

    /// Add a user message to the request
    pub fn add_user(self, content: impl Into<String>) -> Self {
        self.add_message(ChatMessage::user(content))
    }

    /// Set the maximum number of tokens to generate
    pub fn max_tokens(mut self, max_tokens: u32) -> Self {
        self.request.max_tokens = Some(max_tokens);
        self
    }

    /// Set the sampling temperature
    pub fn temperature(mut self, temperature: f32) -> Self {
        self.request.temperature = Some(temperature);
        self
    }

    /// Enable streaming of results
    pub fn stream(mut self, stream: bool) -> Self {
        self.request.stream = Some(stream);
        self
    }

    /// Build the chat completion request
    pub fn build(self) -> ChatCompletionRequest {
        self.request
    }
    
    /// Build and create a streaming chat completion request
    ///
    /// This is a convenience method that builds the request and calls
    /// `create_streaming_chat_completion` on the provided client.
    pub async fn stream_with_client(
        self,
        client: &impl ChatApi,
    ) -> VeniceResult<(ChatCompletionStream, RateLimitInfo)> {
        let mut request = self.request;
        request.stream = Some(true);
        client.create_streaming_chat_completion(request).await
    }
}