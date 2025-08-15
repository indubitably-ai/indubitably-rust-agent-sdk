//! Callback handler for the SDK.
//! 
//! This module provides a callback handler for processing
//! agent events and responses.

use async_trait::async_trait;

use crate::types::{Message, IndubitablyResult};

/// A trait for handling callbacks from agents.
#[async_trait]
pub trait CallbackHandler: Send + Sync {
    /// Handle a message callback.
    async fn on_message(&self, message: &Message) -> IndubitablyResult<()>;
    
    /// Handle an error callback.
    async fn on_error(&self, error: &crate::types::IndubitablyError) -> IndubitablyResult<()>;
}

/// A null callback handler that does nothing.
pub struct NullCallbackHandler;

impl NullCallbackHandler {
    /// Create a new null callback handler.
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl CallbackHandler for NullCallbackHandler {
    async fn on_message(&self, _message: &Message) -> IndubitablyResult<()> {
        Ok(())
    }
    
    async fn on_error(&self, _error: &crate::types::IndubitablyError) -> IndubitablyResult<()> {
        Ok(())
    }
}

impl Default for NullCallbackHandler {
    fn default() -> Self {
        Self::new()
    }
}
