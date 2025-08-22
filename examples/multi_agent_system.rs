//! Multi-Agent System Example
//! 
//! This example demonstrates how to create a system of multiple agents
//! that work together, each with different specializations and tools.

use indubitably_rust_agent_sdk::{
    Agent, types::{Messages, Message}
};
use tokio;

/// Research Agent - specializes in gathering information
fn create_research_agent() -> Agent {
    Agent::new().expect("Failed to create research agent")
}

/// Analysis Agent - specializes in analyzing and summarizing information
fn create_analysis_agent() -> Agent {
    Agent::new().expect("Failed to create analysis agent")
}

/// Creative Agent - specializes in creative tasks and writing
fn create_creative_agent() -> Agent {
    Agent::new().expect("Failed to create creative agent")
}

/// Simple multi-agent system simulation
struct SimpleMultiAgentSystem {
    research_agent: Agent,
    analysis_agent: Agent,
    creative_agent: Agent,
}

impl SimpleMultiAgentSystem {
    fn new() -> Self {
        Self {
            research_agent: create_research_agent(),
            analysis_agent: create_analysis_agent(),
            creative_agent: create_creative_agent(),
        }
    }
    
    async fn execute_research_workflow(&mut self, query: &str) -> Result<String, Box<dyn std::error::Error>> {
        println!("🔍 Research Agent: Gathering information...");
        let research_result = self.research_agent.run(query).await?;
        
        println!("📊 Analysis Agent: Analyzing findings...");
        let analysis_query = format!("Analyze this research: {}", research_result.response);
        let analysis_result = self.analysis_agent.run(&analysis_query).await?;
        
        println!("🎨 Creative Agent: Creating content...");
        let creative_query = format!("Create a story based on this analysis: {}", analysis_result.response);
        let creative_result = self.creative_agent.run(&creative_query).await?;
        
        Ok(creative_result.response)
    }
    
    async fn route_to_agent(&mut self, input: &str) -> Result<String, Box<dyn std::error::Error>> {
        if input.to_lowercase().contains("research") || input.to_lowercase().contains("find") {
            println!("🔍 Routing to Research Agent...");
            let result = self.research_agent.run(input).await?;
            Ok(result.response)
        } else if input.to_lowercase().contains("analyze") || input.to_lowercase().contains("summarize") {
            println!("📊 Routing to Analysis Agent...");
            let result = self.analysis_agent.run(input).await?;
            Ok(result.response)
        } else if input.to_lowercase().contains("create") || input.to_lowercase().contains("write") {
            println!("🎨 Routing to Creative Agent...");
            let result = self.creative_agent.run(input).await?;
            Ok(result.response)
        } else {
            // Default to research agent for general queries
            println!("🔍 Routing to Research Agent (default)...");
            let result = self.research_agent.run(input).await?;
            Ok(result.response)
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🤖 Multi-Agent System Example");
    println!("=============================\n");
    
    // Create the multi-agent system
    let mut system = SimpleMultiAgentSystem::new();
    
    println!("🔧 System initialized with 3 specialized agents");
    println!("📊 Workflow: Research → Analysis → Creative\n");
    
    // Example 1: Research task
    println!("📚 Example 1: Research Task");
    println!("----------------------------");
    
    let research_query = "What are the latest developments in AI technology?";
    println!("Query: {}", research_query);
    
    // Execute the research workflow
    let result = system.execute_research_workflow(research_query).await?;
    println!("Final Result: {}", result);
    
    // Example 2: Interactive multi-agent chat
    println!("\n💬 Example 2: Interactive Multi-Agent Chat");
    println!("===========================================");
    
    let mut conversation_history = Messages::new();
    
    loop {
        print!("\nYou: ");
        std::io::Write::flush(&mut std::io::stdout())?;
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        let input = input.trim();
        
        if input.to_lowercase() == "quit" {
            break;
        }
        
        if input.is_empty() {
            continue;
        }
        
        // Add user message to conversation
        let user_message = Message::user(input);
        conversation_history.push(user_message);
        
        // Route to appropriate agent based on content
        let agent_response = system.route_to_agent(input).await?;
        
        // Add assistant response to conversation
        let assistant_message = Message::assistant(&agent_response);
        conversation_history.push(assistant_message);
        
        println!("System: {}", agent_response);
    }
    
    println!("\n👋 Goodbye! Thanks for exploring the Multi-Agent System!");
    Ok(())
}
