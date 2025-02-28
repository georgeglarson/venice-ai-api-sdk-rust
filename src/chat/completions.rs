use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{
    client::Client,
    error::{RateLimitInfo, VeniceResult},
};

/// The endpoint for chat completions
const CHAT_COMPLETIONS_ENDPOINT: &str = "chat/completions";

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
    /// Alternative to max_tokens, used by compatible libraries
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_completion_tokens: Option<u32>,
    /// Sampling temperature between 0 and 2
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    /// Nucleus sampling parameter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    /// Penalizes repeated tokens
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f32>,
    /// Penalizes repeated topics
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f32>,
    /// Generate multiple completion choices
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<u32>,
    /// Whether to stream the results
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    /// Used for deterministic results
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<u64>,
    /// List of stop sequences
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<Vec<String>>,
    /// Venice-specific parameters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub venice_parameters: Option<VeniceParameters>,
    /// Additional custom parameters
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// Venice-specific parameters for chat completion requests
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VeniceParameters {
    /// Enable web search for chat completions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_web_search: Option<String>,
    /// Include Venice's default system prompt
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_venice_system_prompt: Option<bool>,
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
    /// Function call content if applicable
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_call: Option<serde_json::Value>,
}

impl ChatMessage {
    /// Create a new system message
    pub fn system(content: impl Into<String>) -> Self {
        Self {
            role: ChatRole::System,
            content: content.into(),
            name: None,
            function_call: None,
        }
    }

    /// Create a new user message
    pub fn user(content: impl Into<String>) -> Self {
        Self {
            role: ChatRole::User,
            content: content.into(),
            name: None,
            function_call: None,
        }
    }

    /// Create a new assistant message
    pub fn assistant(content: impl Into<String>) -> Self {
        Self {
            role: ChatRole::Assistant,
            content: content.into(),
            name: None,
            function_call: None,
        }
    }

    /// Create a new function message
    pub fn function(content: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            role: ChatRole::Function,
            content: content.into(),
            name: Some(name.into()),
            function_call: None,
        }
    }
}

impl Default for ChatCompletionRequest {
    fn default() -> Self {
        Self {
            model: "default".to_string(),
            messages: Vec::new(),
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
}

/// Builder for chat completion requests
#[derive(Debug, Clone)]
pub struct ChatCompletionRequestBuilder {
    request: ChatCompletionRequest,
}

impl ChatCompletionRequestBuilder {
    /// Create a new chat completion request builder
    pub fn new(model: impl Into<String>) -> Self {
        Self {
            request: ChatCompletionRequest {
                model: model.into(),
                ..Default::default()
            },
        }
    }

    /// Add a message to the request
    pub fn add_message(mut self, message: ChatMessage) -> Self {
        self.request.messages.push(message);
        self
    }

    /// Add a system message to the request
    pub fn add_system_message(self, content: impl Into<String>) -> Self {
        self.add_message(ChatMessage::system(content))
    }

    /// Add a user message to the request
    pub fn add_user_message(self, content: impl Into<String>) -> Self {
        self.add_message(ChatMessage::user(content))
    }

    /// Add an assistant message to the request
    pub fn add_assistant_message(self, content: impl Into<String>) -> Self {
        self.add_message(ChatMessage::assistant(content))
    }

    /// Set the messages for the request
    pub fn with_messages(mut self, messages: Vec<ChatMessage>) -> Self {
        self.request.messages = messages;
        self
    }

    /// Set the maximum number of tokens to generate
    pub fn with_max_tokens(mut self, max_tokens: u32) -> Self {
        self.request.max_tokens = Some(max_tokens);
        self
    }

    /// Set the maximum number of completion tokens to generate (alternative to max_tokens)
    pub fn with_max_completion_tokens(mut self, max_completion_tokens: u32) -> Self {
        self.request.max_completion_tokens = Some(max_completion_tokens);
        self
    }

    /// Set the sampling temperature
    pub fn with_temperature(mut self, temperature: f32) -> Self {
        self.request.temperature = Some(temperature);
        self
    }

    /// Set the nucleus sampling parameter
    pub fn with_top_p(mut self, top_p: f32) -> Self {
        self.request.top_p = Some(top_p);
        self
    }

    /// Set the frequency penalty
    pub fn with_frequency_penalty(mut self, frequency_penalty: f32) -> Self {
        self.request.frequency_penalty = Some(frequency_penalty);
        self
    }

    /// Set the presence penalty
    pub fn with_presence_penalty(mut self, presence_penalty: f32) -> Self {
        self.request.presence_penalty = Some(presence_penalty);
        self
    }

    /// Set the number of completion choices to generate
    pub fn with_n(mut self, n: u32) -> Self {
        self.request.n = Some(n);
        self
    }

    /// Enable streaming of results
    pub fn with_streaming(mut self, stream: bool) -> Self {
        self.request.stream = Some(stream);
        self
    }

    /// Set the random seed for deterministic results
    pub fn with_seed(mut self, seed: u64) -> Self {
        self.request.seed = Some(seed);
        self
    }

    /// Set the stop sequences
    pub fn with_stop(mut self, stop: Vec<String>) -> Self {
        self.request.stop = Some(stop);
        self
    }

    /// Enable Venice's web search capability
    pub fn with_web_search(mut self, enable: bool) -> Self {
        let venice_parameters = self.request.venice_parameters.get_or_insert(VeniceParameters {
            enable_web_search: None,
            include_venice_system_prompt: None,
        });
        venice_parameters.enable_web_search = Some(if enable { "on".to_string() } else { "off".to_string() });
        self
    }

    /// Control whether to include Venice's default system prompt
    pub fn with_venice_system_prompt(mut self, include: bool) -> Self {
        let venice_parameters = self.request.venice_parameters.get_or_insert(VeniceParameters {
            enable_web_search: None,
            include_venice_system_prompt: None,
        });
        venice_parameters.include_venice_system_prompt = Some(include);
        self
    }

    /// Add a custom parameter to the request
    pub fn with_extra(mut self, key: impl Into<String>, value: impl Into<serde_json::Value>) -> Self {
        self.request.extra.insert(key.into(), value.into());
        self
    }

    /// Build the chat completion request
    pub fn build(self) -> ChatCompletionRequest {
        self.request
    }
}

use crate::traits::chat::ChatCompletionStream;

impl Client {
    /// Create a chat completion
    ///
    /// # Examples
    ///
    /// ```
    /// use venice_ai_api_sdk_rust::{
    ///     Client,
    ///     chat::{ChatCompletionRequestBuilder, ChatMessage},
    /// };
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::new("your-api-key")?;
    ///
    ///     let request = ChatCompletionRequestBuilder::new("llama-3.3-70b")
    ///         .add_system_message("You are a helpful assistant.")
    ///         .add_user_message("Tell me about AI")
    ///         .with_max_tokens(1000)
    ///         .with_temperature(0.7)
    ///         .build();
    ///
    ///     let (response, _) = client.create_chat_completion(request).await?;
    ///
    ///     println!("Response: {}", response.choices[0].message.content);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn create_chat_completion(
        &self,
        request: ChatCompletionRequest,
    ) -> VeniceResult<(ChatCompletionResponse, RateLimitInfo)> {
        // Ensure streaming is disabled
        let mut request = request;
        request.stream = Some(false);
        
        self.post(CHAT_COMPLETIONS_ENDPOINT, &request).await
    }
    
    /// Create a streaming chat completion
    ///
    /// # Examples
    ///
    /// ```
    /// use futures::StreamExt;
    /// use venice_ai_api_sdk_rust::{
    ///     Client,
    ///     chat::{ChatCompletionRequestBuilder, ChatMessage},
    /// };
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::new("your-api-key")?;
    ///
    ///     let request = ChatCompletionRequestBuilder::new("llama-3.3-70b")
    ///         .add_system_message("You are a helpful assistant.")
    ///         .add_user_message("Tell me about AI")
    ///         .with_max_tokens(1000)
    ///         .with_temperature(0.7)
    ///         .with_streaming(true)
    ///         .build();
    ///
    ///     let (mut stream, _) = client.create_streaming_chat_completion(request).await?;
    ///
    ///     while let Some(chunk_result) = stream.next().await {
    ///         match chunk_result {
    ///             Ok(chunk) => {
    ///                 for choice in &chunk.choices {
    ///                     if let Some(content) = &choice.delta.content {
    ///                         print!("{}", content);
    ///                     }
    ///                 }
    ///             }
    ///             Err(err) => eprintln!("Error: {}", err),
    ///         }
    ///     }
    ///
    ///     println!();
    ///     Ok(())
    /// }
    /// ```
    pub async fn create_streaming_chat_completion(
        &self,
        request: ChatCompletionRequest,
    ) -> VeniceResult<(ChatCompletionStream, RateLimitInfo)> {
        // Ensure streaming is enabled
        let mut request = request;
        request.stream = Some(true);
        
        self.post_streaming::<_, crate::traits::chat::ChatCompletionChunk>(CHAT_COMPLETIONS_ENDPOINT, &request).await
    }
}

/// Helper function to create a chat completion
///
/// # Examples
///
/// ```
/// use venice_ai_api_sdk_rust::chat::{
///     create_chat_completion,
///     ChatCompletionRequestBuilder,
///     ChatMessage,
/// };
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let request = ChatCompletionRequestBuilder::new("llama-3.3-70b")
///         .add_system_message("You are a helpful assistant.")
///         .add_user_message("Tell me about AI")
///         .with_max_tokens(1000)
///         .with_temperature(0.7)
///         .build();
///     
///     let (response, _) = create_chat_completion("your-api-key", request).await?;
///     
///     println!("Response: {}", response.choices[0].message.content);
///     
///     Ok(())
/// }
/// ```
pub async fn create_chat_completion(
    api_key: impl Into<String>,
    request: ChatCompletionRequest,
) -> VeniceResult<(ChatCompletionResponse, RateLimitInfo)> {
    let client = Client::new(api_key)?;
    client.create_chat_completion(request).await
}