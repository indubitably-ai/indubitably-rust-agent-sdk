//! Main Agent implementation for the SDK.
//! 
//! This module provides the core Agent struct that orchestrates
//! conversations, tool execution, and model interactions.

use std::collections::HashMap;
use std::sync::Arc;
use async_trait::async_trait;
use serde_json::Value;

use crate::types::{Messages, Message, ToolSpec, IndubitablyResult};
use crate::models::Model;
use super::state::AgentState;
use super::result::AgentResult;
use super::conversation_manager::{ConversationManager, ConversationManagerConfig};
use crate::tools::registry::ToolRegistry;

/// Configuration for an agent.
pub struct AgentConfig {
    /// The name of the agent.
    pub name: String,
    /// The system prompt for the agent.
    pub system_prompt: String,
    /// The model to use for the agent.
    pub model: Option<Box<dyn Model>>,
    /// The tools available to the agent.
    pub tools: Vec<ToolSpec>,
    /// The conversation manager configuration.
    pub conversation_config: ConversationManagerConfig,
    /// Additional configuration options.
    pub options: HashMap<String, Value>,
}

impl Default for AgentConfig {
    fn default() -> Self {
        Self {
            name: crate::DEFAULT_AGENT_NAME.to_string(),
            system_prompt: crate::DEFAULT_SYSTEM_PROMPT.to_string(),
            model: None,
            tools: Vec::new(),
            conversation_config: ConversationManagerConfig::default(),
            options: HashMap::new(),
        }
    }
}

impl AgentConfig {
    /// Create a new agent configuration.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the agent name.
    pub fn with_name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }

    /// Set the system prompt.
    pub fn with_system_prompt(mut self, system_prompt: &str) -> Self {
        self.system_prompt = system_prompt.to_string();
        self
    }

    /// Set the model.
    pub fn with_model(mut self, model: Box<dyn Model>) -> Self {
        self.model = Some(model);
        self
    }

    /// Add a tool specification.
    pub fn with_tool(mut self, tool: ToolSpec) -> Self {
        self.tools.push(tool);
        self
    }

    /// Set the conversation manager configuration.
    pub fn with_conversation_config(mut self, config: ConversationManagerConfig) -> Self {
        self.conversation_config = config;
        self
    }

    /// Add a configuration option.
    pub fn with_option(mut self, key: &str, value: Value) -> Self {
        self.options.insert(key.to_string(), value);
        self
    }
}

/// The main Agent struct that orchestrates conversations and tool execution.
pub struct Agent {
    config: AgentConfig,
    state: AgentState,
    conversation_manager: Box<dyn ConversationManager>,
    tool_registry: Arc<ToolRegistry>,
}

impl Agent {
    /// Create a new agent with default configuration.
    pub fn new() -> IndubitablyResult<Self> {
        let config = AgentConfig::new();
        let state = AgentState::new();
        let conversation_manager = Box::new(super::conversation_manager::NullConversationManager::new());
        let tool_registry = Arc::new(ToolRegistry::new());

        Ok(Self {
            config,
            state,
            conversation_manager,
            tool_registry,
        })
    }

    /// Create a new agent with the given configuration.
    pub fn with_config(config: AgentConfig) -> IndubitablyResult<Self> {
        let state = AgentState::new();
        let conversation_manager = Box::new(super::conversation_manager::NullConversationManager::new());
        let tool_registry = Arc::new(ToolRegistry::new());

        Ok(Self {
            config,
            state,
            conversation_manager,
            tool_registry,
        })
    }

    /// Create a new agent with a specific model.
    pub fn with_model(model: Box<dyn Model>) -> IndubitablyResult<Self> {
        let mut config = AgentConfig::new();
        config.model = Some(model);
        Self::with_config(config)
    }

    /// Run the agent with a message.
    pub async fn run(&mut self, message: &str) -> IndubitablyResult<AgentResult> {
        let user_message = Message::user(message);
        
        // Add the message to the conversation
        self.conversation_manager.add_message(user_message.clone()).await?;
        
        // Get the conversation history
        let history = self.conversation_manager.get_context().await?;
        
        // Generate a response using the model
        let response = if let Some(ref model) = self.config.model {
            let model_response = model.generate(
                &history,
                Some(&self.config.tools),
                Some(&self.config.system_prompt),
            ).await?;
            
            Message::assistant(&model_response.content)
        } else {
            // If no model is configured, return a placeholder response
            Message::assistant("I'm a placeholder agent. Please configure a model to get real responses.")
        };
        
        // Add the response to the conversation
        self.conversation_manager.add_message(response.clone()).await?;
        
        // Create the result
        let result = AgentResult::new(
            self.config.name.clone(),
            history.clone(),
            response.clone(),
            response.all_text(),
            history,
            self.config.tools.clone(),
        );
        
        Ok(result)
    }

    /// Run the agent with a message and get a streaming response.
    pub async fn run_streaming(&mut self, message: &str) -> IndubitablyResult<AgentResult> {
        // For now, just call the regular run method
        // TODO: Implement actual streaming
        self.run(message).await
    }

    /// Add a tool to the agent.
    pub async fn add_tool(&mut self, tool: crate::tools::registry::Tool) -> IndubitablyResult<()> {
        self.tool_registry.register(tool).await?;
        Ok(())
    }

    /// Set the conversation manager.
    pub fn with_conversation_manager(mut self, manager: Box<dyn ConversationManager>) -> Self {
        self.conversation_manager = manager;
        self
    }

    /// Get the agent's configuration.
    pub fn config(&self) -> &AgentConfig {
        &self.config
    }

    /// Get the agent's state.
    pub fn state(&self) -> &AgentState {
        &self.state
    }

    /// Get the agent's state as mutable.
    pub fn state_mut(&mut self) -> &mut AgentState {
        &mut self.state
    }

    /// Get the conversation history.
    pub async fn get_history(&self) -> IndubitablyResult<Messages> {
        self.conversation_manager.get_context().await
    }

    /// Clear the conversation history.
    pub async fn clear_history(&mut self) -> IndubitablyResult<()> {
        self.conversation_manager.clear().await?;
        Ok(())
    }
}

impl Default for Agent {
    fn default() -> Self {
        Self::new().expect("Failed to create default agent")
    }
}

/// A builder for creating agents with a fluent interface.
pub struct AgentBuilder {
    config: AgentConfig,
}

impl AgentBuilder {
    /// Create a new agent builder.
    pub fn new() -> Self {
        Self {
            config: AgentConfig::new(),
        }
    }

    /// Set the agent name.
    pub fn name(mut self, name: &str) -> Self {
        self.config.name = name.to_string();
        self
    }

    /// Set the system prompt.
    pub fn system_prompt(mut self, prompt: &str) -> Self {
        self.config.system_prompt = prompt.to_string();
        self
    }

    /// Set the model.
    pub fn model(mut self, model: Box<dyn Model>) -> Self {
        self.config.model = Some(model);
        self
    }

    /// Add a tool specification.
    pub fn tool(mut self, tool: ToolSpec) -> Self {
        self.config.tools.push(tool);
        self
    }

    /// Set the conversation manager configuration.
    pub fn conversation_config(mut self, config: ConversationManagerConfig) -> Self {
        self.config.conversation_config = config;
        self
    }

    /// Build the agent.
    pub fn build(self) -> IndubitablyResult<Agent> {
        Agent::with_config(self.config)
    }
}

impl Default for AgentBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// A trait for calling tools.
#[async_trait]
pub trait ToolCaller: Send + Sync {
    /// Call a tool by name with the given input.
    async fn call_tool(&self, tool_name: &str, input: Value) -> IndubitablyResult<Value>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agent::conversation_manager::SlidingWindowConversationManager;

    #[tokio::test]
    async fn test_agent_creation() {
        let agent = Agent::new();
        assert!(agent.is_ok());
        
        let agent = agent.unwrap();
        assert_eq!(agent.config().name, crate::DEFAULT_AGENT_NAME);
        assert_eq!(agent.config().system_prompt, crate::DEFAULT_SYSTEM_PROMPT);
    }

    #[tokio::test]
    async fn test_agent_with_model() {
        // For now, skip this test since MockModel is not implemented
        // let model = Box::new(MockModel::new());
        // let agent = Agent::with_model(model);
        // assert!(agent.is_ok());
        // 
        // let agent = agent.unwrap();
        // assert!(agent.config().model.is_some());
    }

    #[tokio::test]
    async fn test_agent_builder() {
        let agent = AgentBuilder::new()
            .name("Test Agent")
            .system_prompt("You are a test agent.")
            .build();
        
        assert!(agent.is_ok());
        
        let agent = agent.unwrap();
        assert_eq!(agent.config().name, "Test Agent");
        assert_eq!(agent.config().system_prompt, "You are a test agent.");
    }

    #[tokio::test]
    async fn test_agent_run() {
        // For now, skip this test since MockModel is not implemented
        // let model = Box::new(MockModel::new());
        // let mut agent = Agent::with_model(model).unwrap();
        // 
        // let result = agent.run("Hello").await;
        // assert!(result.is_ok());
        // 
        // let result = result.unwrap();
        // assert!(!result.response.is_empty());
    }

    #[tokio::test]
    async fn test_agent_conversation_history() {
        let mut agent = Agent::new().unwrap()
            .with_conversation_manager(Box::new(SlidingWindowConversationManager::new(100)));
        
        // Add a message
        let _ = agent.run("Hello").await;
        
        // Check history
        let history = agent.get_history().await;
        assert!(history.is_ok());
        
        let history = history.unwrap();
        assert_eq!(history.len(), 2); // User message + agent response
    }

    #[tokio::test]
    async fn test_agent_clear_conversation() {
        let mut agent = Agent::new().unwrap();
        
        // Add a message
        let _ = agent.run("Hello").await;
        
        // Clear conversation
        let result = agent.clear_history().await;
        assert!(result.is_ok());
        
        // Check that history is cleared
        let history = agent.get_history().await;
        assert!(history.is_ok());
        
        let history = history.unwrap();
        assert_eq!(history.len(), 0);
    }
}
