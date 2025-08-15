//! Ollama model implementation for the SDK.
//! 
//! This module provides integration with Ollama for
//! accessing local models.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::model::{Model, ModelConfig, ModelResponse, ModelUsage, ModelStreamResponse};
use crate::types::{Messages, ToolSpec, StreamEvent, IndubitablyResult};

/// Default Ollama host.
pub const DEFAULT_OLLAMA_HOST: &str = "http://localhost:11434";

/// Default Ollama model ID.
pub const DEFAULT_OLLAMA_MODEL_ID: &str = "llama3";

/// Configuration specific to Ollama models.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OllamaConfig {
    /// The Ollama host URL.
    pub host: String,
    /// The model ID to use.
    pub model_id: String,
    /// The temperature for generation.
    pub temperature: Option<f32>,
    /// The maximum number of tokens to generate.
    pub max_tokens: Option<u32>,
    /// The top-p value for nucleus sampling.
    pub top_p: Option<f32>,
    /// Whether to enable streaming.
    pub streaming: Option<bool>,
    /// Additional Ollama-specific configuration.
    pub extra: HashMap<String, serde_json::Value>,
}

impl Default for OllamaConfig {
    fn default() -> Self {
        Self {
            host: DEFAULT_OLLAMA_HOST.to_string(),
            model_id: DEFAULT_OLLAMA_MODEL_ID.to_string(),
            temperature: Some(0.7),
            max_tokens: Some(4096),
            top_p: Some(1.0),
            streaming: Some(false),
            extra: HashMap::new(),
        }
    }
}

impl OllamaConfig {
    /// Create a new Ollama configuration.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the host URL.
    pub fn with_host(mut self, host: &str) -> Self {
        self.host = host.to_string();
        self
    }

    /// Set the model ID.
    pub fn with_model_id(mut self, model_id: &str) -> Self {
        self.model_id = model_id.to_string();
        self
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

    /// Enable or disable streaming.
    pub fn with_streaming(mut self, streaming: bool) -> Self {
        self.streaming = Some(streaming);
        self
    }

    /// Add extra configuration.
    pub fn with_extra(mut self, key: &str, value: serde_json::Value) -> Self {
        self.extra.insert(key.to_string(), value);
        self
    }
}

/// The Ollama model implementation.
#[derive(Debug)]
pub struct OllamaModel {
    config: ModelConfig,
    ollama_config: OllamaConfig,
}

impl OllamaModel {
    /// Create a new Ollama model.
    pub fn new() -> Self {
        Self {
            config: ModelConfig::default(),
            ollama_config: OllamaConfig::default(),
        }
    }

    /// Create a new Ollama model with the given configuration.
    pub fn with_config(ollama_config: OllamaConfig) -> Self {
        Self {
            config: ModelConfig::new(&ollama_config.model_id)
                .with_temperature(ollama_config.temperature.unwrap_or(0.7))
                .with_max_tokens(ollama_config.max_tokens.unwrap_or(4096))
                .with_top_p(ollama_config.top_p.unwrap_or(1.0))
                .with_streaming(ollama_config.streaming.unwrap_or(false)),
            ollama_config,
        }
    }
}

#[async_trait]
impl Model for OllamaModel {
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
        // TODO: Implement actual Ollama API integration
        Ok(ModelResponse {
            content: "This is a mock response from Ollama. Actual integration coming soon.".to_string(),
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
        // TODO: Implement actual Ollama streaming
        use tokio_stream::wrappers::ReceiverStream;
        use tokio::sync::mpsc;

        let (tx, rx) = mpsc::channel(100);
        
        tokio::spawn(async move {
            let events = vec![
                StreamEvent::message_start(),
                StreamEvent::content_block_start(vec![crate::types::streaming::StreamContent::text("Mock Ollama")]),
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
        Err(crate::types::IndubitablyError::ModelError(
            crate::types::ModelError::InvalidResponseFormat(
                "Ollama model does not support structured output yet".to_string(),
            ),
        ))
    }
}

impl Default for OllamaModel {
    fn default() -> Self {
        Self::new()
    }
}
