//! Serialization utilities for the Venice AI API SDK

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Convert a struct to a JSON Value
pub fn to_json<T: Serialize>(value: &T) -> Result<Value, serde_json::Error> {
    serde_json::to_value(value)
}

/// Convert a JSON Value to a struct
pub fn from_json<T: for<'de> Deserialize<'de>>(value: Value) -> Result<T, serde_json::Error> {
    serde_json::from_value(value)
}

/// Convert a struct to a JSON string
pub fn to_json_string<T: Serialize>(value: &T) -> Result<String, serde_json::Error> {
    serde_json::to_string(value)
}

/// Convert a JSON string to a struct
pub fn from_json_string<T: for<'de> Deserialize<'de>>(value: &str) -> Result<T, serde_json::Error> {
    serde_json::from_str(value)
}

/// Convert a struct to a pretty JSON string
pub fn to_pretty_json_string<T: Serialize>(value: &T) -> Result<String, serde_json::Error> {
    serde_json::to_string_pretty(value)
}