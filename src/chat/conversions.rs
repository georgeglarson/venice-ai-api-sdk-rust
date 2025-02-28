use crate::traits::chat as traits;
use crate::chat::completions as chat;
use crate::models::chat as models;
use std::collections::HashMap;

/// Implement conversion from traits::chat::ChatCompletionRequest to models::chat::ChatCompletionRequest
impl From<traits::ChatCompletionRequest> for models::ChatCompletionRequest {
    fn from(request: traits::ChatCompletionRequest) -> Self {
        Self {
            model: request.model,
            messages: request.messages.into_iter().map(Into::into).collect(),
            max_tokens: request.max_tokens,
            max_completion_tokens: None,
            temperature: request.temperature,
            top_p: None,
            frequency_penalty: None,
            presence_penalty: None,
            n: None,
            stream: request.stream,
            seed: None,
            stop: None,
            venice_parameters: None,
            extra: HashMap::new(),
        }
    }
}

/// Implement conversion from traits::chat::ChatMessage to models::chat::ChatMessage
impl From<traits::ChatMessage> for models::ChatMessage {
    fn from(message: traits::ChatMessage) -> Self {
        Self {
            role: message.role.into(),
            content: message.content,
        }
    }
}

/// Implement conversion from traits::chat::ChatRole to models::chat::ChatRole
impl From<traits::ChatRole> for models::ChatRole {
    fn from(role: traits::ChatRole) -> Self {
        match role {
            traits::ChatRole::System => models::ChatRole::System,
            traits::ChatRole::User => models::ChatRole::User,
            traits::ChatRole::Assistant => models::ChatRole::Assistant,
            traits::ChatRole::Function => panic!("Function role not supported in models::ChatRole"),
        }
    }
}

/// Implement conversion from models::chat::ChatCompletionRequest to traits::chat::ChatCompletionRequest
impl From<models::ChatCompletionRequest> for traits::ChatCompletionRequest {
    fn from(request: models::ChatCompletionRequest) -> Self {
        Self {
            model: request.model,
            messages: request.messages.into_iter().map(Into::into).collect(),
            max_tokens: request.max_tokens,
            temperature: request.temperature,
            stream: request.stream,
        }
    }
}

/// Implement conversion from models::chat::ChatMessage to traits::chat::ChatMessage
impl From<models::ChatMessage> for traits::ChatMessage {
    fn from(message: models::ChatMessage) -> Self {
        Self {
            role: message.role.into(),
            content: message.content,
            name: None,
        }
    }
}

/// Implement conversion from models::chat::ChatRole to traits::chat::ChatRole
impl From<models::ChatRole> for traits::ChatRole {
    fn from(role: models::ChatRole) -> Self {
        match role {
            models::ChatRole::System => traits::ChatRole::System,
            models::ChatRole::User => traits::ChatRole::User,
            models::ChatRole::Assistant => traits::ChatRole::Assistant,
        }
    }
}

/// Implement conversion from traits::chat::ChatCompletionRequest to chat::completions::ChatCompletionRequest
impl From<traits::ChatCompletionRequest> for chat::ChatCompletionRequest {
    fn from(request: traits::ChatCompletionRequest) -> Self {
        Self {
            model: request.model,
            messages: request.messages.into_iter().map(Into::into).collect(),
            max_tokens: request.max_tokens,
            max_completion_tokens: None,
            temperature: request.temperature,
            top_p: None,
            frequency_penalty: None,
            presence_penalty: None,
            n: None,
            stream: request.stream,
            seed: None,
            stop: None,
            venice_parameters: None,
            extra: HashMap::new(),
        }
    }
}

/// Implement conversion from traits::chat::ChatMessage to chat::completions::ChatMessage
impl From<traits::ChatMessage> for chat::ChatMessage {
    fn from(message: traits::ChatMessage) -> Self {
        Self {
            role: message.role.into(),
            content: message.content,
            name: message.name,
            function_call: None,
        }
    }
}

/// Implement conversion from traits::chat::ChatRole to chat::completions::ChatRole
impl From<traits::ChatRole> for chat::ChatRole {
    fn from(role: traits::ChatRole) -> Self {
        match role {
            traits::ChatRole::System => chat::ChatRole::System,
            traits::ChatRole::User => chat::ChatRole::User,
            traits::ChatRole::Assistant => chat::ChatRole::Assistant,
            traits::ChatRole::Function => chat::ChatRole::Function,
        }
    }
}

/// Implement conversion from chat::completions::ChatRole to traits::chat::ChatRole
impl From<chat::ChatRole> for traits::ChatRole {
    fn from(role: chat::ChatRole) -> Self {
        match role {
            chat::ChatRole::System => traits::ChatRole::System,
            chat::ChatRole::User => traits::ChatRole::User,
            chat::ChatRole::Assistant => traits::ChatRole::Assistant,
            chat::ChatRole::Function => traits::ChatRole::Function,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_chat_request_conversion_to_chat() {
        let traits_request = traits::ChatCompletionRequest {
            model: "llama-3.3-70b".to_string(),
            messages: vec![
                traits::ChatMessage {
                    role: traits::ChatRole::User,
                    content: "Hello".to_string(),
                    name: None,
                },
            ],
            max_tokens: Some(100),
            temperature: Some(0.7),
            stream: Some(true),
        };
        
        let chat_request: chat::ChatCompletionRequest = traits_request.into();
        
        assert_eq!(chat_request.model, "llama-3.3-70b");
        assert_eq!(chat_request.messages.len(), 1);
        assert_eq!(chat_request.messages[0].role, chat::ChatRole::User);
        assert_eq!(chat_request.messages[0].content, "Hello");
        assert_eq!(chat_request.max_tokens, Some(100));
        assert_eq!(chat_request.temperature, Some(0.7));
        assert_eq!(chat_request.stream, Some(true));
    }
    
    #[test]
    fn test_chat_request_conversion_to_models() {
        let traits_request = traits::ChatCompletionRequest {
            model: "llama-3.3-70b".to_string(),
            messages: vec![
                traits::ChatMessage {
                    role: traits::ChatRole::User,
                    content: "Hello".to_string(),
                    name: None,
                },
            ],
            max_tokens: Some(100),
            temperature: Some(0.7),
            stream: Some(true),
        };
        
        let models_request: models::ChatCompletionRequest = traits_request.into();
        
        assert_eq!(models_request.model, "llama-3.3-70b");
        assert_eq!(models_request.messages.len(), 1);
        assert_eq!(models_request.messages[0].role, models::ChatRole::User);
        assert_eq!(models_request.messages[0].content, "Hello");
        assert_eq!(models_request.max_tokens, Some(100));
        assert_eq!(models_request.temperature, Some(0.7));
        assert_eq!(models_request.stream, Some(true));
    }
    
    #[test]
    fn test_models_to_traits_conversion() {
        let models_request = models::ChatCompletionRequest::new(
            "llama-3.3-70b",
            vec![
                models::ChatMessage::new(models::ChatRole::User, "Hello"),
            ],
        )
        .max_tokens(100)
        .temperature(0.7)
        .stream(true);
        
        let traits_request: traits::ChatCompletionRequest = models_request.into();
        
        assert_eq!(traits_request.model, "llama-3.3-70b");
        assert_eq!(traits_request.messages.len(), 1);
        assert_eq!(traits_request.messages[0].role, traits::ChatRole::User);
        assert_eq!(traits_request.messages[0].content, "Hello");
        assert_eq!(traits_request.max_tokens, Some(100));
        assert_eq!(traits_request.temperature, Some(0.7));
        assert_eq!(traits_request.stream, Some(true));
    }
}