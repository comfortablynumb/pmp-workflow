use crate::models::{Node, NodeCategory, NodeContext, NodeOutput, NodeSubcategory, NodeType};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

/// Loop node that iterates over arrays
#[derive(Clone)]
pub struct LoopNode {}

#[derive(Debug, Serialize, Deserialize)]
struct LoopParams {
    /// Array to iterate over (can be a reference like $items or direct array)
    items: Value,
    /// Name of the variable to store current item (defaults to "item")
    item_variable: Option<String>,
    /// Name of the variable to store current index (defaults to "index")
    index_variable: Option<String>,
    /// Maximum iterations (safety limit, defaults to 1000)
    max_iterations: Option<usize>,
}

impl LoopNode {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for LoopNode {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeType for LoopNode {
    fn type_name(&self) -> &str {
        "loop"
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
                "items": {
                    "description": "Array to iterate over or variable reference (e.g., $items)",
                    "oneOf": [
                        {"type": "array"},
                        {"type": "string", "pattern": "^\\$[a-zA-Z_][a-zA-Z0-9_]*$"}
                    ]
                },
                "item_variable": {
                    "type": "string",
                    "description": "Variable name for current item (default: 'item')"
                },
                "index_variable": {
                    "type": "string",
                    "description": "Variable name for current index (default: 'index')"
                },
                "max_iterations": {
                    "type": "integer",
                    "description": "Maximum number of iterations (default: 1000)",
                    "minimum": 1,
                    "maximum": 10000
                }
            },
            "required": ["items"]
        })
    }
}

#[async_trait]
impl Node for LoopNode {
    async fn execute(
        &self,
        context: &NodeContext,
        parameters: &serde_json::Value,
    ) -> anyhow::Result<NodeOutput> {
        let params: LoopParams = serde_json::from_value(parameters.clone())?;

        // Get the array to iterate over
        let items_array = if params.items.is_array() {
            params.items.as_array().unwrap().clone()
        } else if params.items.is_string() {
            // Try to resolve from context variables
            let var_name = params.items.as_str().unwrap().trim_start_matches('$');
            if let Some(value) = context.get_variable(var_name) {
                if let Some(arr) = value.as_array() {
                    arr.clone()
                } else {
                    anyhow::bail!("Variable '{}' is not an array", var_name);
                }
            } else {
                anyhow::bail!("Variable '{}' not found in context", var_name);
            }
        } else {
            anyhow::bail!("items must be an array or a variable reference");
        };

        let max_iterations = params.max_iterations.unwrap_or(1000);
        let item_var = params.item_variable.unwrap_or_else(|| "item".to_string());
        let index_var = params.index_variable.unwrap_or_else(|| "index".to_string());

        if items_array.len() > max_iterations {
            anyhow::bail!(
                "Array size ({}) exceeds max_iterations ({})",
                items_array.len(),
                max_iterations
            );
        }

        // In a real implementation, this would trigger execution of child nodes for each item
        // For now, we'll return metadata about the loop
        let result = json!({
            "items_count": items_array.len(),
            "item_variable": item_var,
            "index_variable": index_var,
            "loop_metadata": {
                "total_iterations": items_array.len(),
                "max_iterations": max_iterations
            },
            // For demonstration, include the first few items
            "preview_items": items_array.iter().take(3).collect::<Vec<_>>()
        });

        Ok(NodeOutput::success(result))
    }

    fn validate_parameters(&self, parameters: &Value) -> anyhow::Result<()> {
        let params: LoopParams = serde_json::from_value(parameters.clone())
            .map_err(|e| anyhow::anyhow!("Invalid parameters: {}", e))?;

        // Validate items is either an array or a variable reference
        if !params.items.is_array() && !params.items.is_string() {
            anyhow::bail!("items must be an array or a variable reference (e.g., $items)");
        }

        if params.items.is_string() {
            let s = params.items.as_str().unwrap();
            if !s.starts_with('$') {
                anyhow::bail!("Variable reference must start with $ (e.g., $items)");
            }
        }

        // Validate max_iterations
        if let Some(max) = params.max_iterations {
            if !(1..=10000).contains(&max) {
                anyhow::bail!("max_iterations must be between 1 and 10000");
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_loop_node_with_direct_array() {
        let node = LoopNode::new();
        let context = NodeContext::new("exec-123".to_string(), "node-1".to_string());
        let params = json!({
            "items": [
                {"id": 1, "name": "Item 1"},
                {"id": 2, "name": "Item 2"},
                {"id": 3, "name": "Item 3"}
            ]
        });

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
        assert_eq!(output.data["items_count"], 3);
    }

    #[tokio::test]
    async fn test_loop_node_with_variable_reference() {
        let node = LoopNode::new();
        let mut context = NodeContext::new("exec-123".to_string(), "node-1".to_string());

        // Set up context with items variable
        let items = json!([1, 2, 3, 4, 5]);
        context.set_variable("items".to_string(), items);

        let params = json!({
            "items": "$items",
            "item_variable": "current_item",
            "index_variable": "idx"
        });

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert_eq!(output.data["items_count"], 5);
        assert_eq!(output.data["item_variable"], "current_item");
    }

    #[test]
    fn test_loop_node_validation() {
        let node = LoopNode::new();

        // Valid with direct array
        let params = json!({"items": [1, 2, 3]});
        assert!(node.validate_parameters(&params).is_ok());

        // Valid with variable reference
        let params = json!({"items": "$my_items"});
        assert!(node.validate_parameters(&params).is_ok());

        // Invalid - not an array or variable
        let params = json!({"items": 123});
        assert!(node.validate_parameters(&params).is_err());

        // Invalid - variable reference without $
        let params = json!({"items": "my_items"});
        assert!(node.validate_parameters(&params).is_err());
    }
}
