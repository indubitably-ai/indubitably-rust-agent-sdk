//! Exception and error types for the SDK.
//! 
//! This module defines all the error types used throughout the SDK
//! for consistent error handling and reporting.

use thiserror::Error;

/// Errors that can occur during network operations.
#[derive(Error, Debug)]
pub enum NetworkError {
    /// A timeout occurred.
    #[error("Timeout: {0}")]
    Timeout(String),
    
    /// A connection error occurred.
    #[error("Connection error: {0}")]
    Connection(String),
    
    /// An HTTP error occurred.
    #[error("HTTP error: {0}")]
    Http(String),
    
    /// A general network error occurred.
    #[error("Network error: {0}")]
    General(String),
}

/// The main error type for the Indubitably Rust Agent SDK.
#[derive(Error, Debug)]
pub enum IndubitablyError {
    /// An error occurred during model interaction.
    #[error("Model error: {0}")]
    ModelError(#[from] ModelError),

    /// An error occurred during tool execution.
    #[error("Tool error: {0}")]
    ToolError(#[from] ToolError),

    /// An error occurred during session management.
    #[error("Session error: {0}")]
    SessionError(#[from] SessionError),

    /// An error occurred during streaming.
    #[error("Streaming error: {0}")]
    StreamingError(#[from] StreamingError),

    /// An error occurred during event loop execution.
    #[error("Event loop error: {0}")]
    EventLoopError(#[from] EventLoopError),

    /// An error occurred during conversation management.
    #[error("Conversation error: {0}")]
    ConversationError(#[from] ConversationError),

    /// An error occurred during telemetry operations.
    #[error("Telemetry error: {0}")]
    TelemetryError(#[from] TelemetryError),

    /// An error occurred during hook execution.
    #[error("Hook error: {0}")]
    HookError(#[from] HookError),

    /// An error occurred during MCP operations.
    #[error("MCP error: {0}")]
    McpError(#[from] McpError),

    /// A validation error occurred.
    #[error("Validation error: {0}")]
    ValidationError(String),

    /// A configuration error occurred.
    #[error("Configuration error: {0}")]
    ConfigurationError(String),

    /// An authentication error occurred.
    #[error("Authentication error: {0}")]
    AuthenticationError(String),

    /// A network error occurred.
    #[error("Network error: {0}")]
    NetworkError(String),

    /// A timeout error occurred.
    #[error("Timeout error: {0}")]
    TimeoutError(String),

    /// An internal error occurred.
    #[error("Internal error: {0}")]
    InternalError(String),
}

/// Errors that can occur during model interactions.
#[derive(Error, Debug)]
pub enum ModelError {
    /// The model is throttling requests.
    #[error("Model throttled: {0}")]
    ModelThrottled(String),

    /// The model response format is invalid.
    #[error("Invalid response format: {0}")]
    InvalidResponseFormat(String),

    /// The model request failed.
    #[error("Request failed: {0}")]
    RequestFailed(String),

    /// The model is not available.
    #[error("Model not available: {0}")]
    ModelNotAvailable(String),

    /// The model configuration is invalid.
    #[error("Invalid configuration: {0}")]
    InvalidConfiguration(String),

    /// The model quota has been exceeded.
    #[error("Quota exceeded: {0}")]
    QuotaExceeded(String),

    /// The model context window overflowed.
    #[error("Context window overflow: {0}")]
    ContextWindowOverflow(String),
}

/// Errors that can occur during tool execution.
#[derive(Error, Debug)]
pub enum ToolError {
    /// The tool was not found.
    #[error("Tool not found: {0}")]
    ToolNotFound(String),

    /// The tool execution failed.
    #[error("Tool execution failed: {0}")]
    ExecutionFailed(String),

    /// The tool input is invalid.
    #[error("Invalid tool input: {0}")]
    InvalidInput(String),

    /// The tool output is invalid.
    #[error("Invalid tool output: {0}")]
    InvalidOutput(String),

    /// The tool is not available.
    #[error("Tool not available: {0}")]
    ToolNotAvailable(String),

    /// The tool timed out.
    #[error("Tool timeout: {0}")]
    Timeout(String),
}

/// Errors that can occur during session management.
#[derive(Error, Debug)]
pub enum SessionError {
    /// The session was not found.
    #[error("Session not found: {0}")]
    SessionNotFound(String),

    /// The session creation failed.
    #[error("Session creation failed: {0}")]
    CreationFailed(String),

    /// The session update failed.
    #[error("Session update failed: {0}")]
    UpdateFailed(String),

    /// The session deletion failed.
    #[error("Session deletion failed: {0}")]
    DeletionFailed(String),

    /// The session storage failed.
    #[error("Session storage failed: {0}")]
    StorageFailed(String),
}

/// Errors that can occur during streaming.
#[derive(Error, Debug)]
pub enum StreamingError {
    /// The stream was interrupted.
    #[error("Stream interrupted: {0}")]
    StreamInterrupted(String),

    /// The stream format is invalid.
    #[error("Invalid stream format: {0}")]
    InvalidFormat(String),

    /// The stream connection failed.
    #[error("Stream connection failed: {0}")]
    ConnectionFailed(String),

    /// The stream buffer overflowed.
    #[error("Stream buffer overflow: {0}")]
    BufferOverflow(String),
}

/// Errors that can occur during event loop execution.
#[derive(Error, Debug)]
pub enum EventLoopError {
    /// The event loop cycle failed.
    #[error("Event loop cycle failed: {0}")]
    CycleFailed(String),

    /// The tool execution in the event loop failed.
    #[error("Tool execution failed: {0}")]
    ToolExecutionFailed(String),

    /// The event loop state is invalid.
    #[error("Invalid event loop state: {0}")]
    InvalidState(String),

    /// The event loop exceeded maximum iterations.
    #[error("Maximum iterations exceeded: {0}")]
    MaxIterationsExceeded(String),
}

/// Errors that can occur during conversation management.
#[derive(Error, Debug)]
pub enum ConversationError {
    /// The conversation manager failed.
    #[error("Conversation manager failed: {0}")]
    ManagerFailed(String),

    /// The conversation history is invalid.
    #[error("Invalid conversation history: {0}")]
    InvalidHistory(String),

    /// The conversation context overflowed.
    #[error("Context overflow: {0}")]
    ContextOverflow(String),

    /// The conversation summarization failed.
    #[error("Summarization failed: {0}")]
    SummarizationFailed(String),
}

/// Errors that can occur during telemetry operations.
#[derive(Error, Debug)]
pub enum TelemetryError {
    /// The metrics collection failed.
    #[error("Metrics collection failed: {0}")]
    MetricsFailed(String),

    /// The tracing failed.
    #[error("Tracing failed: {0}")]
    TracingFailed(String),

    /// The telemetry configuration is invalid.
    #[error("Invalid telemetry configuration: {0}")]
    InvalidConfiguration(String),
}

/// Errors that can occur during hook execution.
#[derive(Error, Debug)]
pub enum HookError {
    /// The hook execution failed.
    #[error("Hook execution failed: {0}")]
    ExecutionFailed(String),

    /// The hook registration failed.
    #[error("Hook registration failed: {0}")]
    RegistrationFailed(String),

    /// The hook provider is invalid.
    #[error("Invalid hook provider: {0}")]
    InvalidProvider(String),
}

/// Errors that can occur during MCP operations.
#[derive(Error, Debug)]
pub enum McpError {
    /// The MCP client failed.
    #[error("MCP client failed: {0}")]
    ClientFailed(String),

    /// The MCP server failed.
    #[error("MCP server failed: {0}")]
    ServerFailed(String),

    /// The MCP protocol error.
    #[error("MCP protocol error: {0}")]
    ProtocolError(String),

    /// The MCP connection failed.
    #[error("MCP connection failed: {0}")]
    ConnectionFailed(String),
}

impl From<String> for IndubitablyError {
    fn from(err: String) -> Self {
        IndubitablyError::InternalError(err)
    }
}

impl From<&str> for IndubitablyError {
    fn from(err: &str) -> Self {
        IndubitablyError::InternalError(err.to_string())
    }
}

impl From<std::io::Error> for IndubitablyError {
    fn from(err: std::io::Error) -> Self {
        IndubitablyError::NetworkError(err.to_string())
    }
}

impl From<serde_json::Error> for IndubitablyError {
    fn from(err: serde_json::Error) -> Self {
        IndubitablyError::ValidationError(err.to_string())
    }
}

impl From<tokio::time::error::Elapsed> for IndubitablyError {
    fn from(err: tokio::time::error::Elapsed) -> Self {
        IndubitablyError::NetworkError(format!("Timeout: {}", err))
    }
}

/// A result type that uses the main error type.
pub type IndubitablyResult<T> = Result<T, IndubitablyError>;
