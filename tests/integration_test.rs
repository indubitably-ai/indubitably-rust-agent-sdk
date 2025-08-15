//! Integration tests for the Strands SDK.
//! 
//! These tests verify that the core functionality works together
//! as expected.

use strands_sdk_rust::{
    Agent, AgentBuilder, AgentResult,
    models::{MockModel, BedrockModel},
    tools::{ToolRegistry, Tool},
    types::{Message, Messages, ToolSpec},
};

#[tokio::test]
async fn test_basic_agent_creation() {
    let agent = Agent::new();
    
    assert!(!agent.id().is_empty());
    assert_eq!(agent.name(), "Strands Agent");
    assert!(agent.system_prompt().is_some());
}

#[tokio::test]
async fn test_agent_builder() {
    let agent = AgentBuilder::new()
        .name("Test Agent")
        .system_prompt("You are a test agent.")
        .build();
    
    assert_eq!(agent.name(), "Test Agent");
    assert_eq!(agent.system_prompt().unwrap(), "You are a test agent.");
}

#[tokio::test]
async fn test_agent_with_mock_model() {
    let mut agent = Agent::with_model(Box::new(MockModel::new()));
    
    let result = agent.run("Hello, agent!").await.unwrap();
    
    assert_eq!(result.agent_id, agent.id());
    assert!(!result.response.is_empty());
    assert!(result.response.contains("mock response"));
}

#[tokio::test]
async fn test_agent_conversation_context() {
    let mut agent = Agent::new();
    
    // Initial context should be empty
    let context = agent.conversation_context().await.unwrap();
    assert_eq!(context.len(), 0);
    
    // After running, context should have messages
    let _result = agent.run("Hello").await.unwrap();
    let context = agent.conversation_context().await.unwrap();
    assert_eq!(context.len(), 2); // User message + Assistant response
}

#[tokio::test]
async fn test_agent_clear_conversation() {
    let mut agent = Agent::new();
    
    // Add some conversation
    let _result = agent.run("Hello").await.unwrap();
    let context = agent.conversation_context().await.unwrap();
    assert_eq!(context.len(), 2);
    
    // Clear conversation
    agent.clear_conversation().await.unwrap();
    let context = agent.conversation_context().await.unwrap();
    assert_eq!(context.len(), 0);
}

#[tokio::test]
async fn test_agent_tools() {
    let agent = Agent::new();
    
    let tools = agent.list_tools().await;
    assert_eq!(tools.len(), 0);
}

#[tokio::test]
async fn test_agent_with_tools() {
    let mut registry = ToolRegistry::new();
    
    // Create a simple tool
    let tool = Tool::new(
        "test_tool",
        "A test tool",
        Box::new(|input| {
            let input_str = input.as_str().unwrap_or("default");
            Ok(format!("Processed: {}", input_str))
        }),
    );
    
    registry.register(tool).await.unwrap();
    
    let agent = AgentBuilder::new()
        .tool_registry(std::sync::Arc::new(tokio::sync::RwLock::new(registry)))
        .build();
    
    let tools = agent.list_tools().await;
    assert_eq!(tools.len(), 1);
    assert_eq!(tools[0].name, "test_tool");
}

#[tokio::test]
async fn test_agent_execute_tool() {
    let mut registry = ToolRegistry::new();
    
    // Create a simple tool
    let tool = Tool::new(
        "calculator",
        "A simple calculator",
        Box::new(|input| {
            let input_str = input.as_str().unwrap_or("0");
            let number: i32 = input_str.parse().unwrap_or(0);
            Ok(number * 2)
        }),
    );
    
    registry.register(tool).await.unwrap();
    
    let agent = AgentBuilder::new()
        .tool_registry(std::sync::Arc::new(tokio::sync::RwLock::new(registry)))
        .build();
    
    let result = agent.execute_tool("calculator", serde_json::Value::String("5".to_string())).await.unwrap();
    assert_eq!(result.as_i64().unwrap(), 10);
}

#[tokio::test]
async fn test_agent_streaming() {
    let mut agent = Agent::with_model(Box::new(MockModel::new()));
    
    let stream = agent.stream("Hello, agent!").await.unwrap();
    
    // For now, just verify we get a stream
    // In a full implementation, we would consume the stream
    assert!(true); // Placeholder assertion
}

#[tokio::test]
async fn test_agent_state() {
    let agent = Agent::new();
    
    let state = agent.state().await;
    assert!(state.is_empty());
    assert_eq!(state.message_count(), 0);
}

#[tokio::test]
async fn test_agent_with_custom_config() {
    let agent = AgentBuilder::new()
        .config("max_tokens", serde_json::Value::Number(serde_json::Number::from(1000)))
        .config("temperature", serde_json::Value::Number(serde_json::Number::from_f64(0.5).unwrap()))
        .build();
    
    let config = agent.config();
    assert_eq!(config.get("max_tokens").unwrap().as_u64().unwrap(), 1000);
    assert_eq!(config.get("temperature").unwrap().as_f64().unwrap(), 0.5);
}

#[tokio::test]
async fn test_agent_message_processing() {
    let mut agent = Agent::new();
    
    let message = Message::user("Custom message");
    let result = agent.process_message(message).await.unwrap();
    
    assert_eq!(result.agent_id, agent.id());
    assert!(!result.response.is_empty());
}

#[tokio::test]
async fn test_agent_error_handling() {
    let mut agent = Agent::new();
    
    // Test with an empty message
    let message = Message::user("");
    let result = agent.process_message(message).await;
    
    // Should still work (empty messages are valid)
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_agent_tool_specs() {
    let agent = Agent::new();
    
    let tool_specs = agent.list_tools().await;
    assert_eq!(tool_specs.len(), 0);
    
    // Verify tool spec structure
    if let Some(spec) = tool_specs.first() {
        assert!(!spec.name.is_empty());
        assert!(!spec.description.is_empty());
    }
}

#[tokio::test]
async fn test_agent_conversation_flow() {
    let mut agent = Agent::new();
    
    // First message
    let result1 = agent.run("Hello").await.unwrap();
    assert!(!result1.response.is_empty());
    
    // Second message
    let result2 = agent.run("How are you?").await.unwrap();
    assert!(!result2.response.is_empty());
    
    // Verify conversation context grows
    let context = agent.conversation_context().await.unwrap();
    assert_eq!(context.len(), 4); // 2 user messages + 2 assistant responses
}

#[tokio::test]
async fn test_agent_with_different_models() {
    // Test with mock model
    let mock_agent = Agent::with_model(Box::new(MockModel::new()));
    assert_eq!(mock_agent.name(), "Strands Agent");
    
    // Test with custom name
    let custom_agent = AgentBuilder::new()
        .name("Custom Agent")
        .build();
    assert_eq!(custom_agent.name(), "Custom Agent");
}

#[tokio::test]
async fn test_agent_tool_registry_integration() {
    let mut registry = ToolRegistry::new();
    
    // Add multiple tools
    let tool1 = Tool::new("tool1", "First tool", Box::new(|_| Ok("result1")));
    let tool2 = Tool::new("tool2", "Second tool", Box::new(|_| Ok("result2")));
    
    registry.register(tool1).await.unwrap();
    registry.register(tool2).await.unwrap();
    
    let agent = AgentBuilder::new()
        .tool_registry(std::sync::Arc::new(tokio::sync::RwLock::new(registry)))
        .build();
    
    let tools = agent.list_tools().await;
    assert_eq!(tools.len(), 2);
    
    // Verify tool names
    let tool_names: Vec<&str> = tools.iter().map(|t| t.name.as_str()).collect();
    assert!(tool_names.contains(&"tool1"));
    assert!(tool_names.contains(&"tool2"));
}

#[tokio::test]
async fn test_agent_error_recovery() {
    let mut agent = Agent::new();
    
    // Process a valid message
    let result1 = agent.run("Hello").await.unwrap();
    assert!(result1.response.contains("mock response"));
    
    // Clear conversation
    agent.clear_conversation().await.unwrap();
    
    // Process another message after clearing
    let result2 = agent.run("Hello again").await.unwrap();
    assert!(result2.response.contains("mock response"));
    
    // Verify conversation was cleared
    let context = agent.conversation_context().await.unwrap();
    assert_eq!(context.len(), 2); // Only the new exchange
}
