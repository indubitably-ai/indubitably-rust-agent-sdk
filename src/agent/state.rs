//! Agent state management for the SDK.
//! 
//! This module provides functionality for managing the internal
//! state of agents, including message history and metadata.

use std::collections::HashMap;
use chrono::{DateTime, Utc};

use crate::types::{Message, Messages};

/// The internal state of an agent.
#[derive(Debug, Clone)]
pub struct AgentState {
    /// The messages in the conversation.
    messages: Messages,
    /// When the agent was created.
    created_at: DateTime<Utc>,
    /// When the agent was last updated.
    updated_at: DateTime<Utc>,
    /// Additional metadata for the agent.
    metadata: HashMap<String, serde_json::Value>,
}

impl AgentState {
    /// Create a new agent state.
    pub fn new() -> Self {
        let now = Utc::now();
        Self {
            messages: Vec::new(),
            created_at: now,
            updated_at: now,
            metadata: HashMap::new(),
        }
    }

    /// Add a message to the state.
    pub fn add_message(&mut self, message: Message) {
        self.messages.push(message);
        self.updated_at = Utc::now();
    }

    /// Get all messages.
    pub fn messages(&self) -> &Messages {
        &self.messages
    }

    /// Get the last message.
    pub fn last_message(&self) -> Option<&Message> {
        self.messages.last()
    }

    /// Get the number of messages.
    pub fn message_count(&self) -> usize {
        self.messages.len()
    }

    /// Check if the state is empty.
    pub fn is_empty(&self) -> bool {
        self.messages.is_empty()
    }

    /// Clear all messages.
    pub fn clear_messages(&mut self) {
        self.messages.clear();
        self.updated_at = Utc::now();
    }

    /// Get the creation time.
    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    /// Get the last update time.
    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    /// Get metadata by key.
    pub fn get_metadata(&self, key: &str) -> Option<&serde_json::Value> {
        self.metadata.get(key)
    }

    /// Set metadata by key.
    pub fn set_metadata(&mut self, key: &str, value: serde_json::Value) {
        self.metadata.insert(key.to_string(), value);
        self.updated_at = Utc::now();
    }

    /// Remove metadata by key.
    pub fn remove_metadata(&mut self, key: &str) -> Option<serde_json::Value> {
        let result = self.metadata.remove(key);
        if result.is_some() {
            self.updated_at = Utc::now();
        }
        result
    }

    /// Get all metadata.
    pub fn metadata(&self) -> &HashMap<String, serde_json::Value> {
        &self.metadata
    }
}

impl Default for AgentState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::content::MessageRole;

    #[test]
    fn test_agent_state_creation() {
        let state = AgentState::new();
        
        assert!(state.is_empty());
        assert_eq!(state.message_count(), 0);
        assert!(state.created_at() <= Utc::now());
        assert!(state.updated_at() <= Utc::now());
    }

    #[test]
    fn test_agent_state_messages() {
        let mut state = AgentState::new();
        
        let message = Message::new(
            MessageRole::User,
            vec![crate::types::ContentBlock::default()],
        );
        
        state.add_message(message);
        assert!(!state.is_empty());
        assert_eq!(state.message_count(), 1);
        assert!(state.last_message().is_some());
    }

    #[test]
    fn test_agent_state_metadata() {
        let mut state = AgentState::new();
        
        state.set_metadata("test_key", serde_json::Value::String("test_value".to_string()));
        
        assert_eq!(
            state.get_metadata("test_key"),
            Some(&serde_json::Value::String("test_value".to_string()))
        );
        
        let removed = state.remove_metadata("test_key");
        assert_eq!(removed, Some(serde_json::Value::String("test_value".to_string())));
        assert!(state.get_metadata("test_key").is_none());
    }

    #[test]
    fn test_agent_state_clear() {
        let mut state = AgentState::new();
        
        let message = Message::new(
            MessageRole::User,
            vec![crate::types::ContentBlock::default()],
        );
        
        state.add_message(message);
        assert!(!state.is_empty());
        
        state.clear_messages();
        assert!(state.is_empty());
        assert_eq!(state.message_count(), 0);
    }
}
