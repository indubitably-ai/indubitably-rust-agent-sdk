# Indubitably Rust Agent SDK

<div align="center">
  <div>
    <a href="https://github.com/indubitably-ai/indubitably-rust-agent-sdk">
      <img src="https://github.com/indubitably-ai/indubitably-rust-agent-sdk/raw/main/assets/logo.png" alt="Indubitably AI" width="120px" height="120px">
    </a>
  </div>

  <h1>
    Indubitably AI
  </h1>

  <h2>
    A model-driven approach to building AI agents in Rust with just a few lines of code.
  </h2>

  <div align="center">
    <a href="https://github.com/indubitably-ai/indubitably-rust-agent-sdk/graphs/commit-activity"><img alt="GitHub commit activity" src="https://img.shields.io/github/commit-activity/m/indubitably-ai/indubitably-rust-agent-sdk"/></a>
    <a href="https://github.com/indubitably-ai/indubitably-rust-agent-sdk/issues"><img alt="GitHub open issues" src="https://img.shields.io/github/issues/indubitably-ai/indubitably-rust-agent-sdk"/></a>
    <a href="https://github.com/indubitably-ai/indubitably-rust-agent-sdk/pulls"><img alt="GitHub open pull requests" src="https://img.shields.io/github/issues-pr/indubitably-ai/indubitably-rust-agent-sdk"/></a>
    <a href="https://github.com/indubitably-ai/indubitably-rust-agent-sdk/blob/main/LICENSE"><img alt="License" src="https://img.shields.io/github/license/indubitably-ai/indubitably-rust-agent-sdk"/></a>
    <a href="https://crates.io/crates/indubitably-rust-agent-sdk"><img alt="Crates.io version" src="https://img.shields.io/crates/v/indubitably-rust-agent-sdk"/></a>
    <a href="https://rust-lang.org"><img alt="Rust" src="https://img.shields.io/badge/rust-1.70+-blue.svg"/></a>
  </div>
  
  <p>
    <a href="https://github.com/indubitably-ai/indubitably-rust-agent-sdk">Documentation</a>
    ◆ <a href="https://github.com/indubitably-ai/indubitably-rust-agent-sdk">Samples</a>
    ◆ <a href="https://github.com/indubitably-ai/indubitably-rust-agent-sdk">Rust SDK</a>
    ◆ <a href="https://github.com/indubitably-ai/indubitably-rust-agent-sdk">Tools</a>
    ◆ <a href="https://github.com/indubitably-ai/indubitably-rust-agent-sdk">Agent Builder</a>
    ◆ <a href="https://github.com/indubitably-ai/indubitably-rust-agent-sdk">MCP Server</a>
  </p>
</div>

Indubitably Rust Agent SDK is a simple yet powerful SDK that takes a model-driven approach to building and running AI agents in Rust. From simple conversational assistants to complex autonomous workflows, from local development to production deployment, Indubitably Rust Agent SDK scales with your needs.

## Feature Overview

- **Lightweight & Flexible**: Simple agent loop that just works and is fully customizable
- **Model Agnostic**: Support for various AI providers, Anthropic, OpenAI, Ollama, and custom providers
- **Advanced Capabilities**: Multi-agent systems, autonomous agents, and streaming support
- **Built-in MCP**: Native support for Model Context Protocol (MCP) servers, enabling access to thousands of pre-built tools
- **Rust Native**: Built with Rust for performance, safety, and reliability

## Quick Start

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone the repository
git clone https://github.com/indubitably-ai/indubitably-rust-agent-sdk.git
cd indubitably-rust-agent-sdk

# Build and run the CLI
cargo run --bin indubitably-cli chat "What is the capital of France?"
```

## Examples

The SDK includes comprehensive examples demonstrating how to build various types of AI agents and applications. These examples showcase real-world usage patterns and serve as a starting point for your own projects.

### Getting Started with Examples

```bash
# Navigate to the examples directory
cd examples

# Build all examples
cargo build

# Run a specific example
cargo run --bin simple_chat_assistant
cargo run --bin multi_agent_system
cargo run --bin web_chat_interface

# Run all examples sequentially
./demo.sh

# Run tests to verify examples work correctly
cargo test
```

### Available Examples

#### 1. Simple Chat Assistant (`simple_chat_assistant`)

A command-line chat assistant demonstrating basic agent creation and conversation flow:

```bash
cargo run --bin simple_chat_assistant
```

**Features:**
- Interactive command-line chat interface
- Mock tool implementations (web browser, calculator)
- Tool specification definitions
- Basic conversation management

**Use Case:** Perfect for learning the basics of agent creation and tool integration.

#### 2. Multi-Agent System (`multi_agent_system`)

Demonstrates multiple specialized agents working together in a coordinated workflow:

```bash
cargo run --bin multi_agent_system
```

**Features:**
- Research Agent: Gathers information
- Analysis Agent: Processes and analyzes data
- Creative Agent: Generates content based on analysis
- Coordinated workflow execution

**Use Case:** Ideal for complex tasks requiring multiple specialized agents.

#### 3. Web Chat Interface (`web_chat_interface`)

A web-based chat application using Actix-web framework:

```bash
cargo run --bin web_chat_interface
```

**Features:**
- Web-based chat interface
- Session management
- RESTful API endpoints
- Real-time conversation handling

**Use Case:** Building web applications with AI agent integration.

### Example Architecture

Each example demonstrates key SDK concepts:

- **Agent Creation**: Simple agent instantiation and configuration
- **Tool Integration**: Creating and using custom tools
- **Message Handling**: Managing conversation flow and history
- **Multi-Agent Coordination**: Building complex agent workflows
- **Web Integration**: Deploying agents in web applications

### Running Examples with Scripts

The examples directory includes helpful scripts for automation:

```bash
# Build and run examples with interactive menu
./build_and_run.sh

# Guided demonstration of all examples
./demo.sh

# Quick test run
./build_and_run.sh test
```

### Customizing Examples

Examples are designed to be easily customizable:

1. **Modify Tool Implementations**: Replace mock tools with real functionality
2. **Add New Agents**: Extend multi-agent systems with specialized agents
3. **Customize Web Interface**: Modify the web chat interface for your needs
4. **Integrate Real Models**: Connect to actual AI providers (OpenAI, Anthropic, etc.)

### Testing Examples

Comprehensive tests ensure examples work correctly:

```bash
# Run all tests
cargo test

# Run specific test suites
cargo test --test integration_test

# Run tests with verbose output
cargo test -- --nocapture
```

**Test Coverage:**
- Agent creation and configuration
- Tool specification and metadata
- Message handling and conversation flow
- Multi-agent system coordination
- Web interface functionality

### Example Dependencies

Examples use the following key dependencies:

```toml
[dependencies]
indubitably-rust-agent-sdk = { path = ".." }
tokio = { version = "1.0", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
actix-web = "4.0"
actix-files = "0.6"
async-trait = "0.1"
```

### Next Steps

After exploring the examples:

1. **Study the Code**: Understand how each example implements key concepts
2. **Modify and Extend**: Customize examples for your specific use case
3. **Build Your Own**: Use examples as templates for your applications
4. **Contribute**: Improve examples or add new ones for the community

For detailed documentation on each example, see the [examples/README.md](examples/README.md) file.

## Basic Usage

```rust
use indubitably_rust_agent_sdk::Agent;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new agent
    let mut agent = Agent::new();
    
    // Run the agent with a message
    let result = agent.run("What is the capital of France?").await?;
    
    println!("Response: {}", result.response);
    Ok(())
}
```

## Installation

Ensure you have Rust 1.70+ installed, then:

```bash
# Add to your Cargo.toml
[dependencies]
indubitably-rust-agent-sdk = "0.1.0"
tokio = { version = "1.0", features = ["full"] }
```

## Features at a Glance

### Rust-Based Tools

Easily build tools using Rust functions:

```rust
use indubitably_rust_agent_sdk::tools::tool;

#[tool("word_count", "Count words in text")]
fn word_count(text: &str) -> usize {
    text.split_whitespace().count()
}

// Use the tool
let agent = Agent::new();
let result = agent.run("How many words are in this sentence?").await?;
```

### MCP Support

Seamlessly integrate Model Context Protocol (MCP) servers:

```rust
use indubitably_rust_agent_sdk::tools::mcp::MCPClient;

// Connect to an MCP server
let mcp_client = MCPClient::new("path/to/mcp/server");
let tools = mcp_client.list_tools().await?;

let agent = Agent::new().with_tools(tools);
let result = agent.run("Use the available tools to help me").await?;
```

### Multiple Model Providers

Support for various model providers:

```rust
use indubitably_rust_agent_sdk::{Agent, models::{OpenAIModel, AnthropicModel, OllamaModel}};

// OpenAI
let openai_model = OpenAIModel::new()
    .with_api_key("your-api-key")
    .with_model_id("gpt-4");
let agent = Agent::with_model(Box::new(openai_model));

// Anthropic
let anthropic_model = AnthropicModel::new()
    .with_api_key("your-api-key")
    .with_model_id("claude-3-sonnet-20240229");
let agent = Agent::with_model(Box::new(anthropic_model));

// Ollama (local)
let ollama_model = OllamaModel::new()
    .with_host("http://localhost:11434")
    .with_model_id("llama3");
let agent = Agent::with_model(Box::new(ollama_model));
```

### Multi-Agent Systems

Build complex multi-agent workflows:

```rust
use indubitably_rust_agent_sdk::multiagent::{AgentGraph, AgentNode, AgentEdge};

let mut graph = AgentGraph::new();

// Add agents to the graph
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

// Connect agents
graph.add_edge(AgentEdge {
    source: "researcher".to_string(),
    target: "writer".to_string(),
    condition: None,
});
```

## CLI Usage

The SDK includes a command-line interface for quick testing and experimentation:

```bash
# Start a chat session
indubitably-cli chat "Hello, how are you?"

# Use a specific model
indubitably-cli chat -m openai "What's the weather like?"

# List available tools
indubitably-cli tools

# Show version
indubitably-cli version
```

## Architecture

The SDK is built with a modular architecture:

- **Core Types**: Fundamental data structures for messages, content, and tools
- **Models**: Abstract interface and implementations for various AI providers
- **Agents**: Main agent implementation with conversation management
- **Tools**: Tool registry, execution engine, and decorators
- **Session Management**: Persistence and retrieval of conversation history
- **Telemetry**: Metrics, tracing, and observability
- **Multi-Agent**: Systems for building agent networks and workflows

## Development

```bash
# Clone the repository
git clone https://github.com/indubitably-ai/indubitably-rust-agent-sdk.git
cd indubitably-rust-agent-sdk

# Install dependencies
cargo build

# Run tests
cargo test

# Run integration tests
cargo test --test integration

# Format code
cargo fmt

# Lint code
cargo clippy

# Build documentation
cargo doc --open
```

## Contributing ❤️

We welcome contributions! See our [Contributing Guide](CONTRIBUTING.md) for details on:
- Reporting bugs & features
- Development setup
- Contributing via Pull Requests
- Code of Conduct
- Reporting of security issues

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Security

See [CONTRIBUTING](CONTRIBUTING.md#security-issue-notifications) for more information.

## Migration from Python

This Rust SDK is a complete rewrite of the Python SDK, maintaining the same API design and functionality while leveraging Rust's performance and safety features. Key differences:

- **Async by default**: All operations are async using Tokio
- **Strong typing**: Full type safety with Rust's type system
- **Performance**: Native performance without Python overhead
- **Memory safety**: Rust's ownership model prevents common memory issues
- **Tool ecosystem**: Integration with Rust's rich ecosystem of libraries

## Roadmap

- [ ] Full MCP server implementation
- [ ] Advanced conversation management
- [ ] Streaming support for all models
- [ ] Enhanced tool ecosystem
- [ ] Performance optimizations
- [ ] WebAssembly support
- [ ] Cloud deployment tools

