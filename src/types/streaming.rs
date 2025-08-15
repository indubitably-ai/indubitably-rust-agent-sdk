//! Streaming-related type definitions for the SDK.
//! 
//! This module defines the types used to represent streaming events
//! and streaming functionality.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::tools::ToolUse;

/// A streaming event from the model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamEvent {
    /// The type of stream event.
    #[serde(rename = "type")]
    pub event_type: StreamEventType,
    /// The content of the event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Vec<StreamContent>>,
    /// The tool use information.
    #[serde(rename = "toolUse", skip_serializing_if = "Option::is_none")]
    pub tool_use: Option<ToolUse>,
    /// The tool result information.
    #[serde(rename = "toolResult", skip_serializing_if = "Option::is_none")]
    pub tool_result: Option<serde_json::Value>,
    /// The message delta information.
    #[serde(rename = "messageDelta", skip_serializing_if = "Option::is_none")]
    pub message_delta: Option<MessageDelta>,
    /// Additional metadata for the event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

/// The type of stream event.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum StreamEventType {
    MessageStart,
    ContentBlockStart,
    ContentBlockDelta,
    ContentBlockStop,
    ToolUseStart,
    ToolUseDelta,
    ToolUseStop,
    ToolResultStart,
    ToolResultDelta,
    ToolResultStop,
    MessageDelta,
    MessageStop,
    Error,
}

/// Content within a stream event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamContent {
    /// The type of content.
    #[serde(rename = "type")]
    pub content_type: StreamContentType,
    /// The text content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// The image content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<serde_json::Value>,
    /// The document content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<serde_json::Value>,
}

/// The type of stream content.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum StreamContentType {
    Text,
    Image,
    Document,
}

/// A message delta in a stream.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageDelta {
    /// The role of the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    /// The content delta.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Vec<ContentDelta>>,
    /// The stop reason.
    #[serde(rename = "stopReason", skip_serializing_if = "Option::is_none")]
    pub stop_reason: Option<String>,
    /// The stop sequence.
    #[serde(rename = "stopSequence", skip_serializing_if = "Option::is_none")]
    pub stop_sequence: Option<String>,
}

/// A content delta in a message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentDelta {
    /// The type of content.
    #[serde(rename = "type")]
    pub content_type: String,
    /// The text delta.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// The image delta.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<serde_json::Value>,
    /// The document delta.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<serde_json::Value>,
}

impl StreamEvent {
    /// Create a new message start event.
    pub fn message_start() -> Self {
        Self {
            event_type: StreamEventType::MessageStart,
            content: None,
            tool_use: None,
            tool_result: None,
            message_delta: None,
            metadata: None,
        }
    }

    /// Create a new content block start event.
    pub fn content_block_start(content: Vec<StreamContent>) -> Self {
        Self {
            event_type: StreamEventType::ContentBlockStart,
            content: Some(content),
            tool_use: None,
            tool_result: None,
            message_delta: None,
            metadata: None,
        }
    }

    /// Create a new content block delta event.
    pub fn content_block_delta(content: Vec<StreamContent>) -> Self {
        Self {
            event_type: StreamEventType::ContentBlockDelta,
            content: Some(content),
            tool_use: None,
            tool_result: None,
            message_delta: None,
            metadata: None,
        }
    }

    /// Create a new content block stop event.
    pub fn content_block_stop() -> Self {
        Self {
            event_type: StreamEventType::ContentBlockStop,
            content: None,
            tool_use: None,
            tool_result: None,
            message_delta: None,
            metadata: None,
        }
    }

    /// Create a new tool use start event.
    pub fn tool_use_start(tool_use: ToolUse) -> Self {
        Self {
            event_type: StreamEventType::ToolUseStart,
            content: None,
            tool_use: Some(tool_use),
            tool_result: None,
            message_delta: None,
            metadata: None,
        }
    }

    /// Create a new tool use delta event.
    pub fn tool_use_delta(tool_use: ToolUse) -> Self {
        Self {
            event_type: StreamEventType::ToolUseDelta,
            content: None,
            tool_use: Some(tool_use),
            tool_result: None,
            message_delta: None,
            metadata: None,
        }
    }

    /// Create a new tool use stop event.
    pub fn tool_use_stop() -> Self {
        Self {
            event_type: StreamEventType::ToolUseStop,
            content: None,
            tool_use: None,
            tool_result: None,
            message_delta: None,
            metadata: None,
        }
    }

    /// Create a new tool result start event.
    pub fn tool_result_start(tool_result: serde_json::Value) -> Self {
        Self {
            event_type: StreamEventType::ToolResultStart,
            content: None,
            tool_use: None,
            tool_result: Some(tool_result),
            message_delta: None,
            metadata: None,
        }
    }

    /// Create a new tool result delta event.
    pub fn tool_result_delta(tool_result: serde_json::Value) -> Self {
        Self {
            event_type: StreamEventType::ToolResultDelta,
            content: None,
            tool_use: None,
            tool_result: Some(tool_result),
            message_delta: None,
            metadata: None,
        }
    }

    /// Create a new tool result stop event.
    pub fn tool_result_stop() -> Self {
        Self {
            event_type: StreamEventType::ToolResultStop,
            content: None,
            tool_use: None,
            tool_result: None,
            message_delta: None,
            metadata: None,
        }
    }

    /// Create a new message delta event.
    pub fn message_delta(message_delta: MessageDelta) -> Self {
        Self {
            event_type: StreamEventType::MessageDelta,
            content: None,
            tool_use: None,
            tool_result: None,
            message_delta: Some(message_delta),
            metadata: None,
        }
    }

    /// Create a new message stop event.
    pub fn message_stop() -> Self {
        Self {
            event_type: StreamEventType::MessageStop,
            content: None,
            tool_use: None,
            tool_result: None,
            message_delta: None,
            metadata: None,
        }
    }

    /// Create a new error event.
    pub fn error(error_message: &str) -> Self {
        Self {
            event_type: StreamEventType::Error,
            content: None,
            tool_use: None,
            tool_result: None,
            message_delta: None,
            metadata: Some({
                let mut map = HashMap::new();
                map.insert("error".to_string(), serde_json::Value::String(error_message.to_string()));
                map
            }),
        }
    }
}

impl StreamContent {
    /// Create a new text content block.
    pub fn text(text: &str) -> Self {
        Self {
            content_type: StreamContentType::Text,
            text: Some(text.to_string()),
            image: None,
            document: None,
        }
    }

    /// Create a new image content block.
    pub fn image(image: serde_json::Value) -> Self {
        Self {
            content_type: StreamContentType::Image,
            text: None,
            image: Some(image),
            document: None,
        }
    }

    /// Create a new document content block.
    pub fn document(document: serde_json::Value) -> Self {
        Self {
            content_type: StreamContentType::Document,
            text: None,
            image: None,
            document: Some(document),
        }
    }
}
