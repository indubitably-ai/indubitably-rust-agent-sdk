//! Agent module for the SDK.
//! 
//! This module provides the core agent functionality including
//! the main Agent struct, conversation management, and state handling.

pub mod agent;
pub mod state;
pub mod result;
pub mod conversation_manager;

pub use agent::Agent;
pub use state::AgentState;
pub use result::AgentResult;
pub use conversation_manager::{ConversationManager, ConversationManagerConfig};

// Re-export commonly used types
pub use agent::{AgentBuilder, ToolCaller};
