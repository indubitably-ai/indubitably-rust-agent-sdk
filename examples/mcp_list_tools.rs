use indubitably_rust_agent_sdk::tools::mcp::MCPClientBuilder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create the MCP client with a short timeout
    let mut client = MCPClientBuilder::new().timeout(2).build();

    // Connect (placeholder) and list tools
    client.connect().await?;
    let tools = client.list_tools().await?;

    println!("MCP tools: {}", tools.len());
    for tool in tools {
        println!("- {}: {}", tool.name, tool.description);
    }

    client.disconnect().await?;
    Ok(())
}