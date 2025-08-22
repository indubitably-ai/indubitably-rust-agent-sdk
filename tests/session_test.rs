//! Unit tests for the session management module.
//! 
//! These tests verify that session management functionality works correctly
//! including creation, persistence, and retrieval.

use indubitably_rust_agent_sdk::session::*;
use indubitably_rust_agent_sdk::types::{Message, MessageRole, ContentBlock, SessionType, SessionAgent};
use std::collections::HashMap;

#[tokio::test]
async fn test_session_manager_creation() {
    let manager = SessionManager::new();
    
    assert_eq!(manager.session_count().await, 0);
    assert!(manager.is_empty().await);
}

#[tokio::test]
async fn test_session_creation() {
    let mut manager = SessionManager::new();
    
    let agent = SessionAgent::new("agent_123", "Test Agent");
    let session = Session::new("session_123", SessionType::Conversation, agent);
    
    let session_id = manager.create_session(session).await.unwrap();
    assert_eq!(session_id, "session_123");
    assert_eq!(manager.session_count().await, 1);
}

#[tokio::test]
async fn test_session_retrieval() {
    let mut manager = SessionManager::new();
    
    let agent = SessionAgent::new("agent_123", "Test Agent");
    let session = Session::new("session_123", SessionType::Conversation, agent);
    
    manager.create_session(session).await.unwrap();
    
    let retrieved_session = manager.get_session("session_123").await.unwrap();
    assert_eq!(retrieved_session.id, "session_123");
    assert_eq!(retrieved_session.agent.name, "Test Agent");
}

#[tokio::test]
async fn test_session_retrieval_nonexistent() {
    let manager = SessionManager::new();
    
    let result = manager.get_session("nonexistent").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_session_update() {
    let mut manager = SessionManager::new();
    
    let agent = SessionAgent::new("agent_123", "Test Agent");
    let session = Session::new("session_123", SessionType::Conversation, agent);
    
    manager.create_session(session).await.unwrap();
    
    // Update session
    let mut updated_session = manager.get_session("session_123").await.unwrap();
    updated_session.metadata.insert("updated".to_string(), "true".to_string());
    
    manager.update_session(updated_session).await.unwrap();
    
    let retrieved = manager.get_session("session_123").await.unwrap();
    assert_eq!(retrieved.metadata.get("updated").unwrap(), "true");
}

#[tokio::test]
async fn test_session_deletion() {
    let mut manager = SessionManager::new();
    
    let agent = SessionAgent::new("agent_123", "Test Agent");
    let session = Session::new("session_123", SessionType::Conversation, agent);
    
    manager.create_session(session).await.unwrap();
    assert_eq!(manager.session_count().await, 1);
    
    manager.delete_session("session_123").await.unwrap();
    assert_eq!(manager.session_count().await, 0);
    
    let result = manager.get_session("session_123").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_session_listing() {
    let mut manager = SessionManager::new();
    
    let agent1 = SessionAgent::new("agent_1", "Agent 1");
    let agent2 = SessionAgent::new("agent_2", "Agent 2");
    
    let session1 = Session::new("session_1", SessionType::Conversation, agent1);
    let session2 = Session::new("session_2", SessionType::Task, agent2);
    
    manager.create_session(session1).await.unwrap();
    manager.create_session(session2).await.unwrap();
    
    let sessions = manager.list_sessions().await;
    assert_eq!(sessions.len(), 2);
    
    let session_ids: Vec<&str> = sessions.iter().map(|s| s.id.as_str()).collect();
    assert!(session_ids.contains(&"session_1"));
    assert!(session_ids.contains(&"session_2"));
}

#[tokio::test]
async fn test_session_filtering() {
    let mut manager = SessionManager::new();
    
    let agent1 = SessionAgent::new("agent_1", "Agent 1");
    let agent2 = SessionAgent::new("agent_2", "Agent 2");
    
    let session1 = Session::new("session_1", SessionType::Conversation, agent1);
    let session2 = Session::new("session_2", SessionType::Task, agent2);
    
    manager.create_session(session1).await.unwrap();
    manager.create_session(session2).await.unwrap();
    
    let conversation_sessions = manager.list_sessions_by_type(SessionType::Conversation).await;
    assert_eq!(conversation_sessions.len(), 1);
    assert_eq!(conversation_sessions[0].id, "session_1");
    
    let task_sessions = manager.list_sessions_by_type(SessionType::Task).await;
    assert_eq!(task_sessions.len(), 1);
    assert_eq!(task_sessions[0].id, "session_2");
}

#[tokio::test]
async fn test_session_message_management() {
    let mut manager = SessionManager::new();
    
    let agent = SessionAgent::new("agent_123", "Test Agent");
    let session = Session::new("session_123", SessionType::Conversation, agent);
    
    manager.create_session(session).await.unwrap();
    
    // Add message to session
    let message = SessionMessage::new("msg_1", "user", "Hello");
    manager.add_message("session_123", message).await.unwrap();
    
    let retrieved_session = manager.get_session("session_123").await.unwrap();
    assert_eq!(retrieved_session.messages.len(), 1);
    assert_eq!(retrieved_session.messages[0].content, "Hello");
}

#[tokio::test]
async fn test_session_agent_management() {
    let mut manager = SessionManager::new();
    
    let agent = SessionAgent::new("agent_123", "Test Agent");
    let session = Session::new("session_123", SessionType::Conversation, agent);
    
    manager.create_session(session).await.unwrap();
    
    // Update agent
    let updated_agent = SessionAgent::new("agent_123", "Updated Agent")
        .with_model("gpt-4")
        .with_system_prompt("You are an updated assistant.");
    
    manager.update_session_agent("session_123", updated_agent).await.unwrap();
    
    let retrieved_session = manager.get_session("session_123").await.unwrap();
    assert_eq!(retrieved_session.agent.name, "Updated Agent");
    assert_eq!(retrieved_session.agent.model.as_ref().unwrap(), "gpt-4");
}

#[tokio::test]
async fn test_session_metadata_management() {
    let mut manager = SessionManager::new();
    
    let agent = SessionAgent::new("agent_123", "Test Agent");
    let session = Session::new("session_123", SessionType::Conversation, agent);
    
    manager.create_session(session).await.unwrap();
    
    // Add metadata
    let mut metadata = HashMap::new();
    metadata.insert("user_id".to_string(), "user_123".to_string());
    metadata.insert("created_at".to_string(), "2024-01-01".to_string());
    
    manager.set_session_metadata("session_123", metadata).await.unwrap();
    
    let retrieved_session = manager.get_session("session_123").await.unwrap();
    assert_eq!(retrieved_session.metadata.get("user_id").unwrap(), "user_123");
    assert_eq!(retrieved_session.metadata.get("created_at").unwrap(), "2024-01-01");
}

#[tokio::test]
async fn test_session_cleanup() {
    let mut manager = SessionManager::new();
    
    let agent1 = SessionAgent::new("agent_1", "Agent 1");
    let agent2 = SessionAgent::new("agent_2", "Agent 2");
    
    let session1 = Session::new("session_1", SessionType::Conversation, agent1);
    let session2 = Session::new("session_2", SessionType::Task, agent2);
    
    manager.create_session(session1).await.unwrap();
    manager.create_session(session2).await.unwrap();
    
    assert_eq!(manager.session_count().await, 2);
    
    manager.clear_all_sessions().await;
    assert_eq!(manager.session_count().await, 0);
    assert!(manager.is_empty().await);
}

#[tokio::test]
async fn test_session_persistence() {
    let mut manager = SessionManager::new();
    
    let agent = SessionAgent::new("agent_123", "Test Agent");
    let session = Session::new("session_123", SessionType::Conversation, agent);
    
    manager.create_session(session).await.unwrap();
    
    // Simulate persistence
    let sessions = manager.list_sessions().await;
    let session_data = serde_json::to_string(&sessions).unwrap();
    
    // Simulate retrieval from persistence
    let deserialized_sessions: Vec<Session> = serde_json::from_str(&session_data).unwrap();
    assert_eq!(deserialized_sessions.len(), 1);
    assert_eq!(deserialized_sessions[0].id, "session_123");
}

#[tokio::test]
async fn test_session_conversation_flow() {
    let mut manager = SessionManager::new();
    
    let agent = SessionAgent::new("agent_123", "Test Agent");
    let session = Session::new("session_123", SessionType::Conversation, agent);
    
    manager.create_session(session).await.unwrap();
    
    // Simulate a conversation
    let user_message = SessionMessage::new("msg_1", "user", "Hello, how are you?");
    manager.add_message("session_123", user_message).await.unwrap();
    
    let assistant_message = SessionMessage::new("msg_2", "assistant", "I'm doing well, thank you!");
    manager.add_message("session_123", assistant_message).await.unwrap();
    
    let retrieved_session = manager.get_session("session_123").await.unwrap();
    assert_eq!(retrieved_session.messages.len(), 2);
    assert_eq!(retrieved_session.messages[0].content, "Hello, how are you?");
    assert_eq!(retrieved_session.messages[1].content, "I'm doing well, thank you!");
}

#[tokio::test]
async fn test_session_error_handling() {
    let mut manager = SessionManager::new();
    
    // Try to get non-existent session
    let result = manager.get_session("nonexistent").await;
    assert!(result.is_err());
    
    // Try to update non-existent session
    let agent = SessionAgent::new("agent_123", "Test Agent");
    let session = Session::new("session_123", SessionType::Conversation, agent);
    let result = manager.update_session(session).await;
    assert!(result.is_err());
    
    // Try to delete non-existent session
    let result = manager.delete_session("nonexistent").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_session_concurrent_access() {
    let manager = std::sync::Arc::new(tokio::sync::RwLock::new(SessionManager::new()));
    
    // Spawn multiple tasks to test concurrent access
    let mut handles = vec![];
    
    for i in 0..5 {
        let manager_clone = manager.clone();
        let handle = tokio::spawn(async move {
            let mut manager = manager_clone.write().await;
            let agent = SessionAgent::new(format!("agent_{}", i), format!("Agent {}", i));
            let session = Session::new(format!("session_{}", i), SessionType::Conversation, agent);
            manager.create_session(session).await
        });
        handles.push(handle);
    }
    
    // Wait for all tasks to complete
    for handle in handles {
        handle.await.unwrap().unwrap();
    }
    
    let manager = manager.read().await;
    assert_eq!(manager.session_count().await, 5);
}

#[test]
fn test_session_agent_builder() {
    let agent = SessionAgent::new("agent_123", "Test Agent")
        .with_model("gpt-4")
        .with_system_prompt("You are a helpful assistant.")
        .with_config("temperature", serde_json::json!(0.7))
        .with_config("max_tokens", serde_json::json!(1000));
    
    assert_eq!(agent.id, "agent_123");
    assert_eq!(agent.name, "Test Agent");
    assert_eq!(agent.model.as_ref().unwrap(), "gpt-4");
    assert_eq!(agent.system_prompt.as_ref().unwrap(), "You are a helpful assistant.");
    
    let config = agent.config.as_ref().unwrap();
    assert_eq!(config.get("temperature").unwrap(), 0.7);
    assert_eq!(config.get("max_tokens").unwrap(), 1000);
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
fn test_session_serialization() {
    let agent = SessionAgent::new("agent_123", "Test Agent");
    let session = Session::new("session_123", SessionType::Conversation, agent);
    
    let json = serde_json::to_string(&session).unwrap();
    let deserialized: Session = serde_json::from_str(&json).unwrap();
    
    assert_eq!(deserialized.id, session.id);
    assert_eq!(deserialized.session_type, session.session_type);
    assert_eq!(deserialized.agent.id, session.agent.id);
    assert_eq!(deserialized.agent.name, session.agent.name);
}

#[test]
fn test_session_message_creation() {
    let message = SessionMessage::new("msg_123", "user", "Hello, world!");
    assert_eq!(message.id, "msg_123");
    assert_eq!(message.role, "user");
    assert_eq!(message.content, "Hello, world!");
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
