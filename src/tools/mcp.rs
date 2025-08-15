//! MCP (Model Context Protocol) client for the SDK.
//! 
//! This module provides functionality for connecting to MCP servers
//! and using their tools.

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use tokio::process::Command;

use crate::types::{IndubitablyResult, IndubitablyError, ToolSpec};
use super::registry::Tool;

/// Configuration for an MCP client.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MCPClientConfig {
    /// The command to run the MCP server.
    pub command: String,
    /// The arguments for the command.
    pub args: Vec<String>,
    /// The working directory for the server.
    pub working_directory: Option<String>,
    /// Environment variables for the server.
    pub environment: HashMap<String, String>,
    /// Connection timeout in seconds.
    pub timeout_seconds: u64,
}

impl Default for MCPClientConfig {
    fn default() -> Self {
        Self {
            command: "uvx".to_string(),
            args: vec!["awslabs.aws-documentation-mcp-server@latest".to_string()],
            working_directory: None,
            environment: HashMap::new(),
            timeout_seconds: 30,
        }
    }
}

impl MCPClientConfig {
    /// Create a new MCP client configuration.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the command to run.
    pub fn with_command(mut self, command: &str) -> Self {
        self.command = command.to_string();
        self
    }

    /// Set the command arguments.
    pub fn with_args(mut self, args: Vec<String>) -> Self {
        self.args = args;
        self
    }

    /// Set the working directory.
    pub fn with_working_directory(mut self, working_directory: &str) -> Self {
        self.working_directory = Some(working_directory.to_string());
        self
    }

    /// Add an environment variable.
    pub fn with_environment(mut self, key: &str, value: &str) -> Self {
        self.environment.insert(key.to_string(), value.to_string());
        self
    }

    /// Set the connection timeout.
    pub fn with_timeout(mut self, timeout_seconds: u64) -> Self {
        self.timeout_seconds = timeout_seconds;
        self
    }
}

/// An MCP client that can connect to MCP servers.
#[derive(Debug)]
pub struct MCPClient {
    config: MCPClientConfig,
    server_process: Option<tokio::process::Child>,
    tools: Vec<Tool>,
}

impl MCPClient {
    /// Create a new MCP client.
    pub fn new() -> Self {
        Self {
            config: MCPClientConfig::default(),
            server_process: None,
            tools: Vec::new(),
        }
    }

    /// Create a new MCP client with the given configuration.
    pub fn with_config(config: MCPClientConfig) -> Self {
        Self {
            config,
            server_process: None,
            tools: Vec::new(),
        }
    }

    /// Connect to the MCP server.
    pub async fn connect(&mut self) -> IndubitablyResult<()> {
        // For now, this is a placeholder implementation
        // In a real implementation, you would:
        // 1. Start the server process
        // 2. Establish communication (stdio, TCP, etc.)
        // 3. Perform handshake
        // 4. Discover available tools
        
        tracing::info!("Connecting to MCP server: {} {:?}", self.config.command, self.config.args);
        
        // Simulate connection delay
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        
        // For now, just create some placeholder tools
        self.tools = vec![
            Tool::new(
                "mcp_placeholder_1",
                "Placeholder MCP tool 1",
                Arc::new(|_| Ok(serde_json::Value::String("MCP tool 1 result".to_string()))),
            ),
            Tool::new(
                "mcp_placeholder_2",
                "Placeholder MCP tool 2",
                Arc::new(|_| Ok(serde_json::Value::String("MCP tool 2 result".to_string()))),
            ),
        ];
        
        Ok(())
    }

    /// Disconnect from the MCP server.
    pub async fn disconnect(&mut self) -> IndubitablyResult<()> {
        if let Some(mut process) = self.server_process.take() {
            let _ = process.kill().await;
        }
        
        self.tools.clear();
        Ok(())
    }

    /// Check if the client is connected.
    pub fn is_connected(&self) -> bool {
        !self.tools.is_empty()
    }

    /// Get the available tools from the MCP server.
    pub async fn list_tools(&self) -> IndubitablyResult<Vec<ToolSpec>> {
        if !self.is_connected() {
            return Err(IndubitablyError::McpError(
                crate::types::McpError::ClientFailed(
                    "MCP client not connected".to_string(),
                ),
            ));
        }
        
        let specs: Vec<ToolSpec> = self.tools.iter().map(|tool| tool.spec()).collect();
        Ok(specs)
    }

    /// Get the tools as Tool objects.
    pub async fn get_tools(&self) -> IndubitablyResult<Vec<Tool>> {
        if !self.is_connected() {
            return Err(IndubitablyError::McpError(
                crate::types::McpError::ClientFailed(
                    "MCP client not connected".to_string(),
                ),
            ));
        }
        
        Ok(self.tools.clone())
    }

    /// Execute a tool by name.
    pub async fn execute_tool(&self, tool_name: &str, input: serde_json::Value) -> IndubitablyResult<serde_json::Value> {
        if !self.is_connected() {
            return Err(IndubitablyError::McpError(
                crate::types::McpError::ClientFailed(
                    "MCP client not connected".to_string(),
                ),
            ));
        }
        
        let tool = self.tools.iter().find(|t| t.name == tool_name).ok_or_else(|| {
            IndubitablyError::McpError(
                crate::types::McpError::ClientFailed(
                    format!("Tool '{}' not found", tool_name),
                ),
            )
        })?;
        
        tool.execute(input)
    }

    /// Get information about the MCP server.
    pub async fn get_server_info(&self) -> IndubitablyResult<MCPServerInfo> {
        if !self.is_connected() {
            return Err(IndubitablyError::McpError(
                crate::types::McpError::ClientFailed(
                    "MCP client not connected".to_string(),
                ),
            ));
        }
        
        // For now, return placeholder info
        Ok(MCPServerInfo {
            name: "Placeholder MCP Server".to_string(),
            version: "1.0.0".to_string(),
            description: "A placeholder MCP server for testing".to_string(),
            capabilities: vec!["tools".to_string(), "resources".to_string()],
        })
    }
}

impl Drop for MCPClient {
    fn drop(&mut self) {
        // Ensure we clean up the server process
        if let Some(mut process) = self.server_process.take() {
            let _ = process.kill();
        }
    }
}

/// Information about an MCP server.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MCPServerInfo {
    /// The name of the server.
    pub name: String,
    /// The version of the server.
    pub version: String,
    /// A description of the server.
    pub description: String,
    /// The capabilities of the server.
    pub capabilities: Vec<String>,
}

/// A builder for creating MCP clients with common configurations.
pub struct MCPClientBuilder {
    config: MCPClientConfig,
}

impl MCPClientBuilder {
    /// Create a new MCP client builder.
    pub fn new() -> Self {
        Self {
            config: MCPClientConfig::default(),
        }
    }

    /// Set the command to run.
    pub fn command(mut self, command: &str) -> Self {
        self.config.command = command.to_string();
        self
    }

    /// Set the command arguments.
    pub fn args(mut self, args: Vec<String>) -> Self {
        self.config.args = args;
        self
    }

    /// Set the working directory.
    pub fn working_directory(mut self, working_directory: &str) -> Self {
        self.config.working_directory = Some(working_directory.to_string());
        self
    }

    /// Add an environment variable.
    pub fn environment(mut self, key: &str, value: &str) -> Self {
        self.config.environment.insert(key.to_string(), value.to_string());
        self
    }

    /// Set the connection timeout.
    pub fn timeout(mut self, timeout_seconds: u64) -> Self {
        self.config.timeout_seconds = timeout_seconds;
        self
    }

    /// Build the MCP client.
    pub fn build(self) -> MCPClient {
        MCPClient::with_config(self.config)
    }
}

impl Default for MCPClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// Import Arc for the placeholder tools
use std::sync::Arc;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mcp_client_config() {
        let config = MCPClientConfig::new()
            .with_command("python")
            .with_args(vec!["-m".to_string(), "mcp_server".to_string()])
            .with_working_directory("/tmp")
            .with_environment("DEBUG", "1")
            .with_timeout(60);

        assert_eq!(config.command, "python");
        assert_eq!(config.args, vec!["-m", "mcp_server"]);
        assert_eq!(config.working_directory, Some("/tmp".to_string()));
        assert_eq!(config.environment.get("DEBUG"), Some(&"1".to_string()));
        assert_eq!(config.timeout_seconds, 60);
    }

    #[tokio::test]
    async fn test_mcp_client_builder() {
        let client = MCPClientBuilder::new()
            .command("node")
            .args(vec!["server.js".to_string()])
            .working_directory("./server")
            .environment("NODE_ENV", "production")
            .timeout(120)
            .build();

        assert_eq!(client.config.command, "node");
        assert_eq!(client.config.args, vec!["server.js"]);
        assert_eq!(client.config.working_directory, Some("./server".to_string()));
        assert_eq!(client.config.environment.get("NODE_ENV"), Some(&"production".to_string()));
        assert_eq!(client.config.timeout_seconds, 120);
    }

    #[tokio::test]
    async fn test_mcp_client_lifecycle() {
        let mut client = MCPClient::new();
        
        // Initially not connected
        assert!(!client.is_connected());
        
        // Connect
        let result = client.connect().await;
        assert!(result.is_ok());
        assert!(client.is_connected());
        
        // List tools
        let tools = client.list_tools().await;
        assert!(tools.is_ok());
        assert!(!tools.unwrap().is_empty());
        
        // Execute tool
        let result = client.execute_tool("mcp_placeholder_1", serde_json::Value::Null).await;
        assert!(result.is_ok());
        
        // Disconnect
        let result = client.disconnect().await;
        assert!(result.is_ok());
        assert!(!client.is_connected());
    }

    #[tokio::test]
    async fn test_mcp_client_server_info() {
        let mut client = MCPClient::new();
        client.connect().await.unwrap();
        
        let info = client.get_server_info().await;
        assert!(info.is_ok());
        
        let info = info.unwrap();
        assert_eq!(info.name, "Placeholder MCP Server");
        assert_eq!(info.version, "1.0.0");
        assert!(!info.capabilities.is_empty());
    }
}
