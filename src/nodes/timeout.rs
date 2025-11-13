use crate::models::{Node, NodeCategory, NodeContext, NodeOutput, NodeSubcategory, NodeType};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

/// Timeout node to enforce time limits on operations
#[derive(Clone)]
pub struct TimeoutNode {}

#[derive(Debug, Serialize, Deserialize)]
struct TimeoutParams {
    /// Timeout duration in seconds
    timeout_seconds: Option<u64>,
    /// Timeout duration in milliseconds
    timeout_milliseconds: Option<u64>,
    /// Action on timeout: "error", "default", "skip"
    on_timeout: Option<String>,
    /// Default value to return on timeout
    default_value: Option<Value>,
}

impl TimeoutNode {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for TimeoutNode {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeType for TimeoutNode {
    fn type_name(&self) -> &str {
        "timeout"
    }

    fn category(&self) -> NodeCategory {
        NodeCategory::Control
    }

    fn subcategory(&self) -> NodeSubcategory {
        NodeSubcategory::General
    }

    fn parameter_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "timeout_seconds": {
                    "type": "integer",
                    "description": "Timeout in seconds",
                    "minimum": 1,
                    "maximum": 3600
                },
                "timeout_milliseconds": {
                    "type": "integer",
                    "description": "Timeout in milliseconds",
                    "minimum": 1
                },
                "on_timeout": {
                    "type": "string",
                    "enum": ["error", "default", "skip"],
                    "description": "Action when timeout occurs (default: 'error')"
                },
                "default_value": {
                    "description": "Value to return on timeout (if on_timeout='default')"
                }
            },
            "oneOf": [
                {"required": ["timeout_seconds"]},
                {"required": ["timeout_milliseconds"]}
            ]
        })
    }
}

#[async_trait]
impl Node for TimeoutNode {
    async fn execute(
        &self,
        context: &NodeContext,
        parameters: &serde_json::Value,
    ) -> anyhow::Result<NodeOutput> {
        let params: TimeoutParams = serde_json::from_value(parameters.clone())?;

        let timeout_ms = if let Some(seconds) = params.timeout_seconds {
            seconds * 1000
        } else if let Some(ms) = params.timeout_milliseconds {
            ms
        } else {
            anyhow::bail!("Must specify timeout_seconds or timeout_milliseconds");
        };

        let on_timeout = params.on_timeout.unwrap_or_else(|| "error".to_string());

        // In a real implementation, this would wrap child node execution
        // with tokio::time::timeout and handle timeouts appropriately

        let result = json!({
            "timeout_ms": timeout_ms,
            "on_timeout": on_timeout,
            "has_default_value": params.default_value.is_some(),
            "context_execution_id": context.execution_id
        });

        Ok(NodeOutput::success(result))
    }

    fn validate_parameters(&self, parameters: &Value) -> anyhow::Result<()> {
        let params: TimeoutParams = serde_json::from_value(parameters.clone())?;

        // Ensure at least one timeout parameter
        if params.timeout_seconds.is_none() && params.timeout_milliseconds.is_none() {
            anyhow::bail!("Must specify timeout_seconds or timeout_milliseconds");
        }

        // Validate timeout ranges
        if let Some(seconds) = params.timeout_seconds
            && !(1..=3600).contains(&seconds)
        {
            anyhow::bail!("timeout_seconds must be between 1 and 3600");
        }

        // Validate on_timeout action
        if let Some(action) = &params.on_timeout
            && !["error", "default", "skip"].contains(&action.as_str())
        {
            anyhow::bail!("on_timeout must be 'error', 'default', or 'skip'");
        }

        // Validate that default_value is provided if on_timeout is "default"
        if params.on_timeout.as_deref() == Some("default") && params.default_value.is_none() {
            anyhow::bail!("default_value must be provided when on_timeout='default'");
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_timeout_node() {
        let node = TimeoutNode::new();
        let context = NodeContext::new("exec-123".to_string(), "node-1".to_string());
        let params = json!({
            "timeout_seconds": 30,
            "on_timeout": "error"
        });

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
    }

    #[test]
    fn test_timeout_validation() {
        let node = TimeoutNode::new();

        // Valid
        let params = json!({"timeout_seconds": 10});
        assert!(node.validate_parameters(&params).is_ok());

        // Missing timeout
        let params = json!({"on_timeout": "error"});
        assert!(node.validate_parameters(&params).is_err());

        // Default without default_value
        let params = json!({
            "timeout_seconds": 10,
            "on_timeout": "default"
        });
        assert!(node.validate_parameters(&params).is_err());
    }
}
