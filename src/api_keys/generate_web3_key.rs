use serde::{Deserialize, Serialize};

use crate::{
    client::Client,
    error::{RateLimitInfo, VeniceResult},
};

/// The endpoint for generating a Web3 API key
const GENERATE_WEB3_KEY_ENDPOINT: &str = "api_keys/generate_web3_key";

/// Request for generating a Web3 API key
#[derive(Debug, Clone, Serialize)]
pub struct GenerateWeb3KeyRequest {
    /// The wallet address to associate with the key
    pub wallet_address: String,
    /// Optional name for the API key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Response from generating a Web3 API key
#[derive(Debug, Deserialize)]
pub struct GenerateWeb3KeyResponse {
    /// The generated API key data
    pub data: Web3KeyData,
    /// Type of object
    pub object: String,
}

/// Data for a generated Web3 API key
#[derive(Debug, Deserialize, Clone)]
pub struct Web3KeyData {
    /// The API key identifier
    pub id: String,
    /// The type of object
    pub object: String,
    /// The name/description of the API key
    #[serde(rename = "description")]
    pub name: String,
    /// When the API key was created (parsed from ISO date string)
    #[serde(rename = "createdAt")]
    pub created_at: String,
    /// The full API key value (only returned on creation)
    pub key: String,
    /// The wallet address associated with this key
    pub wallet_address: String,
    /// When the API key expires
    #[serde(rename = "expiresAt", default)]
    pub expires_at: Option<String>,
}

impl Client {
    /// Generate a Web3 API key
    ///
    /// # Examples
    ///
    /// ```
    /// use venice_ai_api_sdk_rust::{Client, api_keys::GenerateWeb3KeyRequest};
    /// 
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::new("your-api-key")?;
    ///     
    ///     let request = GenerateWeb3KeyRequest {
    ///         wallet_address: "0x1234567890123456789012345678901234567890".to_string(),
    ///         name: Some("My Web3 API Key".to_string()),
    ///     };
    ///     
    ///     let (response, _) = client.generate_web3_key(request).await?;
    ///     
    ///     println!("Generated Web3 API Key:");
    ///     println!("ID: {}", response.data.id);
    ///     println!("Key: {}", response.data.key);
    ///     println!("Wallet Address: {}", response.data.wallet_address);
    ///     
    ///     Ok(())
    /// }
    /// ```
    pub async fn generate_web3_key(
        &self,
        request: GenerateWeb3KeyRequest,
    ) -> VeniceResult<(GenerateWeb3KeyResponse, RateLimitInfo)> {
        self.post(GENERATE_WEB3_KEY_ENDPOINT, &request).await
    }
}

/// Helper function to generate a Web3 API key
///
/// # Examples
///
/// ```
/// use venice_ai_api_sdk_rust::api_keys::{generate_web3_key, GenerateWeb3KeyRequest};
/// 
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let request = GenerateWeb3KeyRequest {
///         wallet_address: "0x1234567890123456789012345678901234567890".to_string(),
///         name: Some("My Web3 API Key".to_string()),
///     };
///     
///     let (response, _) = generate_web3_key("your-api-key", request).await?;
///     
///     println!("Generated Web3 API Key: {}", response.data.key);
///     
///     Ok(())
/// }
/// ```
pub async fn generate_web3_key(
    api_key: impl Into<String>,
    request: GenerateWeb3KeyRequest,
) -> VeniceResult<(GenerateWeb3KeyResponse, RateLimitInfo)> {
    let client = Client::new(api_key)?;
    client.generate_web3_key(request).await
}