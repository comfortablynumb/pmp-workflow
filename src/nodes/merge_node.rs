use crate::models::{Node, NodeCategory, NodeContext, NodeOutput, NodeSubcategory, NodeType};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

/// Merge node that combines multiple execution paths
#[derive(Clone)]
pub struct MergeNode {}

#[derive(Debug, Serialize, Deserialize)]
struct MergeParams {
    /// Strategy for merging: "all" (wait for all), "any" (first to complete), "majority"
    strategy: Option<String>,
    /// Timeout in seconds (optional)
    timeout: Option<u64>,
    /// How to combine outputs: "array", "object", "first", "last"
    combine_mode: Option<String>,
}

impl MergeNode {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for MergeNode {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeType for MergeNode {
    fn type_name(&self) -> &str {
        "merge"
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
                "strategy": {
                    "type": "string",
                    "enum": ["all", "any", "majority"],
                    "description": "Merge strategy (default: 'all')"
                },
                "timeout": {
                    "type": "integer",
                    "description": "Timeout in seconds"
                },
                "combine_mode": {
                    "type": "string",
                    "enum": ["array", "object", "first", "last"],
                    "description": "How to combine outputs (default: 'object')"
                }
            }
        })
    }
}

#[async_trait]
impl Node for MergeNode {
    async fn execute(
        &self,
        context: &NodeContext,
        parameters: &serde_json::Value,
    ) -> anyhow::Result<NodeOutput> {
        let params: MergeParams = serde_json::from_value(parameters.clone())?;

        let strategy = params.strategy.unwrap_or_else(|| "all".to_string());
        let combine_mode = params.combine_mode.unwrap_or_else(|| "object".to_string());

        // In a real implementation, this would wait for multiple paths to complete
        // and merge their outputs according to the strategy

        let result = json!({
            "strategy": strategy,
            "combine_mode": combine_mode,
            "timeout": params.timeout,
            "merged": true,
            "inputs_merged": context.inputs.len()
        });

        Ok(NodeOutput::success(result))
    }

    fn validate_parameters(&self, parameters: &Value) -> anyhow::Result<()> {
        let params: MergeParams = serde_json::from_value(parameters.clone())?;

        if let Some(strategy) = &params.strategy {
            if !["all", "any", "majority"].contains(&strategy.as_str()) {
                anyhow::bail!("strategy must be 'all', 'any', or 'majority'");
            }
        }

        if let Some(mode) = &params.combine_mode {
            if !["array", "object", "first", "last"].contains(&mode.as_str()) {
                anyhow::bail!("combine_mode must be 'array', 'object', 'first', or 'last'");
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_merge_node() {
        let node = MergeNode::new();
        let context = NodeContext::new("exec-123".to_string(), "node-1".to_string());
        let params = json!({"strategy": "all", "combine_mode": "object"});

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
    }
}
