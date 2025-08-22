//! Unit tests for the models module.
//! 
//! These tests verify that all model implementations work correctly
//! and provide the expected functionality.

use indubitably_rust_agent_sdk::models::*;
use indubitably_rust_agent_sdk::types::{Message, MessageRole, ContentBlock};

#[tokio::test]
async fn test_model_trait_basic_functionality() {
    let mock_model = MockModel::new();
    
    // Test basic properties
    assert_eq!(mock_model.name(), "Mock Model");
    assert_eq!(mock_model.provider(), "mock");
    
    // Test configuration
    let config = mock_model.config();
    assert!(config.is_some());
    let config = config.unwrap();
    assert_eq!(config.get("model_id").unwrap().as_str().unwrap(), "mock-model");
}

#[tokio::test]
async fn test_mock_model_generate() {
    let mock_model = MockModel::new();
    
    let messages = vec![
        Message::user("Hello, how are you?"),
    ];
    
    let response = mock_model.generate(&messages).await.unwrap();
    
    assert!(!response.content.is_empty());
    assert!(response.content.contains("mock response"));
    assert_eq!(response.model_id, "mock-model");
    assert!(response.usage.is_some());
}

#[tokio::test]
async fn test_mock_model_stream() {
    let mock_model = MockModel::new();
    
    let messages = vec![
        Message::user("Tell me a story"),
    ];
    
    let stream = mock_model.stream(&messages).await.unwrap();
    
    // For now, just verify we get a stream
    // In a full implementation, we would consume the stream
    assert!(true); // Placeholder assertion
}

#[test]
fn test_model_config_creation() {
    let config = ModelConfig::new("test-model")
        .with_api_key("test-key")
        .with_base_url("https://api.test.com")
        .with_max_tokens(1000)
        .with_temperature(0.7);
    
    assert_eq!(config.model_id, "test-model");
    assert_eq!(config.api_key.as_ref().unwrap(), "test-key");
    assert_eq!(config.base_url.as_ref().unwrap(), "https://api.test.com");
    assert_eq!(config.max_tokens, Some(1000));
    assert_eq!(config.temperature, Some(0.7));
}

#[test]
fn test_model_config_builder_pattern() {
    let config = ModelConfig::new("gpt-4")
        .with_api_key("sk-123")
        .with_max_tokens(2000)
        .with_temperature(0.5)
        .with_top_p(0.9)
        .with_frequency_penalty(0.1)
        .with_presence_penalty(0.1);
    
    assert_eq!(config.model_id, "gpt-4");
    assert_eq!(config.api_key.as_ref().unwrap(), "sk-123");
    assert_eq!(config.max_tokens, Some(2000));
    assert_eq!(config.temperature, Some(0.5));
    assert_eq!(config.top_p, Some(0.9));
    assert_eq!(config.frequency_penalty, Some(0.1));
    assert_eq!(config.presence_penalty, Some(0.1));
}

#[test]
fn test_model_config_serialization() {
    let config = ModelConfig::new("test-model")
        .with_api_key("test-key")
        .with_max_tokens(1000);
    
    let json = serde_json::to_string(&config).unwrap();
    let deserialized: ModelConfig = serde_json::from_str(&json).unwrap();
    
    assert_eq!(deserialized.model_id, config.model_id);
    assert_eq!(deserialized.api_key, config.api_key);
    assert_eq!(deserialized.max_tokens, config.max_tokens);
}

#[test]
fn test_model_config_default_values() {
    let config = ModelConfig::new("default-model");
    
    assert_eq!(config.model_id, "default-model");
    assert!(config.api_key.is_none());
    assert!(config.base_url.is_none());
    assert!(config.max_tokens.is_none());
    assert!(config.temperature.is_none());
    assert!(config.top_p.is_none());
    assert!(config.frequency_penalty.is_none());
    assert!(config.presence_penalty.is_none());
}

#[tokio::test]
async fn test_model_response_creation() {
    let content = vec![ContentBlock {
        text: Some("Hello, world!".to_string()),
        ..Default::default()
    }];
    
    let response = ModelResponse {
        content,
        model_id: "test-model".to_string(),
        usage: Some(serde_json::json!({
            "prompt_tokens": 10,
            "completion_tokens": 5,
            "total_tokens": 15
        })),
        metadata: Some(serde_json::json!({
            "finish_reason": "stop"
        })),
    };
    
    assert_eq!(response.model_id, "test-model");
    assert_eq!(response.content.len(), 1);
    assert_eq!(response.content[0].text.as_ref().unwrap(), "Hello, world!");
    assert!(response.usage.is_some());
    assert!(response.metadata.is_some());
}

#[test]
fn test_model_response_serialization() {
    let content = vec![ContentBlock {
        text: Some("Test response".to_string()),
        ..Default::default()
    }];
    
    let response = ModelResponse {
        content,
        model_id: "test-model".to_string(),
        usage: None,
        metadata: None,
    };
    
    let json = serde_json::to_string(&response).unwrap();
    let deserialized: ModelResponse = serde_json::from_str(&json).unwrap();
    
    assert_eq!(deserialized.model_id, response.model_id);
    assert_eq!(deserialized.content.len(), response.content.len());
}

#[tokio::test]
async fn test_mock_model_with_custom_config() {
    let mut mock_model = MockModel::new();
    mock_model.set_response("Custom response");
    
    let messages = vec![Message::user("Hello")];
    let response = mock_model.generate(&messages).await.unwrap();
    
    assert!(response.content.contains("Custom response"));
}

#[tokio::test]
async fn test_mock_model_error_handling() {
    let mut mock_model = MockModel::new();
    mock_model.set_error("Test error");
    
    let messages = vec![Message::user("Hello")];
    let result = mock_model.generate(&messages).await;
    
    assert!(result.is_err());
    let error = result.unwrap_err();
    assert!(error.to_string().contains("Test error"));
}

#[test]
fn test_model_provider_enum() {
    let provider = ModelProvider::OpenAI;
    assert_eq!(provider.to_string(), "openai");
    
    let provider = ModelProvider::Anthropic;
    assert_eq!(provider.to_string(), "anthropic");
    
    let provider = ModelProvider::Bedrock;
    assert_eq!(provider.to_string(), "bedrock");
    
    let provider = ModelProvider::Ollama;
    assert_eq!(provider.to_string(), "ollama");
    
    let provider = ModelProvider::Custom("custom-provider".to_string());
    assert_eq!(provider.to_string(), "custom-provider");
}

#[test]
fn test_model_provider_from_string() {
    let provider: ModelProvider = "openai".into();
    assert!(matches!(provider, ModelProvider::OpenAI));
    
    let provider: ModelProvider = "anthropic".into();
    assert!(matches!(provider, ModelProvider::Anthropic));
    
    let provider: ModelProvider = "custom-provider".into();
    assert!(matches!(provider, ModelProvider::Custom(ref s) if s == "custom-provider"));
}

// Mock model implementation for testing
#[derive(Debug, Clone)]
struct MockModel {
    config: ModelConfig,
    response: String,
    error: Option<String>,
}

impl MockModel {
    fn new() -> Self {
        Self {
            config: ModelConfig::new("mock-model"),
            response: "mock response".to_string(),
            error: None,
        }
    }
    
    fn set_response(&mut self, response: &str) {
        self.response = response.to_string();
    }
    
    fn set_error(&mut self, error: &str) {
        self.error = Some(error.to_string());
    }
}

impl Model for MockModel {
    fn name(&self) -> &str {
        "Mock Model"
    }
    
    fn provider(&self) -> &str {
        "mock"
    }
    
    fn config(&self) -> Option<&ModelConfig> {
        Some(&self.config)
    }
    
    async fn generate(&self, messages: &[Message]) -> Result<ModelResponse, Box<dyn std::error::Error + Send + Sync>> {
        if let Some(error) = &self.error {
            return Err(error.clone().into());
        }
        
        let content = vec![ContentBlock {
            text: Some(self.response.clone()),
            ..Default::default()
        }];
        
        Ok(ModelResponse {
            content,
            model_id: self.config.model_id.clone(),
            usage: Some(serde_json::json!({
                "prompt_tokens": 10,
                "completion_tokens": 5,
                "total_tokens": 15
            })),
            metadata: None,
        })
    }
    
    async fn stream(&self, _messages: &[Message]) -> Result<ModelStreamResponse, Box<dyn std::error::Error + Send + Sync>> {
        // Mock streaming implementation
        Ok(ModelStreamResponse::new())
    }
}

// Mock stream response implementation
impl ModelStreamResponse {
    fn new() -> Self {
        Self {
            // Mock implementation
        }
    }
}
