//! Metrics collection for the SDK.
//! 
//! This module provides functionality for collecting and
//! reporting metrics about agent performance and usage.

use std::collections::HashMap;

/// A metrics collector for the SDK.
pub struct Metrics {
    /// The metrics data.
    data: HashMap<String, f64>,
}

impl Metrics {
    /// Create a new metrics collector.
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }
    
    /// Increment a counter metric.
    pub fn increment(&mut self, name: &str, value: f64) {
        *self.data.entry(name.to_string()).or_insert(0.0) += value;
    }
    
    /// Set a gauge metric.
    pub fn set(&mut self, name: &str, value: f64) {
        self.data.insert(name.to_string(), value);
    }
    
    /// Get a metric value.
    pub fn get(&self, name: &str) -> Option<f64> {
        self.data.get(name).copied()
    }
    
    /// Get all metrics.
    pub fn all(&self) -> &HashMap<String, f64> {
        &self.data
    }
}

impl Default for Metrics {
    fn default() -> Self {
        Self::new()
    }
}
