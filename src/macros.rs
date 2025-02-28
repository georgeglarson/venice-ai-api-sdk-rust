//! Macros for reducing code duplication

/// Define a builder method that sets a field
#[macro_export]
macro_rules! define_builder_method {
    ($name:ident, $field:ident, $type:ty) => {
        /// Set the $field field
        pub fn $name(mut self, value: $type) -> Self {
            self.$field = Some(value);
            self
        }
    };
    ($name:ident, $field:ident, $type:ty, $doc:expr) => {
        #[doc = $doc]
        pub fn $name(mut self, value: $type) -> Self {
            self.$field = Some(value);
            self
        }
    };
}

/// Define a builder method that sets a field directly (not Option)
#[macro_export]
macro_rules! define_builder_method_direct {
    ($name:ident, $field:ident, $type:ty) => {
        /// Set the $field field
        pub fn $name(mut self, value: $type) -> Self {
            self.$field = value;
            self
        }
    };
    ($name:ident, $field:ident, $type:ty, $doc:expr) => {
        #[doc = $doc]
        pub fn $name(mut self, value: $type) -> Self {
            self.$field = value;
            self
        }
    };
}

/// Define a builder method that adds an item to a vector
#[macro_export]
macro_rules! define_builder_add_method {
    ($name:ident, $field:ident, $type:ty) => {
        /// Add an item to the $field vector
        pub fn $name(mut self, value: $type) -> Self {
            self.$field.push(value);
            self
        }
    };
    ($name:ident, $field:ident, $type:ty, $doc:expr) => {
        #[doc = $doc]
        pub fn $name(mut self, value: $type) -> Self {
            self.$field.push(value);
            self
        }
    };
}

/// Define an endpoint implementation for a trait
#[macro_export]
macro_rules! define_endpoint {
    ($trait_name:ident, $method_name:ident, $endpoint:expr, $request_type:ty, $response_type:ty) => {
        #[async_trait::async_trait]
        impl $trait_name for crate::client::Client {
            async fn $method_name(
                &self,
                request: $request_type,
            ) -> crate::error::VeniceResult<($response_type, crate::error::RateLimitInfo)> {
                self.post($endpoint, &request).await
            }
        }
    };
    ($trait_name:ident, $method_name:ident, $endpoint:expr, $response_type:ty) => {
        #[async_trait::async_trait]
        impl $trait_name for crate::client::Client {
            async fn $method_name(
                &self,
            ) -> crate::error::VeniceResult<($response_type, crate::error::RateLimitInfo)> {
                self.get($endpoint).await
            }
        }
    };
}