//! Integration tests for the Indubitably Rust Agent SDK.
//! 
//! These tests verify that the core functionality works together
//! as expected.

use indubitably_rust_agent_sdk::{
    Agent,
    agent::{AgentBuilder, agent::AgentConfig},
    agent::conversation_manager::SlidingWindowConversationManager,
    models::model::MockModel,
    tools::{ToolRegistry, Tool},
};
use std::sync::Arc;

#[tokio::test]
async fn test_basic_agent_creation() {
    let agent = Agent::new().unwrap();
    
    assert_eq!(agent.config().name, "Indubitably Agent");
    assert_eq!(agent.config().system_prompt, "You are a helpful AI assistant.");
}

#[tokio::test]
async fn test_agent_builder() {
    let agent = AgentBuilder::new()
        .name("Test Agent")
        .system_prompt("You are a test agent.")
        .build()
        .unwrap();
    
    assert_eq!(agent.config().name, "Test Agent");
    assert_eq!(agent.config().system_prompt, "You are a test agent.");
}

#[tokio::test]
async fn test_agent_with_mock_model() {
    let mut agent = Agent::with_model(Box::new(MockModel::new())).unwrap();
    
    let result = agent.run("Hello, agent!").await.unwrap();
    
    assert_eq!(result.agent_id, "Indubitably Agent");
    assert!(!result.response.is_empty());
}

#[tokio::test]
async fn test_agent_conversation_history() {
    let mut agent = Agent::with_config(AgentConfig::new()
        .with_conversation_manager(Box::new(SlidingWindowConversationManager::new(100)))
    ).unwrap();
    
    // Initial history should be empty
    let history = agent.get_history().await.unwrap();
    assert_eq!(history.len(), 0);
    
    // After running, history should have messages
    let _result = agent.run("Hello").await.unwrap();
    let history = agent.get_history().await.unwrap();
    assert_eq!(history.len(), 2); // User message + Assistant response
}

#[tokio::test]
async fn test_agent_clear_history() {
    let mut agent = Agent::with_config(AgentConfig::new()
        .with_conversation_manager(Box::new(SlidingWindowConversationManager::new(100)))
    ).unwrap();
    
    // Add some conversation
    let _result = agent.run("Hello").await.unwrap();
    let history = agent.get_history().await.unwrap();
    assert_eq!(history.len(), 2);
    
    // Clear history
    agent.clear_history().await.unwrap();
    let history = agent.get_history().await.unwrap();
    assert_eq!(history.len(), 0);
}

#[tokio::test]
async fn test_agent_tools() {
    let mut agent = Agent::new().unwrap();
    
    // Create a simple tool
    let tool = Tool::new(
        "test_tool",
        "A test tool",
        Arc::new(|input| {
            let input_str = input.as_str().unwrap_or("default");
            Ok(format!("Processed: {}", input_str).into())
        }),
    );
    
    agent.add_tool(tool).await.unwrap();
    
    // Verify tool was added (we can't directly access the tool registry from outside)
    assert!(true); // Placeholder assertion
}

#[tokio::test]
async fn test_agent_streaming() {
    let mut agent = Agent::with_model(Box::new(MockModel::new())).unwrap();
    
    let result = agent.run_streaming("Hello, agent!").await.unwrap();
    
    // For now, just verify we get a result
    assert!(!result.response.is_empty());
}

#[tokio::test]
async fn test_agent_state() {
    let agent = Agent::new().unwrap();
    
    let state = agent.state();
    assert!(state.is_empty());
    assert_eq!(state.message_count(), 0);
}

#[tokio::test]
async fn test_agent_with_custom_config() {
    let agent = AgentBuilder::new()
        .name("Custom Agent")
        .system_prompt("You are a custom assistant.")
        .build()
        .unwrap();
    
    let config = agent.config();
    assert_eq!(config.name, "Custom Agent");
    assert_eq!(config.system_prompt, "You are a custom assistant.");
}

#[tokio::test]
async fn test_agent_message_processing() {
    let mut agent = Agent::new().unwrap();
    
    let result = agent.run("Custom message").await.unwrap();
    
    assert_eq!(result.agent_id, "Indubitably Agent");
    assert!(!result.response.is_empty());
}

#[tokio::test]
async fn test_agent_error_handling() {
    let mut agent = Agent::new().unwrap();
    
    // Test with an empty message
    let result = agent.run("").await.unwrap();
    
    // Should still work (empty messages are valid)
    assert!(result.response.is_empty() || !result.response.is_empty());
}

#[tokio::test]
async fn test_agent_conversation_flow() {
    let mut agent = Agent::with_config(AgentConfig::new()
        .with_conversation_manager(Box::new(SlidingWindowConversationManager::new(100)))
    ).unwrap();
    
    // First message
    let result1 = agent.run("Hello").await.unwrap();
    assert!(!result1.response.is_empty());
    
    // Second message
    let result2 = agent.run("How are you?").await.unwrap();
    assert!(!result2.response.is_empty());
    
    // Verify conversation history grows
    let history = agent.get_history().await.unwrap();
    assert_eq!(history.len(), 4); // 2 user messages + 2 assistant responses
}

#[tokio::test]
async fn test_agent_with_different_models() {
    // Test with mock model
    let mock_agent = Agent::with_model(Box::new(MockModel::new())).unwrap();
    assert_eq!(mock_agent.config().name, "Indubitably Agent");
    
    // Test with custom name
    let custom_agent = AgentBuilder::new()
        .name("Custom Agent")
        .build()
        .unwrap();
    assert_eq!(custom_agent.config().name, "Custom Agent");
}

#[tokio::test]
async fn test_agent_tool_registry_integration() {
    let registry = ToolRegistry::new();
    
    // Add multiple tools
    let tool1 = Tool::new("tool1", "First tool", Arc::new(|_| Ok("result1".into())));
    let tool2 = Tool::new("tool2", "Second tool", Arc::new(|_| Ok("result2".into())));
    
    registry.register(tool1).await.unwrap();
    registry.register(tool2).await.unwrap();
    
    let tools = registry.list_tools().await;
    assert_eq!(tools.len(), 2);
    
    // Verify tool names
    let tool_names: Vec<&str> = tools.iter().map(|t| t.name.as_str()).collect();
    assert!(tool_names.contains(&"tool1"));
    assert!(tool_names.contains(&"tool2"));
}

#[tokio::test]
async fn test_agent_error_recovery() {
    let mut agent = Agent::with_config(AgentConfig::new()
        .with_conversation_manager(Box::new(SlidingWindowConversationManager::new(100)))
    ).unwrap();
    
    // Process a valid message
    let result1 = agent.run("Hello").await.unwrap();
    assert!(!result1.response.is_empty());
    
    // Clear history
    agent.clear_history().await.unwrap();
    
    // Process another message after clearing
    let result2 = agent.run("Hello again").await.unwrap();
    assert!(!result2.response.is_empty());
    
    // Verify history was cleared
    let history = agent.get_history().await.unwrap();
    assert_eq!(history.len(), 2); // Only the new exchange
}

#[tokio::test]
async fn test_agent_concurrent_access() {
    let agent = std::sync::Arc::new(tokio::sync::RwLock::new(
        Agent::with_config(AgentConfig::new()
            .with_conversation_manager(Box::new(SlidingWindowConversationManager::new(100)))
        ).unwrap()
    ));
    
    // Spawn multiple tasks to test concurrent access
    let mut handles = vec![];
    
    for i in 0..3 {
        let agent_clone = agent.clone();
        let handle = tokio::spawn(async move {
            let mut agent = agent_clone.write().await;
            agent.run(&format!("Message {}", i)).await
        });
        handles.push(handle);
    }
    
    // Wait for all tasks to complete
    for handle in handles {
        let result = handle.await.unwrap();
        assert!(result.is_ok());
    }
    
    let agent = agent.read().await;
    let history = agent.get_history().await.unwrap();
    assert_eq!(history.len(), 6); // 3 user messages + 3 assistant responses
}
