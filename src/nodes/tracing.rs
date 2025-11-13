use crate::models::{Node, NodeCategory, NodeContext, NodeOutput, NodeSubcategory, NodeType};
use async_trait::async_trait;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TracingParams {
    /// Tracing operation to perform
    pub operation: String,
    /// Span name
    pub span_name: Option<String>,
    /// Trace ID
    pub trace_id: Option<String>,
    /// Span ID
    pub span_id: Option<String>,
    /// Span attributes as JSON object
    #[serde(default)]
    pub attributes: serde_json::Value,
    /// Span events as array
    pub events: Option<Vec<serde_json::Value>>,
    /// Parent span ID
    pub parent_span_id: Option<String>,
}

/// Tracing node - OpenTelemetry distributed tracing support
pub struct TracingNode;

impl TracingNode {
    pub fn new() -> Self {
        Self
    }
}

impl Default for TracingNode {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeType for TracingNode {
    fn type_name(&self) -> &str {
        "tracing"
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
                    "description": "Tracing operation to perform",
                    "enum": [
                        "start_span",
                        "end_span",
                        "add_event",
                        "set_attribute",
                        "create_trace",
                        "get_trace_context"
                    ]
                },
                "span_name": {
                    "type": "string",
                    "description": "Name of the span (required for start_span)",
                    "minLength": 1
                },
                "trace_id": {
                    "type": "string",
                    "description": "Trace ID (auto-generated if not provided)"
                },
                "span_id": {
                    "type": "string",
                    "description": "Span ID (required for end_span, add_event, set_attribute)"
                },
                "attributes": {
                    "type": "object",
                    "description": "Span attributes as key-value pairs (e.g., {\"http.method\": \"GET\", \"http.status_code\": 200})"
                },
                "events": {
                    "type": "array",
                    "description": "Span events (each event is an object with name, timestamp, attributes)",
                    "items": {
                        "type": "object"
                    }
                },
                "parent_span_id": {
                    "type": "string",
                    "description": "Parent span ID for creating nested spans"
                }
            },
            "required": ["operation"],
            "additionalProperties": false
        })
    }

    fn required_credential_type(&self) -> Option<&str> {
        None
    }
}

#[async_trait]
impl Node for TracingNode {
    async fn execute(
        &self,
        context: &NodeContext,
        parameters: &serde_json::Value,
    ) -> anyhow::Result<NodeOutput> {
        let params: TracingParams = serde_json::from_value(parameters.clone())?;

        // TODO: In a real implementation:
        // 1. Integrate with OpenTelemetry SDK
        // 2. For start_span: Create and start a new span with given name and attributes
        // 3. For end_span: End an existing span by span_id
        // 4. For add_event: Add an event to an existing span (log point within span)
        // 5. For set_attribute: Set/update an attribute on an existing span
        // 6. For create_trace: Create a new trace with root span
        // 7. For get_trace_context: Get current trace context (trace_id, span_id)
        // 8. Support nested spans via parent_span_id
        // 9. Export traces to backends (Jaeger, Zipkin, etc.)
        // 10. Propagate trace context across service boundaries

        // For now, we'll return a placeholder response
        let result = serde_json::json!({
            "message": "Tracing operation executed (placeholder implementation)",
            "operation": &params.operation,
            "span_name": params.span_name,
            "trace_id": params.trace_id.unwrap_or_else(|| "trace-12345".to_string()),
            "span_id": params.span_id.unwrap_or_else(|| "span-67890".to_string()),
            "attributes": params.attributes,
            "parent_span_id": params.parent_span_id,
            "context_execution_id": &context.execution_id,
            "trace_status": "active",
            "success": true
        });

        Ok(NodeOutput::success(result))
    }

    fn validate_parameters(&self, parameters: &serde_json::Value) -> anyhow::Result<()> {
        let params: TracingParams = serde_json::from_value(parameters.clone())
            .map_err(|e| anyhow::anyhow!("Invalid parameters: {}", e))?;

        // Validate operation
        let valid_operations = [
            "start_span",
            "end_span",
            "add_event",
            "set_attribute",
            "create_trace",
            "get_trace_context",
        ];

        if !valid_operations.contains(&params.operation.as_str()) {
            anyhow::bail!(
                "Invalid operation: {}. Must be one of: {}",
                params.operation,
                valid_operations.join(", ")
            );
        }

        // start_span and create_trace require span_name
        if (params.operation == "start_span" || params.operation == "create_trace")
            && params.span_name.is_none()
        {
            anyhow::bail!(
                "{} operation requires 'span_name' parameter",
                params.operation
            );
        }

        // Validate span_name is not empty if provided
        if let Some(ref span_name) = params.span_name
            && span_name.trim().is_empty()
        {
            anyhow::bail!("span_name cannot be empty");
        }

        // Operations that require span_id
        let requires_span_id = ["end_span", "add_event", "set_attribute"];
        if requires_span_id.contains(&params.operation.as_str()) && params.span_id.is_none() {
            anyhow::bail!(
                "{} operation requires 'span_id' parameter",
                params.operation
            );
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
    fn test_tracing_node_type() {
        let node = TracingNode::new();
        assert_eq!(node.type_name(), "tracing");
        assert!(matches!(node.category(), NodeCategory::Action));
        assert!(matches!(node.subcategory(), NodeSubcategory::General));
        assert_eq!(node.required_credential_type(), None);
    }

    #[test]
    fn test_tracing_parameter_schema() {
        let node = TracingNode::new();
        let schema = node.parameter_schema();

        assert_eq!(schema["type"], "object");
        assert!(schema["properties"]["operation"].is_object());
        assert!(schema["properties"]["span_name"].is_object());
        assert!(schema["properties"]["trace_id"].is_object());
    }

    #[tokio::test]
    async fn test_tracing_start_span() {
        let node = TracingNode::new();
        let context = NodeContext::new(Uuid::new_v4().to_string(), "test-workflow".to_string());

        let params = json!({
            "operation": "start_span",
            "span_name": "workflow.execution",
            "attributes": {
                "workflow.id": "wf-123",
                "workflow.name": "test-workflow"
            }
        });

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
    }

    #[tokio::test]
    async fn test_tracing_nested_span() {
        let node = TracingNode::new();
        let context = NodeContext::new(Uuid::new_v4().to_string(), "test-workflow".to_string());

        let params = json!({
            "operation": "start_span",
            "span_name": "node.execution",
            "parent_span_id": "span-parent-123",
            "attributes": {
                "node.type": "http_request",
                "node.id": "node-456"
            }
        });

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
    }

    #[test]
    fn test_tracing_validation() {
        let node = TracingNode::new();

        // Valid start_span
        let valid_params = json!({
            "operation": "start_span",
            "span_name": "test.span"
        });
        assert!(node.validate_parameters(&valid_params).is_ok());

        // Invalid operation
        let invalid_params = json!({
            "operation": "invalid_operation"
        });
        assert!(node.validate_parameters(&invalid_params).is_err());

        // Missing span_name for start_span
        let invalid_params = json!({
            "operation": "start_span"
        });
        assert!(node.validate_parameters(&invalid_params).is_err());

        // Empty span_name
        let invalid_params = json!({
            "operation": "start_span",
            "span_name": ""
        });
        assert!(node.validate_parameters(&invalid_params).is_err());

        // Missing span_id for end_span
        let invalid_params = json!({
            "operation": "end_span"
        });
        assert!(node.validate_parameters(&invalid_params).is_err());
    }
}
