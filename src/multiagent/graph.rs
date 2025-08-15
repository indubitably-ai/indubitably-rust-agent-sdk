//! Agent graph for the SDK.
//! 
//! This module provides functionality for building and managing
//! agent graphs and workflows.

use std::collections::HashMap;

/// A node in an agent graph.
pub struct AgentNode {
    /// The agent ID.
    pub agent_id: String,
    /// The node type.
    pub node_type: String,
    /// The node configuration.
    pub config: HashMap<String, serde_json::Value>,
}

/// An edge in an agent graph.
pub struct AgentEdge {
    /// The source node ID.
    pub source: String,
    /// The target node ID.
    pub target: String,
    /// The edge condition.
    pub condition: Option<String>,
}

/// An agent graph for managing multi-agent workflows.
pub struct AgentGraph {
    /// The nodes in the graph.
    nodes: HashMap<String, AgentNode>,
    /// The edges in the graph.
    edges: Vec<AgentEdge>,
}

impl AgentGraph {
    /// Create a new agent graph.
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            edges: Vec::new(),
        }
    }
    
    /// Add a node to the graph.
    pub fn add_node(&mut self, node: AgentNode) {
        self.nodes.insert(node.agent_id.clone(), node);
    }
    
    /// Add an edge to the graph.
    pub fn add_edge(&mut self, edge: AgentEdge) {
        self.edges.push(edge);
    }
    
    /// Get a node by ID.
    pub fn get_node(&self, node_id: &str) -> Option<&AgentNode> {
        self.nodes.get(node_id)
    }
    
    /// Get all nodes.
    pub fn nodes(&self) -> &HashMap<String, AgentNode> {
        &self.nodes
    }
    
    /// Get all edges.
    pub fn edges(&self) -> &[AgentEdge] {
        &self.edges
    }
}

impl Default for AgentGraph {
    fn default() -> Self {
        Self::new()
    }
}
