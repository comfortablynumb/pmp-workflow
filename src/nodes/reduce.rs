use crate::models::{Node, NodeCategory, NodeContext, NodeOutput, NodeSubcategory, NodeType};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

/// Reduce node that aggregates array values
#[derive(Clone)]
pub struct ReduceNode {}

#[derive(Debug, Serialize, Deserialize)]
struct ReduceParams {
    /// Array to aggregate (can be a reference like $items or direct array)
    items: Value,
    /// Operation: sum, avg, min, max, count, concat
    operation: String,
    /// Field name to aggregate (optional, used for object arrays)
    field: Option<String>,
}

impl ReduceNode {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for ReduceNode {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeType for ReduceNode {
    fn type_name(&self) -> &str {
        "reduce"
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
                    "description": "Array to aggregate or variable reference (e.g., $items)",
                    "oneOf": [
                        {"type": "array"},
                        {"type": "string", "pattern": "^\\$[a-zA-Z_][a-zA-Z0-9_]*$"}
                    ]
                },
                "operation": {
                    "type": "string",
                    "enum": ["sum", "avg", "min", "max", "count", "concat"],
                    "description": "Aggregation operation to perform"
                },
                "field": {
                    "type": "string",
                    "description": "Field name to aggregate (for object arrays, supports dot notation)"
                }
            },
            "required": ["items", "operation"]
        })
    }
}

#[async_trait]
impl Node for ReduceNode {
    async fn execute(
        &self,
        context: &NodeContext,
        parameters: &serde_json::Value,
    ) -> anyhow::Result<NodeOutput> {
        let params: ReduceParams = serde_json::from_value(parameters.clone())?;

        // Get the array to aggregate
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

        // Extract values to aggregate
        let values: Vec<Value> = if let Some(field) = &params.field {
            items_array
                .iter()
                .filter_map(|item| get_field_value(item, field))
                .collect()
        } else {
            items_array
        };

        // Perform aggregation
        let result_value = match params.operation.as_str() {
            "count" => json!(values.len()),
            "sum" => {
                let sum: f64 = values.iter().filter_map(|v| v.as_f64()).sum();
                json!(sum)
            }
            "avg" => {
                let nums: Vec<f64> = values.iter().filter_map(|v| v.as_f64()).collect();
                if nums.is_empty() {
                    json!(null)
                } else {
                    let avg = nums.iter().sum::<f64>() / nums.len() as f64;
                    json!(avg)
                }
            }
            "min" => values
                .iter()
                .filter_map(|v| v.as_f64())
                .min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
                .map(|v| json!(v))
                .unwrap_or(json!(null)),
            "max" => values
                .iter()
                .filter_map(|v| v.as_f64())
                .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
                .map(|v| json!(v))
                .unwrap_or(json!(null)),
            "concat" => {
                let strings: Vec<String> = values
                    .iter()
                    .map(|v| {
                        if let Some(s) = v.as_str() {
                            s.to_string()
                        } else {
                            v.to_string()
                        }
                    })
                    .collect();
                json!(strings.join(""))
            }
            _ => anyhow::bail!("Unsupported operation: {}", params.operation),
        };

        let result = json!({
            "result": result_value,
            "operation": params.operation,
            "count": values.len()
        });

        Ok(NodeOutput::success(result))
    }

    fn validate_parameters(&self, parameters: &Value) -> anyhow::Result<()> {
        let params: ReduceParams = serde_json::from_value(parameters.clone())
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

        // Validate operation
        let valid_operations = ["sum", "avg", "min", "max", "count", "concat"];
        if !valid_operations.contains(&params.operation.as_str()) {
            anyhow::bail!(
                "Invalid operation: {}. Must be one of: {:?}",
                params.operation,
                valid_operations
            );
        }

        Ok(())
    }
}

/// Helper function to extract a field value from an object using dot notation
fn get_field_value(item: &Value, field: &str) -> Option<Value> {
    let parts: Vec<&str> = field.split('.').collect();
    let mut current = item;

    for part in parts {
        if part.is_empty() {
            continue;
        }
        current = current.get(part)?;
    }

    Some(current.clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_reduce_node_sum() {
        let node = ReduceNode::new();
        let context = NodeContext::new("exec-123".to_string(), "node-1".to_string());
        let params = json!({
            "items": [
                {"id": 1, "amount": 100},
                {"id": 2, "amount": 250},
                {"id": 3, "amount": 150}
            ],
            "operation": "sum",
            "field": "amount"
        });

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
        assert_eq!(output.data["result"], 500.0);
        assert_eq!(output.data["operation"], "sum");
    }

    #[tokio::test]
    async fn test_reduce_node_avg() {
        let node = ReduceNode::new();
        let context = NodeContext::new("exec-123".to_string(), "node-1".to_string());
        let params = json!({
            "items": [10, 20, 30, 40],
            "operation": "avg"
        });

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert_eq!(output.data["result"], 25.0);
    }

    #[tokio::test]
    async fn test_reduce_node_count() {
        let node = ReduceNode::new();
        let mut context = NodeContext::new("exec-123".to_string(), "node-1".to_string());

        let items = json!([1, 2, 3, 4, 5]);
        context.set_variable("numbers".to_string(), items);

        let params = json!({
            "items": "$numbers",
            "operation": "count"
        });

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert_eq!(output.data["result"], 5);
    }

    #[test]
    fn test_reduce_node_validation() {
        let node = ReduceNode::new();

        // Valid parameters
        let params = json!({
            "items": [1, 2, 3],
            "operation": "sum"
        });
        assert!(node.validate_parameters(&params).is_ok());

        // Invalid - unsupported operation
        let params = json!({
            "items": [1, 2, 3],
            "operation": "multiply"
        });
        assert!(node.validate_parameters(&params).is_err());

        // Valid with field
        let params = json!({
            "items": [{"value": 1}],
            "operation": "max",
            "field": "value"
        });
        assert!(node.validate_parameters(&params).is_ok());
    }
}
