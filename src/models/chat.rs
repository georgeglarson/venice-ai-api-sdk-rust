//! Chat models for the Venice AI API
//!
//! This module provides models for the chat completion API.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A role for a chat message
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ChatRole {
    /// The system role, used for setting the behavior of the assistant
    System,
    /// The user role, used for user messages
    User,
    /// The assistant role, used for assistant responses
    Assistant,
}

impl std::fmt::Display for ChatRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChatRole::System => write!(f, "system"),
            ChatRole::User => write!(f, "user"),
            ChatRole::Assistant => write!(f, "assistant"),
        }
    }
}

/// A chat message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    /// The role of the message sender
    pub role: ChatRole,
    /// The content of the message
    pub content: String,
}

impl ChatMessage {
    /// Create a new chat message
    pub fn new(role: ChatRole, content: impl Into<String>) -> Self {
        Self {
            role,
            content: content.into(),
        }
    }
    
    /// Create a new system message
    pub fn system(content: impl Into<String>) -> Self {
        Self::new(ChatRole::System, content)
    }
    
    /// Create a new user message
    pub fn user(content: impl Into<String>) -> Self {
        Self::new(ChatRole::User, content)
    }
    
    /// Create a new assistant message
    pub fn assistant(content: impl Into<String>) -> Self {
        Self::new(ChatRole::Assistant, content)
    }
}

/// A request to create a chat completion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatCompletionRequest {
    /// The model to use for the completion
    pub model: String,
    
    /// The messages to generate a completion for
    pub messages: Vec<ChatMessage>,
    
    /// The maximum number of tokens to generate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
    
    /// The maximum number of tokens to generate (alias for max_tokens)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_completion_tokens: Option<u32>,
    
    /// The sampling temperature
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    
    /// The nucleus sampling parameter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    
    /// The frequency penalty
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f32>,
    
    /// The presence penalty
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f32>,
    
    /// The number of completions to generate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<u32>,
    
    /// Whether to stream the response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    
    /// The random seed to use for sampling
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<u64>,
    
    /// The stop sequences to use
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<Vec<String>>,
    
    /// Venice-specific parameters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub venice_parameters: Option<HashMap<String, serde_json::Value>>,
    
    /// Extra parameters to include in the request
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

impl ChatCompletionRequest {
    /// Create a new chat completion request
    pub fn new(model: impl Into<String>, messages: Vec<ChatMessage>) -> Self {
        Self {
            model: model.into(),
            messages,
            max_tokens: None,
            max_completion_tokens: None,
            temperature: None,
            top_p: None,
            frequency_penalty: None,
            presence_penalty: None,
            n: None,
            stream: None,
            seed: None,
            stop: None,
            venice_parameters: None,
            extra: HashMap::new(),
        }
    }
    
    /// Set the maximum number of tokens to generate
    pub fn max_tokens(mut self, max_tokens: u32) -> Self {
        self.max_tokens = Some(max_tokens);
        self
    }
    
    /// Set the maximum number of tokens to generate (alias for max_tokens)
    pub fn max_completion_tokens(mut self, max_completion_tokens: u32) -> Self {
        self.max_completion_tokens = Some(max_completion_tokens);
        self
    }
    
    /// Set the sampling temperature
    pub fn temperature(mut self, temperature: f32) -> Self {
        self.temperature = Some(temperature);
        self
    }
    
    /// Set the nucleus sampling parameter
    pub fn top_p(mut self, top_p: f32) -> Self {
        self.top_p = Some(top_p);
        self
    }
    
    /// Set the frequency penalty
    pub fn frequency_penalty(mut self, frequency_penalty: f32) -> Self {
        self.frequency_penalty = Some(frequency_penalty);
        self
    }
    
    /// Set the presence penalty
    pub fn presence_penalty(mut self, presence_penalty: f32) -> Self {
        self.presence_penalty = Some(presence_penalty);
        self
    }
    
    /// Set the number of completions to generate
    pub fn n(mut self, n: u32) -> Self {
        self.n = Some(n);
        self
    }
    
    /// Set whether to stream the response
    pub fn stream(mut self, stream: bool) -> Self {
        self.stream = Some(stream);
        self
    }
    
    /// Set the random seed to use for sampling
    pub fn seed(mut self, seed: u64) -> Self {
        self.seed = Some(seed);
        self
    }
    
    /// Set the stop sequences to use
    pub fn stop(mut self, stop: Vec<String>) -> Self {
        self.stop = Some(stop);
        self
    }
    
    /// Add a stop sequence
    pub fn add_stop(mut self, stop: impl Into<String>) -> Self {
        let stop = stop.into();
        match &mut self.stop {
            Some(stops) => {
                stops.push(stop);
            }
            None => {
                self.stop = Some(vec![stop]);
            }
        }
        self
    }
    
    /// Set a Venice-specific parameter
    pub fn venice_parameter(mut self, key: impl Into<String>, value: impl Into<serde_json::Value>) -> Self {
        let key = key.into();
        let value = value.into();
        match &mut self.venice_parameters {
            Some(params) => {
                params.insert(key, value);
            }
            None => {
                let mut params = HashMap::new();
                params.insert(key, value);
                self.venice_parameters = Some(params);
            }
        }
        self
    }
    
    /// Set an extra parameter
    pub fn extra(mut self, key: impl Into<String>, value: impl Into<serde_json::Value>) -> Self {
        self.extra.insert(key.into(), value.into());
        self
    }
}

/// A chat completion choice
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatCompletionChoice {
    /// The index of the choice
    pub index: u32,
    
    /// The message
    pub message: ChatMessage,
    
    /// The reason the completion stopped
    pub finish_reason: Option<String>,
}

/// A chat completion response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatCompletionResponse {
    /// The ID of the completion
    pub id: String,
    
    /// The object type
    pub object: String,
    
    /// The timestamp of the completion
    pub created: u64,
    
    /// The model used for the completion
    pub model: String,
    
    /// The choices
    pub choices: Vec<ChatCompletionChoice>,
    
    /// The usage statistics
    pub usage: Option<ChatCompletionUsage>,
}

/// Usage statistics for a chat completion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatCompletionUsage {
    /// The number of prompt tokens used
    pub prompt_tokens: u32,
    
    /// The number of completion tokens used
    pub completion_tokens: u32,
    
    /// The total number of tokens used
    pub total_tokens: u32,
}

/// A streaming chat completion chunk
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatCompletionChunk {
    /// The ID of the completion
    pub id: String,
    
    /// The object type
    pub object: String,
    
    /// The timestamp of the chunk
    pub created: u64,
    
    /// The model used for the completion
    pub model: String,
    
    /// The choices
    pub choices: Vec<ChatCompletionChunkChoice>,
}

/// A streaming chat completion choice
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatCompletionChunkChoice {
    /// The index of the choice
    pub index: u32,
    
    /// The delta
    pub delta: ChatCompletionChunkDelta,
    
    /// The reason the completion stopped
    pub finish_reason: Option<String>,
}

/// A streaming chat completion delta
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatCompletionChunkDelta {
    /// The role of the message sender
    pub role: Option<ChatRole>,
    
    /// The content of the message
    pub content: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_chat_message_creation() {
        let system = ChatMessage::system("You are a helpful assistant.");
        assert_eq!(system.role, ChatRole::System);
        assert_eq!(system.content, "You are a helpful assistant.");
        
        let user = ChatMessage::user("Hello!");
        assert_eq!(user.role, ChatRole::User);
        assert_eq!(user.content, "Hello!");
        
        let assistant = ChatMessage::assistant("Hi there!");
        assert_eq!(assistant.role, ChatRole::Assistant);
        assert_eq!(assistant.content, "Hi there!");
    }
    
    #[test]
    fn test_chat_completion_request_builder() {
        let request = ChatCompletionRequest::new("gpt-4", vec![
            ChatMessage::system("You are a helpful assistant."),
            ChatMessage::user("Hello!"),
        ])
        .max_tokens(100)
        .temperature(0.7)
        .stream(true)
        .add_stop("\n")
        .venice_parameter("some_param", "value")
        .extra("custom_param", 42);
        
        assert_eq!(request.model, "gpt-4");
        assert_eq!(request.messages.len(), 2);
        assert_eq!(request.max_tokens, Some(100));
        assert_eq!(request.temperature, Some(0.7));
        assert_eq!(request.stream, Some(true));
        assert_eq!(request.stop, Some(vec!["\n".to_string()]));
        
        let venice_params = request.venice_parameters.unwrap();
        assert_eq!(venice_params.get("some_param").unwrap().as_str().unwrap(), "value");
        
        assert_eq!(request.extra.get("custom_param").unwrap().as_i64().unwrap(), 42);
    }
}