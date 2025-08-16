# Examples for Indubitably Rust Agent SDK

This directory contains small, focused Rust examples that demonstrate how to use the SDK. Each example is self-contained and can be run with Cargo.

- Requirements: Rust 1.70+ and Cargo

## How to run

- Run a specific example:

```bash
cargo run --example chat_basic
```

- List all available examples:

```bash
cargo run --example mcp_list_tools
cargo run --example multiagent_graph
```

## Examples

- chat_basic: Create an `Agent` with a mock model and send a single message.
- mcp_list_tools: Connect to the placeholder MCP client and list available tools.
- multiagent_graph: Build a tiny multi-agent graph with two nodes and one edge.

## Tests

The repository includes tests that run these examples to ensure they continue to build and execute:

```bash
cargo test --tests
```

These tests help keep the examples working over time as part of the library.