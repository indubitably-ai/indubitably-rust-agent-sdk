//! Core model trait and implementations for the SDK.
//! 
//! This module provides the abstract `Model` trait that all
//! model providers must implement, along with common types and utilities.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::pin::Pin;
use tokio_stream::Stream;

use crate::types::{Messages, ToolSpec, IndubitablyResult, StreamEvent};

/// Configuration for a model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfig {
    /// The model ID.
    pub model_id: String,
    /// The temperature for generation.
    pub temperature: Option<f32>,
    /// The maximum number of tokens to generate.
    pub max_tokens: Option<u32>,
    /// The top-p value for nucleus sampling.
    pub top_p: Option<f32>,
    /// The top-k value for top-k sampling.
    pub top_k: Option<u32>,
    /// Whether to enable streaming.
    pub streaming: bool,
    /// Additional configuration options.
    pub extra: HashMap<String, serde_json::Value>,
}

impl Default for ModelConfig {
    fn default() -> Self {
        Self {
            model_id: "default".to_string(),
            temperature: Some(0.7),
            max_tokens: Some(4096),
            top_p: Some(1.0),
            top_k: Some(250),
            streaming: false,
            extra: HashMap::new(),
        }
    }
}

impl ModelConfig {
    /// Create a new model configuration.
    pub fn new(model_id: &str) -> Self {
        Self {
            model_id: model_id.to_string(),
            ..Default::default()
        }
    }

    /// Set the temperature.
    pub fn with_temperature(mut self, temperature: f32) -> Self {
        self.temperature = Some(temperature);
        self
    }

    /// Set the maximum tokens.
    pub fn with_max_tokens(mut self, max_tokens: u32) -> Self {
        self.max_tokens = Some(max_tokens);
        self
    }

    /// Set the top-p value.
    pub fn with_top_p(mut self, top_p: f32) -> Self {
        self.top_p = Some(top_p);
        self
    }

    /// Set the top-k value.
    pub fn with_top_k(mut self, top_k: u32) -> Self {
        self.top_k = Some(top_k);
        self
    }

    /// Enable or disable streaming.
    pub fn with_streaming(mut self, streaming: bool) -> Self {
        self.streaming = streaming;
        self
    }

    /// Add extra configuration.
    pub fn with_extra(mut self, key: &str, value: serde_json::Value) -> Self {
        self.extra.insert(key.to_string(), value);
        self
    }
}

/// Response from a model generation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelResponse {
    /// The generated content.
    pub content: String,
    /// Token usage information.
    pub usage: Option<ModelUsage>,
    /// Additional metadata.
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Token usage information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelUsage {
    /// Number of input tokens.
    pub input_tokens: u32,
    /// Number of output tokens.
    pub output_tokens: u32,
    /// Total number of tokens.
    pub total_tokens: u32,
}

/// Stream response from a model.
pub type ModelStreamResponse = Pin<Box<dyn Stream<Item = IndubitablyResult<StreamEvent>> + Send>>;

/// The core model trait that all model providers must implement.
#[async_trait]
pub trait Model: Send + Sync {
    /// Get the model configuration.
    fn config(&self) -> &ModelConfig;

    /// Update the model configuration.
    fn update_config(&mut self, config: ModelConfig);

    /// Get a mutable reference to the model configuration.
    fn config_mut(&mut self) -> &mut ModelConfig;

    /// Generate a response from the model.
    async fn generate(
        &self,
        messages: &Messages,
        tool_specs: Option<&[ToolSpec]>,
        system_prompt: Option<&str>,
    ) -> IndubitablyResult<ModelResponse>;

    /// Stream a response from the model.
    async fn stream(
        &self,
        messages: &Messages,
        tool_specs: Option<&[ToolSpec]>,
        system_prompt: Option<&str>,
    ) -> IndubitablyResult<ModelStreamResponse>;

    /// Get structured output from the model.
    async fn structured_output(
        &self,
        output_model: &str,
        messages: &Messages,
        system_prompt: Option<&str>,
    ) -> IndubitablyResult<serde_json::Value>;

    /// Check if the model supports streaming.
    fn supports_streaming(&self) -> bool {
        self.config().streaming
    }

    /// Get the model ID.
    fn model_id(&self) -> &str {
        &self.config().model_id
    }

    /// Get the temperature setting.
    fn temperature(&self) -> Option<f32> {
        self.config().temperature
    }

    /// Get the maximum tokens setting.
    fn max_tokens(&self) -> Option<u32> {
        self.config().max_tokens
    }
}

/// A mock model for testing purposes.
#[derive(Debug, Clone)]
pub struct MockModel {
    config: ModelConfig,
}

impl MockModel {
    /// Create a new mock model.
    pub fn new() -> Self {
        Self {
            config: ModelConfig::new("mock"),
        }
    }

    /// Create a new mock model with the given configuration.
    pub fn with_config(config: ModelConfig) -> Self {
        Self { config }
    }
}

#[async_trait]
impl Model for MockModel {
    fn config(&self) -> &ModelConfig {
        &self.config
    }

    fn update_config(&mut self, config: ModelConfig) {
        self.config = config;
    }

    fn config_mut(&mut self) -> &mut ModelConfig {
        &mut self.config
    }

    async fn generate(
        &self,
        _messages: &Messages,
        _tool_specs: Option<&[ToolSpec]>,
        _system_prompt: Option<&str>,
    ) -> IndubitablyResult<ModelResponse> {
        Ok(ModelResponse {
            content: "This is a mock response from the mock model.".to_string(),
            usage: Some(ModelUsage {
                input_tokens: 10,
                output_tokens: 15,
                total_tokens: 25,
            }),
            metadata: HashMap::new(),
        })
    }

    async fn stream(
        &self,
        _messages: &Messages,
        _tool_specs: Option<&[ToolSpec]>,
        _system_prompt: Option<&str>,
    ) -> IndubitablyResult<ModelStreamResponse> {
        use tokio_stream::wrappers::ReceiverStream;
        use tokio::sync::mpsc;

        let (tx, rx) = mpsc::channel(100);
        
        tokio::spawn(async move {
            let events = vec![
                StreamEvent::message_start(),
                StreamEvent::content_block_start(vec![crate::types::streaming::StreamContent::text("Mock")]),
                StreamEvent::content_block_delta(vec![crate::types::streaming::StreamContent::text(" streaming")]),
                StreamEvent::content_block_stop(),
                StreamEvent::message_stop(),
            ];

            for event in events {
                if tx.send(Ok(event)).await.is_err() {
                    break;
                }
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            }
        });

        Ok(Box::pin(ReceiverStream::new(rx)))
    }

    async fn structured_output(
        &self,
        _output_model: &str,
        _messages: &Messages,
        _system_prompt: Option<&str>,
    ) -> IndubitablyResult<serde_json::Value> {
        Ok(serde_json::json!({
            "mock": true,
            "content": "Mock structured output"
        }))
    }
}

impl Default for MockModel {
    fn default() -> Self {
        Self::new()
    }
}
