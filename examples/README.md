# Indubitably Rust Agent SDK Examples

This directory contains comprehensive examples demonstrating how to use the Indubitably Rust Agent SDK to build AI-powered chat assistants with various capabilities.

## 🚀 Quick Start

1. **Navigate to the examples directory:**
   ```bash
   cd examples
   ```

2. **Build all examples:**
   ```bash
   cargo build
   ```

3. **Run any example:**
   ```bash
   cargo run --bin simple_chat_assistant
   cargo run --bin multi_agent_system
   cargo run --bin web_chat_interface
   ```

## 📚 Available Examples

### 1. Simple Chat Assistant (`simple_chat_assistant.rs`)

A basic command-line chat assistant that demonstrates:
- Creating agents with custom configurations
- Implementing mock tools (web browser, calculator)
- Interactive chat sessions
- Tool integration and execution

**Features:**
- Web browsing simulation with mock data
- Mathematical calculations
- Interactive command-line interface
- Session management

**Usage:**
```bash
cargo run --bin simple_chat_assistant
```

**Example interactions:**
- "What's the weather like today?"
- "Can you calculate 15 * 23 for me?"
- "Search for information about AI"

### 2. Multi-Agent System (`multi_agent_system.rs`)

A sophisticated system demonstrating multiple specialized agents working together:
- **Research Agent**: Gathers information using web tools
- **Analysis Agent**: Analyzes and summarizes data
- **Creative Agent**: Creates content based on analyzed information

**Features:**
- Agent specialization and role-based responses
- Workflow orchestration (Research → Analysis → Creative)
- Intelligent routing based on user intent
- Collaborative problem-solving

**Usage:**
```bash
cargo run --bin multi_agent_system
```

**Example interactions:**
- "Research the latest AI developments"
- "Analyze the research findings"
- "Create a story about AI technology"

### 3. Web Chat Interface (`web_chat_interface.rs`)

A modern web-based chat interface using Actix-web:
- Beautiful, responsive web UI
- Real-time chat capabilities
- Session management
- RESTful API endpoints

**Features:**
- Modern web interface with CSS animations
- Session persistence
- Real-time chat updates
- Health check endpoints
- Static file serving

**Usage:**
```bash
cargo run --bin web_chat_interface
```

Then open your browser to `http://localhost:8080`

## 🛠️ Tool Implementations

All examples include mock implementations of useful tools:

### Web Browser Tool
- Simulates web browsing with mock data
- Supports different URL patterns (news, weather, search)
- Returns structured content with metadata

### Calculator Tool
- Basic mathematical operations (add, subtract, multiply, divide)
- Error handling for edge cases
- Structured output with operation details

### Text Analyzer Tool
- Content analysis and summarization
- Key insights extraction
- Confidence scoring

### Content Generator Tool
- Creative content creation
- Multiple output formats (stories, presentations, articles)
- Tone customization

## 🔧 Customization

### Adding New Tools

To add a new tool to any example:

```rust
fn create_custom_tool() -> Tool {
    let function = Arc::new(|input: Value| {
        // Your tool logic here
        let result = process_input(input);
        Ok(json!({ "result": result }))
    });
    
    Tool::new(
        "custom_tool",
        "Description of what your tool does",
        function
    ).with_metadata(
        ToolMetadata::new()
            .with_input_schema(json!({
                "type": "object",
                "properties": {
                    "input_field": {"type": "string"}
                },
                "required": ["input_field"]
            }))
    )
}
```

### Modifying Agent Behavior

Customize agent behavior by modifying the system prompt:

```rust
let config = AgentConfig::new()
    .with_name("Custom Agent")
    .with_system_prompt(
        "You are a specialized agent that... [your custom instructions]"
    )
    .with_tools(create_tool_specs());
```

## 🏗️ Architecture Patterns

### Single Agent Pattern
- Simple, focused functionality
- Direct tool execution
- Suitable for basic chat applications

### Multi-Agent Pattern
- Specialized agents for different tasks
- Workflow orchestration
- Complex problem decomposition
- Scalable architecture

### Web Service Pattern
- HTTP-based communication
- Session management
- Stateless design
- Easy integration with frontend applications

## 🧪 Testing the Examples

### Unit Tests
```bash
cargo test
```

### Integration Tests
```bash
cargo test --test integration_test
```

### Manual Testing
1. Run the example
2. Follow the interactive prompts
3. Test different types of queries
4. Verify tool execution

## 📖 Learning Path

1. **Start with** `simple_chat_assistant` to understand basic concepts
2. **Progress to** `multi_agent_system` to learn about agent collaboration
3. **Finish with** `web_chat_interface` to see full-stack integration

## 🚨 Common Issues

### Build Errors
- Ensure you're in the examples directory
- Check that the main SDK is built: `cargo build` from the root
- Verify Rust toolchain: `rustc --version`

### Runtime Errors
- Check that all dependencies are available
- Verify port availability for web examples
- Ensure proper permissions for file operations

### Tool Execution Issues
- Verify tool schemas match implementation
- Check input validation in tool functions
- Review error handling in tool implementations

## 🔮 Next Steps

After exploring these examples:

1. **Extend the tools** with real implementations
2. **Add new agent types** for specific domains
3. **Integrate with external APIs** for real data
4. **Build custom workflows** for your use case
5. **Deploy to production** with proper error handling

## 📚 Additional Resources

- [SDK Documentation](../README.md)
- [API Reference](../src/lib.rs)
- [Contributing Guidelines](../CONTRIBUTING.md)
- [Issue Tracker](https://github.com/your-repo/issues)

## 🤝 Contributing

We welcome contributions! To add new examples:

1. Follow the existing code style
2. Include comprehensive documentation
3. Add appropriate tests
4. Update this README
5. Submit a pull request

---

**Happy coding with the Indubitably Rust Agent SDK! 🦀✨**
