//! Agent result types for the SDK.
//! 
//! This module defines the result structures returned by agents
//! after processing messages and executing tools.

use chrono::{DateTime, Utc};

use crate::types::{Message, Messages, ToolSpec};

/// The result of an agent's processing.
#[derive(Debug, Clone)]
pub struct AgentResult {
    /// The ID of the agent that produced this result.
    pub agent_id: String,
    /// The conversation context used to generate this result.
    pub conversation_context: Messages,
    /// The response message from the agent.
    pub response_message: Message,
    /// The text response from the agent.
    pub response: String,
    /// The messages in the conversation.
    pub messages: Messages,
    /// The tools that were available to the agent.
    pub available_tools: Vec<ToolSpec>,
    /// When this result was created.
    pub created_at: DateTime<Utc>,
    /// Additional metadata for the result.
    pub metadata: std::collections::HashMap<String, serde_json::Value>,
}

impl AgentResult {
    /// Create a new agent result.
    pub fn new(
        agent_id: String,
        conversation_context: Messages,
        response_message: Message,
        response: String,
        messages: Messages,
        available_tools: Vec<ToolSpec>,
    ) -> Self {
        Self {
            agent_id,
            conversation_context,
            response_message,
            response,
            messages,
            available_tools,
            created_at: Utc::now(),
            metadata: std::collections::HashMap::new(),
        }
    }

    /// Get the agent ID.
    pub fn agent_id(&self) -> &str {
        &self.agent_id
    }

    /// Get the conversation context.
    pub fn conversation_context(&self) -> &Messages {
        &self.conversation_context
    }

    /// Get the messages in the conversation.
    pub fn messages(&self) -> &Messages {
        &self.messages
    }

    /// Get the response message.
    pub fn response_message(&self) -> &Message {
        &self.response_message
    }

    /// Get the text response.
    pub fn response(&self) -> &str {
        &self.response
    }

    /// Get the available tools.
    pub fn available_tools(&self) -> &[ToolSpec] {
        &self.available_tools
    }

    /// Get the creation time.
    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    /// Add metadata to the result.
    pub fn with_metadata(mut self, key: &str, value: serde_json::Value) -> Self {
        self.metadata.insert(key.to_string(), value);
        self
    }

    /// Get metadata by key.
    pub fn get_metadata(&self, key: &str) -> Option<&serde_json::Value> {
        self.metadata.get(key)
    }

    /// Check if the agent used any tools.
    pub fn used_tools(&self) -> bool {
        !self.available_tools.is_empty()
    }

    /// Get the number of available tools.
    pub fn tool_count(&self) -> usize {
        self.available_tools.len()
    }

    /// Get the conversation length.
    pub fn conversation_length(&self) -> usize {
        self.messages.len()
    }
}

impl Default for AgentResult {
    fn default() -> Self {
        Self {
            agent_id: "default".to_string(),
            conversation_context: Vec::new(),
            response_message: Message::assistant(""),
            response: "".to_string(),
            messages: Vec::new(),
            available_tools: Vec::new(),
            created_at: Utc::now(),
            metadata: std::collections::HashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::content::MessageRole;

    #[test]
    fn test_agent_result_creation() {
        let agent_id = "test_agent".to_string();
        let conversation_context = vec![Message::user("Hello")];
        let response_message = Message::assistant("Hi there!");
        let response = "Hi there!".to_string();
        let messages = vec![Message::user("Hello"), Message::assistant("Hi there!")];
        let available_tools = vec![ToolSpec::new("test_tool", "A test tool")];

        let result = AgentResult::new(
            agent_id.clone(),
            conversation_context.clone(),
            response_message.clone(),
            response.clone(),
            messages.clone(),
            available_tools.clone(),
        );

        assert_eq!(result.agent_id(), &agent_id);
        assert_eq!(result.conversation_context(), &conversation_context);
        assert_eq!(result.response_message(), &response_message);
        assert_eq!(result.response(), &response);
        assert_eq!(result.messages(), &messages);
        assert_eq!(result.available_tools(), &available_tools);
        assert!(result.created_at() <= Utc::now());
    }

    #[test]
    fn test_agent_result_metadata() {
        let mut result = AgentResult::default();
        
        result = result.with_metadata("test_key", serde_json::Value::String("test_value".to_string()));
        
        assert_eq!(
            result.get_metadata("test_key"),
            Some(&serde_json::Value::String("test_value".to_string()))
        );
    }

    #[test]
    fn test_agent_result_tools() {
        let result = AgentResult::default();
        
        assert!(!result.used_tools());
        assert_eq!(result.tool_count(), 0);
    }

    #[test]
    fn test_agent_result_conversation() {
        let conversation_context = vec![Message::user("Hello"), Message::assistant("Hi!")];
        let response_message = Message::assistant("How can I help?");
        let messages = vec![Message::user("Hello"), Message::assistant("Hi!"), Message::assistant("How can I help?")];
        
        let result = AgentResult::new(
            "test_agent".to_string(),
            conversation_context.clone(),
            response_message.clone(),
            "How can I help?".to_string(),
            messages.clone(),
            Vec::new(),
        );
        
        assert_eq!(result.conversation_length(), 3);
        assert_eq!(result.messages(), &messages);
    }
}
