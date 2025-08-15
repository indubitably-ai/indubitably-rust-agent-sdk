//! Conversation management for the SDK.
//! 
//! This module provides functionality for managing conversation
//! context, history, and memory for agents.

use async_trait::async_trait;

use crate::types::{Messages, Message, IndubitablyResult};

/// Configuration for conversation managers.
#[derive(Debug, Clone)]
pub struct ConversationManagerConfig {
    /// The maximum number of messages to keep.
    pub max_messages: usize,
    /// Whether to enable summarization.
    pub enable_summarization: bool,
    /// The model to use for summarization.
    pub summary_model: Option<String>,
}

impl Default for ConversationManagerConfig {
    fn default() -> Self {
        Self {
            max_messages: 100,
            enable_summarization: false,
            summary_model: None,
        }
    }
}

impl ConversationManagerConfig {
    /// Create a new conversation manager configuration.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the maximum number of messages.
    pub fn with_max_messages(mut self, max_messages: usize) -> Self {
        self.max_messages = max_messages;
        self
    }

    /// Enable summarization.
    pub fn with_summarization(mut self, enable: bool) -> Self {
        self.enable_summarization = enable;
        self
    }

    /// Set the summarization model.
    pub fn with_summary_model(mut self, model: &str) -> Self {
        self.summary_model = Some(model.to_string());
        self
    }
}

/// A trait for managing conversation context and history.
#[async_trait]
pub trait ConversationManager: Send + Sync {
    /// Get the current conversation context.
    async fn get_context(&self) -> IndubitablyResult<Messages>;
    
    /// Get the conversation history.
    async fn get_history(&self) -> IndubitablyResult<Messages> {
        self.get_context().await
    }
    
    /// Add a message to the conversation.
    async fn add_message(&mut self, message: Message) -> IndubitablyResult<()>;
    
    /// Clear the conversation history.
    async fn clear(&mut self) -> IndubitablyResult<()>;
    
    /// Clear the conversation history (alias for clear).
    async fn clear_history(&mut self) -> IndubitablyResult<()> {
        self.clear().await
    }
    
    /// Get the number of messages in the conversation.
    async fn message_count(&self) -> IndubitablyResult<usize>;
    
    /// Check if the conversation is empty.
    async fn is_empty(&self) -> IndubitablyResult<bool>;
}

/// A conversation manager that doesn't store any history.
pub struct NullConversationManager;

impl NullConversationManager {
    /// Create a new null conversation manager.
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl ConversationManager for NullConversationManager {
    async fn get_context(&self) -> IndubitablyResult<Messages> {
        Ok(Vec::new())
    }
    
    async fn add_message(&mut self, _message: Message) -> IndubitablyResult<()> {
        // Do nothing - null manager doesn't store messages
        Ok(())
    }
    
    async fn clear(&mut self) -> IndubitablyResult<()> {
        // Do nothing - null manager doesn't store messages
        Ok(())
    }
    
    async fn message_count(&self) -> IndubitablyResult<usize> {
        Ok(0)
    }
    
    async fn is_empty(&self) -> IndubitablyResult<bool> {
        Ok(true)
    }
}

impl Default for NullConversationManager {
    fn default() -> Self {
        Self::new()
    }
}

/// A conversation manager that maintains a sliding window of messages.
pub struct SlidingWindowConversationManager {
    /// The maximum number of messages to keep.
    max_messages: usize,
    /// The messages in the conversation.
    messages: Messages,
}

impl SlidingWindowConversationManager {
    /// Create a new sliding window conversation manager.
    pub fn new(max_messages: usize) -> Self {
        Self {
            max_messages,
            messages: Vec::new(),
        }
    }
    
    /// Create a new sliding window conversation manager with default settings.
    pub fn default() -> Self {
        Self::new(100) // Default to keeping last 100 messages
    }
    
    /// Set the maximum number of messages to keep.
    pub fn with_max_messages(mut self, max_messages: usize) -> Self {
        self.max_messages = max_messages;
        self
    }
    
    /// Get the maximum number of messages.
    pub fn max_messages(&self) -> usize {
        self.max_messages
    }
}

#[async_trait]
impl ConversationManager for SlidingWindowConversationManager {
    async fn get_context(&self) -> IndubitablyResult<Messages> {
        Ok(self.messages.clone())
    }
    
    async fn add_message(&mut self, message: Message) -> IndubitablyResult<()> {
        self.messages.push(message);
        
        // Maintain sliding window
        if self.messages.len() > self.max_messages {
            self.messages.remove(0);
        }
        
        Ok(())
    }
    
    async fn clear(&mut self) -> IndubitablyResult<()> {
        self.messages.clear();
        Ok(())
    }
    
    async fn message_count(&self) -> IndubitablyResult<usize> {
        Ok(self.messages.len())
    }
    
    async fn is_empty(&self) -> IndubitablyResult<bool> {
        Ok(self.messages.is_empty())
    }
}

impl Default for SlidingWindowConversationManager {
    fn default() -> Self {
        Self::default()
    }
}

/// A conversation manager that summarizes old messages to maintain context.
pub struct SummarizingConversationManager {
    /// The maximum number of recent messages to keep.
    max_recent_messages: usize,
    /// The recent messages to keep in full.
    recent_messages: Messages,
    /// A summary of older messages.
    summary: Option<String>,
    /// The model to use for summarization.
    summary_model: Option<String>,
}

impl SummarizingConversationManager {
    /// Create a new summarizing conversation manager.
    pub fn new(max_recent_messages: usize) -> Self {
        Self {
            max_recent_messages,
            recent_messages: Vec::new(),
            summary: None,
            summary_model: None,
        }
    }
    
    /// Create a new summarizing conversation manager with default settings.
    pub fn default() -> Self {
        Self::new(20) // Default to keeping last 20 messages
    }
    
    /// Set the maximum number of recent messages.
    pub fn with_max_recent_messages(mut self, max_recent_messages: usize) -> Self {
        self.max_recent_messages = max_recent_messages;
        self
    }
    
    /// Set the model to use for summarization.
    pub fn with_summary_model(mut self, model: &str) -> Self {
        self.summary_model = Some(model.to_string());
        self
    }
}

#[async_trait]
impl ConversationManager for SummarizingConversationManager {
    async fn get_context(&self) -> IndubitablyResult<Messages> {
        let mut context = Vec::new();
        
        // Add summary if available
        if let Some(ref summary) = self.summary {
            let summary_message = Message::system(&format!("Previous conversation summary: {}", summary));
            context.push(summary_message);
        }
        
        // Add recent messages
        context.extend(self.recent_messages.clone());
        
        Ok(context)
    }
    
    async fn add_message(&mut self, message: Message) -> IndubitablyResult<()> {
        self.recent_messages.push(message);
        
        // Maintain sliding window for recent messages
        if self.recent_messages.len() > self.max_recent_messages {
            // For now, just remove the oldest message
            // In a full implementation, this would trigger summarization
            self.recent_messages.remove(0);
        }
        
        Ok(())
    }
    
    async fn clear(&mut self) -> IndubitablyResult<()> {
        self.recent_messages.clear();
        self.summary = None;
        Ok(())
    }
    
    async fn message_count(&self) -> IndubitablyResult<usize> {
        let summary_count = if self.summary.is_some() { 1 } else { 0 };
        Ok(self.recent_messages.len() + summary_count)
    }
    
    async fn is_empty(&self) -> IndubitablyResult<bool> {
        Ok(self.recent_messages.is_empty() && self.summary.is_none())
    }
}

impl Default for SummarizingConversationManager {
    fn default() -> Self {
        Self::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::content::MessageRole;

    #[tokio::test]
    async fn test_null_conversation_manager() {
        let mut manager = NullConversationManager::new();
        
        assert_eq!(manager.message_count().await.unwrap(), 0);
        assert!(manager.is_empty().await.unwrap());
        
        let message = Message::user("Hello");
        manager.add_message(message).await.unwrap();
        
        // Null manager should still be empty
        assert_eq!(manager.message_count().await.unwrap(), 0);
        assert!(manager.is_empty().await.unwrap());
    }

    #[tokio::test]
    async fn test_sliding_window_conversation_manager() {
        let mut manager = SlidingWindowConversationManager::new(3);
        
        assert_eq!(manager.message_count().await.unwrap(), 0);
        assert!(manager.is_empty().await.unwrap());
        
        // Add messages
        manager.add_message(Message::user("Hello")).await.unwrap();
        manager.add_message(Message::assistant("Hi!")).await.unwrap();
        manager.add_message(Message::user("How are you?")).await.unwrap();
        
        assert_eq!(manager.message_count().await.unwrap(), 3);
        assert!(!manager.is_empty().await.unwrap());
        
        // Add one more message - should trigger sliding window
        manager.add_message(Message::assistant("I'm good!")).await.unwrap();
        
        assert_eq!(manager.message_count().await.unwrap(), 3);
        
        // Clear conversation
        manager.clear().await.unwrap();
        assert_eq!(manager.message_count().await.unwrap(), 0);
        assert!(manager.is_empty().await.unwrap());
    }

    #[tokio::test]
    async fn test_summarizing_conversation_manager() {
        let mut manager = SummarizingConversationManager::new(2);
        
        assert_eq!(manager.message_count().await.unwrap(), 0);
        assert!(manager.is_empty().await.unwrap());
        
        // Add messages
        manager.add_message(Message::user("Hello")).await.unwrap();
        manager.add_message(Message::assistant("Hi!")).await.unwrap();
        
        assert_eq!(manager.message_count().await.unwrap(), 2);
        assert!(!manager.is_empty().await.unwrap());
        
        // Clear conversation
        manager.clear().await.unwrap();
        assert_eq!(manager.message_count().await.unwrap(), 0);
        assert!(manager.is_empty().await.unwrap());
    }
}
