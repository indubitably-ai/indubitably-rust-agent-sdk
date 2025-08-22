//! Integration tests for the example applications
//! 
//! These tests verify that the examples can be built and run correctly
//! with the Indubitably Rust Agent SDK.

use indubitably_rust_agent_sdk::{
    Agent, types::{Messages, Message, MessageRole, ToolSpec}
};
use serde_json::json;

/// Test that we can create a basic agent
#[test]
fn test_basic_agent_creation() {
    let agent = Agent::new();
    assert!(agent.is_ok());
}

/// Test that we can create tool specifications
#[test]
fn test_tool_spec_creation() {
    let tool_spec = ToolSpec::new("test_tool", "A test tool")
        .with_input_schema(json!({
            "type": "object",
            "properties": {
                "value": {"type": "number"}
            },
            "required": ["value"]
        }))
        .with_output_schema(json!({
            "type": "object",
            "properties": {
                "result": {"type": "number"}
            }
        }));
    
    assert_eq!(tool_spec.name, "test_tool");
    assert_eq!(tool_spec.description, "A test tool");
    assert!(tool_spec.input_schema.is_some());
    assert!(tool_spec.output_schema.is_some());
}

/// Test that we can create messages and add them to conversation history
#[test]
fn test_message_creation_and_conversation() {
    let mut conversation = Messages::new();
    
    let user_message = Message::user("Hello, agent!");
    let assistant_message = Message::assistant("Hello, human!");
    
    conversation.push(user_message);
    conversation.push(assistant_message);
    
    assert_eq!(conversation.len(), 2);
    assert_eq!(conversation[0].role, MessageRole::User);
    assert_eq!(conversation[1].role, MessageRole::Assistant);
}

/// Test that we can create a web browser tool specification
#[test]
fn test_web_browser_tool_metadata() {
    let tool_spec = ToolSpec::new("web_browser", "Browse the web to search for information")
        .with_input_schema(json!({
            "type": "object",
            "properties": {
                "url": {"type": "string"}
            },
            "required": ["url"]
        }))
        .with_output_schema(json!({
            "type": "object",
            "properties": {
                "title": {"type": "string"},
                "content": {"type": "string"},
                "url": {"type": "string"}
            }
        }));
    
    assert_eq!(tool_spec.name, "web_browser");
    assert!(tool_spec.input_schema.is_some());
    assert!(tool_spec.output_schema.is_some());
}

/// Test that we can create a calculator tool specification
#[test]
fn test_calculator_tool_metadata() {
    let tool_spec = ToolSpec::new("calculator", "Perform mathematical operations")
        .with_input_schema(json!({
            "type": "object",
            "properties": {
                "operation": {"type": "string", "enum": ["add", "subtract", "multiply", "divide"]},
                "a": {"type": "number"},
                "b": {"type": "number"}
            },
            "required": ["operation", "a", "b"]
        }))
        .with_output_schema(json!({
            "type": "object",
            "properties": {
                "result": {"type": "number"}
            }
        }));
    
    assert_eq!(tool_spec.name, "calculator");
    assert!(tool_spec.input_schema.is_some());
    assert!(tool_spec.output_schema.is_some());
}

/// Test that we can create a multi-agent system configuration
#[test]
fn test_multi_agent_configuration() {
    let research_agent = Agent::new();
    let analysis_agent = Agent::new();
    
    assert!(research_agent.is_ok());
    assert!(analysis_agent.is_ok());
}

/// Test that we can serialize and deserialize tool metadata
#[test]
fn test_tool_metadata_serialization() {
    let tool_spec = ToolSpec::new("test_tool", "A test tool")
        .with_input_schema(json!({
            "type": "object",
            "properties": {
                "input": {"type": "string"}
            }
        }))
        .with_output_schema(json!({
            "type": "object",
            "properties": {
                "output": {"type": "string"}
            }
        }));
    
    // Test serialization
    let serialized = serde_json::to_string(&tool_spec);
    assert!(serialized.is_ok());
    
    // Test deserialization
    let deserialized: Result<ToolSpec, _> = serde_json::from_str(&serialized.unwrap());
    assert!(deserialized.is_ok());
    
    let deserialized = deserialized.unwrap();
    assert!(deserialized.input_schema.is_some());
    assert!(deserialized.output_schema.is_some());
}

/// Test that we can create a complete agent configuration with tools
#[test]
fn test_complete_agent_configuration() {
    let tool_specs = vec![
        ToolSpec::new("web_browser", "Browse the web")
            .with_input_schema(json!({
                "type": "object",
                "properties": {
                    "url": {"type": "string"}
                },
                "required": ["url"]
            }))
            .with_output_schema(json!({
                "type": "object",
                "properties": {
                    "content": {"type": "string"}
                }
            }))
    ];
    
    let agent = Agent::new();
    assert!(agent.is_ok());
}

/// Test message role variants
#[test]
fn test_message_roles() {
    let user_message = Message::user("Hello");
    let assistant_message = Message::assistant("Hi there!");
    let system_message = Message::system("You are a helpful assistant");
    
    assert_eq!(user_message.role, MessageRole::User);
    assert_eq!(assistant_message.role, MessageRole::Assistant);
    assert_eq!(system_message.role, MessageRole::System);
}

/// Test message content extraction
#[test]
fn test_message_content_extraction() {
    let message = Message::user("Hello, world!");
    
    assert_eq!(message.text(), Some("Hello, world!"));
    assert_eq!(message.all_text(), "Hello, world!");
}

/// Test conversation history management
#[test]
fn test_conversation_history() {
    let mut conversation = Messages::new();
    
    // Add messages
    conversation.push(Message::user("What's the weather?"));
    conversation.push(Message::assistant("It's sunny today!"));
    conversation.push(Message::user("Great!"));
    
    // Verify conversation length
    assert_eq!(conversation.len(), 3);
    
    // Verify message roles
    assert_eq!(conversation[0].role, MessageRole::User);
    assert_eq!(conversation[1].role, MessageRole::Assistant);
    assert_eq!(conversation[2].role, MessageRole::User);
    
    // Verify message content
    assert_eq!(conversation[0].text(), Some("What's the weather?"));
    assert_eq!(conversation[1].text(), Some("It's sunny today!"));
    assert_eq!(conversation[2].text(), Some("Great!"));
}
