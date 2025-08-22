//! Unit tests for the multiagent module.
//! 
//! These tests verify that multiagent functionality works correctly
//! including agent graphs, workflows, and swarm operations.

use indubitably_rust_agent_sdk::multiagent::*;
use indubitably_rust_agent_sdk::types::{Message, MessageRole, ContentBlock};
use std::collections::HashMap;

#[test]
fn test_agent_node_creation() {
    let mut config = HashMap::new();
    config.insert("model".to_string(), "gpt-4".to_string());
    config.insert("temperature".to_string(), "0.7".to_string());
    
    let node = AgentNode {
        agent_id: "researcher".to_string(),
        node_type: "research".to_string(),
        config,
    };
    
    assert_eq!(node.agent_id, "researcher");
    assert_eq!(node.node_type, "research");
    assert_eq!(node.config.get("model").unwrap(), "gpt-4");
    assert_eq!(node.config.get("temperature").unwrap(), "0.7");
}

#[test]
fn test_agent_edge_creation() {
    let edge = AgentEdge {
        source: "researcher".to_string(),
        target: "writer".to_string(),
        condition: Some("research_complete".to_string()),
    };
    
    assert_eq!(edge.source, "researcher");
    assert_eq!(edge.target, "writer");
    assert_eq!(edge.condition.unwrap(), "research_complete");
}

#[test]
fn test_agent_edge_without_condition() {
    let edge = AgentEdge {
        source: "researcher".to_string(),
        target: "writer".to_string(),
        condition: None,
    };
    
    assert_eq!(edge.source, "researcher");
    assert_eq!(edge.target, "writer");
    assert!(edge.condition.is_none());
}

#[test]
fn test_agent_graph_creation() {
    let graph = AgentGraph::new();
    
    assert_eq!(graph.node_count(), 0);
    assert_eq!(graph.edge_count(), 0);
    assert!(graph.is_empty());
}

#[test]
fn test_agent_graph_add_node() {
    let mut graph = AgentGraph::new();
    
    let node = AgentNode {
        agent_id: "researcher".to_string(),
        node_type: "research".to_string(),
        config: HashMap::new(),
    };
    
    graph.add_node(node);
    
    assert_eq!(graph.node_count(), 1);
    assert!(!graph.is_empty());
}

#[test]
fn test_agent_graph_add_edge() {
    let mut graph = AgentGraph::new();
    
    let node1 = AgentNode {
        agent_id: "researcher".to_string(),
        node_type: "research".to_string(),
        config: HashMap::new(),
    };
    
    let node2 = AgentNode {
        agent_id: "writer".to_string(),
        node_type: "writing".to_string(),
        config: HashMap::new(),
    };
    
    graph.add_node(node1);
    graph.add_node(node2);
    
    let edge = AgentEdge {
        source: "researcher".to_string(),
        target: "writer".to_string(),
        condition: None,
    };
    
    graph.add_edge(edge);
    
    assert_eq!(graph.node_count(), 2);
    assert_eq!(graph.edge_count(), 1);
}

#[test]
fn test_agent_graph_remove_node() {
    let mut graph = AgentGraph::new();
    
    let node = AgentNode {
        agent_id: "researcher".to_string(),
        node_type: "research".to_string(),
        config: HashMap::new(),
    };
    
    graph.add_node(node);
    assert_eq!(graph.node_count(), 1);
    
    graph.remove_node("researcher");
    assert_eq!(graph.node_count(), 0);
    assert!(graph.is_empty());
}

#[test]
fn test_agent_graph_remove_edge() {
    let mut graph = AgentGraph::new();
    
    let node1 = AgentNode {
        agent_id: "researcher".to_string(),
        node_type: "research".to_string(),
        config: HashMap::new(),
    };
    
    let node2 = AgentNode {
        agent_id: "writer".to_string(),
        node_type: "writing".to_string(),
        config: HashMap::new(),
    };
    
    graph.add_node(node1);
    graph.add_node(node2);
    
    let edge = AgentEdge {
        source: "researcher".to_string(),
        target: "writer".to_string(),
        condition: None,
    };
    
    graph.add_edge(edge);
    assert_eq!(graph.edge_count(), 1);
    
    graph.remove_edge("researcher", "writer");
    assert_eq!(graph.edge_count(), 0);
}

#[test]
fn test_agent_graph_get_node() {
    let mut graph = AgentGraph::new();
    
    let node = AgentNode {
        agent_id: "researcher".to_string(),
        node_type: "research".to_string(),
        config: HashMap::new(),
    };
    
    graph.add_node(node);
    
    let retrieved_node = graph.get_node("researcher").unwrap();
    assert_eq!(retrieved_node.agent_id, "researcher");
    assert_eq!(retrieved_node.node_type, "research");
}

#[test]
fn test_agent_graph_get_node_nonexistent() {
    let graph = AgentGraph::new();
    
    let result = graph.get_node("nonexistent");
    assert!(result.is_none());
}

#[test]
fn test_agent_graph_get_neighbors() {
    let mut graph = AgentGraph::new();
    
    let node1 = AgentNode {
        agent_id: "researcher".to_string(),
        node_type: "research".to_string(),
        config: HashMap::new(),
    };
    
    let node2 = AgentNode {
        agent_id: "writer".to_string(),
        node_type: "writing".to_string(),
        config: HashMap::new(),
    };
    
    let node3 = AgentNode {
        agent_id: "reviewer".to_string(),
        node_type: "review".to_string(),
        config: HashMap::new(),
    };
    
    graph.add_node(node1);
    graph.add_node(node2);
    graph.add_node(node3);
    
    let edge1 = AgentEdge {
        source: "researcher".to_string(),
        target: "writer".to_string(),
        condition: None,
    };
    
    let edge2 = AgentEdge {
        source: "researcher".to_string(),
        target: "reviewer".to_string(),
        condition: None,
    };
    
    graph.add_edge(edge1);
    graph.add_edge(edge2);
    
    let neighbors = graph.get_neighbors("researcher");
    assert_eq!(neighbors.len(), 2);
    
    let neighbor_ids: Vec<&str> = neighbors.iter().map(|n| n.agent_id.as_str()).collect();
    assert!(neighbor_ids.contains(&"writer"));
    assert!(neighbor_ids.contains(&"reviewer"));
}

#[test]
fn test_agent_graph_topological_sort() {
    let mut graph = AgentGraph::new();
    
    let node1 = AgentNode {
        agent_id: "researcher".to_string(),
        node_type: "research".to_string(),
        config: HashMap::new(),
    };
    
    let node2 = AgentNode {
        agent_id: "writer".to_string(),
        node_type: "writing".to_string(),
        config: HashMap::new(),
    };
    
    let node3 = AgentNode {
        agent_id: "reviewer".to_string(),
        node_type: "review".to_string(),
        config: HashMap::new(),
    };
    
    graph.add_node(node1);
    graph.add_node(node2);
    graph.add_node(node3);
    
    let edge1 = AgentEdge {
        source: "researcher".to_string(),
        target: "writer".to_string(),
        condition: None,
    };
    
    let edge2 = AgentEdge {
        source: "writer".to_string(),
        target: "reviewer".to_string(),
        condition: None,
    };
    
    graph.add_edge(edge1);
    graph.add_edge(edge2);
    
    let sorted = graph.topological_sort().unwrap();
    assert_eq!(sorted.len(), 3);
    
    // Check that researcher comes before writer, and writer before reviewer
    let researcher_idx = sorted.iter().position(|id| id == "researcher").unwrap();
    let writer_idx = sorted.iter().position(|id| id == "writer").unwrap();
    let reviewer_idx = sorted.iter().position(|id| id == "reviewer").unwrap();
    
    assert!(researcher_idx < writer_idx);
    assert!(writer_idx < reviewer_idx);
}

#[test]
fn test_agent_graph_cycle_detection() {
    let mut graph = AgentGraph::new();
    
    let node1 = AgentNode {
        agent_id: "researcher".to_string(),
        node_type: "research".to_string(),
        config: HashMap::new(),
    };
    
    let node2 = AgentNode {
        agent_id: "writer".to_string(),
        node_type: "writing".to_string(),
        config: HashMap::new(),
    };
    
    graph.add_node(node1);
    graph.add_node(node2);
    
    let edge1 = AgentEdge {
        source: "researcher".to_string(),
        target: "writer".to_string(),
        condition: None,
    };
    
    let edge2 = AgentEdge {
        source: "writer".to_string(),
        target: "researcher".to_string(),
        condition: None,
    };
    
    graph.add_edge(edge1);
    graph.add_edge(edge2);
    
    // Should detect cycle
    assert!(graph.has_cycle());
}

#[test]
fn test_agent_graph_no_cycle() {
    let mut graph = AgentGraph::new();
    
    let node1 = AgentNode {
        agent_id: "researcher".to_string(),
        node_type: "research".to_string(),
        config: HashMap::new(),
    };
    
    let node2 = AgentNode {
        agent_id: "writer".to_string(),
        node_type: "writing".to_string(),
        config: HashMap::new(),
    };
    
    graph.add_node(node1);
    graph.add_node(node2);
    
    let edge = AgentEdge {
        source: "researcher".to_string(),
        target: "writer".to_string(),
        condition: None,
    };
    
    graph.add_edge(edge);
    
    // Should not have cycle
    assert!(!graph.has_cycle());
}

#[tokio::test]
async fn test_agent_swarm_creation() {
    let swarm = AgentSwarm::new();
    
    assert_eq!(swarm.agent_count(), 0);
    assert!(swarm.is_empty());
}

#[tokio::test]
async fn test_agent_swarm_add_agent() {
    let mut swarm = AgentSwarm::new();
    
    let agent = MockAgent::new("swarm_agent_1");
    swarm.add_agent(agent);
    
    assert_eq!(swarm.agent_count(), 1);
    assert!(!swarm.is_empty());
}

#[tokio::test]
async fn test_agent_swarm_remove_agent() {
    let mut swarm = AgentSwarm::new();
    
    let agent = MockAgent::new("swarm_agent_1");
    swarm.add_agent(agent);
    
    assert_eq!(swarm.agent_count(), 1);
    
    swarm.remove_agent("swarm_agent_1");
    assert_eq!(swarm.agent_count(), 0);
    assert!(swarm.is_empty());
}

#[tokio::test]
async fn test_agent_swarm_get_agent() {
    let mut swarm = AgentSwarm::new();
    
    let agent = MockAgent::new("swarm_agent_1");
    swarm.add_agent(agent);
    
    let retrieved_agent = swarm.get_agent("swarm_agent_1").unwrap();
    assert_eq!(retrieved_agent.id(), "swarm_agent_1");
}

#[tokio::test]
async fn test_agent_swarm_get_agent_nonexistent() {
    let swarm = AgentSwarm::new();
    
    let result = swarm.get_agent("nonexistent");
    assert!(result.is_none());
}

#[tokio::test]
async fn test_agent_swarm_broadcast_message() {
    let mut swarm = AgentSwarm::new();
    
    let agent1 = MockAgent::new("swarm_agent_1");
    let agent2 = MockAgent::new("swarm_agent_2");
    let agent3 = MockAgent::new("swarm_agent_3");
    
    swarm.add_agent(agent1);
    swarm.add_agent(agent2);
    swarm.add_agent(agent3);
    
    let message = Message::user("Hello, swarm!");
    let results = swarm.broadcast_message(message).await;
    
    assert_eq!(results.len(), 3);
    
    for result in results {
        assert!(result.is_ok());
        let response = result.unwrap();
        assert!(response.contains("swarm response"));
    }
}

#[tokio::test]
async fn test_agent_swarm_targeted_message() {
    let mut swarm = AgentSwarm::new();
    
    let agent1 = MockAgent::new("swarm_agent_1");
    let agent2 = MockAgent::new("swarm_agent_2");
    
    swarm.add_agent(agent1);
    swarm.add_agent(agent2);
    
    let message = Message::user("Hello, specific agent!");
    let result = swarm.send_message_to_agent("swarm_agent_1", message).await;
    
    assert!(result.is_ok());
    let response = result.unwrap();
    assert!(response.contains("swarm response"));
}

#[tokio::test]
async fn test_agent_swarm_targeted_message_nonexistent() {
    let swarm = AgentSwarm::new();
    
    let message = Message::user("Hello, nonexistent agent!");
    let result = swarm.send_message_to_agent("nonexistent", message).await;
    
    assert!(result.is_err());
}

#[tokio::test]
async fn test_agent_swarm_concurrent_operations() {
    let swarm = std::sync::Arc::new(tokio::sync::RwLock::new(AgentSwarm::new()));
    
    // Spawn multiple tasks to test concurrent access
    let mut handles = vec![];
    
    for i in 0..5 {
        let swarm_clone = swarm.clone();
        let handle = tokio::spawn(async move {
            let mut swarm = swarm_clone.write().await;
            let agent = MockAgent::new(format!("swarm_agent_{}", i));
            swarm.add_agent(agent);
        });
        handles.push(handle);
    }
    
    // Wait for all tasks to complete
    for handle in handles {
        handle.await.unwrap();
    }
    
    let swarm = swarm.read().await;
    assert_eq!(swarm.agent_count(), 5);
}

#[test]
fn test_agent_graph_serialization() {
    let mut graph = AgentGraph::new();
    
    let node = AgentNode {
        agent_id: "researcher".to_string(),
        node_type: "research".to_string(),
        config: HashMap::new(),
    };
    
    graph.add_node(node);
    
    let json = serde_json::to_string(&graph).unwrap();
    let deserialized: AgentGraph = serde_json::from_str(&json).unwrap();
    
    assert_eq!(deserialized.node_count(), 1);
    assert_eq!(deserialized.edge_count(), 0);
}

#[test]
fn test_agent_node_serialization() {
    let mut config = HashMap::new();
    config.insert("model".to_string(), "gpt-4".to_string());
    
    let node = AgentNode {
        agent_id: "researcher".to_string(),
        node_type: "research".to_string(),
        config,
    };
    
    let json = serde_json::to_string(&node).unwrap();
    let deserialized: AgentNode = serde_json::from_str(&json).unwrap();
    
    assert_eq!(deserialized.agent_id, node.agent_id);
    assert_eq!(deserialized.node_type, node.node_type);
    assert_eq!(deserialized.config.get("model").unwrap(), "gpt-4");
}

#[test]
fn test_agent_edge_serialization() {
    let edge = AgentEdge {
        source: "researcher".to_string(),
        target: "writer".to_string(),
        condition: Some("research_complete".to_string()),
    };
    
    let json = serde_json::to_string(&edge).unwrap();
    let deserialized: AgentEdge = serde_json::from_str(&json).unwrap();
    
    assert_eq!(deserialized.source, edge.source);
    assert_eq!(deserialized.target, edge.target);
    assert_eq!(deserialized.condition.unwrap(), "research_complete");
}

// Mock agent implementation for testing
#[derive(Debug, Clone)]
struct MockAgent {
    id: String,
}

impl MockAgent {
    fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
        }
    }
}

impl Agent for MockAgent {
    fn id(&self) -> &str {
        &self.id
    }
    
    fn name(&self) -> &str {
        "Mock Agent"
    }
    
    fn system_prompt(&self) -> Option<&str> {
        Some("You are a mock agent.")
    }
    
    async fn run(&self, _message: &str) -> Result<AgentResult, Box<dyn std::error::Error + Send + Sync>> {
        Ok(AgentResult {
            agent_id: self.id.clone(),
            response: "swarm response".to_string(),
            metadata: None,
        })
    }
    
    async fn stream(&self, _message: &str) -> Result<ModelStreamResponse, Box<dyn std::error::Error + Send + Sync>> {
        // Mock streaming implementation
        Ok(ModelStreamResponse::new())
    }
}

// Mock stream response implementation
impl ModelStreamResponse {
    fn new() -> Self {
        Self {
            // Mock implementation
        }
    }
}

// Mock agent result implementation
impl AgentResult {
    fn new(agent_id: String, response: String, metadata: Option<serde_json::Value>) -> Self {
        Self {
            agent_id,
            response,
            metadata,
        }
    }
}
