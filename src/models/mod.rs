//! Model implementations for the SDK.
//! 
//! This module contains the abstract Model trait and concrete
//! implementations for various model providers.

pub mod model;
pub mod bedrock;
pub mod openai;
pub mod anthropic;
pub mod ollama;

pub use model::Model;
pub use bedrock::BedrockModel;
pub use openai::OpenAIModel;
pub use anthropic::AnthropicModel;
pub use ollama::OllamaModel;

// Re-export commonly used types
pub use model::{ModelConfig, ModelResponse, ModelStreamResponse};
