use crate::models::{Node, NodeCategory, NodeContext, NodeOutput, NodeSubcategory, NodeType};
use async_trait::async_trait;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct MetricsParams {
    /// Metrics operation to perform
    pub operation: String,
    /// Metric name
    pub metric_name: Option<String>,
    /// Metric value
    pub value: Option<f64>,
    /// Unit of measurement
    pub unit: Option<String>,
    /// Tags for the metric as JSON object
    #[serde(default)]
    pub tags: serde_json::Value,
    /// Namespace for organizing metrics
    pub namespace: Option<String>,
    /// Timestamp for the metric (Unix timestamp in seconds)
    pub timestamp: Option<i64>,
}

/// Metrics node - emits custom metrics for workflow monitoring
pub struct MetricsNode;

impl MetricsNode {
    pub fn new() -> Self {
        Self
    }
}

impl Default for MetricsNode {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeType for MetricsNode {
    fn type_name(&self) -> &str {
        "metrics"
    }

    fn category(&self) -> NodeCategory {
        NodeCategory::Action
    }

    fn subcategory(&self) -> NodeSubcategory {
        NodeSubcategory::General
    }

    fn parameter_schema(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "operation": {
                    "type": "string",
                    "description": "Metrics operation to perform",
                    "enum": [
                        "emit_counter",
                        "emit_gauge",
                        "emit_histogram",
                        "emit_timer",
                        "increment_counter",
                        "decrement_gauge",
                        "record_value"
                    ]
                },
                "metric_name": {
                    "type": "string",
                    "description": "Name of the metric",
                    "minLength": 1
                },
                "value": {
                    "type": "number",
                    "description": "Metric value (required for most operations)"
                },
                "unit": {
                    "type": "string",
                    "description": "Unit of measurement (e.g., 'seconds', 'bytes', 'count')"
                },
                "tags": {
                    "type": "object",
                    "description": "Tags for the metric as key-value pairs (e.g., {\"env\": \"prod\", \"region\": \"us-east-1\"})"
                },
                "namespace": {
                    "type": "string",
                    "description": "Namespace for organizing metrics (e.g., 'workflows', 'api')"
                },
                "timestamp": {
                    "type": "integer",
                    "description": "Unix timestamp in seconds (defaults to current time)"
                }
            },
            "required": ["operation", "metric_name"],
            "additionalProperties": false
        })
    }

    fn required_credential_type(&self) -> Option<&str> {
        None
    }
}

#[async_trait]
impl Node for MetricsNode {
    async fn execute(
        &self,
        context: &NodeContext,
        parameters: &serde_json::Value,
    ) -> anyhow::Result<NodeOutput> {
        let params: MetricsParams = serde_json::from_value(parameters.clone())?;

        // TODO: In a real implementation:
        // 1. Integrate with metrics backends (Prometheus, CloudWatch, Datadog, etc.)
        // 2. For emit_counter: Emit a counter metric (cumulative value)
        // 3. For emit_gauge: Emit a gauge metric (point-in-time value)
        // 4. For emit_histogram: Emit a histogram metric (distribution of values)
        // 5. For emit_timer: Emit a timer metric (duration measurement)
        // 6. For increment_counter: Increment an existing counter by 1 or specified value
        // 7. For decrement_gauge: Decrement an existing gauge by 1 or specified value
        // 8. For record_value: Record a value for general-purpose metrics
        // 9. Apply tags for metric dimensions
        // 10. Use namespace to organize metrics logically

        // For now, we'll return a placeholder response
        let result = serde_json::json!({
            "message": "Metric emitted (placeholder implementation)",
            "operation": &params.operation,
            "metric_name": params.metric_name,
            "value": params.value,
            "unit": params.unit,
            "tags": params.tags,
            "namespace": params.namespace,
            "timestamp": params.timestamp,
            "context_execution_id": &context.execution_id,
            "metric_status": "emitted",
            "success": true
        });

        Ok(NodeOutput::success(result))
    }

    fn validate_parameters(&self, parameters: &serde_json::Value) -> anyhow::Result<()> {
        let params: MetricsParams = serde_json::from_value(parameters.clone())
            .map_err(|e| anyhow::anyhow!("Invalid parameters: {}", e))?;

        // Validate operation
        let valid_operations = [
            "emit_counter",
            "emit_gauge",
            "emit_histogram",
            "emit_timer",
            "increment_counter",
            "decrement_gauge",
            "record_value",
        ];

        if !valid_operations.contains(&params.operation.as_str()) {
            anyhow::bail!(
                "Invalid operation: {}. Must be one of: {}",
                params.operation,
                valid_operations.join(", ")
            );
        }

        // Validate metric_name is not empty
        if let Some(ref metric_name) = params.metric_name
            && metric_name.trim().is_empty()
        {
            anyhow::bail!("metric_name cannot be empty");
        }

        // Operations that require value
        let requires_value = [
            "emit_counter",
            "emit_gauge",
            "emit_histogram",
            "emit_timer",
            "record_value",
        ];
        if requires_value.contains(&params.operation.as_str()) && params.value.is_none() {
            anyhow::bail!("{} operation requires 'value' parameter", params.operation);
        }

        // Validate timestamp is positive if provided
        if let Some(timestamp) = params.timestamp
            && timestamp < 0
        {
            anyhow::bail!("timestamp must be a positive Unix timestamp");
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use uuid::Uuid;

    #[test]
    fn test_metrics_node_type() {
        let node = MetricsNode::new();
        assert_eq!(node.type_name(), "metrics");
        assert!(matches!(node.category(), NodeCategory::Action));
        assert!(matches!(node.subcategory(), NodeSubcategory::General));
        assert_eq!(node.required_credential_type(), None);
    }

    #[test]
    fn test_metrics_parameter_schema() {
        let node = MetricsNode::new();
        let schema = node.parameter_schema();

        assert_eq!(schema["type"], "object");
        assert!(schema["properties"]["operation"].is_object());
        assert!(schema["properties"]["metric_name"].is_object());
        assert!(schema["properties"]["value"].is_object());
    }

    #[tokio::test]
    async fn test_metrics_emit_counter() {
        let node = MetricsNode::new();
        let context = NodeContext::new(Uuid::new_v4().to_string(), "test-workflow".to_string());

        let params = json!({
            "operation": "emit_counter",
            "metric_name": "workflow.executions",
            "value": 1.0,
            "tags": {"env": "production", "region": "us-east-1"},
            "namespace": "workflows"
        });

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
    }

    #[tokio::test]
    async fn test_metrics_emit_gauge() {
        let node = MetricsNode::new();
        let context = NodeContext::new(Uuid::new_v4().to_string(), "test-workflow".to_string());

        let params = json!({
            "operation": "emit_gauge",
            "metric_name": "workflow.active_executions",
            "value": 42.0,
            "unit": "count"
        });

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
    }

    #[test]
    fn test_metrics_validation() {
        let node = MetricsNode::new();

        // Valid parameters
        let valid_params = json!({
            "operation": "emit_counter",
            "metric_name": "test.metric",
            "value": 10.5
        });
        assert!(node.validate_parameters(&valid_params).is_ok());

        // Invalid operation
        let invalid_params = json!({
            "operation": "invalid_operation",
            "metric_name": "test.metric"
        });
        assert!(node.validate_parameters(&invalid_params).is_err());

        // Missing value for emit_counter
        let invalid_params = json!({
            "operation": "emit_counter",
            "metric_name": "test.metric"
        });
        assert!(node.validate_parameters(&invalid_params).is_err());

        // Empty metric_name
        let invalid_params = json!({
            "operation": "emit_gauge",
            "metric_name": "",
            "value": 5.0
        });
        assert!(node.validate_parameters(&invalid_params).is_err());

        // Negative timestamp
        let invalid_params = json!({
            "operation": "emit_counter",
            "metric_name": "test.metric",
            "value": 1.0,
            "timestamp": -100
        });
        assert!(node.validate_parameters(&invalid_params).is_err());
    }
}
