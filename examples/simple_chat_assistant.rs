//! Simple Chat Assistant Example
//! 
//! This example demonstrates how to create a basic chat assistant
//! using the Indubitably Rust Agent SDK.

use indubitably_rust_agent_sdk::{
    Agent, types::{Messages, Message}
};
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🤖 Simple Chat Assistant Example");
    println!("================================\n");
    
    // Create the agent
    let mut agent = Agent::new()?;
    
    println!("User: What's the weather like today?");
    
    // Run the agent
    let result = agent.run("What's the weather like today?").await?;
    
    println!("Assistant: {}", result.response);
    
    // Another example
    println!("\nUser: Can you calculate 15 * 23 for me?");
    
    let result = agent.run("Can you calculate 15 * 23 for me?").await?;
    println!("Assistant: {}", result.response);
    
    // Interactive chat loop
    println!("\n💬 Interactive Chat Mode (type 'quit' to exit)");
    println!("===============================================");
    
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
        
        // Get agent response
        let result = agent.run(input).await?;
        
        // Add assistant response to conversation
        let assistant_message = Message::assistant(&result.response);
        conversation_history.push(assistant_message);
        
        println!("Assistant: {}", result.response);
    }
    
    println!("\n👋 Goodbye! Thanks for trying the Simple Chat Assistant!");
    Ok(())
}
