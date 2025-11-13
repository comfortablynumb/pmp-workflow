use crate::models::{Node, NodeCategory, NodeContext, NodeOutput, NodeSubcategory, NodeType};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

/// Try/Catch node for error handling with fallback paths
#[derive(Clone)]
pub struct TryCatchNode {}

#[derive(Debug, Serialize, Deserialize)]
struct TryCatchParams {
    /// Whether to continue on error (default: true)
    continue_on_error: Option<bool>,
    /// Error handling strategy: "catch", "ignore", "log"
    error_strategy: Option<String>,
    /// Default value to return on error
    default_value: Option<Value>,
}

impl TryCatchNode {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for TryCatchNode {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeType for TryCatchNode {
    fn type_name(&self) -> &str {
        "try_catch"
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
                "continue_on_error": {
                    "type": "boolean",
                    "description": "Continue workflow execution on error (default: true)"
                },
                "error_strategy": {
                    "type": "string",
                    "enum": ["catch", "ignore", "log"],
                    "description": "How to handle errors (default: 'catch')"
                },
                "default_value": {
                    "description": "Default value to return on error"
                }
            }
        })
    }
}

#[async_trait]
impl Node for TryCatchNode {
    async fn execute(
        &self,
        context: &NodeContext,
        parameters: &serde_json::Value,
    ) -> anyhow::Result<NodeOutput> {
        let params: TryCatchParams = serde_json::from_value(parameters.clone())?;

        let continue_on_error = params.continue_on_error.unwrap_or(true);
        let error_strategy = params.error_strategy.unwrap_or_else(|| "catch".to_string());

        // In a real implementation, this would wrap child node execution
        // and catch any errors, routing to appropriate paths

        let result = json!({
            "try_catch_active": true,
            "continue_on_error": continue_on_error,
            "error_strategy": error_strategy,
            "has_default_value": params.default_value.is_some(),
            "context_execution_id": context.execution_id
        });

        Ok(NodeOutput::success(result))
    }

    fn validate_parameters(&self, parameters: &Value) -> anyhow::Result<()> {
        let params: TryCatchParams = serde_json::from_value(parameters.clone())?;

        if let Some(strategy) = &params.error_strategy
            && !["catch", "ignore", "log"].contains(&strategy.as_str())
        {
            anyhow::bail!("error_strategy must be 'catch', 'ignore', or 'log'");
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_try_catch_node() {
        let node = TryCatchNode::new();
        let context = NodeContext::new("exec-123".to_string(), "node-1".to_string());
        let params = json!({
            "continue_on_error": true,
            "error_strategy": "catch"
        });

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
    }

    #[test]
    fn test_try_catch_validation() {
        let node = TryCatchNode::new();

        let params = json!({"error_strategy": "catch"});
        assert!(node.validate_parameters(&params).is_ok());

        let params = json!({"error_strategy": "invalid"});
        assert!(node.validate_parameters(&params).is_err());
    }
}
