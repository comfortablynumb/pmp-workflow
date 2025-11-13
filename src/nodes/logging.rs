use crate::models::{Node, NodeCategory, NodeContext, NodeOutput, NodeSubcategory, NodeType};
use async_trait::async_trait;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LoggingParams {
    /// Logging operation to perform
    pub operation: String,
    /// Log message
    pub message: Option<String>,
    /// Log level (for log_structured)
    pub level: Option<String>,
    /// Context data as JSON object
    #[serde(default)]
    pub context: serde_json::Value,
    /// Tags for log categorization
    pub tags: Option<Vec<String>>,
    /// Correlation ID for distributed tracing
    pub correlation_id: Option<String>,
    /// User ID associated with this log
    pub user_id: Option<String>,
}

/// Logging node - structured logging for workflow events
pub struct LoggingNode;

impl LoggingNode {
    pub fn new() -> Self {
        Self
    }
}

impl Default for LoggingNode {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeType for LoggingNode {
    fn type_name(&self) -> &str {
        "logging"
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
                    "description": "Logging operation to perform",
                    "enum": [
                        "log_info",
                        "log_warn",
                        "log_error",
                        "log_debug",
                        "log_trace",
                        "log_structured"
                    ]
                },
                "message": {
                    "type": "string",
                    "description": "Log message",
                    "minLength": 1
                },
                "level": {
                    "type": "string",
                    "description": "Log level (required for log_structured)",
                    "enum": ["trace", "debug", "info", "warn", "error"]
                },
                "context": {
                    "type": "object",
                    "description": "Context data as key-value pairs (e.g., {\"user_id\": \"123\", \"action\": \"login\"})"
                },
                "tags": {
                    "type": "array",
                    "description": "Tags for log categorization",
                    "items": {
                        "type": "string"
                    }
                },
                "correlation_id": {
                    "type": "string",
                    "description": "Correlation ID for distributed tracing"
                },
                "user_id": {
                    "type": "string",
                    "description": "User ID associated with this log"
                }
            },
            "required": ["operation", "message"],
            "additionalProperties": false
        })
    }

    fn required_credential_type(&self) -> Option<&str> {
        None
    }
}

#[async_trait]
impl Node for LoggingNode {
    async fn execute(
        &self,
        context: &NodeContext,
        parameters: &serde_json::Value,
    ) -> anyhow::Result<NodeOutput> {
        let params: LoggingParams = serde_json::from_value(parameters.clone())?;

        // TODO: In a real implementation:
        // 1. Integrate with logging backends (Elasticsearch, CloudWatch Logs, Datadog, etc.)
        // 2. For log_info: Log at INFO level
        // 3. For log_warn: Log at WARN level
        // 4. For log_error: Log at ERROR level
        // 5. For log_debug: Log at DEBUG level
        // 6. For log_trace: Log at TRACE level
        // 7. For log_structured: Log with custom level and structured data
        // 8. Include context data in all logs
        // 9. Add correlation_id for request tracing across services
        // 10. Use tags for log categorization and filtering

        // For now, we'll return a placeholder response
        let result = serde_json::json!({
            "message": "Log entry created (placeholder implementation)",
            "operation": &params.operation,
            "log_message": params.message,
            "level": params.level.unwrap_or_else(|| {
                match params.operation.as_str() {
                    "log_info" => "info",
                    "log_warn" => "warn",
                    "log_error" => "error",
                    "log_debug" => "debug",
                    "log_trace" => "trace",
                    _ => "info"
                }.to_string()
            }),
            "context_data": params.context,
            "tags": params.tags,
            "correlation_id": params.correlation_id,
            "user_id": params.user_id,
            "context_execution_id": &context.execution_id,
            "log_status": "logged",
            "success": true
        });

        Ok(NodeOutput::success(result))
    }

    fn validate_parameters(&self, parameters: &serde_json::Value) -> anyhow::Result<()> {
        let params: LoggingParams = serde_json::from_value(parameters.clone())
            .map_err(|e| anyhow::anyhow!("Invalid parameters: {}", e))?;

        // Validate operation
        let valid_operations = [
            "log_info",
            "log_warn",
            "log_error",
            "log_debug",
            "log_trace",
            "log_structured",
        ];

        if !valid_operations.contains(&params.operation.as_str()) {
            anyhow::bail!(
                "Invalid operation: {}. Must be one of: {}",
                params.operation,
                valid_operations.join(", ")
            );
        }

        // Validate message is not empty
        if let Some(ref message) = params.message
            && message.trim().is_empty()
        {
            anyhow::bail!("message cannot be empty");
        }

        // log_structured requires level parameter
        if params.operation == "log_structured" && params.level.is_none() {
            anyhow::bail!("log_structured operation requires 'level' parameter");
        }

        // Validate level if provided
        if let Some(ref level) = params.level {
            let valid_levels = ["trace", "debug", "info", "warn", "error"];
            if !valid_levels.contains(&level.as_str()) {
                anyhow::bail!(
                    "Invalid level: {}. Must be one of: {}",
                    level,
                    valid_levels.join(", ")
                );
            }
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
    fn test_logging_node_type() {
        let node = LoggingNode::new();
        assert_eq!(node.type_name(), "logging");
        assert!(matches!(node.category(), NodeCategory::Action));
        assert!(matches!(node.subcategory(), NodeSubcategory::General));
        assert_eq!(node.required_credential_type(), None);
    }

    #[test]
    fn test_logging_parameter_schema() {
        let node = LoggingNode::new();
        let schema = node.parameter_schema();

        assert_eq!(schema["type"], "object");
        assert!(schema["properties"]["operation"].is_object());
        assert!(schema["properties"]["message"].is_object());
        assert!(schema["properties"]["level"].is_object());
    }

    #[tokio::test]
    async fn test_logging_log_info() {
        let node = LoggingNode::new();
        let context = NodeContext::new(Uuid::new_v4().to_string(), "test-workflow".to_string());

        let params = json!({
            "operation": "log_info",
            "message": "Workflow execution started",
            "tags": ["workflow", "execution"],
            "context": {"workflow_id": "wf-123"}
        });

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
    }

    #[tokio::test]
    async fn test_logging_structured() {
        let node = LoggingNode::new();
        let context = NodeContext::new(Uuid::new_v4().to_string(), "test-workflow".to_string());

        let params = json!({
            "operation": "log_structured",
            "message": "User action performed",
            "level": "info",
            "correlation_id": "corr-123",
            "user_id": "user-456",
            "context": {"action": "create_resource", "resource_id": "res-789"}
        });

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
    }

    #[test]
    fn test_logging_validation() {
        let node = LoggingNode::new();

        // Valid parameters
        let valid_params = json!({
            "operation": "log_info",
            "message": "Test log message"
        });
        assert!(node.validate_parameters(&valid_params).is_ok());

        // Invalid operation
        let invalid_params = json!({
            "operation": "invalid_operation",
            "message": "Test"
        });
        assert!(node.validate_parameters(&invalid_params).is_err());

        // Missing level for log_structured
        let invalid_params = json!({
            "operation": "log_structured",
            "message": "Test"
        });
        assert!(node.validate_parameters(&invalid_params).is_err());

        // Empty message
        let invalid_params = json!({
            "operation": "log_info",
            "message": ""
        });
        assert!(node.validate_parameters(&invalid_params).is_err());

        // Invalid level
        let invalid_params = json!({
            "operation": "log_structured",
            "message": "Test",
            "level": "invalid_level"
        });
        assert!(node.validate_parameters(&invalid_params).is_err());
    }
}
