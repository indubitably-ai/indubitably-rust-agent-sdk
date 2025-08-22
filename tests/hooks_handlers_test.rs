//! Unit tests for the hooks and handlers modules.
//! 
//! These tests verify that hooks and handlers functionality works correctly
//! including event handling, callback processing, and hook registration.

use indubitably_rust_agent_sdk::hooks::*;
use indubitably_rust_agent_sdk::handlers::*;
use indubitably_rust_agent_sdk::types::{Message, MessageRole, ContentBlock, EventType, Event};
use std::collections::HashMap;

#[test]
fn test_hook_registry_creation() {
    let registry = HookRegistry::new();
    
    assert_eq!(registry.hook_count(), 0);
    assert!(registry.is_empty());
}

#[test]
fn test_hook_registry_register_hook() {
    let mut registry = HookRegistry::new();
    
    let hook = Hook::new("test_hook", EventType::MessageReceived, |event| {
        Ok(())
    });
    
    registry.register(hook).unwrap();
    
    assert_eq!(registry.hook_count(), 1);
    assert!(!registry.is_empty());
}

#[test]
fn test_hook_registry_register_duplicate_hook() {
    let mut registry = HookRegistry::new();
    
    let hook1 = Hook::new("duplicate_hook", EventType::MessageReceived, |event| {
        Ok(())
    });
    
    let hook2 = Hook::new("duplicate_hook", EventType::MessageReceived, |event| {
        Ok(())
    });
    
    registry.register(hook1).unwrap();
    let result = registry.register(hook2);
    
    // Should fail due to duplicate name
    assert!(result.is_err());
    assert_eq!(registry.hook_count(), 1);
}

#[test]
fn test_hook_registry_get_hook() {
    let mut registry = HookRegistry::new();
    
    let hook = Hook::new("test_hook", EventType::MessageReceived, |event| {
        Ok(())
    });
    
    registry.register(hook).unwrap();
    
    let retrieved_hook = registry.get_hook("test_hook").unwrap();
    assert_eq!(retrieved_hook.name(), "test_hook");
    assert_eq!(retrieved_hook.event_type(), EventType::MessageReceived);
}

#[test]
fn test_hook_registry_get_hook_nonexistent() {
    let registry = HookRegistry::new();
    
    let result = registry.get_hook("nonexistent");
    assert!(result.is_none());
}

#[test]
fn test_hook_registry_list_hooks_by_event_type() {
    let mut registry = HookRegistry::new();
    
    let hook1 = Hook::new("hook1", EventType::MessageReceived, |event| {
        Ok(())
    });
    
    let hook2 = Hook::new("hook2", EventType::MessageReceived, |event| {
        Ok(())
    });
    
    let hook3 = Hook::new("hook3", EventType::ToolExecuted, |event| {
        Ok(())
    });
    
    registry.register(hook1).unwrap();
    registry.register(hook2).unwrap();
    registry.register(hook3).unwrap();
    
    let message_hooks = registry.list_hooks_by_event_type(EventType::MessageReceived);
    assert_eq!(message_hooks.len(), 2);
    
    let tool_hooks = registry.list_hooks_by_event_type(EventType::ToolExecuted);
    assert_eq!(tool_hooks.len(), 1);
}

#[test]
fn test_hook_registry_remove_hook() {
    let mut registry = HookRegistry::new();
    
    let hook = Hook::new("removable_hook", EventType::MessageReceived, |event| {
        Ok(())
    });
    
    registry.register(hook).unwrap();
    assert_eq!(registry.hook_count(), 1);
    
    let removed_hook = registry.remove_hook("removable_hook").unwrap();
    assert_eq!(removed_hook.name(), "removable_hook");
    assert_eq!(registry.hook_count(), 0);
}

#[test]
fn test_hook_registry_remove_hook_nonexistent() {
    let registry = HookRegistry::new();
    
    let result = registry.remove_hook("nonexistent");
    assert!(result.is_none());
}

#[test]
fn test_hook_registry_clear() {
    let mut registry = HookRegistry::new();
    
    let hook1 = Hook::new("hook1", EventType::MessageReceived, |event| {
        Ok(())
    });
    
    let hook2 = Hook::new("hook2", EventType::ToolExecuted, |event| {
        Ok(())
    });
    
    registry.register(hook1).unwrap();
    registry.register(hook2).unwrap();
    
    assert_eq!(registry.hook_count(), 2);
    
    registry.clear();
    assert_eq!(registry.hook_count(), 0);
    assert!(registry.is_empty());
}

#[test]
fn test_hook_creation() {
    let hook = Hook::new("test_hook", EventType::MessageReceived, |event| {
        Ok(())
    });
    
    assert_eq!(hook.name(), "test_hook");
    assert_eq!(hook.event_type(), EventType::MessageReceived);
}

#[test]
fn test_hook_execution() {
    let mut counter = 0;
    let hook = Hook::new("counter_hook", EventType::MessageReceived, move |event| {
        counter += 1;
        Ok(())
    });
    
    let event = Event::new(EventType::MessageReceived, "test_event");
    let result = hook.execute(&event);
    
    assert!(result.is_ok());
    assert_eq!(counter, 1);
}

#[test]
fn test_hook_execution_with_error() {
    let hook = Hook::new("error_hook", EventType::MessageReceived, |event| {
        Err("Hook execution failed".into())
    });
    
    let event = Event::new(EventType::MessageReceived, "test_event");
    let result = hook.execute(&event);
    
    assert!(result.is_err());
    let error = result.unwrap_err();
    assert!(error.to_string().contains("Hook execution failed"));
}

#[test]
fn test_hook_serialization() {
    let hook = Hook::new("serializable_hook", EventType::MessageReceived, |event| {
        Ok(())
    });
    
    let json = serde_json::to_string(&hook).unwrap();
    let deserialized: Hook = serde_json::from_str(&json).unwrap();
    
    assert_eq!(deserialized.name(), hook.name());
    assert_eq!(deserialized.event_type(), hook.event_type());
}

#[test]
fn test_event_creation() {
    let event = Event::new(EventType::MessageReceived, "test_event");
    
    assert_eq!(event.event_type, EventType::MessageReceived);
    assert_eq!(event.data, "test_event");
    assert!(event.timestamp > 0);
}

#[test]
fn test_event_with_metadata() {
    let mut metadata = HashMap::new();
    metadata.insert("user_id".to_string(), "user_123".to_string());
    metadata.insert("session_id".to_string(), "session_456".to_string());
    
    let event = Event::with_metadata(EventType::MessageReceived, "test_event", metadata);
    
    assert_eq!(event.event_type, EventType::MessageReceived);
    assert_eq!(event.data, "test_event");
    assert_eq!(event.metadata.get("user_id").unwrap(), "user_123");
    assert_eq!(event.metadata.get("session_id").unwrap(), "session_456");
}

#[test]
fn test_event_serialization() {
    let event = Event::new(EventType::MessageReceived, "test_event");
    
    let json = serde_json::to_string(&event).unwrap();
    let deserialized: Event = serde_json::from_str(&json).unwrap();
    
    assert_eq!(deserialized.event_type, event.event_type);
    assert_eq!(deserialized.data, event.data);
}

#[test]
fn test_event_type_conversion() {
    let event_type: EventType = "message_received".into();
    assert!(matches!(event_type, EventType::MessageReceived));
    
    let event_type: EventType = "tool_executed".into();
    assert!(matches!(event_type, EventType::ToolExecuted));
    
    let event_type: EventType = "agent_started".into();
    assert!(matches!(event_type, EventType::AgentStarted));
    
    let event_type: EventType = "agent_stopped".into();
    assert!(matches!(event_type, EventType::AgentStopped));
    
    let event_type: EventType = "custom_event".into();
    assert!(matches!(event_type, EventType::Custom(ref s) if s == "custom_event"));
}

#[test]
fn test_callback_handler_creation() {
    let handler = CallbackHandler::new();
    
    assert_eq!(handler.callback_count(), 0);
    assert!(handler.is_empty());
}

#[test]
fn test_callback_handler_register_callback() {
    let mut handler = CallbackHandler::new();
    
    let callback = Callback::new("test_callback", |data| {
        Ok(())
    });
    
    handler.register(callback).unwrap();
    
    assert_eq!(handler.callback_count(), 1);
    assert!(!handler.is_empty());
}

#[test]
fn test_callback_handler_register_duplicate_callback() {
    let mut handler = CallbackHandler::new();
    
    let callback1 = Callback::new("duplicate_callback", |data| {
        Ok(())
    });
    
    let callback2 = Callback::new("duplicate_callback", |data| {
        Ok(())
    });
    
    handler.register(callback1).unwrap();
    let result = handler.register(callback2);
    
    // Should fail due to duplicate name
    assert!(result.is_err());
    assert_eq!(handler.callback_count(), 1);
}

#[test]
fn test_callback_handler_get_callback() {
    let mut handler = CallbackHandler::new();
    
    let callback = Callback::new("test_callback", |data| {
        Ok(())
    });
    
    handler.register(callback).unwrap();
    
    let retrieved_callback = handler.get_callback("test_callback").unwrap();
    assert_eq!(retrieved_callback.name(), "test_callback");
}

#[test]
fn test_callback_handler_get_callback_nonexistent() {
    let handler = CallbackHandler::new();
    
    let result = handler.get_callback("nonexistent");
    assert!(result.is_none());
}

#[test]
fn test_callback_handler_execute_callback() {
    let mut handler = CallbackHandler::new();
    
    let mut counter = 0;
    let callback = Callback::new("counter_callback", move |data| {
        counter += 1;
        Ok(())
    });
    
    handler.register(callback).unwrap();
    
    let result = handler.execute_callback("counter_callback", "test_data");
    assert!(result.is_ok());
    assert_eq!(counter, 1);
}

#[test]
fn test_callback_handler_execute_callback_nonexistent() {
    let handler = CallbackHandler::new();
    
    let result = handler.execute_callback("nonexistent", "test_data");
    assert!(result.is_err());
}

#[test]
fn test_callback_handler_remove_callback() {
    let mut handler = CallbackHandler::new();
    
    let callback = Callback::new("removable_callback", |data| {
        Ok(())
    });
    
    handler.register(callback).unwrap();
    assert_eq!(handler.callback_count(), 1);
    
    let removed_callback = handler.remove_callback("removable_callback").unwrap();
    assert_eq!(removed_callback.name(), "removable_callback");
    assert_eq!(handler.callback_count(), 0);
}

#[test]
fn test_callback_handler_clear() {
    let mut handler = CallbackHandler::new();
    
    let callback1 = Callback::new("callback1", |data| {
        Ok(())
    });
    
    let callback2 = Callback::new("callback2", |data| {
        Ok(())
    });
    
    handler.register(callback1).unwrap();
    handler.register(callback2).unwrap();
    
    assert_eq!(handler.callback_count(), 2);
    
    handler.clear();
    assert_eq!(handler.callback_count(), 0);
    assert!(handler.is_empty());
}

#[test]
fn test_callback_creation() {
    let callback = Callback::new("test_callback", |data| {
        Ok(())
    });
    
    assert_eq!(callback.name(), "test_callback");
}

#[test]
fn test_callback_execution() {
    let mut counter = 0;
    let callback = Callback::new("counter_callback", move |data| {
        counter += 1;
        Ok(())
    });
    
    let result = callback.execute("test_data");
    
    assert!(result.is_ok());
    assert_eq!(counter, 1);
}

#[test]
fn test_callback_execution_with_error() {
    let callback = Callback::new("error_callback", |data| {
        Err("Callback execution failed".into())
    });
    
    let result = callback.execute("test_data");
    
    assert!(result.is_err());
    let error = result.unwrap_err();
    assert!(error.to_string().contains("Callback execution failed"));
}

#[test]
fn test_callback_serialization() {
    let callback = Callback::new("serializable_callback", |data| {
        Ok(())
    });
    
    let json = serde_json::to_string(&callback).unwrap();
    let deserialized: Callback = serde_json::from_str(&json).unwrap();
    
    assert_eq!(deserialized.name(), callback.name());
}

#[test]
fn test_hook_registry_concurrent_access() {
    let registry = std::sync::Arc::new(tokio::sync::RwLock::new(HookRegistry::new()));
    
    // Spawn multiple tasks to test concurrent access
    let mut handles = vec![];
    
    for i in 0..5 {
        let registry_clone = registry.clone();
        let handle = tokio::spawn(async move {
            let mut registry = registry_clone.write().await;
            let hook = Hook::new(format!("hook_{}", i), EventType::MessageReceived, |event| {
                Ok(())
            });
            registry.register(hook)
        });
        handles.push(handle);
    }
    
    // Wait for all tasks to complete
    for handle in handles {
        let result = handle.await.unwrap();
        assert!(result.is_ok());
    }
    
    let registry = registry.read().await;
    assert_eq!(registry.hook_count(), 5);
}

#[test]
fn test_callback_handler_concurrent_access() {
    let handler = std::sync::Arc::new(tokio::sync::RwLock::new(CallbackHandler::new()));
    
    // Spawn multiple tasks to test concurrent access
    let mut handles = vec![];
    
    for i in 0..5 {
        let handler_clone = handler.clone();
        let handle = tokio::spawn(async move {
            let mut handler = handler_clone.write().await;
            let callback = Callback::new(format!("callback_{}", i), |data| {
                Ok(())
            });
            handler.register(callback)
        });
        handles.push(handle);
    }
    
    // Wait for all tasks to complete
    for handle in handles {
        let result = handle.await.unwrap();
        assert!(result.is_ok());
    }
    
    let handler = handler.read().await;
    assert_eq!(handler.callback_count(), 5);
}

#[test]
fn test_hook_registry_event_filtering() {
    let mut registry = HookRegistry::new();
    
    let hook1 = Hook::new("hook1", EventType::MessageReceived, |event| {
        Ok(())
    });
    
    let hook2 = Hook::new("hook2", EventType::ToolExecuted, |event| {
        Ok(())
    });
    
    let hook3 = Hook::new("hook3", EventType::AgentStarted, |event| {
        Ok(())
    });
    
    registry.register(hook1).unwrap();
    registry.register(hook2).unwrap();
    registry.register(hook3).unwrap();
    
    let message_hooks = registry.list_hooks_by_event_type(EventType::MessageReceived);
    assert_eq!(message_hooks.len(), 1);
    assert_eq!(message_hooks[0].name(), "hook1");
    
    let tool_hooks = registry.list_hooks_by_event_type(EventType::ToolExecuted);
    assert_eq!(tool_hooks.len(), 1);
    assert_eq!(tool_hooks[0].name(), "hook2");
    
    let agent_hooks = registry.list_hooks_by_event_type(EventType::AgentStarted);
    assert_eq!(agent_hooks.len(), 1);
    assert_eq!(agent_hooks[0].name(), "hook3");
}

#[test]
fn test_event_metadata_operations() {
    let mut event = Event::new(EventType::MessageReceived, "test_event");
    
    // Add metadata
    event.add_metadata("key1", "value1");
    event.add_metadata("key2", "value2");
    
    assert_eq!(event.metadata.get("key1").unwrap(), "value1");
    assert_eq!(event.metadata.get("key2").unwrap(), "value2");
    
    // Update metadata
    event.add_metadata("key1", "updated_value");
    assert_eq!(event.metadata.get("key1").unwrap(), "updated_value");
    
    // Remove metadata
    event.remove_metadata("key1");
    assert!(event.metadata.get("key1").is_none());
    assert_eq!(event.metadata.get("key2").unwrap(), "value2");
}
