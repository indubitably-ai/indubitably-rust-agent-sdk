//! Tool registry for managing and organizing tools.
//! 
//! This module provides functionality for registering, discovering,
//! and managing tools that agents can use.

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};

use crate::types::{ToolSpec, IndubitablyResult, IndubitablyError};

/// A tool that can be executed by an agent.
#[derive(Clone)]
pub struct Tool {
    /// The name of the tool.
    pub name: String,
    /// The description of the tool.
    pub description: String,
    /// The function that implements the tool.
    pub function: ToolFunction,
    /// Metadata about the tool.
    pub metadata: ToolMetadata,
}

/// A function that implements a tool.
pub type ToolFunction = Arc<dyn Fn(serde_json::Value) -> IndubitablyResult<serde_json::Value> + Send + Sync>;

/// Metadata about a tool.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolMetadata {
    /// The input schema for the tool.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_schema: Option<serde_json::Value>,
    /// The output schema for the tool.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_schema: Option<serde_json::Value>,
    /// Additional metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<HashMap<String, serde_json::Value>>,
}

impl Default for ToolMetadata {
    fn default() -> Self {
        Self {
            input_schema: None,
            output_schema: None,
            extra: None,
        }
    }
}

impl ToolMetadata {
    /// Create a new tool metadata.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the input schema.
    pub fn with_input_schema(mut self, schema: serde_json::Value) -> Self {
        self.input_schema = Some(schema);
        self
    }

    /// Set the output schema.
    pub fn with_output_schema(mut self, schema: serde_json::Value) -> Self {
        self.output_schema = Some(schema);
        self
    }

    /// Add extra metadata.
    pub fn with_extra(mut self, key: &str, value: serde_json::Value) -> Self {
        if self.extra.is_none() {
            self.extra = Some(HashMap::new());
        }
        if let Some(ref mut extra) = self.extra {
            extra.insert(key.to_string(), value);
        }
        self
    }
}

impl Tool {
    /// Create a new tool.
    pub fn new(name: &str, description: &str, function: ToolFunction) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            function,
            metadata: ToolMetadata::default(),
        }
    }

    /// Set the metadata for the tool.
    pub fn with_metadata(mut self, metadata: ToolMetadata) -> Self {
        self.metadata = metadata;
        self
    }

    /// Execute the tool with the given input.
    pub fn execute(&self, input: serde_json::Value) -> IndubitablyResult<serde_json::Value> {
        (self.function)(input)
    }

    /// Get the tool specification.
    pub fn spec(&self) -> ToolSpec {
        ToolSpec::new(&self.name, &self.description)
            .with_input_schema(self.metadata.input_schema.clone().unwrap_or_default())
            .with_output_schema(self.metadata.output_schema.clone().unwrap_or_default())
    }
}

/// A registry for managing tools.
pub struct ToolRegistry {
    tools: Arc<RwLock<HashMap<String, Tool>>>,
}

impl ToolRegistry {
    /// Create a new tool registry.
    pub fn new() -> Self {
        Self {
            tools: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Register a tool in the registry.
    pub async fn register(&self, tool: Tool) -> Result<(), IndubitablyError> {
        let mut tools = self.tools.write().await;
        tools.insert(tool.name.clone(), tool);
        Ok(())
    }

    /// Unregister a tool from the registry.
    pub async fn unregister(&self, name: &str) -> Result<(), IndubitablyError> {
        let mut tools = self.tools.write().await;
        tools.remove(name);
        Ok(())
    }

    /// Get a tool by name.
    pub async fn get(&self, name: &str) -> Option<Tool> {
        let tools = self.tools.read().await;
        tools.get(name).cloned()
    }

    /// Get all tool names.
    pub async fn list_names(&self) -> Vec<String> {
        let tools = self.tools.read().await;
        tools.keys().cloned().collect()
    }

    /// Get all tools.
    pub async fn list_tools(&self) -> Vec<Tool> {
        let tools = self.tools.read().await;
        tools.values().cloned().collect()
    }

    /// Get tool specifications for all tools.
    pub async fn list_specs(&self) -> Vec<ToolSpec> {
        let tools = self.tools.read().await;
        tools.values().map(|tool| tool.spec()).collect()
    }

    /// Check if a tool exists.
    pub async fn exists(&self, name: &str) -> bool {
        let tools = self.tools.read().await;
        tools.contains_key(name)
    }

    /// Get the number of tools in the registry.
    pub async fn count(&self) -> usize {
        let tools = self.tools.read().await;
        tools.len()
    }

    /// Clear all tools from the registry.
    pub async fn clear(&self) -> Result<(), IndubitablyError> {
        let mut tools = self.tools.write().await;
        tools.clear();
        Ok(())
    }
}

impl Default for ToolRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for ToolRegistry {
    fn clone(&self) -> Self {
        Self {
            tools: Arc::clone(&self.tools),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_tool_registry() {
        let registry = ToolRegistry::new();
        
        // Test empty registry
        assert_eq!(registry.count().await, 0);
        assert!(!registry.exists("test_tool").await);
        
        // Test tool registration
        let tool = Tool::new(
            "test_tool",
            "A test tool",
            Arc::new(|input| {
                let input_str = input.as_str().unwrap_or("default");
                Ok(serde_json::Value::String(format!("Processed: {}", input_str)))
            }),
        );
        
        registry.register(tool).await.unwrap();
        assert_eq!(registry.count().await, 1);
        assert!(registry.exists("test_tool").await);
        
        // Test tool execution by getting the tool and executing it
        let tool = registry.get("test_tool").await.unwrap();
        let result = tool.execute(serde_json::Value::String("hello".to_string())).unwrap();
        
        assert_eq!(result.as_str().unwrap(), "Processed: hello");
        
        // Test tool unregistration
        registry.unregister("test_tool").await.unwrap();
        assert_eq!(registry.count().await, 0);
        assert!(!registry.exists("test_tool").await);
    }

    #[tokio::test]
    async fn test_duplicate_registration() {
        let registry = ToolRegistry::new();
        
        let tool1 = Tool::new(
            "duplicate_tool",
            "First tool",
            Arc::new(|_| Ok(serde_json::Value::String("first".to_string()))),
        );
        
        let tool2 = Tool::new(
            "duplicate_tool",
            "Second tool",
            Arc::new(|_| Ok(serde_json::Value::String("second".to_string()))),
        );
        
        registry.register(tool1).await.unwrap();
        let result = registry.register(tool2).await;
        assert!(result.is_ok()); // Current implementation allows overwriting
        
        // Verify that the second tool overwrote the first
        let tool = registry.get("duplicate_tool").await.unwrap();
        let output = tool.execute(serde_json::Value::Null).unwrap();
        assert_eq!(output.as_str().unwrap(), "second");
    }

    #[tokio::test]
    async fn test_tool_not_found() {
        let registry = ToolRegistry::new();
        
        let result = registry.get("nonexistent_tool").await;
        assert!(result.is_none());
    }
}
