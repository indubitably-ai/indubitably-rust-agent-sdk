use indubitably_rust_agent_sdk::multiagent::AgentGraph;
use indubitably_rust_agent_sdk::multiagent::graph::{AgentNode, AgentEdge};
use std::collections::HashMap;

fn main() {
    let mut graph = AgentGraph::new();

    graph.add_node(AgentNode {
        agent_id: "researcher".to_string(),
        node_type: "research".to_string(),
        config: HashMap::new(),
    });

    graph.add_node(AgentNode {
        agent_id: "writer".to_string(),
        node_type: "writing".to_string(),
        config: HashMap::new(),
    });

    graph.add_edge(AgentEdge {
        source: "researcher".to_string(),
        target: "writer".to_string(),
        condition: None,
    });

    println!("Nodes: {} | Edges: {}", graph.nodes().len(), graph.edges().len());
}