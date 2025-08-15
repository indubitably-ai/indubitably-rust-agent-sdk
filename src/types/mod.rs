//! Core type definitions for the Indubitably Rust Agent SDK
//! 
//! This module contains all the fundamental types used throughout the SDK,
//! including messages, content blocks, tools, and other data structures.

pub mod content;
pub mod tools;
pub mod streaming;
pub mod exceptions;
pub mod guardrails;
pub mod media;
pub mod traces;
pub mod collections;
pub mod event_loop;
pub mod session;

pub use content::*;
pub use tools::*;
pub use streaming::*;
pub use exceptions::*;
pub use guardrails::*;
pub use media::*;
pub use traces::*;
pub use collections::*;
pub use event_loop::*;
pub use session::*;

// Re-export commonly used types
pub use content::{Message, Messages, ContentBlock, SystemContentBlock};
pub use tools::{ToolSpec, ToolUse, ToolResult};
pub use streaming::StreamEvent;
pub use session::{Session, SessionAgent, SessionMessage};
