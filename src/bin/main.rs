//! Indubitably CLI binary for the SDK.
//! 
//! This binary provides a command-line interface for interacting
//! with the Indubitably Rust Agent SDK, including chat functionality and tool management.

use clap::{Parser, Subcommand};
use std::path::PathBuf;
use tokio;

use indubitably_rust_agent_sdk::{
    agent::AgentBuilder,
    models::{BedrockModel, OpenAIModel, AnthropicModel, OllamaModel},
    tools::registry::ToolRegistry,
    types::IndubitablyResult,
};

#[derive(Parser)]
#[command(name = "indubitably-cli")]
#[command(about = "Indubitably Rust Agent SDK CLI - A model-driven approach to building AI agents")]
#[command(version = env!("CARGO_PKG_VERSION"))]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start a chat session with an agent
    Chat {
        /// The message to send to the agent
        message: String,
        
        /// The model to use (bedrock, openai, anthropic, ollama)
        #[arg(short, long, default_value = "bedrock")]
        model: String,
        
        /// The system prompt for the agent
        #[arg(short, long)]
        system_prompt: Option<String>,
        
        /// Enable verbose output
        #[arg(short, long)]
        verbose: bool,
    },
    
    /// List available tools
    Tools {
        /// Show detailed tool information
        #[arg(short, long)]
        detailed: bool,
    },
    
    /// Show version information
    Version,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Chat { message, model, system_prompt, verbose } => {
            chat_command(message, model, system_prompt, verbose).await?;
        }
        Commands::Tools { detailed } => {
            tools_command(detailed).await?;
        }
        Commands::Version => {
            version_command();
        }
    }
    
    Ok(())
}

async fn chat_command(
    message: String,
    model: String,
    system_prompt: Option<String>,
    verbose: bool,
) -> IndubitablyResult<()> {
    if verbose {
        println!("Starting chat with model: {}", model);
        if let Some(prompt) = &system_prompt {
            println!("System prompt: {}", prompt);
        }
    }
    
    // Create the appropriate model
    let model_box: Box<dyn indubitably_rust_agent_sdk::models::Model> = match model.to_lowercase().as_str() {
        "bedrock" => {
            if verbose {
                println!("Using Amazon Bedrock model");
            }
            Box::new(BedrockModel::new())
        }
        "openai" => {
            if verbose {
                println!("Using OpenAI model");
            }
            Box::new(OpenAIModel::new())
        }
        "anthropic" => {
            if verbose {
                println!("Using Anthropic Claude model");
            }
            Box::new(AnthropicModel::new())
        }
        "ollama" => {
            if verbose {
                println!("Using Ollama model");
            }
            Box::new(OllamaModel::new())
        }
        _ => {
            eprintln!("Unknown model: {}. Using Bedrock as default.", model);
            Box::new(BedrockModel::new())
        }
    };
    
    // Build the agent
    let mut agent_builder = AgentBuilder::new().model(model_box);
    
    if let Some(prompt) = system_prompt {
        agent_builder = agent_builder.system_prompt(&prompt);
    }
    
    let mut agent = agent_builder.build()?;
    
    if verbose {
        println!("Agent created successfully");
        println!("Sending message: {}", message);
    }
    
    // Run the agent
    let result = agent.run(&message).await?;
    
    if verbose {
        println!("Response received in {} messages", result.messages.len());
    }
    
    println!("Agent: {}", result.response);
    
    Ok(())
}

async fn tools_command(detailed: bool) -> IndubitablyResult<()> {
    let registry = ToolRegistry::new();
    
    let tool_count = registry.count().await;
    
    if tool_count == 0 {
        println!("No tools available.");
        println!("To add tools, use the SDK programmatically or load them from a directory.");
        return Ok(());
    }
    
    println!("Available tools ({}):", tool_count);
    
    if detailed {
        let tools = registry.list_tools().await;
        for tool in tools {
            println!("  - {}: {}", tool.name, tool.description);
        }
    } else {
        let names = registry.list_names().await;
        for name in names {
            println!("  - {}", name);
        }
    }
    
    Ok(())
}

fn version_command() {
    println!("Indubitably CLI version {}", env!("CARGO_PKG_VERSION"));
    println!("Indubitably SDK version {}", indubitably_rust_agent_sdk::VERSION);
}

fn help_command() {
    println!("Indubitably CLI - A model-driven approach to building AI agents");
    println!();
    println!("Usage:");
    println!("  indubitably-cli <COMMAND>");
    println!();
    println!("Commands:");
    println!("  chat     Start a chat session with an agent");
    println!("  tools    List available tools");
    println!("  version  Show version information");
    println!("  help     Show this help message");
    println!();
    println!("Examples:");
    println!("  indubitably-cli chat \"Hello, how are you?\"");
    println!("  indubitably-cli chat -m openai \"What's the weather like?\"");
    println!("  indubitably-cli chat -m openai -s \"You are a helpful assistant\" \"Tell me a joke\"");
    println!("  indubitably-cli tools --detailed");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_parsing() {
        let args = vec!["indubitably-cli", "chat", "Hello"];
        let cli = Cli::try_parse_from(args);
        assert!(cli.is_ok());
    }

    #[test]
    fn test_version_command() {
        // This is a simple test that just ensures the function doesn't panic
        version_command();
    }
}
