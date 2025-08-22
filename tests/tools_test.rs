//! Unit tests for the tools module.
//! 
//! These tests verify that all tool functionality works correctly
//! including registration, execution, and decorators.

use indubitably_rust_agent_sdk::tools::*;
use indubitably_rust_agent_sdk::types::{ToolSpec, ToolUse, ToolResult, ToolResultContent};
use std::collections::HashMap;

#[tokio::test]
async fn test_tool_registry_creation() {
    let registry = ToolRegistry::new();
    
    assert_eq!(registry.len().await, 0);
    assert!(registry.is_empty().await);
}

#[tokio::test]
async fn test_tool_registration() {
    let mut registry = ToolRegistry::new();
    
    let tool = Tool::new(
        "test_tool",
        "A test tool",
        Box::new(|input| {
            let input_str = input.as_str().unwrap_or("default");
            Ok(format!("Processed: {}", input_str))
        }),
    );
    
    registry.register(tool).await.unwrap();
    
    assert_eq!(registry.len().await, 1);
    assert!(!registry.is_empty().await);
}

#[tokio::test]
async fn test_tool_registration_duplicate() {
    let mut registry = ToolRegistry::new();
    
    let tool1 = Tool::new("duplicate", "First tool", Box::new(|_| Ok("result1")));
    let tool2 = Tool::new("duplicate", "Second tool", Box::new(|_| Ok("result2")));
    
    registry.register(tool1).await.unwrap();
    let result = registry.register(tool2).await;
    
    // Should fail due to duplicate name
    assert!(result.is_err());
    assert_eq!(registry.len().await, 1);
}

#[tokio::test]
async fn test_tool_execution() {
    let mut registry = ToolRegistry::new();
    
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
    
    let result = registry.execute("calculator", serde_json::Value::String("5".to_string())).await.unwrap();
    assert_eq!(result.as_i64().unwrap(), 10);
}

#[tokio::test]
async fn test_tool_execution_nonexistent() {
    let registry = ToolRegistry::new();
    
    let result = registry.execute("nonexistent", serde_json::Value::Null).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_tool_listing() {
    let mut registry = ToolRegistry::new();
    
    let tool1 = Tool::new("tool1", "First tool", Box::new(|_| Ok("result1")));
    let tool2 = Tool::new("tool2", "Second tool", Box::new(|_| Ok("result2")));
    
    registry.register(tool1).await.unwrap();
    registry.register(tool2).await.unwrap();
    
    let tools = registry.list_tools().await;
    assert_eq!(tools.len(), 2);
    
    let tool_names: Vec<&str> = tools.iter().map(|t| t.name.as_str()).collect();
    assert!(tool_names.contains(&"tool1"));
    assert!(tool_names.contains(&"tool2"));
}

#[tokio::test]
async fn test_tool_metadata() {
    let mut registry = ToolRegistry::new();
    
    let mut metadata = HashMap::new();
    metadata.insert("version".to_string(), "1.0.0".to_string());
    metadata.insert("category".to_string(), "utility".to_string());
    
    let tool = Tool::new("metadata_tool", "Tool with metadata", Box::new(|_| Ok("result")))
        .with_metadata(metadata);
    
    registry.register(tool).await.unwrap();
    
    let tools = registry.list_tools().await;
    let tool = tools.first().unwrap();
    
    assert_eq!(tool.metadata.get("version").unwrap(), "1.0.0");
    assert_eq!(tool.metadata.get("category").unwrap(), "utility");
}

#[tokio::test]
async fn test_tool_spec_creation() {
    let spec = ToolSpec::new("test_tool", "A test tool");
    assert_eq!(spec.name, "test_tool");
    assert_eq!(spec.description, "A test tool");
    assert!(spec.input_schema.is_none());
    assert!(spec.output_schema.is_none());
}

#[tokio::test]
async fn test_tool_spec_with_schemas() {
    let input_schema = serde_json::json!({
        "type": "object",
        "properties": {
            "text": {"type": "string"}
        },
        "required": ["text"]
    });
    
    let output_schema = serde_json::json!({
        "type": "object",
        "properties": {
            "result": {"type": "string"}
        }
    });
    
    let spec = ToolSpec::new("schema_tool", "Tool with schemas")
        .with_input_schema(input_schema.clone())
        .with_output_schema(output_schema.clone());
    
    assert_eq!(spec.name, "schema_tool");
    assert!(spec.input_schema.is_some());
    assert!(spec.output_schema.is_some());
    assert_eq!(spec.input_schema.unwrap(), input_schema);
    assert_eq!(spec.output_schema.unwrap(), output_schema);
}

#[tokio::test]
async fn test_tool_use_creation() {
    let tool_use = ToolUse::new("test_tool", "tool_123");
    assert_eq!(tool_use.name, "test_tool");
    assert_eq!(tool_use.tool_use_id, "tool_123");
    assert!(tool_use.input.is_none());
}

#[tokio::test]
async fn test_tool_use_with_input() {
    let input = serde_json::json!({"text": "hello"});
    let tool_use = ToolUse::new("test_tool", "tool_123").with_input(input.clone());
    
    assert_eq!(tool_use.name, "test_tool");
    assert_eq!(tool_use.tool_use_id, "tool_123");
    assert_eq!(tool_use.input.unwrap(), input);
}

#[tokio::test]
async fn test_tool_result_creation() {
    let content = vec![ToolResultContent::text("Success!")];
    let result = ToolResult::new("tool_123", content);
    
    assert_eq!(result.tool_use_id, "tool_123");
    assert_eq!(result.content.len(), 1);
    assert!(result.is_error.is_none());
}

#[tokio::test]
async fn test_tool_result_error() {
    let result = ToolResult::error("tool_123", "Something went wrong");
    
    assert_eq!(result.tool_use_id, "tool_123");
    assert_eq!(result.content.len(), 1);
    assert_eq!(result.is_error, Some(true));
    
    let content = &result.content[0];
    assert_eq!(content.text.as_ref().unwrap(), "Something went wrong");
}

#[tokio::test]
async fn test_tool_executor_basic() {
    let executor = ToolExecutor::new();
    
    let tool = Tool::new(
        "test_executor",
        "Test executor tool",
        Box::new(|input| {
            let input_str = input.as_str().unwrap_or("default");
            Ok(format!("Executed: {}", input_str))
        }),
    );
    
    let context = ToolExecutionContext::new();
    let result = executor.execute(&tool, serde_json::Value::String("test".to_string()), context).await.unwrap();
    
    assert_eq!(result.as_str().unwrap(), "Executed: test");
}

#[tokio::test]
async fn test_tool_executor_with_context() {
    let executor = ToolExecutor::new();
    
    let tool = Tool::new(
        "context_tool",
        "Tool that uses context",
        Box::new(|input| {
            let input_str = input.as_str().unwrap_or("default");
            Ok(format!("Context: {}", input_str))
        }),
    );
    
    let mut context = ToolExecutionContext::new();
    context.set("user_id", "user_123");
    context.set("session_id", "session_456");
    
    let result = executor.execute(&tool, serde_json::Value::String("test".to_string()), context).await.unwrap();
    
    assert_eq!(result.as_str().unwrap(), "Context: test");
}

#[tokio::test]
async fn test_tool_decorator_basic() {
    let decorator = ToolDecorator::new("decorated_tool");
    
    let tool = Tool::new(
        "original_tool",
        "Original tool",
        Box::new(|input| {
            let input_str = input.as_str().unwrap_or("default");
            Ok(format!("Original: {}", input_str))
        }),
    );
    
    let decorated_tool = decorator.decorate(tool);
    
    assert_eq!(decorated_tool.name(), "decorated_tool");
    assert_eq!(decorated_tool.description(), "Original tool");
}

#[tokio::test]
async fn test_tool_registry_clear() {
    let mut registry = ToolRegistry::new();
    
    let tool = Tool::new("test_tool", "A test tool", Box::new(|_| Ok("result")));
    registry.register(tool).await.unwrap();
    
    assert_eq!(registry.len().await, 1);
    
    registry.clear().await;
    assert_eq!(registry.len().await, 0);
    assert!(registry.is_empty().await);
}

#[tokio::test]
async fn test_tool_registry_remove() {
    let mut registry = ToolRegistry::new();
    
    let tool = Tool::new("removable_tool", "A removable tool", Box::new(|_| Ok("result")));
    registry.register(tool).await.unwrap();
    
    assert_eq!(registry.len().await, 1);
    
    let removed = registry.remove("removable_tool").await.unwrap();
    assert_eq!(removed.name(), "removable_tool");
    assert_eq!(registry.len().await, 0);
}

#[tokio::test]
async fn test_tool_registry_remove_nonexistent() {
    let registry = ToolRegistry::new();
    
    let result = registry.remove("nonexistent").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_tool_function_signature() {
    let tool = Tool::new(
        "signature_tool",
        "Tool with signature",
        Box::new(|input| {
            // Test that we can access the input
            let input_str = input.as_str().unwrap_or("default");
            Ok(format!("Signature: {}", input_str))
        }),
    );
    
    assert_eq!(tool.name(), "signature_tool");
    assert_eq!(tool.description(), "Tool with signature");
    
    // Test execution
    let result = tool.execute(serde_json::Value::String("test".to_string())).await.unwrap();
    assert_eq!(result.as_str().unwrap(), "Signature: test");
}

#[tokio::test]
async fn test_tool_error_handling() {
    let tool = Tool::new(
        "error_tool",
        "Tool that returns errors",
        Box::new(|_| {
            Err("Intentional error".into())
        }),
    );
    
    let result = tool.execute(serde_json::Value::Null).await;
    assert!(result.is_err());
    let error = result.unwrap_err();
    assert!(error.to_string().contains("Intentional error"));
}

#[tokio::test]
async fn test_tool_registry_concurrent_access() {
    let registry = std::sync::Arc::new(tokio::sync::RwLock::new(ToolRegistry::new()));
    
    // Spawn multiple tasks to test concurrent access
    let mut handles = vec![];
    
    for i in 0..5 {
        let registry_clone = registry.clone();
        let handle = tokio::spawn(async move {
            let mut registry = registry_clone.write().await;
            let tool = Tool::new(
                format!("tool_{}", i),
                format!("Tool {}", i),
                Box::new(|_| Ok(format!("result_{}", i))),
            );
            registry.register(tool).await
        });
        handles.push(handle);
    }
    
    // Wait for all tasks to complete
    for handle in handles {
        handle.await.unwrap().unwrap();
    }
    
    let registry = registry.read().await;
    assert_eq!(registry.len().await, 5);
}

#[test]
fn test_tool_metadata_serialization() {
    let mut metadata = HashMap::new();
    metadata.insert("version".to_string(), "1.0.0".to_string());
    metadata.insert("author".to_string(), "Test Author".to_string());
    
    let json = serde_json::to_string(&metadata).unwrap();
    let deserialized: HashMap<String, String> = serde_json::from_str(&json).unwrap();
    
    assert_eq!(deserialized.get("version").unwrap(), "1.0.0");
    assert_eq!(deserialized.get("author").unwrap(), "Test Author");
}

#[test]
fn test_tool_spec_serialization() {
    let spec = ToolSpec::new("serializable_tool", "A serializable tool")
        .with_input_schema(serde_json::json!({"type": "string"}))
        .with_output_schema(serde_json::json!({"type": "string"}));
    
    let json = serde_json::to_string(&spec).unwrap();
    let deserialized: ToolSpec = serde_json::from_str(&json).unwrap();
    
    assert_eq!(deserialized.name, spec.name);
    assert_eq!(deserialized.description, spec.description);
    assert!(deserialized.input_schema.is_some());
    assert!(deserialized.output_schema.is_some());
}
