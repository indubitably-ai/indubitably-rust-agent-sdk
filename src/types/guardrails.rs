//! Guardrails-related type definitions for the SDK.
//! 
//! This module defines the types used to represent guardrails
//! and content filtering.

use serde::{Deserialize, Serialize};

/// A guardrail configuration for content filtering.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Guardrail {
    /// The name of the guardrail.
    pub name: String,
    /// The type of guardrail.
    pub guardrail_type: GuardrailType,
    /// The configuration for the guardrail.
    pub config: serde_json::Value,
}

/// The type of guardrail.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum GuardrailType {
    ContentFilter,
    Toxicity,
    Bias,
    Custom(String),
}

impl Guardrail {
    /// Create a new guardrail.
    pub fn new(name: &str, guardrail_type: GuardrailType, config: serde_json::Value) -> Self {
        Self {
            name: name.to_string(),
            guardrail_type,
            config,
        }
    }
}
