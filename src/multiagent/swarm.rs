//! Agent swarm for the SDK.
//! 
//! This module provides functionality for building and managing
//! agent swarms and collective behaviors.

use std::collections::HashMap;

/// An agent swarm for managing collective behaviors.
pub struct AgentSwarm {
    /// The agents in the swarm.
    agents: HashMap<String, String>,
    /// The swarm configuration.
    config: HashMap<String, serde_json::Value>,
}

impl AgentSwarm {
    /// Create a new agent swarm.
    pub fn new() -> Self {
        Self {
            agents: HashMap::new(),
            config: HashMap::new(),
        }
    }
    
    /// Add an agent to the swarm.
    pub fn add_agent(&mut self, agent_id: &str, agent_type: &str) {
        self.agents.insert(agent_id.to_string(), agent_type.to_string());
    }
    
    /// Remove an agent from the swarm.
    pub fn remove_agent(&mut self, agent_id: &str) {
        self.agents.remove(agent_id);
    }
    
    /// Get the number of agents in the swarm.
    pub fn agent_count(&self) -> usize {
        self.agents.len()
    }
    
    /// Get all agents in the swarm.
    pub fn agents(&self) -> &HashMap<String, String> {
        &self.agents
    }
    
    /// Set a configuration value.
    pub fn set_config(&mut self, key: &str, value: serde_json::Value) {
        self.config.insert(key.to_string(), value);
    }
    
    /// Get a configuration value.
    pub fn get_config(&self, key: &str) -> Option<&serde_json::Value> {
        self.config.get(key)
    }
}

impl Default for AgentSwarm {
    fn default() -> Self {
        Self::new()
    }
}
