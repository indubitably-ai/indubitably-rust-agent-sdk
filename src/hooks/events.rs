//! Hook events for the SDK.
//! 
//! This module defines the events that can trigger hooks
//! in the agent system.

use serde::{Deserialize, Serialize};

/// A hook event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HookEvent {
    /// The event type.
    pub event_type: String,
    /// The event data.
    pub data: serde_json::Value,
}

impl HookEvent {
    /// Create a new hook event.
    pub fn new(event_type: &str, data: serde_json::Value) -> Self {
        Self {
            event_type: event_type.to_string(),
            data,
        }
    }
}
