use std::marker::PhantomData;

use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};

use crate::{
    error::{RateLimitInfo, VeniceResult},
};

/// A paginated response from the API
#[derive(Debug, Clone)]
pub struct PaginatedResponse<T> {
    /// The data returned in this page
    pub data: Vec<T>,
    /// Whether there are more items available
    pub has_more: bool,
    /// The cursor to use for the next page, if any
    pub next_cursor: Option<String>,
    /// Rate limit information from the response
    pub rate_limit_info: RateLimitInfo,
}

/// Parameters for paginated requests
#[derive(Debug, Clone, Serialize)]
pub struct PaginationParams {
    /// Maximum number of items to return
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Cursor for pagination
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl Default for PaginationParams {
    fn default() -> Self {
        Self {
            limit: None,
            cursor: None,
        }
    }
}

impl PaginationParams {
    /// Create a new set of pagination parameters
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the maximum number of items to return
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Set the cursor for pagination
    pub fn cursor(mut self, cursor: impl Into<String>) -> Self {
        self.cursor = Some(cursor.into());
        self
    }
}

/// A paginator for iterating through paginated results
#[async_trait]
pub trait Paginator<T> {
    /// Get the next page of results
    async fn next_page(&mut self) -> VeniceResult<Option<PaginatedResponse<T>>>;
    
    /// Get all pages of results
    async fn all_pages(&mut self) -> VeniceResult<Vec<T>>;
}

/// A generic paginator implementation
pub struct GenericPaginator<T, R, F>
where
    T: Clone + Send + Sync + 'static,
    R: DeserializeOwned + Send + Sync + 'static,
    F: Fn(PaginationParams) -> VeniceResult<(R, RateLimitInfo)> + Send + Sync + 'static,
{
    /// The function to call to get a page of results
    fetch_page: F,
    /// The current pagination parameters
    params: PaginationParams,
    /// Whether there are more pages
    has_more: bool,
    /// Phantom data for the item type
    _item_type: PhantomData<T>,
    /// Phantom data for the response type
    _response_type: PhantomData<R>,
}

impl<T, R, F> GenericPaginator<T, R, F>
where
    T: Clone + Send + Sync + 'static,
    R: DeserializeOwned + Send + Sync + 'static,
    F: Fn(PaginationParams) -> VeniceResult<(R, RateLimitInfo)> + Send + Sync + 'static,
{
    /// Create a new paginator
    pub fn new(fetch_page: F, params: PaginationParams) -> Self {
        Self {
            fetch_page,
            params,
            has_more: true,
            _item_type: PhantomData,
            _response_type: PhantomData,
        }
    }
}

/// A generic async paginator implementation
pub struct AsyncGenericPaginator<T, R, F>
where
    T: Clone + Send + Sync + 'static,
    R: DeserializeOwned + Send + Sync + 'static,
    F: Fn(PaginationParams) -> futures::future::BoxFuture<'static, VeniceResult<(R, RateLimitInfo)>> + Send + Sync + 'static,
{
    /// The function to call to get a page of results
    fetch_page: F,
    /// The current pagination parameters
    params: PaginationParams,
    /// Whether there are more pages
    has_more: bool,
    /// Phantom data for the item type
    _item_type: PhantomData<T>,
    /// Phantom data for the response type
    _response_type: PhantomData<R>,
}

impl<T, R, F> AsyncGenericPaginator<T, R, F>
where
    T: Clone + Send + Sync + 'static,
    R: DeserializeOwned + Send + Sync + 'static,
    F: Fn(PaginationParams) -> futures::future::BoxFuture<'static, VeniceResult<(R, RateLimitInfo)>> + Send + Sync + 'static,
{
    /// Create a new paginator
    pub fn new(fetch_page: F, params: PaginationParams) -> Self {
        Self {
            fetch_page,
            params,
            has_more: true,
            _item_type: PhantomData,
            _response_type: PhantomData,
        }
    }
}

/// A trait for extracting pagination information from a response
pub trait PaginationInfo<T> {
    /// Extract the data from the response
    fn get_data(&self) -> Vec<T>;
    
    /// Check if there are more pages
    fn has_more(&self) -> bool;
    
    /// Get the cursor for the next page
    fn next_cursor(&self) -> Option<String>;
}

#[async_trait]
impl<T, R, F> Paginator<T> for GenericPaginator<T, R, F>
where
    T: Clone + Send + Sync + 'static,
    R: DeserializeOwned + PaginationInfo<T> + Send + Sync + 'static,
    F: Fn(PaginationParams) -> VeniceResult<(R, RateLimitInfo)> + Send + Sync + 'static,
{
    async fn next_page(&mut self) -> VeniceResult<Option<PaginatedResponse<T>>> {
        if !self.has_more {
            return Ok(None);
        }
        
        let (response, rate_limit_info) = (self.fetch_page)(self.params.clone())?;
        
        let data = response.get_data();
        let has_more = response.has_more();
        let next_cursor = response.next_cursor();
        
        // Update the paginator state
        self.has_more = has_more;
        if let Some(cursor) = next_cursor.clone() {
            self.params.cursor = Some(cursor);
        } else {
            self.has_more = false;
        }
        
        Ok(Some(PaginatedResponse {
            data,
            has_more,
            next_cursor,
            rate_limit_info,
        }))
    }
    
    async fn all_pages(&mut self) -> VeniceResult<Vec<T>> {
        let mut all_items = Vec::new();
        
        while let Some(page) = self.next_page().await? {
            all_items.extend(page.data);
            
            if !page.has_more {
                break;
            }
        }
        
        Ok(all_items)
    }
}

#[async_trait]
impl<T, R, F> Paginator<T> for AsyncGenericPaginator<T, R, F>
where
    T: Clone + Send + Sync + 'static,
    R: DeserializeOwned + PaginationInfo<T> + Send + Sync + 'static,
    F: Fn(PaginationParams) -> futures::future::BoxFuture<'static, VeniceResult<(R, RateLimitInfo)>> + Send + Sync + 'static,
{
    async fn next_page(&mut self) -> VeniceResult<Option<PaginatedResponse<T>>> {
        if !self.has_more {
            return Ok(None);
        }
        
        let (response, rate_limit_info) = (self.fetch_page)(self.params.clone()).await?;
        
        let data = response.get_data();
        let has_more = response.has_more();
        let next_cursor = response.next_cursor();
        
        // Update the paginator state
        self.has_more = has_more;
        if let Some(cursor) = next_cursor.clone() {
            self.params.cursor = Some(cursor);
        } else {
            self.has_more = false;
        }
        
        Ok(Some(PaginatedResponse {
            data,
            has_more,
            next_cursor,
            rate_limit_info,
        }))
    }
    
    async fn all_pages(&mut self) -> VeniceResult<Vec<T>> {
        let mut all_items = Vec::new();
        
        while let Some(page) = self.next_page().await? {
            all_items.extend(page.data);
            
            if !page.has_more {
                break;
            }
        }
        
        Ok(all_items)
    }
}

/// Helper function to create a paginator for a specific endpoint
pub fn create_paginator<T, R, F>(
    fetch_page: F,
    params: PaginationParams,
) -> impl Paginator<T>
where
    T: Clone + Send + Sync + 'static,
    R: DeserializeOwned + PaginationInfo<T> + Send + Sync + 'static,
    F: Fn(PaginationParams) -> VeniceResult<(R, RateLimitInfo)> + Send + Sync + 'static,
{
    GenericPaginator::new(fetch_page, params)
}

/// Helper function to create an async paginator for a specific endpoint
pub fn create_async_paginator<T, R, F, Fut>(
    fetch_page: F,
    params: PaginationParams,
) -> impl Paginator<T>
where
    T: Clone + Send + Sync + 'static,
    R: DeserializeOwned + PaginationInfo<T> + Send + Sync + 'static,
    Fut: futures::Future<Output = VeniceResult<(R, RateLimitInfo)>> + Send + 'static,
    F: Fn(PaginationParams) -> Fut + Send + Sync + 'static,
{
    let boxed_fetch_page = move |params: PaginationParams| -> futures::future::BoxFuture<'static, VeniceResult<(R, RateLimitInfo)>> {
        Box::pin(fetch_page(params))
    };
    
    AsyncGenericPaginator::new(boxed_fetch_page, params)
}