use crate::models::{Node, NodeCategory, NodeContext, NodeOutput, NodeSubcategory, NodeType};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

/// Split node that executes multiple branches in parallel
#[derive(Clone)]
pub struct SplitNode {}

#[derive(Debug, Serialize, Deserialize)]
struct SplitParams {
    /// Number of parallel branches
    branches: Option<usize>,
    /// Whether to wait for all branches to complete
    wait_for_all: Option<bool>,
}

impl SplitNode {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for SplitNode {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeType for SplitNode {
    fn type_name(&self) -> &str {
        "split"
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
                "branches": {
                    "type": "integer",
                    "description": "Number of parallel branches (default: 2)",
                    "minimum": 2,
                    "maximum": 10
                },
                "wait_for_all": {
                    "type": "boolean",
                    "description": "Wait for all branches to complete (default: true)"
                }
            }
        })
    }
}

#[async_trait]
impl Node for SplitNode {
    async fn execute(
        &self,
        _context: &NodeContext,
        parameters: &serde_json::Value,
    ) -> anyhow::Result<NodeOutput> {
        let params: SplitParams = serde_json::from_value(parameters.clone())?;

        let branches = params.branches.unwrap_or(2);
        let wait_for_all = params.wait_for_all.unwrap_or(true);

        // In a real implementation, this would spawn multiple parallel execution paths

        let result = json!({
            "branches": branches,
            "wait_for_all": wait_for_all,
            "split_initiated": true
        });

        Ok(NodeOutput::success(result))
    }

    fn validate_parameters(&self, parameters: &Value) -> anyhow::Result<()> {
        let params: SplitParams = serde_json::from_value(parameters.clone())?;

        if let Some(branches) = params.branches
            && !(2..=10).contains(&branches)
        {
            anyhow::bail!("branches must be between 2 and 10");
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_split_node() {
        let node = SplitNode::new();
        let context = NodeContext::new("exec-123".to_string(), "node-1".to_string());
        let params = json!({"branches": 3});

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
    }
}
