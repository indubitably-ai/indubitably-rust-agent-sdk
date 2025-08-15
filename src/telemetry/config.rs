//! Telemetry configuration for the SDK.
//! 
//! This module provides configuration options for
//! metrics, tracing, and other observability features.

use serde::{Deserialize, Serialize};

/// Configuration for telemetry features.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetryConfig {
    /// Whether metrics are enabled.
    pub metrics_enabled: bool,
    /// Whether tracing is enabled.
    pub tracing_enabled: bool,
    /// The metrics endpoint.
    pub metrics_endpoint: Option<String>,
    /// The tracing endpoint.
    pub tracing_endpoint: Option<String>,
}

impl Default for TelemetryConfig {
    fn default() -> Self {
        Self {
            metrics_enabled: false,
            tracing_enabled: false,
            metrics_endpoint: None,
            tracing_endpoint: None,
        }
    }
}

impl TelemetryConfig {
    /// Create a new telemetry configuration.
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Enable metrics.
    pub fn with_metrics(mut self, enabled: bool) -> Self {
        self.metrics_enabled = enabled;
        self
    }
    
    /// Enable tracing.
    pub fn with_tracing(mut self, enabled: bool) -> Self {
        self.tracing_enabled = enabled;
        self
    }
    
    /// Set the metrics endpoint.
    pub fn with_metrics_endpoint(mut self, endpoint: &str) -> Self {
        self.metrics_endpoint = Some(endpoint.to_string());
        self
    }
    
    /// Set the tracing endpoint.
    pub fn with_tracing_endpoint(mut self, endpoint: &str) -> Self {
        self.tracing_endpoint = Some(endpoint.to_string());
        self
    }
}
