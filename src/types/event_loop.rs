//! Event loop type definitions for the SDK.
//! 
//! This module defines the types used to represent event loop
//! states and configurations.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// The state of an event loop.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventLoopState {
    /// The event loop is idle.
    Idle,
    /// The event loop is processing a message.
    Processing,
    /// The event loop is waiting for a tool result.
    WaitingForTool,
    /// The event loop is complete.
    Complete,
    /// The event loop encountered an error.
    Error(String),
}

/// Configuration for an event loop.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventLoopConfig {
    /// The maximum number of iterations.
    pub max_iterations: usize,
    /// The timeout for tool execution.
    pub tool_timeout: u64,
    /// Whether to enable streaming.
    pub enable_streaming: bool,
    /// Additional configuration options.
    pub options: HashMap<String, serde_json::Value>,
}

impl Default for EventLoopConfig {
    fn default() -> Self {
        Self {
            max_iterations: 10,
            tool_timeout: 30,
            enable_streaming: false,
            options: HashMap::new(),
        }
    }
}

impl EventLoopConfig {
    /// Create a new event loop configuration.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the maximum number of iterations.
    pub fn with_max_iterations(mut self, max_iterations: usize) -> Self {
        self.max_iterations = max_iterations;
        self
    }

    /// Set the tool timeout.
    pub fn with_tool_timeout(mut self, tool_timeout: u64) -> Self {
        self.tool_timeout = tool_timeout;
        self
    }

    /// Enable or disable streaming.
    pub fn with_streaming(mut self, enable_streaming: bool) -> Self {
        self.enable_streaming = enable_streaming;
        self
    }

    /// Add a configuration option.
    pub fn with_option(mut self, key: &str, value: serde_json::Value) -> Self {
        self.options.insert(key.to_string(), value);
        self
    }
}
