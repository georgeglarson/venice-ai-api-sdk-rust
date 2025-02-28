use reqwest::Client as ReqwestClient;

use crate::config::ClientConfig;
use crate::error::{VeniceError, VeniceResult};

/// Create a new reqwest client with the given configuration
pub fn create_client(config: &ClientConfig) -> VeniceResult<ReqwestClient> {
    let headers = config.create_default_headers()?;
    let mut client_builder = ReqwestClient::builder().default_headers(headers);
    
    if let Some(timeout) = config.timeout_secs {
        client_builder = client_builder.timeout(std::time::Duration::from_secs(timeout));
    }
    
    client_builder.build().map_err(VeniceError::HttpError)
}