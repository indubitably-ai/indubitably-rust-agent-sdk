//! Session manager trait for the SDK.
//! 
//! This module defines the SessionManager trait that all
//! session management implementations must implement.

use async_trait::async_trait;

use crate::types::{Session, IndubitablyResult};

/// A trait for managing sessions.
#[async_trait]
pub trait SessionManager: Send + Sync {
    /// Create a new session.
    async fn create_session(&mut self, session: Session) -> IndubitablyResult<()>;
    
    /// Get a session by ID.
    async fn get_session(&self, session_id: &str) -> IndubitablyResult<Option<Session>>;
    
    /// Update an existing session.
    async fn update_session(&mut self, session: Session) -> IndubitablyResult<()>;
    
    /// Delete a session by ID.
    async fn delete_session(&mut self, session_id: &str) -> IndubitablyResult<()>;
    
    /// List all sessions.
    async fn list_sessions(&self) -> IndubitablyResult<Vec<Session>>;
    
    /// Check if a session exists.
    async fn session_exists(&self, session_id: &str) -> IndubitablyResult<bool>;
}
