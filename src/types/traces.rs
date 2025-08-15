//! Tracing and telemetry type definitions for the SDK.
//! 
//! This module defines the types used to represent traces,
//! spans, and telemetry data.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A trace span for telemetry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraceSpan {
    /// The span ID.
    pub id: String,
    /// The span name.
    pub name: String,
    /// The span attributes.
    pub attributes: HashMap<String, AttributeValue>,
    /// The span start time.
    pub start_time: u64,
    /// The span end time.
    pub end_time: Option<u64>,
}

/// A trace attribute value.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AttributeValue {
    String(String),
    Number(f64),
    Boolean(bool),
    Array(Vec<AttributeValue>),
    Object(HashMap<String, AttributeValue>),
}

impl TraceSpan {
    /// Create a new trace span.
    pub fn new(id: &str, name: &str, start_time: u64) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            attributes: HashMap::new(),
            start_time,
            end_time: None,
        }
    }

    /// Add an attribute to the span.
    pub fn add_attribute(&mut self, key: &str, value: AttributeValue) {
        self.attributes.insert(key.to_string(), value);
    }

    /// End the span.
    pub fn end(&mut self, end_time: u64) {
        self.end_time = Some(end_time);
    }
}
