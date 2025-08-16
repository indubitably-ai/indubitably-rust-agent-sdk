use indubitably_rust_agent_sdk::agent::AgentBuilder;
use indubitably_rust_agent_sdk::models::model::MockModel;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Build an agent with the mock model for deterministic output
    let model = MockModel::new();
    let mut agent = AgentBuilder::new()
        .system_prompt("You are a helpful assistant.")
        .model(Box::new(model))
        .build()?;

    // Ask the agent a simple question
    let result = agent.run("Say hello in one short sentence.").await?;

    // Print the agent's response text
    println!("{}", result.response);
    Ok(())
}