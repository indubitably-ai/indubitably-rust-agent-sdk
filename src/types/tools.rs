//! Tool-related type definitions for the SDK.
//! 
//! This module defines the types used to represent tools, tool specifications,
//! tool use requests, and tool results.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A tool specification that describes a tool's interface.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ToolSpec {
    /// The name of the tool.
    pub name: String,
    /// A description of what the tool does.
    pub description: String,
    /// The input schema for the tool.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_schema: Option<serde_json::Value>,
    /// The output schema for the tool.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_schema: Option<serde_json::Value>,
    /// Additional metadata for the tool.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

/// A tool use request from a model.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ToolUse {
    /// The name of the tool to use.
    pub name: String,
    /// The input parameters for the tool.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<serde_json::Value>,
    /// The tool use ID for tracking.
    #[serde(rename = "toolUseId")]
    pub tool_use_id: String,
}

/// The result of a tool execution.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ToolResult {
    /// The tool use ID this result corresponds to.
    #[serde(rename = "toolUseId")]
    pub tool_use_id: String,
    /// The content of the tool result.
    pub content: Vec<ToolResultContent>,
    /// Whether the tool execution was successful.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_error: Option<bool>,
}

/// Content within a tool result.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ToolResultContent {
    /// The type of content.
    #[serde(rename = "type")]
    pub content_type: ToolResultContentType,
    /// The text content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// The image content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<serde_json::Value>,
}

/// The type of tool result content.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ToolResultContentType {
    Text,
    Image,
}

impl ToolSpec {
    /// Create a new tool specification.
    pub fn new(name: &str, description: &str) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            input_schema: None,
            output_schema: None,
            metadata: None,
        }
    }

    /// Set the input schema for the tool.
    pub fn with_input_schema(mut self, schema: serde_json::Value) -> Self {
        self.input_schema = Some(schema);
        self
    }

    /// Set the output schema for the tool.
    pub fn with_output_schema(mut self, schema: serde_json::Value) -> Self {
        self.output_schema = Some(schema);
        self
    }

    /// Add metadata to the tool.
    pub fn with_metadata(mut self, key: &str, value: serde_json::Value) -> Self {
        if self.metadata.is_none() {
            self.metadata = Some(HashMap::new());
        }
        if let Some(ref mut metadata) = self.metadata {
            metadata.insert(key.to_string(), value);
        }
        self
    }
}

impl ToolUse {
    /// Create a new tool use request.
    pub fn new(name: &str, tool_use_id: &str) -> Self {
        Self {
            name: name.to_string(),
            input: None,
            tool_use_id: tool_use_id.to_string(),
        }
    }

    /// Set the input parameters for the tool.
    pub fn with_input(mut self, input: serde_json::Value) -> Self {
        self.input = Some(input);
        self
    }
}

impl ToolResult {
    /// Create a new tool result.
    pub fn new(tool_use_id: &str, content: Vec<ToolResultContent>) -> Self {
        Self {
            tool_use_id: tool_use_id.to_string(),
            content,
            is_error: None,
        }
    }

    /// Create a new error tool result.
    pub fn error(tool_use_id: &str, error_message: &str) -> Self {
        Self {
            tool_use_id: tool_use_id.to_string(),
            content: vec![ToolResultContent {
                content_type: ToolResultContentType::Text,
                text: Some(error_message.to_string()),
                image: None,
            }],
            is_error: Some(true),
        }
    }

    /// Set whether the tool execution was successful.
    pub fn with_is_error(mut self, is_error: bool) -> Self {
        self.is_error = Some(is_error);
        self
    }
}

impl ToolResultContent {
    /// Create a new text content block.
    pub fn text(text: &str) -> Self {
        Self {
            content_type: ToolResultContentType::Text,
            text: Some(text.to_string()),
            image: None,
        }
    }

    /// Create a new image content block.
    pub fn image(image: serde_json::Value) -> Self {
        Self {
            content_type: ToolResultContentType::Image,
            text: None,
            image: Some(image),
        }
    }
}
