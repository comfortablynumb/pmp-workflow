use crate::models::{Node, NodeCategory, NodeContext, NodeOutput, NodeSubcategory, NodeType};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use tokio::time::Duration;

/// Retry node with exponential backoff
#[derive(Clone)]
pub struct RetryNode {}

#[derive(Debug, Serialize, Deserialize)]
struct RetryParams {
    /// Maximum number of retry attempts (default: 3)
    max_attempts: Option<u32>,
    /// Initial delay in milliseconds (default: 1000)
    initial_delay_ms: Option<u64>,
    /// Backoff multiplier (default: 2.0)
    backoff_multiplier: Option<f64>,
    /// Maximum delay in milliseconds (default: 30000)
    max_delay_ms: Option<u64>,
    /// Retry on specific error patterns (regex)
    retry_on_errors: Option<Vec<String>>,
}

impl RetryNode {
    pub fn new() -> Self {
        Self {}
    }

    fn calculate_delay(&self, attempt: u32, params: &RetryParams) -> Duration {
        let initial_delay = params.initial_delay_ms.unwrap_or(1000);
        let multiplier = params.backoff_multiplier.unwrap_or(2.0);
        let max_delay = params.max_delay_ms.unwrap_or(30000);

        let delay = (initial_delay as f64) * multiplier.powi(attempt as i32);
        let capped_delay = delay.min(max_delay as f64) as u64;

        Duration::from_millis(capped_delay)
    }
}

impl Default for RetryNode {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeType for RetryNode {
    fn type_name(&self) -> &str {
        "retry"
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
                "max_attempts": {
                    "type": "integer",
                    "description": "Maximum retry attempts (default: 3)",
                    "minimum": 1,
                    "maximum": 10
                },
                "initial_delay_ms": {
                    "type": "integer",
                    "description": "Initial delay in milliseconds (default: 1000)",
                    "minimum": 0
                },
                "backoff_multiplier": {
                    "type": "number",
                    "description": "Exponential backoff multiplier (default: 2.0)",
                    "minimum": 1.0,
                    "maximum": 10.0
                },
                "max_delay_ms": {
                    "type": "integer",
                    "description": "Maximum delay in milliseconds (default: 30000)",
                    "minimum": 0
                },
                "retry_on_errors": {
                    "type": "array",
                    "items": {"type": "string"},
                    "description": "Retry only on errors matching these patterns"
                }
            }
        })
    }
}

#[async_trait]
impl Node for RetryNode {
    async fn execute(
        &self,
        context: &NodeContext,
        parameters: &serde_json::Value,
    ) -> anyhow::Result<NodeOutput> {
        let params: RetryParams = serde_json::from_value(parameters.clone())?;

        let max_attempts = params.max_attempts.unwrap_or(3);

        // In a real implementation, this would wrap child node execution
        // and retry on failures with exponential backoff

        // Simulate retry logic
        let delays: Vec<u64> = (0..max_attempts)
            .map(|i| self.calculate_delay(i, &params).as_millis() as u64)
            .collect();

        let result = json!({
            "max_attempts": max_attempts,
            "retry_strategy": "exponential_backoff",
            "delays_ms": delays,
            "initial_delay_ms": params.initial_delay_ms.unwrap_or(1000),
            "backoff_multiplier": params.backoff_multiplier.unwrap_or(2.0),
            "max_delay_ms": params.max_delay_ms.unwrap_or(30000),
            "context_execution_id": context.execution_id
        });

        Ok(NodeOutput::success(result))
    }

    fn validate_parameters(&self, parameters: &Value) -> anyhow::Result<()> {
        let params: RetryParams = serde_json::from_value(parameters.clone())?;

        if let Some(attempts) = params.max_attempts
            && !(1..=10).contains(&attempts)
        {
            anyhow::bail!("max_attempts must be between 1 and 10");
        }

        if let Some(multiplier) = params.backoff_multiplier
            && !(1.0..=10.0).contains(&multiplier)
        {
            anyhow::bail!("backoff_multiplier must be between 1.0 and 10.0");
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_retry_node() {
        let node = RetryNode::new();
        let context = NodeContext::new("exec-123".to_string(), "node-1".to_string());
        let params = json!({
            "max_attempts": 3,
            "initial_delay_ms": 100,
            "backoff_multiplier": 2.0
        });

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert_eq!(output.data["max_attempts"], 3);
    }

    #[test]
    fn test_retry_backoff_calculation() {
        let node = RetryNode::new();
        let params = RetryParams {
            max_attempts: Some(3),
            initial_delay_ms: Some(100),
            backoff_multiplier: Some(2.0),
            max_delay_ms: Some(10000),
            retry_on_errors: None,
        };

        assert_eq!(node.calculate_delay(0, &params).as_millis(), 100);
        assert_eq!(node.calculate_delay(1, &params).as_millis(), 200);
        assert_eq!(node.calculate_delay(2, &params).as_millis(), 400);
    }
}
