//! Session-related type definitions for the SDK.
//! 
//! This module defines the types used to represent sessions,
//! session agents, and session messages.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

use super::content::Message;

/// A session represents a conversation or interaction with an agent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    /// The unique identifier for the session.
    pub id: String,
    /// The type of session.
    pub session_type: SessionType,
    /// The agent associated with this session.
    pub agent: SessionAgent,
    /// The messages in this session.
    pub messages: Vec<SessionMessage>,
    /// When the session was created.
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    /// When the session was last updated.
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
    /// Additional metadata for the session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

/// The type of session.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SessionType {
    Conversation,
    Task,
    Workflow,
    Custom(String),
}

/// An agent within a session.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionAgent {
    /// The unique identifier for the agent.
    pub id: String,
    /// The name of the agent.
    pub name: String,
    /// The model used by the agent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// The system prompt for the agent.
    #[serde(rename = "systemPrompt", skip_serializing_if = "Option::is_none")]
    pub system_prompt: Option<String>,
    /// Additional configuration for the agent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<HashMap<String, serde_json::Value>>,
}

/// A message within a session.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionMessage {
    /// The unique identifier for the message.
    pub id: String,
    /// The role of the message sender.
    pub role: String,
    /// The content of the message.
    pub content: String,
    /// When the message was created.
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    /// Additional metadata for the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

impl Session {
    /// Create a new session.
    pub fn new(id: &str, session_type: SessionType, agent: SessionAgent) -> Self {
        let now = Utc::now();
        Self {
            id: id.to_string(),
            session_type,
            agent,
            messages: Vec::new(),
            created_at: now,
            updated_at: now,
            metadata: None,
        }
    }

    /// Add a message to the session.
    pub fn add_message(&mut self, message: SessionMessage) {
        self.messages.push(message);
        self.updated_at = Utc::now();
    }

    /// Get the last message in the session.
    pub fn last_message(&self) -> Option<&SessionMessage> {
        self.messages.last()
    }

    /// Get the number of messages in the session.
    pub fn message_count(&self) -> usize {
        self.messages.len()
    }

    /// Check if the session is empty.
    pub fn is_empty(&self) -> bool {
        self.messages.is_empty()
    }

    /// Add metadata to the session.
    pub fn add_metadata(&mut self, key: &str, value: serde_json::Value) {
        if self.metadata.is_none() {
            self.metadata = Some(HashMap::new());
        }
        if let Some(ref mut metadata) = self.metadata {
            metadata.insert(key.to_string(), value);
        }
    }
}

impl SessionAgent {
    /// Create a new session agent.
    pub fn new(id: &str, name: &str) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            model: None,
            system_prompt: None,
            config: None,
        }
    }

    /// Set the model for the agent.
    pub fn with_model(mut self, model: &str) -> Self {
        self.model = Some(model.to_string());
        self
    }

    /// Set the system prompt for the agent.
    pub fn with_system_prompt(mut self, system_prompt: &str) -> Self {
        self.system_prompt = Some(system_prompt.to_string());
        self
    }

    /// Add configuration to the agent.
    pub fn with_config(mut self, key: &str, value: serde_json::Value) -> Self {
        if self.config.is_none() {
            self.config = Some(HashMap::new());
        }
        if let Some(ref mut config) = self.config {
            config.insert(key.to_string(), value);
        }
        self
    }
}

impl SessionMessage {
    /// Create a new session message.
    pub fn new(id: &str, role: &str, content: &str) -> Self {
        Self {
            id: id.to_string(),
            role: role.to_string(),
            content: content.to_string(),
            created_at: Utc::now(),
            metadata: None,
        }
    }

    /// Create a new session message from a regular message.
    pub fn from_message(id: &str, message: &Message) -> Self {
        Self {
            id: id.to_string(),
            role: match message.role {
                super::content::MessageRole::User => "user".to_string(),
                super::content::MessageRole::Assistant => "assistant".to_string(),
                super::content::MessageRole::System => "system".to_string(),
                super::content::MessageRole::Tool => "tool".to_string(),
            },
            content: message.all_text(),
            created_at: Utc::now(),
            metadata: None,
        }
    }

    /// Add metadata to the message.
    pub fn add_metadata(&mut self, key: &str, value: serde_json::Value) {
        if self.metadata.is_none() {
            self.metadata = Some(HashMap::new());
        }
        if let Some(ref mut metadata) = self.metadata {
            metadata.insert(key.to_string(), value);
        }
    }
}

impl From<&str> for SessionType {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "conversation" => SessionType::Conversation,
            "task" => SessionType::Task,
            "workflow" => SessionType::Workflow,
            _ => SessionType::Custom(s.to_string()),
        }
    }
}
