use crate::models::{Node, NodeCategory, NodeContext, NodeOutput, NodeSubcategory, NodeType};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

/// Circuit Breaker node to prevent cascading failures
#[derive(Clone)]
pub struct CircuitBreakerNode {}

#[derive(Debug, Serialize, Deserialize)]
struct CircuitBreakerParams {
    /// Failure threshold to open circuit (default: 5)
    failure_threshold: Option<u32>,
    /// Success threshold to close circuit (default: 2)
    success_threshold: Option<u32>,
    /// Timeout in seconds before attempting half-open (default: 60)
    timeout_seconds: Option<u64>,
    /// Circuit identifier for tracking state
    circuit_id: Option<String>,
}

impl CircuitBreakerNode {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for CircuitBreakerNode {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeType for CircuitBreakerNode {
    fn type_name(&self) -> &str {
        "circuit_breaker"
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
                "failure_threshold": {
                    "type": "integer",
                    "description": "Number of failures before opening circuit (default: 5)",
                    "minimum": 1,
                    "maximum": 100
                },
                "success_threshold": {
                    "type": "integer",
                    "description": "Number of successes to close circuit (default: 2)",
                    "minimum": 1,
                    "maximum": 10
                },
                "timeout_seconds": {
                    "type": "integer",
                    "description": "Seconds before attempting half-open (default: 60)",
                    "minimum": 1,
                    "maximum": 3600
                },
                "circuit_id": {
                    "type": "string",
                    "description": "Unique identifier for this circuit"
                }
            }
        })
    }
}

#[async_trait]
impl Node for CircuitBreakerNode {
    async fn execute(
        &self,
        context: &NodeContext,
        parameters: &serde_json::Value,
    ) -> anyhow::Result<NodeOutput> {
        let params: CircuitBreakerParams = serde_json::from_value(parameters.clone())?;

        let failure_threshold = params.failure_threshold.unwrap_or(5);
        let success_threshold = params.success_threshold.unwrap_or(2);
        let timeout_seconds = params.timeout_seconds.unwrap_or(60);
        let circuit_id = params
            .circuit_id
            .unwrap_or_else(|| format!("circuit-{}", context.node_id));

        // In a real implementation, this would:
        // 1. Check circuit state from shared state store (Redis, etc.)
        // 2. If OPEN: return error or fallback immediately
        // 3. If HALF_OPEN: allow one request through, update state based on result
        // 4. If CLOSED: execute normally, track failures
        // 5. Update circuit state based on thresholds

        let result = json!({
            "circuit_id": circuit_id,
            "failure_threshold": failure_threshold,
            "success_threshold": success_threshold,
            "timeout_seconds": timeout_seconds,
            "current_state": "closed", // Would be tracked in state store
            "context_execution_id": context.execution_id
        });

        Ok(NodeOutput::success(result))
    }

    fn validate_parameters(&self, parameters: &Value) -> anyhow::Result<()> {
        let params: CircuitBreakerParams = serde_json::from_value(parameters.clone())?;

        if let Some(threshold) = params.failure_threshold
            && !(1..=100).contains(&threshold)
        {
            anyhow::bail!("failure_threshold must be between 1 and 100");
        }

        if let Some(threshold) = params.success_threshold
            && !(1..=10).contains(&threshold)
        {
            anyhow::bail!("success_threshold must be between 1 and 10");
        }

        if let Some(timeout) = params.timeout_seconds
            && !(1..=3600).contains(&timeout)
        {
            anyhow::bail!("timeout_seconds must be between 1 and 3600");
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_circuit_breaker_node() {
        let node = CircuitBreakerNode::new();
        let context = NodeContext::new("exec-123".to_string(), "node-1".to_string());
        let params = json!({
            "failure_threshold": 5,
            "success_threshold": 2,
            "timeout_seconds": 60
        });

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert_eq!(output.data["failure_threshold"], 5);
        assert_eq!(output.data["current_state"], "closed");
    }

    #[test]
    fn test_circuit_breaker_validation() {
        let node = CircuitBreakerNode::new();

        // Valid
        let params = json!({"failure_threshold": 5});
        assert!(node.validate_parameters(&params).is_ok());

        // Invalid threshold
        let params = json!({"failure_threshold": 200});
        assert!(node.validate_parameters(&params).is_err());
    }
}
