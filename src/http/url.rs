use url::Url;

use crate::error::{VeniceError, VeniceResult};

/// Build a URL from a base URL and an endpoint
pub fn build_url(base_url: &str, endpoint: &str) -> VeniceResult<Url> {
    let mut url_str = base_url.to_string();
    if !url_str.ends_with('/') && !endpoint.starts_with('/') {
        url_str.push('/');
    }
    url_str.push_str(endpoint);

    Url::parse(&url_str).map_err(|err| {
        VeniceError::InvalidInput(format!("Invalid URL: {} - {}", url_str, err))
    })
}