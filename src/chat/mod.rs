//! Chat API endpoints
//!
//! This module contains types and functions for working with Venice.ai's chat API.

mod completions;
mod conversions;
mod model_feature_suffix;
mod streaming;
#[cfg(test)]
mod tests;
#[cfg(test)]
mod test_client;

pub use completions::*;
pub use model_feature_suffix::*;
pub use streaming::*;
#[cfg(test)]
pub use test_client::*;