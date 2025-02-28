//! Chat API implementation
//!
//! This module provides an implementation of the chat API.

use async_trait::async_trait;
use crate::error::{RateLimitInfo, VeniceResult};
use crate::http::SharedHttpClient;
use crate::models::chat::ChatCompletionRequest;
use crate::traits::chat::{ChatApi, ChatCompletionStream};

/// Implementation of the chat API
#[derive(Debug, Clone)]
pub struct ChatApiImpl {
    /// The HTTP client to use for requests
    http_client: SharedHttpClient,
}

impl ChatApiImpl {
    /// Create a new chat API implementation
    pub fn new(http_client: SharedHttpClient) -> Self {
        Self { http_client }
    }
}

#[async_trait]
impl ChatApi for ChatApiImpl {
    async fn create_chat_completion(
        &self,
        request: crate::traits::chat::ChatCompletionRequest,
    ) -> VeniceResult<(crate::traits::chat::ChatCompletionResponse, RateLimitInfo)> {
        // Convert the request to the models type
        let models_request: ChatCompletionRequest = request.into();
        
        // Send the request
        let (response, rate_limit_info) = self.http_client.post::<_, crate::models::chat::ChatCompletionResponse>("chat/completions", &models_request).await?;
        
        // Convert the response to the traits type
        let traits_response = crate::traits::chat::ChatCompletionResponse {
            id: response.id,
            object: response.object,
            created: response.created,
            model: response.model,
            choices: response.choices.into_iter().map(|choice| {
                crate::traits::chat::ChatCompletionChoice {
                    index: choice.index,
                    message: crate::traits::chat::ChatMessage {
                        role: choice.message.role.into(),
                        content: choice.message.content,
                        name: None,
                    },
                    finish_reason: choice.finish_reason,
                }
            }).collect(),
            usage: response.usage.map(|usage| {
                crate::traits::chat::ChatCompletionUsage {
                    prompt_tokens: usage.prompt_tokens,
                    completion_tokens: usage.completion_tokens,
                    total_tokens: usage.total_tokens,
                }
            }),
        };
        
        Ok((traits_response, rate_limit_info))
    }
    
    async fn create_streaming_chat_completion(
        &self,
        request: crate::traits::chat::ChatCompletionRequest,
    ) -> VeniceResult<(ChatCompletionStream, RateLimitInfo)> {
        // Convert the request to the models type
        let mut models_request: ChatCompletionRequest = request.into();
        
        // Ensure streaming is enabled
        models_request.stream = Some(true);
        
        // Send the request
        let (stream, rate_limit_info) = self.http_client.post_streaming::<_, crate::traits::chat::ChatCompletionChunk>("chat/completions", &models_request).await?;
        
        Ok((stream, rate_limit_info))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::http::{HttpClientConfig, new_shared_http_client};
    use crate::traits::chat::ChatCompletionBuilder;
    
    #[tokio::test]
    async fn test_create_chat_completion() {
        // Create a mock HTTP client
        let config = HttpClientConfig {
            api_key: "test_api_key".to_string(),
            base_url: "https://api.venice.ai".to_string(),
            custom_headers: reqwest::header::HeaderMap::new(),
            timeout_secs: None,
        };
        let http_client = new_shared_http_client(config).unwrap();
        
        // Create the chat API implementation
        let chat_api = ChatApiImpl::new(http_client);
        
        // Create a request
        let _request = ChatCompletionBuilder::new("llama-3.3-70b")
            .add_user("Hello")
            .max_tokens(100)
            .temperature(0.7)
            .build();
        
        // TODO: Mock the HTTP client to return a response
        // For now, we'll just check that the method exists and has the right signature
        let _: ChatApiImpl = chat_api;
    }
}