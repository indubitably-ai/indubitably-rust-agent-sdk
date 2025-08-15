//! Streaming event loop for the SDK.
//! 
//! This module provides a streaming version of the event loop
//! for real-time agent interactions.

use crate::types::{Messages, StreamEvent, IndubitablyResult};

/// A streaming event loop for real-time agent interactions.
pub struct StreamingEventLoop {
    /// The underlying event loop.
    event_loop: super::event_loop::EventLoop,
}

impl StreamingEventLoop {
    /// Create a new streaming event loop.
    pub fn new() -> Self {
        Self {
            event_loop: super::event_loop::EventLoop::new(),
        }
    }
    
    /// Stream events from the event loop.
    pub async fn stream(&mut self, _messages: &Messages) -> IndubitablyResult<Box<dyn tokio_stream::Stream<Item = IndubitablyResult<StreamEvent>> + Send>> {
        // For now, return an error as streaming is not fully implemented
        Err(crate::types::IndubitablyError::StreamingError(
            crate::types::StreamingError::StreamInterrupted(
                "Streaming not yet implemented".to_string(),
            ),
        ))
    }
}

impl Default for StreamingEventLoop {
    fn default() -> Self {
        Self::new()
    }
}
