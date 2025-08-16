//! Indubitably Rust Agent SDK
//! 
//! A model-driven approach to building AI agents in just a few lines of code.
//! 
//! ## Quick Start
//! 
//! ```rust
//! use indubitably_rust_agent_sdk::Agent;
//! 
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let mut agent = Agent::new().expect("failed to create agent");
//!     let result = agent.run("What is the capital of France?").await?;
//!     println!("Response: {}", result.response);
//!     Ok(())
//! }
//! ```

pub mod agent;
pub mod models;
pub mod types;
pub mod tools;
pub mod session;
pub mod telemetry;
pub mod hooks;
pub mod handlers;
pub mod event_loop;
pub mod multiagent;

// Re-export main types for convenience
pub use agent::Agent;
pub use models::Model;
pub use types::*;

// Re-export error types
pub use types::exceptions::*;

/// Current version of the SDK
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Default system prompt for agents
pub const DEFAULT_SYSTEM_PROMPT: &str = "You are a helpful AI assistant.";

/// Default agent name
pub const DEFAULT_AGENT_NAME: &str = "Indubitably Agent";

/// Default agent ID
pub const DEFAULT_AGENT_ID: &str = "default";
