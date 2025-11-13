use crate::models::{Node, NodeCategory, NodeContext, NodeOutput, NodeSubcategory, NodeType};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

/// Sort node that sorts arrays
#[derive(Clone)]
pub struct SortNode {}

#[derive(Debug, Serialize, Deserialize)]
struct SortParams {
    /// Array to sort (can be a reference like $items or direct array)
    items: Value,
    /// Field name to sort by (optional for primitive arrays)
    sort_by: Option<String>,
    /// Sort order: asc (ascending) or desc (descending)
    #[serde(default = "default_order")]
    order: String,
}

fn default_order() -> String {
    "asc".to_string()
}

impl SortNode {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for SortNode {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeType for SortNode {
    fn type_name(&self) -> &str {
        "sort"
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
                    "description": "Array to sort or variable reference (e.g., $items)",
                    "oneOf": [
                        {"type": "array"},
                        {"type": "string", "pattern": "^\\$[a-zA-Z_][a-zA-Z0-9_]*$"}
                    ]
                },
                "sort_by": {
                    "type": "string",
                    "description": "Field name to sort by (supports nested fields with dot notation). Required for object arrays, optional for primitive arrays"
                },
                "order": {
                    "type": "string",
                    "enum": ["asc", "desc"],
                    "default": "asc",
                    "description": "Sort order: asc (ascending) or desc (descending)"
                }
            },
            "required": ["items"]
        })
    }
}

#[async_trait]
impl Node for SortNode {
    async fn execute(
        &self,
        context: &NodeContext,
        parameters: &serde_json::Value,
    ) -> anyhow::Result<NodeOutput> {
        let params: SortParams = serde_json::from_value(parameters.clone())?;

        // Get the array to sort
        let mut items_array = if params.items.is_array() {
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

        // Sort the array
        if let Some(field) = &params.sort_by {
            // Sort by field
            items_array.sort_by(|a, b| {
                let a_val = get_field_value(a, field);
                let b_val = get_field_value(b, field);

                let cmp = compare_values(&a_val, &b_val);

                if params.order == "desc" {
                    cmp.reverse()
                } else {
                    cmp
                }
            });
        } else {
            // Sort primitive values directly
            items_array.sort_by(|a, b| {
                let cmp = compare_values(&Some(a.clone()), &Some(b.clone()));

                if params.order == "desc" {
                    cmp.reverse()
                } else {
                    cmp
                }
            });
        }

        let result = json!({
            "sorted_items": items_array,
            "count": items_array.len()
        });

        Ok(NodeOutput::success(result))
    }

    fn validate_parameters(&self, parameters: &Value) -> anyhow::Result<()> {
        let params: SortParams = serde_json::from_value(parameters.clone())
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

        // Validate order
        if params.order != "asc" && params.order != "desc" {
            anyhow::bail!("order must be 'asc' or 'desc'");
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

/// Compare two JSON values for sorting
fn compare_values(a: &Option<Value>, b: &Option<Value>) -> std::cmp::Ordering {
    match (a, b) {
        (None, None) => std::cmp::Ordering::Equal,
        (None, Some(_)) => std::cmp::Ordering::Less,
        (Some(_), None) => std::cmp::Ordering::Greater,
        (Some(a_val), Some(b_val)) => {
            // Try to compare as numbers
            if let (Some(a_num), Some(b_num)) = (a_val.as_f64(), b_val.as_f64()) {
                return a_num
                    .partial_cmp(&b_num)
                    .unwrap_or(std::cmp::Ordering::Equal);
            }

            // Try to compare as strings
            if let (Some(a_str), Some(b_str)) = (a_val.as_str(), b_val.as_str()) {
                return a_str.cmp(b_str);
            }

            // Try to compare as booleans
            if let (Some(a_bool), Some(b_bool)) = (a_val.as_bool(), b_val.as_bool()) {
                return a_bool.cmp(&b_bool);
            }

            // Default: compare as strings
            a_val.to_string().cmp(&b_val.to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_sort_node_ascending() {
        let node = SortNode::new();
        let context = NodeContext::new("exec-123".to_string(), "node-1".to_string());
        let params = json!({
            "items": [
                {"id": 3, "name": "Charlie"},
                {"id": 1, "name": "Alice"},
                {"id": 2, "name": "Bob"}
            ],
            "sort_by": "id",
            "order": "asc"
        });

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
        assert_eq!(output.data["sorted_items"][0]["id"], 1);
        assert_eq!(output.data["sorted_items"][1]["id"], 2);
        assert_eq!(output.data["sorted_items"][2]["id"], 3);
    }

    #[tokio::test]
    async fn test_sort_node_descending() {
        let node = SortNode::new();
        let context = NodeContext::new("exec-123".to_string(), "node-1".to_string());
        let params = json!({
            "items": [
                {"price": 100},
                {"price": 250},
                {"price": 150}
            ],
            "sort_by": "price",
            "order": "desc"
        });

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert_eq!(output.data["sorted_items"][0]["price"], 250);
        assert_eq!(output.data["sorted_items"][1]["price"], 150);
        assert_eq!(output.data["sorted_items"][2]["price"], 100);
    }

    #[tokio::test]
    async fn test_sort_node_primitive_array() {
        let node = SortNode::new();
        let mut context = NodeContext::new("exec-123".to_string(), "node-1".to_string());

        let items = json!([5, 2, 8, 1, 9]);
        context.set_variable("numbers".to_string(), items);

        let params = json!({
            "items": "$numbers",
            "order": "asc"
        });

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert_eq!(output.data["sorted_items"][0], 1);
        assert_eq!(output.data["sorted_items"][1], 2);
        assert_eq!(output.data["sorted_items"][4], 9);
    }

    #[test]
    fn test_sort_node_validation() {
        let node = SortNode::new();

        // Valid parameters
        let params = json!({
            "items": [3, 1, 2],
            "order": "asc"
        });
        assert!(node.validate_parameters(&params).is_ok());

        // Valid with sort_by
        let params = json!({
            "items": [{"id": 1}],
            "sort_by": "id",
            "order": "desc"
        });
        assert!(node.validate_parameters(&params).is_ok());

        // Invalid order
        let params = json!({
            "items": [1, 2, 3],
            "order": "invalid"
        });
        assert!(node.validate_parameters(&params).is_err());
    }
}
