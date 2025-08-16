#![cfg(feature = "legacy-types")]
//! Tests for the types module.
//! 
//! These tests verify that all type definitions work correctly
//! and provide the expected functionality.

use indubitably_rust_agent_sdk::types::*;

#[test]
fn test_message_creation() {
    let message = Message::user("Hello, world!");
    assert_eq!(message.role, MessageRole::User);
    assert_eq!(message.text().unwrap(), "Hello, world!");
    
    let message = Message::assistant("Hi there!");
    assert_eq!(message.role, MessageRole::Assistant);
    assert_eq!(message.text().unwrap(), "Hi there!");
    
    let message = Message::system("You are a helpful assistant.");
    assert_eq!(message.role, MessageRole::System);
    assert_eq!(message.text().unwrap(), "You are a helpful assistant.");
}

#[test]
fn test_message_from_string() {
    let message: Message = "Hello".into();
    assert_eq!(message.role, MessageRole::User);
    assert_eq!(message.text().unwrap(), "Hello");
    
    let message: Message = "World".to_string().into();
    assert_eq!(message.role, MessageRole::User);
    assert_eq!(message.text().unwrap(), "World");
}

#[test]
fn test_message_all_text() {
    let message = Message::user("Hello");
    assert_eq!(message.all_text(), "Hello");
    
    // Test with multiple content blocks
    let mut message = Message::new(MessageRole::User, vec![]);
    message.content.push(ContentBlock {
        text: Some("Hello".to_string()),
        ..Default::default()
    });
    message.content.push(ContentBlock {
        text: Some("World".to_string()),
        ..Default::default()
    });
    
    assert_eq!(message.all_text(), "Hello World");
}

#[test]
fn test_content_block_default() {
    let block = ContentBlock::default();
    assert!(block.text.is_none());
    assert!(block.image.is_none());
    assert!(block.document.is_none());
    assert!(block.tool_use.is_none());
    assert!(block.tool_result.is_none());
}

#[test]
fn test_tool_spec_creation() {
    let spec = ToolSpec::new("test_tool", "A test tool");
    assert_eq!(spec.name, "test_tool");
    assert_eq!(spec.description, "A test tool");
    assert!(spec.input_schema.is_none());
    assert!(spec.output_schema.is_none());
}

#[test]
fn test_tool_spec_builder() {
    let spec = ToolSpec::new("test_tool", "A test tool")
        .with_input_schema(serde_json::json!({"type": "string"}))
        .with_output_schema(serde_json::json!({"type": "string"}))
        .with_metadata("version", serde_json::json!("1.0.0"));
    
    assert_eq!(spec.name, "test_tool");
    assert!(spec.input_schema.is_some());
    assert!(spec.output_schema.is_some());
    assert!(spec.metadata.is_some());
    
    let metadata = spec.metadata.unwrap();
    assert_eq!(metadata.get("version").unwrap(), "1.0.0");
}

#[test]
fn test_tool_use_creation() {
    let tool_use = ToolUse::new("test_tool", "tool_123");
    assert_eq!(tool_use.name, "test_tool");
    assert_eq!(tool_use.tool_use_id, "tool_123");
    assert!(tool_use.input.is_none());
}

#[test]
fn test_tool_use_with_input() {
    let tool_use = ToolUse::new("test_tool", "tool_123")
        .with_input(serde_json::json!({"text": "hello"}));
    
    assert_eq!(tool_use.name, "test_tool");
    assert_eq!(tool_use.tool_use_id, "tool_123");
    assert!(tool_use.input.is_some());
    
    let input = tool_use.input.unwrap();
    assert_eq!(input.get("text").unwrap(), "hello");
}

#[test]
fn test_tool_result_creation() {
    let content = vec![ToolResultContent::text("Success!")];
    let result = ToolResult::new("tool_123", content);
    
    assert_eq!(result.tool_use_id, "tool_123");
    assert_eq!(result.content.len(), 1);
    assert!(result.is_error.is_none());
}

#[test]
fn test_tool_result_error() {
    let result = ToolResult::error("tool_123", "Something went wrong");
    
    assert_eq!(result.tool_use_id, "tool_123");
    assert_eq!(result.content.len(), 1);
    assert_eq!(result.is_error, Some(true));
    
    let content = &result.content[0];
    assert_eq!(content.text.as_ref().unwrap(), "Something went wrong");
}

#[test]
fn test_tool_result_content() {
    let text_content = ToolResultContent::text("Hello, world!");
    assert_eq!(text_content.content_type, ToolResultContentType::Text);
    assert_eq!(text_content.text.as_ref().unwrap(), "Hello, world!");
    assert!(text_content.image.is_none());
    
    let image_content = ToolResultContent::image(serde_json::json!({"url": "test.jpg"}));
    assert_eq!(image_content.content_type, ToolResultContentType::Image);
    assert!(image_content.text.is_none());
    assert!(image_content.image.is_some());
}

#[test]
fn test_stream_event_creation() {
    let event = StreamEvent::message_start();
    assert_eq!(event.event_type, StreamEventType::MessageStart);
    
    let event = StreamEvent::content_block_start(vec![StreamContent::text("Hello")]);
    assert_eq!(event.event_type, StreamEventType::ContentBlockStart);
    assert!(event.content.is_some());
    
    let event = StreamEvent::error("Something went wrong");
    assert_eq!(event.event_type, StreamEventType::Error);
    assert!(event.metadata.is_some());
    
    let metadata = event.metadata.unwrap();
    assert_eq!(metadata.get("error").unwrap(), "Something went wrong");
}

#[test]
fn test_stream_content() {
    let text_content = StreamContent::text("Hello, world!");
    assert_eq!(text_content.content_type, StreamContentType::Text);
    assert_eq!(text_content.text.as_ref().unwrap(), "Hello, world!");
    
    let image_content = StreamContent::image(serde_json::json!({"url": "test.jpg"}));
    assert_eq!(image_content.content_type, StreamContentType::Image);
    assert!(image_content.image.as_ref().unwrap().get("url").unwrap(), "test.jpg");
}

#[test]
fn test_session_creation() {
    let agent = SessionAgent::new("agent_123", "Test Agent");
    let session = Session::new("session_123", SessionType::Conversation, agent);
    
    assert_eq!(session.id, "session_123");
    assert_eq!(session.session_type, SessionType::Conversation);
    assert_eq!(session.agent.id, "agent_123");
    assert_eq!(session.agent.name, "Test Agent");
    assert_eq!(session.messages.len(), 0);
}

#[test]
fn test_session_agent_builder() {
    let agent = SessionAgent::new("agent_123", "Test Agent")
        .with_model("gpt-4")
        .with_system_prompt("You are a helpful assistant.")
        .with_config("temperature", serde_json::json!(0.7));
    
    assert_eq!(agent.id, "agent_123");
    assert_eq!(agent.name, "Test Agent");
    assert_eq!(agent.model.as_ref().unwrap(), "gpt-4");
    assert_eq!(agent.system_prompt.as_ref().unwrap(), "You are a helpful assistant.");
    
    let config = agent.config.unwrap();
    assert_eq!(config.get("temperature").unwrap(), 0.7);
}

#[test]
fn test_session_message_creation() {
    let message = SessionMessage::new("msg_123", "user", "Hello");
    assert_eq!(message.id, "msg_123");
    assert_eq!(message.role, "user");
    assert_eq!(message.content, "Hello");
}

#[test]
fn test_session_message_from_message() {
    let content_block = ContentBlock {
        text: Some("Hello, world!".to_string()),
        ..Default::default()
    };
    let message = Message::new(MessageRole::User, vec![content_block]);
    
    let session_message = SessionMessage::from_message("msg_123", &message);
    assert_eq!(session_message.id, "msg_123");
    assert_eq!(session_message.role, "user");
    assert_eq!(session_message.content, "Hello, world!");
}

#[test]
fn test_session_type_conversion() {
    let session_type: SessionType = "conversation".into();
    assert!(matches!(session_type, SessionType::Conversation));
    
    let session_type: SessionType = "task".into();
    assert!(matches!(session_type, SessionType::Task));
    
    let session_type: SessionType = "workflow".into();
    assert!(matches!(session_type, SessionType::Workflow));
    
    let session_type: SessionType = "custom_type".into();
    assert!(matches!(session_type, SessionType::Custom(ref s) if s == "custom_type"));
}

#[test]
fn test_guard_content() {
    let guard_text = GuardContentText {
        qualifiers: vec![GuardQualifier::Query, GuardQualifier::GroundingSource],
        text: "Test text".to_string(),
    };
    
    let guard_content = GuardContent { text: guard_text };
    assert_eq!(guard_content.text.qualifiers.len(), 2);
    assert_eq!(guard_content.text.text, "Test text");
}

#[test]
fn test_reasoning_content() {
    let reasoning_text = ReasoningTextBlock {
        signature: Some("sig_123".to_string()),
        text: "This is the reasoning".to_string(),
    };
    
    let reasoning_content = ReasoningContentBlock {
        reasoning_text,
        redacted_content: vec![1, 2, 3],
    };
    
    assert_eq!(reasoning_content.reasoning_text.signature.as_ref().unwrap(), "sig_123");
    assert_eq!(reasoning_content.reasoning_text.text, "This is the reasoning");
    assert_eq!(reasoning_content.redacted_content, vec![1, 2, 3]);
}

#[test]
fn test_cache_point() {
    let cache_point = CachePoint {
        r#type: "default".to_string(),
    };
    
    assert_eq!(cache_point.r#type, "default");
}

#[test]
fn test_document_content() {
    let doc_content = DocumentContent::text("This is a text document");
    assert_eq!(doc_content.content_type, DocumentType::Text);
    assert_eq!(doc_content.source.media_type, "text/plain");
    assert_eq!(doc_content.source.data.text.as_ref().unwrap(), "This is a text document");
    
    let doc_content = DocumentContent::pdf_base64("base64_encoded_pdf");
    assert_eq!(doc_content.content_type, DocumentType::Pdf);
    assert_eq!(doc_content.source.media_type, "application/pdf");
    assert_eq!(doc_content.source.data.base64.as_ref().unwrap(), "base64_encoded_pdf");
}

#[test]
fn test_image_content() {
    let image_content = ImageContent::base64("base64_encoded_image", "image/jpeg");
    assert_eq!(image_content.content_type, ImageType::Image);
    assert_eq!(image_content.source.media_type, "image/jpeg");
    assert_eq!(image_content.source.data.base64.as_ref().unwrap(), "base64_encoded_image");
    
    let image_content = ImageContent::url("https://example.com/image.jpg", "image/jpeg");
    assert_eq!(image_content.content_type, ImageType::Image);
    assert_eq!(image_content.source.media_type, "image/jpeg");
    assert_eq!(image_content.source.data.url.as_ref().unwrap(), "https://example.com/image.jpg");
}

#[test]
fn test_video_content() {
    let video_content = VideoContent::base64("base64_encoded_video", "video/mp4");
    assert_eq!(video_content.content_type, VideoType::Video);
    assert_eq!(video_content.source.media_type, "video/mp4");
    assert_eq!(video_content.source.data.base64.as_ref().unwrap(), "base64_encoded_video");
    
    let video_content = VideoContent::url("https://example.com/video.mp4", "video/mp4");
    assert_eq!(video_content.content_type, VideoType::Video);
    assert_eq!(video_content.source.media_type, "video/mp4");
    assert_eq!(video_content.source.data.url.as_ref().unwrap(), "https://example.com/video.mp4");
}

#[test]
fn test_collection() {
    let items = vec!["item1", "item2", "item3"];
    let collection = Collection::new(items, 10, 0, 3);
    
    assert_eq!(collection.len(), 3);
    assert_eq!(collection.total_count, 10);
    assert_eq!(collection.page, 0);
    assert_eq!(collection.page_size, 3);
    assert!(collection.has_more);
    
    assert_eq!(collection.get(0).unwrap(), &"item1");
    assert_eq!(collection.get(1).unwrap(), &"item2");
    assert_eq!(collection.get(2).unwrap(), &"item3");
}

#[test]
fn test_collection_iteration() {
    let items = vec!["item1", "item2", "item3"];
    let collection = Collection::new(items, 3, 0, 3);
    
    let collected: Vec<&str> = collection.iter().collect();
    assert_eq!(collected, vec!["item1", "item2", "item3"]);
    
    let collected: Vec<&str> = collection.into_iter().collect();
    assert_eq!(collected, vec!["item1", "item2", "item3"]);
}

#[test]
fn test_event_loop_config() {
    let config = EventLoopConfig::new()
        .with_max_iterations(20)
        .with_tool_timeout(60)
        .with_streaming(true)
        .with_option("custom_key", serde_json::json!("custom_value"));
    
    assert_eq!(config.max_iterations, 20);
    assert_eq!(config.tool_timeout, 60);
    assert_eq!(config.enable_streaming, true);
    assert_eq!(config.options.get("custom_key").unwrap(), "custom_value");
}

#[test]
fn test_trace_span() {
    let mut span = TraceSpan::new("test_span", 1234567890);
    assert_eq!(span.id, "test_span");
    assert_eq!(span.name, "test_span");
    assert_eq!(span.start_time, 1234567890);
    assert!(span.end_time.is_none());
    
    span.add_attribute("key1", AttributeValue::String("value1".to_string()));
    span.add_attribute("key2", AttributeValue::Number(42.0));
    
    assert_eq!(span.attributes.len(), 2);
    assert_eq!(span.attributes.get("key1").unwrap(), &AttributeValue::String("value1".to_string()));
    assert_eq!(span.attributes.get("key2").unwrap(), &AttributeValue::Number(42.0));
    
    span.end(1234567990);
    assert_eq!(span.end_time, Some(1234567990));
}

#[test]
fn test_attribute_value() {
    let string_val = AttributeValue::String("hello".to_string());
    let number_val = AttributeValue::Number(42.0);
    let bool_val = AttributeValue::Boolean(true);
    
    let array_val = AttributeValue::Array(vec![string_val.clone(), number_val.clone()]);
    let mut object_map = std::collections::HashMap::new();
    object_map.insert("key".to_string(), bool_val.clone());
    let object_val = AttributeValue::Object(object_map);
    
    assert!(matches!(string_val, AttributeValue::String(_)));
    assert!(matches!(number_val, AttributeValue::Number(_)));
    assert!(matches!(bool_val, AttributeValue::Boolean(_)));
    assert!(matches!(array_val, AttributeValue::Array(_)));
    assert!(matches!(object_val, AttributeValue::Object(_)));
}
