use crate::{
    error::{RateLimitInfo, VeniceResult},
    chat::completions::ChatCompletionRequest,
    traits::chat::ChatCompletionStream,
};

/// Helper function to create a streaming chat completion
///
/// # Examples
///
/// ```
/// use venice_ai_api_sdk_rust::chat::{
///     create_streaming_chat_completion,
///     ChatCompletionRequestBuilder,
/// };
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let request = ChatCompletionRequestBuilder::new("llama-3.3-70b")
///         .add_system_message("You are a helpful assistant.")
///         .add_user_message("Tell me about AI")
///         .with_max_tokens(1000)
///         .with_temperature(0.7)
///         .with_streaming(true)
///         .build();
///     
///     let (mut stream, _) = create_streaming_chat_completion("your-api-key", request).await?;
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
    api_key: impl Into<String>,
    request: ChatCompletionRequest,
) -> VeniceResult<(ChatCompletionStream, RateLimitInfo)> {
    let client = crate::Client::new(api_key)?;
    client.create_streaming_chat_completion(request).await
}