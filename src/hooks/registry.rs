//! Hook registry for the SDK.
//! 
//! This module provides a registry for managing hooks
//! and their event handlers.

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use super::events::HookEvent;

/// A hook function.
pub type HookFunction = Box<dyn Fn(HookEvent) -> Result<(), Box<dyn std::error::Error + Send + Sync>> + Send + Sync>;

/// A registry for managing hooks.
pub struct HookRegistry {
    /// The registered hooks.
    hooks: Arc<RwLock<HashMap<String, Vec<HookFunction>>>>,
}

impl HookRegistry {
    /// Create a new hook registry.
    pub fn new() -> Self {
        Self {
            hooks: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// Register a hook for an event type.
    pub async fn register_hook(&self, event_type: &str, hook: HookFunction) {
        let mut hooks = self.hooks.write().await;
        hooks.entry(event_type.to_string()).or_insert_with(Vec::new).push(hook);
    }
    
    /// Trigger hooks for an event type.
    pub async fn trigger_hooks(&self, event: HookEvent) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let hooks = self.hooks.read().await;
        if let Some(event_hooks) = hooks.get(&event.event_type) {
            for hook in event_hooks {
                hook(event.clone())?;
            }
        }
        Ok(())
    }
}

impl Default for HookRegistry {
    fn default() -> Self {
        Self::new()
    }
}
