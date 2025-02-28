use futures::{Stream, StreamExt, TryStreamExt};
use reqwest::Response;
use serde::de::DeserializeOwned;
use std::pin::Pin;

use crate::error::{RateLimitInfo, VeniceError, VeniceResult};

/// Process a response from the API
pub async fn process_response<T: DeserializeOwned>(
    response: Response,
) -> VeniceResult<(T, RateLimitInfo)> {
    let rate_limit_info = RateLimitInfo::from_headers(response.headers());
    let status = response.status();

    if status.as_u16() == 429 {
        return Err(VeniceError::RateLimitExceeded(format!(
            "Rate limit exceeded: {}",
            rate_limit_info
        )));
    }

    if !status.is_success() {
        let error_text = response.text().await.unwrap_or_default();
        let error_response = serde_json::from_str::<serde_json::Value>(&error_text)
            .unwrap_or_else(|_| serde_json::json!({"error": {"message": error_text}}));

        // Handle different error response formats
        let (code, message) = if let Some(error_obj) = error_response.get("error") {
            if let Some(error_obj) = error_obj.as_object() {
                // Standard error format with error object
                let code = error_obj
                    .get("code")
                    .and_then(|c| c.as_str())
                    .unwrap_or("unknown")
                    .to_string();

                let message = error_obj
                    .get("message")
                    .and_then(|m| m.as_str())
                    .unwrap_or("Unknown error")
                    .to_string();
                
                (code, message)
            } else if let Some(error_str) = error_obj.as_str() {
                // Simple error format with just an error string
                ("api_error".to_string(), error_str.to_string())
            } else {
                // Fallback for other formats
                ("unknown".to_string(), format!("Unexpected error format: {}", error_response))
            }
        } else {
            // Fallback for completely unexpected formats
            ("unknown".to_string(), format!("Unexpected error response: {}", error_text))
        };

        return Err(VeniceError::ApiError {
            status,
            code,
            message,
        });
    }

    match response.json::<T>().await {
        Ok(data) => Ok((data, rate_limit_info)),
        Err(err) => Err(VeniceError::ParseError(format!(
            "Failed to parse response: {}",
            err
        ))),
    }
}

/// Process a binary response from the API
pub async fn process_binary_response(
    response: Response,
) -> VeniceResult<(Vec<u8>, String, RateLimitInfo)> {
    let rate_limit_info = RateLimitInfo::from_headers(response.headers());
    let status = response.status();

    if status.as_u16() == 429 {
        return Err(VeniceError::RateLimitExceeded(format!(
            "Rate limit exceeded: {}",
            rate_limit_info
        )));
    }

    if !status.is_success() {
        let error_text = response.text().await.unwrap_or_default();
        let error_response = serde_json::from_str::<serde_json::Value>(&error_text)
            .unwrap_or_else(|_| serde_json::json!({"error": {"message": error_text}}));

        // Handle different error response formats
        let (code, message) = if let Some(error_obj) = error_response.get("error") {
            if let Some(error_obj) = error_obj.as_object() {
                // Standard error format with error object
                let code = error_obj
                    .get("code")
                    .and_then(|c| c.as_str())
                    .unwrap_or("unknown")
                    .to_string();

                let message = error_obj
                    .get("message")
                    .and_then(|m| m.as_str())
                    .unwrap_or("Unknown error")
                    .to_string();
                
                (code, message)
            } else if let Some(error_str) = error_obj.as_str() {
                // Simple error format with just an error string
                ("api_error".to_string(), error_str.to_string())
            } else {
                // Fallback for other formats
                ("unknown".to_string(), format!("Unexpected error format: {}", error_response))
            }
        } else {
            // Fallback for completely unexpected formats
            ("unknown".to_string(), format!("Request failed with status: {} - {}", status, error_text))
        };

        return Err(VeniceError::ApiError {
            status,
            code,
            message,
        });
    }

    // Get the content type
    let mime_type = response
        .headers()
        .get(reqwest::header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream")
        .to_string();

    // Get the binary response data
    let binary_data = response
        .bytes()
        .await
        .map_err(|e| VeniceError::ParseError(format!("Failed to read response bytes: {}", e)))?
        .to_vec();

    Ok((binary_data, mime_type, rate_limit_info))
}

/// Process a streaming response from the API
pub async fn process_streaming_response<T: DeserializeOwned + 'static + Send>(
    response: Response,
) -> VeniceResult<(Pin<Box<dyn Stream<Item = VeniceResult<T>> + Send>>, RateLimitInfo)> {
    let rate_limit_info = RateLimitInfo::from_headers(response.headers());
    let status = response.status();

    if status.as_u16() == 429 {
        return Err(VeniceError::RateLimitExceeded(format!(
            "Rate limit exceeded: {}",
            rate_limit_info
        )));
    }

    if !status.is_success() {
        let error_text = response.text().await.unwrap_or_default();
        let error_response = serde_json::from_str::<serde_json::Value>(&error_text)
            .unwrap_or_else(|_| serde_json::json!({"error": {"message": error_text}}));

        // Handle different error response formats
        let (code, message) = if let Some(error_obj) = error_response.get("error") {
            if let Some(error_obj) = error_obj.as_object() {
                // Standard error format with error object
                let code = error_obj
                    .get("code")
                    .and_then(|c| c.as_str())
                    .unwrap_or("unknown")
                    .to_string();

                let message = error_obj
                    .get("message")
                    .and_then(|m| m.as_str())
                    .unwrap_or("Unknown error")
                    .to_string();
                
                (code, message)
            } else if let Some(error_str) = error_obj.as_str() {
                // Simple error format with just an error string
                ("api_error".to_string(), error_str.to_string())
            } else {
                // Fallback for other formats
                ("unknown".to_string(), format!("Unexpected error format: {}", error_response))
            }
        } else {
            // Fallback for completely unexpected formats
            ("unknown".to_string(), format!("Request failed with status: {} - {}", status, error_text))
        };

        return Err(VeniceError::ApiError {
            status,
            code,
            message,
        });
    }

    // Create a stream from the response body
    let stream = response
        .bytes_stream()
        .map_err(|e| VeniceError::HttpError(e))
        .and_then(|chunk| async move {
            // Each chunk is a SSE message in the format:
            // data: {...}\n\n
            let chunk_str = String::from_utf8(chunk.to_vec())
                .map_err(|e| VeniceError::ParseError(format!("Invalid UTF-8: {}", e)))?;
            
            // Process each line in the chunk
            let mut result = None;
            for line in chunk_str.lines() {
                if line.starts_with("data: ") {
                    let data = line.trim_start_matches("data: ");
                    if data == "[DONE]" {
                        // End of stream marker
                        continue;
                    }
                    
                    // Parse the JSON data
                    result = Some(serde_json::from_str::<T>(data)
                        .map_err(|e| VeniceError::ParseError(format!("Failed to parse JSON: {}", e)))?);
                }
            }
            
            // Return the parsed data if found
            match result {
                Some(data) => Ok(data),
                None => Err(VeniceError::ParseError("No data found in chunk".to_string())),
            }
        })
        .filter_map(|result| async move {
            match result {
                Ok(data) => Some(Ok(data)),
                Err(e) => {
                    // Filter out the "No data found in chunk" errors
                    if let VeniceError::ParseError(msg) = &e {
                        if msg == "No data found in chunk" {
                            return None;
                        }
                    }
                    Some(Err(e))
                }
            }
        });

    Ok((Box::pin(stream), rate_limit_info))
}