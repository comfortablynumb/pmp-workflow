use crate::models::{Node, NodeCategory, NodeContext, NodeOutput, NodeSubcategory, NodeType};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

/// Flatten node that flattens nested arrays/objects
#[derive(Clone)]
pub struct FlattenNode {}

#[derive(Debug, Serialize, Deserialize)]
struct FlattenParams {
    /// Array or nested structure to flatten (can be a reference like $items or direct array)
    items: Value,
    /// Depth to flatten: number or "infinite"
    #[serde(default = "default_depth")]
    depth: FlattenDepth,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
enum FlattenDepth {
    Number(usize),
    Infinite(String),
}

fn default_depth() -> FlattenDepth {
    FlattenDepth::Number(1)
}

impl FlattenNode {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for FlattenNode {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeType for FlattenNode {
    fn type_name(&self) -> &str {
        "flatten"
    }

    fn category(&self) -> NodeCategory {
        NodeCategory::Action
    }

    fn subcategory(&self) -> NodeSubcategory {
        NodeSubcategory::General
    }

    fn parameter_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "items": {
                    "description": "Array or nested structure to flatten, or variable reference (e.g., $items)",
                    "oneOf": [
                        {"type": "array"},
                        {"type": "string", "pattern": "^\\$[a-zA-Z_][a-zA-Z0-9_]*$"}
                    ]
                },
                "depth": {
                    "description": "Depth to flatten: a number (e.g., 1, 2) or 'infinite' for complete flattening",
                    "oneOf": [
                        {"type": "integer", "minimum": 1},
                        {"type": "string", "enum": ["infinite"]}
                    ],
                    "default": 1
                }
            },
            "required": ["items"]
        })
    }
}

#[async_trait]
impl Node for FlattenNode {
    async fn execute(
        &self,
        context: &NodeContext,
        parameters: &serde_json::Value,
    ) -> anyhow::Result<NodeOutput> {
        let params: FlattenParams = serde_json::from_value(parameters.clone())?;

        // Get the array to flatten
        let items_value = if params.items.is_array() {
            params.items.clone()
        } else if params.items.is_string() {
            // Try to resolve from context variables
            let var_name = params.items.as_str().unwrap().trim_start_matches('$');
            if let Some(value) = context.get_variable(var_name) {
                value.clone()
            } else {
                anyhow::bail!("Variable '{}' not found in context", var_name);
            }
        } else {
            anyhow::bail!("items must be an array or a variable reference");
        };

        // Determine max depth
        let max_depth = match params.depth {
            FlattenDepth::Number(n) => Some(n),
            FlattenDepth::Infinite(ref s) if s == "infinite" => None,
            _ => anyhow::bail!("Invalid depth value"),
        };

        // Flatten the structure
        let flattened = flatten_value(&items_value, max_depth, 0);

        let result = json!({
            "flattened_items": flattened,
            "count": if let Some(arr) = flattened.as_array() {
                arr.len()
            } else {
                1
            }
        });

        Ok(NodeOutput::success(result))
    }

    fn validate_parameters(&self, parameters: &Value) -> anyhow::Result<()> {
        let params: FlattenParams = serde_json::from_value(parameters.clone())
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

        // Validate depth
        match params.depth {
            FlattenDepth::Number(n) => {
                if n < 1 {
                    anyhow::bail!("depth must be at least 1");
                }
            }
            FlattenDepth::Infinite(ref s) => {
                if s != "infinite" {
                    anyhow::bail!("depth string must be 'infinite'");
                }
            }
        }

        Ok(())
    }
}

/// Flatten a value recursively up to a certain depth
fn flatten_value(value: &Value, max_depth: Option<usize>, current_depth: usize) -> Value {
    // Check if we've reached max depth
    if let Some(max) = max_depth
        && current_depth >= max
    {
        return value.clone();
    }

    match value {
        Value::Array(arr) => {
            let mut result = Vec::new();

            for item in arr {
                match item {
                    Value::Array(_) => {
                        // Recursively flatten nested arrays
                        let flattened = flatten_value(item, max_depth, current_depth + 1);
                        if let Value::Array(inner) = flattened {
                            result.extend(inner);
                        } else {
                            result.push(flattened);
                        }
                    }
                    _ => {
                        result.push(item.clone());
                    }
                }
            }

            Value::Array(result)
        }
        other => other.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_flatten_node_depth_1() {
        let node = FlattenNode::new();
        let context = NodeContext::new("exec-123".to_string(), "node-1".to_string());
        let params = json!({
            "items": [
                [1, 2],
                [3, 4],
                [5, 6]
            ],
            "depth": 1
        });

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
        assert_eq!(output.data["count"], 6);
        assert_eq!(output.data["flattened_items"], json!([1, 2, 3, 4, 5, 6]));
    }

    #[tokio::test]
    async fn test_flatten_node_depth_2() {
        let node = FlattenNode::new();
        let context = NodeContext::new("exec-123".to_string(), "node-1".to_string());
        let params = json!({
            "items": [
                [[1, 2], [3, 4]],
                [[5, 6], [7, 8]]
            ],
            "depth": 2
        });

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert_eq!(output.data["count"], 8);
        assert_eq!(
            output.data["flattened_items"],
            json!([1, 2, 3, 4, 5, 6, 7, 8])
        );
    }

    #[tokio::test]
    async fn test_flatten_node_infinite() {
        let node = FlattenNode::new();
        let mut context = NodeContext::new("exec-123".to_string(), "node-1".to_string());

        let items = json!([[1, [2, [3, [4, [5]]]]], [6, [7, [8]]]]);
        context.set_variable("nested".to_string(), items);

        let params = json!({
            "items": "$nested",
            "depth": "infinite"
        });

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert_eq!(output.data["count"], 8);
        assert_eq!(
            output.data["flattened_items"],
            json!([1, 2, 3, 4, 5, 6, 7, 8])
        );
    }

    #[tokio::test]
    async fn test_flatten_node_mixed_types() {
        let node = FlattenNode::new();
        let context = NodeContext::new("exec-123".to_string(), "node-1".to_string());
        let params = json!({
            "items": [
                ["a", "b"],
                ["c"],
                ["d", "e", "f"]
            ],
            "depth": 1
        });

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert_eq!(output.data["count"], 6);
        assert_eq!(
            output.data["flattened_items"],
            json!(["a", "b", "c", "d", "e", "f"])
        );
    }

    #[test]
    fn test_flatten_node_validation() {
        let node = FlattenNode::new();

        // Valid parameters
        let params = json!({
            "items": [[1, 2], [3, 4]],
            "depth": 1
        });
        assert!(node.validate_parameters(&params).is_ok());

        // Valid with infinite
        let params = json!({
            "items": [[1, 2]],
            "depth": "infinite"
        });
        assert!(node.validate_parameters(&params).is_ok());

        // Valid with variable reference
        let params = json!({
            "items": "$my_items",
            "depth": 2
        });
        assert!(node.validate_parameters(&params).is_ok());
    }
}
