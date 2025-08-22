//! Unit tests for the event loop module.
//! 
//! These tests verify that event loop functionality works correctly
//! including event processing, streaming, and configuration.

use indubitably_rust_agent_sdk::event_loop::*;
use indubitably_rust_agent_sdk::types::{Message, MessageRole, ContentBlock, StreamEvent, StreamEventType, StreamContent, StreamContentType};
use std::collections::HashMap;

#[test]
fn test_event_loop_config_creation() {
    let config = EventLoopConfig::new();
    
    assert_eq!(config.max_iterations, 10);
    assert_eq!(config.tool_timeout, 30);
    assert_eq!(config.enable_streaming, false);
    assert!(config.options.is_empty());
}

#[test]
fn test_event_loop_config_builder() {
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
fn test_event_loop_config_validation() {
    let config = EventLoopConfig::new()
        .with_max_iterations(0)
        .with_tool_timeout(0);
    
    // Should handle edge cases gracefully
    assert_eq!(config.max_iterations, 0);
    assert_eq!(config.tool_timeout, 0);
}

#[test]
fn test_event_loop_config_serialization() {
    let config = EventLoopConfig::new()
        .with_max_iterations(15)
        .with_tool_timeout(45)
        .with_streaming(true);
    
    let json = serde_json::to_string(&config).unwrap();
    let deserialized: EventLoopConfig = serde_json::from_str(&json).unwrap();
    
    assert_eq!(deserialized.max_iterations, config.max_iterations);
    assert_eq!(deserialized.tool_timeout, config.tool_timeout);
    assert_eq!(deserialized.enable_streaming, config.enable_streaming);
}

#[tokio::test]
async fn test_event_loop_creation() {
    let config = EventLoopConfig::new();
    let event_loop = EventLoop::new(config);
    
    assert_eq!(event_loop.iteration_count(), 0);
    assert!(!event_loop.is_running());
}

#[tokio::test]
async fn test_event_loop_basic_execution() {
    let config = EventLoopConfig::new()
        .with_max_iterations(5);
    
    let mut event_loop = EventLoop::new(config);
    
    let result = event_loop.run().await;
    assert!(result.is_ok());
    assert_eq!(event_loop.iteration_count(), 5);
}

#[tokio::test]
async fn test_event_loop_with_custom_processor() {
    let config = EventLoopConfig::new()
        .with_max_iterations(3);
    
    let mut event_loop = EventLoop::new(config);
    
    let mut counter = 0;
    event_loop.set_processor(Box::new(move |_| {
        counter += 1;
        Ok(())
    }));
    
    let result = event_loop.run().await;
    assert!(result.is_ok());
    assert_eq!(counter, 3);
}

#[tokio::test]
async fn test_event_loop_early_termination() {
    let config = EventLoopConfig::new()
        .with_max_iterations(10);
    
    let mut event_loop = EventLoop::new(config);
    
    let mut counter = 0;
    event_loop.set_processor(Box::new(move |_| {
        counter += 1;
        if counter >= 3 {
            Err("Early termination".into())
        } else {
            Ok(())
        }
    }));
    
    let result = event_loop.run().await;
    assert!(result.is_err());
    assert_eq!(counter, 3);
}

#[tokio::test]
async fn test_event_loop_timeout_handling() {
    let config = EventLoopConfig::new()
        .with_max_iterations(10)
        .with_tool_timeout(1); // 1 second timeout
    
    let mut event_loop = EventLoop::new(config);
    
    event_loop.set_processor(Box::new(|_| {
        // Simulate long-running operation
        std::thread::sleep(std::time::Duration::from_millis(100));
        Ok(())
    }));
    
    let result = event_loop.run().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_event_loop_state_management() {
    let config = EventLoopConfig::new()
        .with_max_iterations(3);
    
    let mut event_loop = EventLoop::new(config);
    
    assert!(!event_loop.is_running());
    
    let handle = tokio::spawn(async move {
        event_loop.run().await
    });
    
    // Give it a moment to start
    tokio::time::sleep(std::time::Duration::from_millis(10)).await;
    
    let result = handle.await.unwrap();
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_event_loop_error_recovery() {
    let config = EventLoopConfig::new()
        .with_max_iterations(5);
    
    let mut event_loop = EventLoop::new(config);
    
    let mut error_count = 0;
    event_loop.set_processor(Box::new(move |_| {
        if error_count < 2 {
            error_count += 1;
            Err("Temporary error".into())
        } else {
            Ok(())
        }
    }));
    
    let result = event_loop.run().await;
    assert!(result.is_ok());
    assert_eq!(error_count, 2);
}

#[tokio::test]
async fn test_event_loop_concurrent_access() {
    let config = EventLoopConfig::new()
        .with_max_iterations(10);
    
    let event_loop = std::sync::Arc::new(tokio::sync::RwLock::new(EventLoop::new(config)));
    
    // Spawn multiple tasks to test concurrent access
    let mut handles = vec![];
    
    for i in 0..3 {
        let event_loop_clone = event_loop.clone();
        let handle = tokio::spawn(async move {
            let mut event_loop = event_loop_clone.write().await;
            let mut counter = 0;
            event_loop.set_processor(Box::new(move |_| {
                counter += 1;
                Ok(())
            }));
            event_loop.run().await
        });
        handles.push(handle);
    }
    
    // Wait for all tasks to complete
    for handle in handles {
        let result = handle.await.unwrap();
        assert!(result.is_ok());
    }
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
fn test_stream_content_creation() {
    let text_content = StreamContent::text("Hello, world!");
    assert_eq!(text_content.content_type, StreamContentType::Text);
    assert_eq!(text_content.text.as_ref().unwrap(), "Hello, world!");
    
    let image_content = StreamContent::image(serde_json::json!({"url": "test.jpg"}));
    assert_eq!(image_content.content_type, StreamContentType::Image);
    assert!(image_content.image.is_some());
    assert_eq!(image_content.image.unwrap().get("url").unwrap(), "test.jpg");
}

#[test]
fn test_stream_event_serialization() {
    let event = StreamEvent::message_start();
    
    let json = serde_json::to_string(&event).unwrap();
    let deserialized: StreamEvent = serde_json::from_str(&json).unwrap();
    
    assert_eq!(deserialized.event_type, event.event_type);
}

#[test]
fn test_stream_content_serialization() {
    let content = StreamContent::text("Test content");
    
    let json = serde_json::to_string(&content).unwrap();
    let deserialized: StreamContent = serde_json::from_str(&json).unwrap();
    
    assert_eq!(deserialized.content_type, content.content_type);
    assert_eq!(deserialized.text, content.text);
}

#[tokio::test]
async fn test_streaming_event_loop() {
    let config = EventLoopConfig::new()
        .with_max_iterations(3)
        .with_streaming(true);
    
    let mut event_loop = EventLoop::new(config);
    
    let mut events = Vec::new();
    event_loop.set_stream_processor(Box::new(move |event| {
        events.push(event.clone());
        Ok(())
    }));
    
    let result = event_loop.run().await;
    assert!(result.is_ok());
    assert!(!events.is_empty());
}

#[tokio::test]
async fn test_event_loop_with_metrics() {
    let config = EventLoopConfig::new()
        .with_max_iterations(5);
    
    let mut event_loop = EventLoop::new(config);
    
    let mut metrics = HashMap::new();
    event_loop.set_metrics_collector(Box::new(move |metric_name, value| {
        metrics.insert(metric_name.to_string(), value);
    }));
    
    let result = event_loop.run().await;
    assert!(result.is_ok());
    assert!(!metrics.is_empty());
}

#[test]
fn test_event_loop_config_clone() {
    let config = EventLoopConfig::new()
        .with_max_iterations(15)
        .with_tool_timeout(45);
    
    let cloned_config = config.clone();
    
    assert_eq!(cloned_config.max_iterations, config.max_iterations);
    assert_eq!(cloned_config.tool_timeout, config.tool_timeout);
    assert_eq!(cloned_config.enable_streaming, config.enable_streaming);
}

#[test]
fn test_event_loop_config_debug() {
    let config = EventLoopConfig::new()
        .with_max_iterations(10)
        .with_tool_timeout(30);
    
    let debug_str = format!("{:?}", config);
    assert!(debug_str.contains("max_iterations"));
    assert!(debug_str.contains("tool_timeout"));
}

#[tokio::test]
async fn test_event_loop_processor_error_handling() {
    let config = EventLoopConfig::new()
        .with_max_iterations(3);
    
    let mut event_loop = EventLoop::new(config);
    
    event_loop.set_processor(Box::new(|_| {
        Err("Processor error".into())
    }));
    
    let result = event_loop.run().await;
    assert!(result.is_err());
    let error = result.unwrap_err();
    assert!(error.to_string().contains("Processor error"));
}

#[tokio::test]
async fn test_event_loop_stream_processor_error_handling() {
    let config = EventLoopConfig::new()
        .with_max_iterations(3)
        .with_streaming(true);
    
    let mut event_loop = EventLoop::new(config);
    
    event_loop.set_stream_processor(Box::new(|_| {
        Err("Stream processor error".into())
    }));
    
    let result = event_loop.run().await;
    assert!(result.is_err());
    let error = result.unwrap_err();
    assert!(error.to_string().contains("Stream processor error"));
}

#[tokio::test]
async fn test_event_loop_metrics_collector_error_handling() {
    let config = EventLoopConfig::new()
        .with_max_iterations(3);
    
    let mut event_loop = EventLoop::new(config);
    
    event_loop.set_metrics_collector(Box::new(|_, _| {
        // Metrics collector errors should not stop the event loop
        panic!("Metrics collector panic");
    }));
    
    // Should handle metrics collector errors gracefully
    let result = event_loop.run().await;
    assert!(result.is_ok());
}

#[test]
fn test_stream_event_type_conversion() {
    let event_type: StreamEventType = "message_start".into();
    assert!(matches!(event_type, StreamEventType::MessageStart));
    
    let event_type: StreamEventType = "content_block_start".into();
    assert!(matches!(event_type, StreamEventType::ContentBlockStart));
    
    let event_type: StreamEventType = "content_block_delta".into();
    assert!(matches!(event_type, StreamEventType::ContentBlockDelta));
    
    let event_type: StreamEventType = "content_block_stop".into();
    assert!(matches!(event_type, StreamEventType::ContentBlockStop));
    
    let event_type: StreamEventType = "message_delta".into();
    assert!(matches!(event_type, StreamEventType::MessageDelta));
    
    let event_type: StreamEventType = "message_stop".into();
    assert!(matches!(event_type, StreamEventType::MessageStop));
    
    let event_type: StreamEventType = "error".into();
    assert!(matches!(event_type, StreamEventType::Error));
    
    let event_type: StreamEventType = "custom_event".into();
    assert!(matches!(event_type, StreamEventType::Custom(ref s) if s == "custom_event"));
}

#[test]
fn test_stream_content_type_conversion() {
    let content_type: StreamContentType = "text".into();
    assert!(matches!(content_type, StreamContentType::Text));
    
    let content_type: StreamContentType = "image".into();
    assert!(matches!(content_type, StreamContentType::Image));
    
    let content_type: StreamContentType = "custom_content".into();
    assert!(matches!(content_type, StreamContentType::Custom(ref s) if s == "custom_content"));
}

#[test]
fn test_event_loop_config_with_custom_options() {
    let mut config = EventLoopConfig::new();
    config.add_option("retry_count", serde_json::json!(3));
    config.add_option("backoff_delay", serde_json::json!(1000));
    
    assert_eq!(config.options.len(), 2);
    assert_eq!(config.options.get("retry_count").unwrap(), 3);
    assert_eq!(config.options.get("backoff_delay").unwrap(), 1000);
}

#[test]
fn test_event_loop_config_option_overwrite() {
    let mut config = EventLoopConfig::new();
    config.add_option("key", serde_json::json!("value1"));
    config.add_option("key", serde_json::json!("value2"));
    
    assert_eq!(config.options.len(), 1);
    assert_eq!(config.options.get("key").unwrap(), "value2");
}
