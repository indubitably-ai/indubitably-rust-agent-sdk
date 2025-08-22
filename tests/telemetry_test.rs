//! Unit tests for the telemetry module.
//! 
//! These tests verify that telemetry functionality works correctly
//! including metrics, tracing, and observability.

use indubitably_rust_agent_sdk::telemetry::*;
use indubitably_rust_agent_sdk::types::{TraceSpan, AttributeValue};
use std::collections::HashMap;

#[test]
fn test_telemetry_config_creation() {
    let config = TelemetryConfig::new();
    
    assert_eq!(config.enabled, true);
    assert_eq!(config.metrics_enabled, true);
    assert_eq!(config.tracing_enabled, true);
    assert_eq!(config.logging_enabled, true);
    assert!(config.metrics_endpoint.is_none());
    assert!(config.tracing_endpoint.is_none());
}

#[test]
fn test_telemetry_config_builder() {
    let config = TelemetryConfig::new()
        .with_metrics_endpoint("http://localhost:9090")
        .with_tracing_endpoint("http://localhost:14268")
        .with_logging_level("info")
        .with_metrics_enabled(false)
        .with_tracing_enabled(false);
    
    assert_eq!(config.metrics_endpoint.as_ref().unwrap(), "http://localhost:9090");
    assert_eq!(config.tracing_endpoint.as_ref().unwrap(), "http://localhost:14268");
    assert_eq!(config.logging_level.as_ref().unwrap(), "info");
    assert_eq!(config.metrics_enabled, false);
    assert_eq!(config.tracing_enabled, false);
}

#[test]
fn test_telemetry_config_serialization() {
    let config = TelemetryConfig::new()
        .with_metrics_endpoint("http://localhost:9090")
        .with_tracing_endpoint("http://localhost:14268");
    
    let json = serde_json::to_string(&config).unwrap();
    let deserialized: TelemetryConfig = serde_json::from_str(&json).unwrap();
    
    assert_eq!(deserialized.metrics_endpoint, config.metrics_endpoint);
    assert_eq!(deserialized.tracing_endpoint, config.tracing_endpoint);
}

#[test]
fn test_telemetry_config_clone() {
    let config = TelemetryConfig::new()
        .with_metrics_endpoint("http://localhost:9090");
    
    let cloned_config = config.clone();
    
    assert_eq!(cloned_config.metrics_endpoint, config.metrics_endpoint);
    assert_eq!(cloned_config.enabled, config.enabled);
}

#[test]
fn test_metrics_registry_creation() {
    let registry = MetricsRegistry::new();
    
    assert_eq!(registry.metric_count(), 0);
    assert!(registry.is_empty());
}

#[test]
fn test_metrics_registry_counter() {
    let mut registry = MetricsRegistry::new();
    
    let counter = Counter::new("test_counter", "A test counter");
    registry.register_counter(counter).unwrap();
    
    assert_eq!(registry.metric_count(), 1);
    
    registry.increment_counter("test_counter", 1).unwrap();
    registry.increment_counter("test_counter", 2).unwrap();
    
    let value = registry.get_counter_value("test_counter").unwrap();
    assert_eq!(value, 3);
}

#[test]
fn test_metrics_registry_gauge() {
    let mut registry = MetricsRegistry::new();
    
    let gauge = Gauge::new("test_gauge", "A test gauge");
    registry.register_gauge(gauge).unwrap();
    
    assert_eq!(registry.metric_count(), 1);
    
    registry.set_gauge_value("test_gauge", 42.0).unwrap();
    registry.set_gauge_value("test_gauge", 100.0).unwrap();
    
    let value = registry.get_gauge_value("test_gauge").unwrap();
    assert_eq!(value, 100.0);
}

#[test]
fn test_metrics_registry_histogram() {
    let mut registry = MetricsRegistry::new();
    
    let histogram = Histogram::new("test_histogram", "A test histogram");
    registry.register_histogram(histogram).unwrap();
    
    assert_eq!(registry.metric_count(), 1);
    
    registry.record_histogram_value("test_histogram", 1.0).unwrap();
    registry.record_histogram_value("test_histogram", 2.0).unwrap();
    registry.record_histogram_value("test_histogram", 3.0).unwrap();
    
    let stats = registry.get_histogram_stats("test_histogram").unwrap();
    assert_eq!(stats.count, 3);
    assert_eq!(stats.sum, 6.0);
    assert_eq!(stats.min, 1.0);
    assert_eq!(stats.max, 3.0);
}

#[test]
fn test_metrics_registry_metric_not_found() {
    let registry = MetricsRegistry::new();
    
    let result = registry.increment_counter("nonexistent", 1);
    assert!(result.is_err());
    
    let result = registry.set_gauge_value("nonexistent", 42.0);
    assert!(result.is_err());
    
    let result = registry.record_histogram_value("nonexistent", 1.0);
    assert!(result.is_err());
}

#[test]
fn test_metrics_registry_duplicate_registration() {
    let mut registry = MetricsRegistry::new();
    
    let counter1 = Counter::new("duplicate", "First counter");
    let counter2 = Counter::new("duplicate", "Second counter");
    
    registry.register_counter(counter1).unwrap();
    let result = registry.register_counter(counter2);
    
    // Should fail due to duplicate name
    assert!(result.is_err());
    assert_eq!(registry.metric_count(), 1);
}

#[test]
fn test_metrics_registry_clear() {
    let mut registry = MetricsRegistry::new();
    
    let counter = Counter::new("test_counter", "A test counter");
    let gauge = Gauge::new("test_gauge", "A test gauge");
    
    registry.register_counter(counter).unwrap();
    registry.register_gauge(gauge).unwrap();
    
    assert_eq!(registry.metric_count(), 2);
    
    registry.clear();
    assert_eq!(registry.metric_count(), 0);
    assert!(registry.is_empty());
}

#[test]
fn test_counter_creation() {
    let counter = Counter::new("test_counter", "A test counter");
    
    assert_eq!(counter.name(), "test_counter");
    assert_eq!(counter.description(), "A test counter");
    assert_eq!(counter.value(), 0);
}

#[test]
fn test_counter_increment() {
    let mut counter = Counter::new("test_counter", "A test counter");
    
    assert_eq!(counter.value(), 0);
    
    counter.increment(1);
    assert_eq!(counter.value(), 1);
    
    counter.increment(5);
    assert_eq!(counter.value(), 6);
}

#[test]
fn test_counter_reset() {
    let mut counter = Counter::new("test_counter", "A test counter");
    
    counter.increment(10);
    assert_eq!(counter.value(), 10);
    
    counter.reset();
    assert_eq!(counter.value(), 0);
}

#[test]
fn test_gauge_creation() {
    let gauge = Gauge::new("test_gauge", "A test gauge");
    
    assert_eq!(gauge.name(), "test_gauge");
    assert_eq!(gauge.description(), "A test gauge");
    assert_eq!(gauge.value(), 0.0);
}

#[test]
fn test_gauge_set_value() {
    let mut gauge = Gauge::new("test_gauge", "A test gauge");
    
    assert_eq!(gauge.value(), 0.0);
    
    gauge.set_value(42.0);
    assert_eq!(gauge.value(), 42.0);
    
    gauge.set_value(100.5);
    assert_eq!(gauge.value(), 100.5);
}

#[test]
fn test_gauge_reset() {
    let mut gauge = Gauge::new("test_gauge", "A test gauge");
    
    gauge.set_value(42.0);
    assert_eq!(gauge.value(), 42.0);
    
    gauge.reset();
    assert_eq!(gauge.value(), 0.0);
}

#[test]
fn test_histogram_creation() {
    let histogram = Histogram::new("test_histogram", "A test histogram");
    
    assert_eq!(histogram.name(), "test_histogram");
    assert_eq!(histogram.description(), "A test histogram");
    assert_eq!(histogram.count(), 0);
    assert_eq!(histogram.sum(), 0.0);
}

#[test]
fn test_histogram_record_value() {
    let mut histogram = Histogram::new("test_histogram", "A test histogram");
    
    assert_eq!(histogram.count(), 0);
    assert_eq!(histogram.sum(), 0.0);
    
    histogram.record_value(1.0);
    assert_eq!(histogram.count(), 1);
    assert_eq!(histogram.sum(), 1.0);
    
    histogram.record_value(2.0);
    assert_eq!(histogram.count(), 2);
    assert_eq!(histogram.sum(), 3.0);
}

#[test]
fn test_histogram_stats() {
    let mut histogram = Histogram::new("test_histogram", "A test histogram");
    
    histogram.record_value(1.0);
    histogram.record_value(2.0);
    histogram.record_value(3.0);
    
    let stats = histogram.stats();
    assert_eq!(stats.count, 3);
    assert_eq!(stats.sum, 6.0);
    assert_eq!(stats.min, 1.0);
    assert_eq!(stats.max, 3.0);
    assert_eq!(stats.mean(), 2.0);
}

#[test]
fn test_histogram_reset() {
    let mut histogram = Histogram::new("test_histogram", "A test histogram");
    
    histogram.record_value(1.0);
    histogram.record_value(2.0);
    assert_eq!(histogram.count(), 2);
    
    histogram.reset();
    assert_eq!(histogram.count(), 0);
    assert_eq!(histogram.sum(), 0.0);
}

#[test]
fn test_trace_span_creation() {
    let span = TraceSpan::new("test_span", 1234567890);
    
    assert_eq!(span.id, "test_span");
    assert_eq!(span.name, "test_span");
    assert_eq!(span.start_time, 1234567890);
    assert!(span.end_time.is_none());
    assert!(span.attributes.is_empty());
}

#[test]
fn test_trace_span_add_attribute() {
    let mut span = TraceSpan::new("test_span", 1234567890);
    
    span.add_attribute("key1", AttributeValue::String("value1".to_string()));
    span.add_attribute("key2", AttributeValue::Number(42.0));
    span.add_attribute("key3", AttributeValue::Boolean(true));
    
    assert_eq!(span.attributes.len(), 3);
    assert_eq!(span.attributes.get("key1").unwrap(), &AttributeValue::String("value1".to_string()));
    assert_eq!(span.attributes.get("key2").unwrap(), &AttributeValue::Number(42.0));
    assert_eq!(span.attributes.get("key3").unwrap(), &AttributeValue::Boolean(true));
}

#[test]
fn test_trace_span_end() {
    let mut span = TraceSpan::new("test_span", 1234567890);
    
    assert!(span.end_time.is_none());
    
    span.end(1234567990);
    assert_eq!(span.end_time, Some(1234567990));
}

#[test]
fn test_trace_span_duration() {
    let mut span = TraceSpan::new("test_span", 1234567890);
    
    // Duration should be None before ending
    assert!(span.duration().is_none());
    
    span.end(1234567990);
    let duration = span.duration().unwrap();
    assert_eq!(duration, 100); // 1234567990 - 1234567890
}

#[test]
fn test_trace_span_serialization() {
    let mut span = TraceSpan::new("test_span", 1234567890);
    span.add_attribute("key1", AttributeValue::String("value1".to_string()));
    span.end(1234567990);
    
    let json = serde_json::to_string(&span).unwrap();
    let deserialized: TraceSpan = serde_json::from_str(&json).unwrap();
    
    assert_eq!(deserialized.id, span.id);
    assert_eq!(deserialized.name, span.name);
    assert_eq!(deserialized.start_time, span.start_time);
    assert_eq!(deserialized.end_time, span.end_time);
}

#[test]
fn test_attribute_value_creation() {
    let string_val = AttributeValue::String("hello".to_string());
    let number_val = AttributeValue::Number(42.0);
    let bool_val = AttributeValue::Boolean(true);
    
    let array_val = AttributeValue::Array(vec![string_val.clone(), number_val.clone()]);
    let mut object_map = HashMap::new();
    object_map.insert("key".to_string(), bool_val.clone());
    let object_val = AttributeValue::Object(object_map);
    
    assert!(matches!(string_val, AttributeValue::String(_)));
    assert!(matches!(number_val, AttributeValue::Number(_)));
    assert!(matches!(bool_val, AttributeValue::Boolean(_)));
    assert!(matches!(array_val, AttributeValue::Array(_)));
    assert!(matches!(object_val, AttributeValue::Object(_)));
}

#[test]
fn test_attribute_value_serialization() {
    let string_val = AttributeValue::String("hello".to_string());
    let number_val = AttributeValue::Number(42.0);
    let bool_val = AttributeValue::Boolean(true);
    
    let json_string = serde_json::to_string(&string_val).unwrap();
    let json_number = serde_json::to_string(&number_val).unwrap();
    let json_bool = serde_json::to_string(&bool_val).unwrap();
    
    let deserialized_string: AttributeValue = serde_json::from_str(&json_string).unwrap();
    let deserialized_number: AttributeValue = serde_json::from_str(&json_number).unwrap();
    let deserialized_bool: AttributeValue = serde_json::from_str(&json_bool).unwrap();
    
    assert_eq!(deserialized_string, string_val);
    assert_eq!(deserialized_number, number_val);
    assert_eq!(deserialized_bool, bool_val);
}

#[test]
fn test_metrics_registry_concurrent_access() {
    let registry = std::sync::Arc::new(tokio::sync::RwLock::new(MetricsRegistry::new()));
    
    // Spawn multiple tasks to test concurrent access
    let mut handles = vec![];
    
    for i in 0..5 {
        let registry_clone = registry.clone();
        let handle = tokio::spawn(async move {
            let mut registry = registry_clone.write().await;
            let counter = Counter::new(format!("counter_{}", i), format!("Counter {}", i));
            registry.register_counter(counter)
        });
        handles.push(handle);
    }
    
    // Wait for all tasks to complete
    for handle in handles {
        let result = handle.await.unwrap();
        assert!(result.is_ok());
    }
    
    let registry = registry.read().await;
    assert_eq!(registry.metric_count(), 5);
}

#[test]
fn test_telemetry_config_validation() {
    let config = TelemetryConfig::new()
        .with_metrics_endpoint("invalid-url")
        .with_tracing_endpoint("invalid-url");
    
    // Should handle invalid URLs gracefully
    assert_eq!(config.metrics_endpoint.as_ref().unwrap(), "invalid-url");
    assert_eq!(config.tracing_endpoint.as_ref().unwrap(), "invalid-url");
}

#[test]
fn test_metrics_registry_metric_types() {
    let mut registry = MetricsRegistry::new();
    
    let counter = Counter::new("test_counter", "A test counter");
    let gauge = Gauge::new("test_gauge", "A test gauge");
    let histogram = Histogram::new("test_histogram", "A test histogram");
    
    registry.register_counter(counter).unwrap();
    registry.register_gauge(gauge).unwrap();
    registry.register_histogram(histogram).unwrap();
    
    assert_eq!(registry.metric_count(), 3);
    
    // Test that we can access each type
    registry.increment_counter("test_counter", 1).unwrap();
    registry.set_gauge_value("test_gauge", 42.0).unwrap();
    registry.record_histogram_value("test_histogram", 1.0).unwrap();
}

#[test]
fn test_histogram_edge_cases() {
    let mut histogram = Histogram::new("test_histogram", "A test histogram");
    
    // Test with zero values
    histogram.record_value(0.0);
    assert_eq!(histogram.count(), 1);
    assert_eq!(histogram.sum(), 0.0);
    
    // Test with negative values
    histogram.record_value(-1.0);
    assert_eq!(histogram.count(), 2);
    assert_eq!(histogram.sum(), -1.0);
    
    // Test with very large values
    histogram.record_value(f64::MAX);
    assert_eq!(histogram.count(), 3);
    assert_eq!(histogram.sum(), f64::MAX - 1.0);
}

#[test]
fn test_trace_span_edge_cases() {
    let mut span = TraceSpan::new("test_span", 0);
    
    // Test with zero timestamp
    assert_eq!(span.start_time, 0);
    
    // Test ending with same timestamp
    span.end(0);
    assert_eq!(span.end_time, Some(0));
    assert_eq!(span.duration().unwrap(), 0);
    
    // Test with very large timestamps
    let mut span = TraceSpan::new("test_span", u64::MAX);
    span.end(u64::MAX);
    assert_eq!(span.duration().unwrap(), 0);
}
