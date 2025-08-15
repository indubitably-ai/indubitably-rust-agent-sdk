//! Hooks system for the SDK.
//! 
//! This module provides a hooks system for extending
//! agent functionality with custom behaviors.

pub mod events;
pub mod registry;

pub use events::*;
pub use registry::HookRegistry;
