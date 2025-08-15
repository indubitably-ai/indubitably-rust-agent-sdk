//! Tool decorator for easily creating tools from functions.
//! 
//! This module provides a decorator macro and utilities for
//! converting Rust functions into tools that agents can use.

use std::sync::Arc;
use serde_json::Value;

use crate::types::IndubitablyResult;
use super::registry::Tool;

/// Create a tool from a function.
/// 
/// This macro creates a tool that can be registered with the tool registry.
/// The function should take a `serde_json::Value` and return a `IndubitablyResult<serde_json::Value>`.
/// 
/// # Example
/// 
/// ```rust
/// use indubitably_rust_agent_sdk::tools::tool;
/// 
/// #[tool("calculator", "A simple calculator tool")]
/// fn calculator(input: Value) -> IndubitablyResult<Value> {
///     // Tool implementation
///     Ok(Value::String("result".to_string()))
/// }
/// ```
#[macro_export]
macro_rules! tool {
    ($name:expr, $description:expr) => {
        pub fn $name(input: serde_json::Value) -> $crate::types::IndubitablyResult<serde_json::Value> {
            // This is a placeholder - the actual implementation will be provided
            // by the function that uses this macro
            unimplemented!("Tool function not implemented")
        }
        
        pub fn create_$name() -> $crate::tools::registry::Tool {
            use std::sync::Arc;
            $crate::tools::registry::Tool::new(
                $name,
                $description,
                Arc::new($name),
            )
        }
    };
}

/// Create a tool from a function with custom metadata.
pub fn create_tool<F>(name: &str, description: &str, function: F) -> Tool
where
    F: Fn(Value) -> IndubitablyResult<Value> + Send + Sync + 'static,
{
    Tool::new(name, description, Arc::new(function))
}

/// Create a tool from a function that takes a string input.
pub fn create_string_tool<F>(name: &str, description: &str, function: F) -> Tool
where
    F: Fn(&str) -> IndubitablyResult<String> + Send + Sync + 'static,
{
    let wrapped_function = move |input: Value| {
        let input_str = input.as_str().ok_or_else(|| {
            crate::types::IndubitablyError::ToolError(
                crate::types::ToolError::InvalidInput("Expected string input".to_string()),
            )
        })?;
        
        let result = function(input_str)?;
        Ok(Value::String(result))
    };
    
    Tool::new(name, description, Arc::new(wrapped_function))
}

/// Create a tool from a function that takes a JSON object input.
pub fn create_json_tool<F>(name: &str, description: &str, function: F) -> Tool
where
    F: Fn(Value) -> IndubitablyResult<Value> + Send + Sync + 'static,
{
    Tool::new(name, description, Arc::new(function))
}

/// Create a tool from a function that takes no input.
pub fn create_no_input_tool<F>(name: &str, description: &str, function: F) -> Tool
where
    F: Fn() -> IndubitablyResult<Value> + Send + Sync + 'static,
{
    let wrapped_function = move |_input: Value| {
        function()
    };
    
    Tool::new(name, description, Arc::new(wrapped_function))
}

/// Create a tool from a function that takes multiple string inputs.
pub fn create_multi_string_tool<F>(name: &str, description: &str, function: F) -> Tool
where
    F: Fn(Vec<&str>) -> IndubitablyResult<String> + Send + Sync + 'static,
{
    let wrapped_function = move |input: Value| {
        let input_array = input.as_array().ok_or_else(|| {
            crate::types::IndubitablyError::ToolError(
                crate::types::ToolError::InvalidInput("Expected array input".to_string()),
            )
        })?;
        
        let strings: Result<Vec<&str>, _> = input_array
            .iter()
            .map(|v| v.as_str().ok_or_else(|| {
                crate::types::IndubitablyError::ToolError(
                    crate::types::ToolError::InvalidInput("Expected string in array".to_string()),
                )
            }))
            .collect();
        
        let strings = strings?;
        let result = function(strings)?;
        Ok(Value::String(result))
    };
    
    Tool::new(name, description, Arc::new(wrapped_function))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_create_string_tool() {
        let tool = create_string_tool(
            "uppercase",
            "Convert text to uppercase",
            |input| Ok(input.to_uppercase()),
        );
        
        assert_eq!(tool.name, "uppercase");
        assert_eq!(tool.description, "Convert text to uppercase");
        
        let result = tool.execute(json!("hello"));
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), json!("HELLO"));
    }

    #[test]
    fn test_create_no_input_tool() {
        let tool = create_no_input_tool(
            "timestamp",
            "Get current timestamp",
            || Ok(json!(chrono::Utc::now().timestamp())),
        );
        
        assert_eq!(tool.name, "timestamp");
        assert_eq!(tool.description, "Get current timestamp");
        
        let result = tool.execute(json!(null));
        assert!(result.is_ok());
    }

    #[test]
    fn test_create_multi_string_tool() {
        let tool = create_multi_string_tool(
            "join",
            "Join strings with separator",
            |strings| Ok(strings.join(" ")),
        );
        
        assert_eq!(tool.name, "join");
        assert_eq!(tool.description, "Join strings with separator");
        
        let result = tool.execute(json!(["hello", "world"]));
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), json!("hello world"));
    }
}
