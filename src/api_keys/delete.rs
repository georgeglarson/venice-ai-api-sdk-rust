use serde::Deserialize;

use crate::{
    client::Client,
    error::{RateLimitInfo, VeniceResult},
};

/// The endpoint for deleting API keys
const API_KEYS_ENDPOINT: &str = "api_keys";

/// Response from deleting an API key
#[derive(Debug, Deserialize)]
pub struct DeleteApiKeyResponse {
    /// Whether the deletion was successful
    pub deleted: bool,
    /// The ID of the deleted API key
    pub id: String,
    /// Type of object
    pub object: String,
}

impl Client {
    /// Delete an API key by ID
    ///
    /// # Examples
    ///
    /// ```
    /// use venice_ai_api_sdk_rust::Client;
    /// 
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::new("your-api-key")?;
    ///     let api_key_id = "api_key_123456";
    ///     
    ///     let (response, _) = client.delete_api_key(api_key_id).await?;
    ///     
    ///     if response.deleted {
    ///         println!("Successfully deleted API key: {}", response.id);
    ///     } else {
    ///         println!("Failed to delete API key");
    ///     }
    ///     
    ///     Ok(())
    /// }
    /// ```
    pub async fn delete_api_key(
        &self,
        api_key_id: impl AsRef<str>,
    ) -> VeniceResult<(DeleteApiKeyResponse, RateLimitInfo)> {
        let endpoint = format!("{}/{}", API_KEYS_ENDPOINT, api_key_id.as_ref());
        self.delete(&endpoint).await
    }
}

/// Helper function to delete an API key
///
/// # Examples
///
/// ```
/// use venice_ai_api_sdk_rust::api_keys::delete_api_key;
/// 
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let api_key_id = "api_key_123456";
///     
///     let (response, _) = delete_api_key("your-api-key", api_key_id).await?;
///     
///     if response.deleted {
///         println!("Successfully deleted API key: {}", response.id);
///     } else {
///         println!("Failed to delete API key");
///     }
///     
///     Ok(())
/// }
/// ```
pub async fn delete_api_key(
    api_key: impl Into<String>,
    api_key_id: impl AsRef<str>,
) -> VeniceResult<(DeleteApiKeyResponse, RateLimitInfo)> {
    let client = Client::new(api_key)?;
    client.delete_api_key(api_key_id).await
}