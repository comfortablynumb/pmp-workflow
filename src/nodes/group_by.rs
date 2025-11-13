use crate::models::{Node, NodeCategory, NodeContext, NodeOutput, NodeSubcategory, NodeType};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::collections::HashMap;

/// GroupBy node that groups items by field value
#[derive(Clone)]
pub struct GroupByNode {}

#[derive(Debug, Serialize, Deserialize)]
struct GroupByParams {
    /// Array to group (can be a reference like $items or direct array)
    items: Value,
    /// Field name to group by
    group_by: String,
}

impl GroupByNode {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for GroupByNode {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeType for GroupByNode {
    fn type_name(&self) -> &str {
        "group_by"
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
                    "description": "Array to group or variable reference (e.g., $items)",
                    "oneOf": [
                        {"type": "array"},
                        {"type": "string", "pattern": "^\\$[a-zA-Z_][a-zA-Z0-9_]*$"}
                    ]
                },
                "group_by": {
                    "type": "string",
                    "description": "Field name to group by (supports nested fields with dot notation)"
                }
            },
            "required": ["items", "group_by"]
        })
    }
}

#[async_trait]
impl Node for GroupByNode {
    async fn execute(
        &self,
        context: &NodeContext,
        parameters: &serde_json::Value,
    ) -> anyhow::Result<NodeOutput> {
        let params: GroupByParams = serde_json::from_value(parameters.clone())?;

        // Get the array to group
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

        // Group items by field value
        let mut groups: HashMap<String, Vec<Value>> = HashMap::new();

        for item in items_array {
            let group_key = if let Some(field_value) = get_field_value(&item, &params.group_by) {
                // Convert the field value to a string key
                match field_value {
                    Value::String(s) => s,
                    Value::Number(n) => n.to_string(),
                    Value::Bool(b) => b.to_string(),
                    Value::Null => "null".to_string(),
                    _ => field_value.to_string(),
                }
            } else {
                // If field doesn't exist, group under "undefined"
                "undefined".to_string()
            };

            groups.entry(group_key).or_default().push(item);
        }

        // Convert HashMap to JSON object
        let mut result_obj = serde_json::Map::new();
        for (key, values) in groups {
            result_obj.insert(key, json!(values));
        }

        let result = json!({
            "groups": Value::Object(result_obj.clone()),
            "group_count": result_obj.len()
        });

        Ok(NodeOutput::success(result))
    }

    fn validate_parameters(&self, parameters: &Value) -> anyhow::Result<()> {
        let params: GroupByParams = serde_json::from_value(parameters.clone())
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

        // Validate group_by is not empty
        if params.group_by.is_empty() {
            anyhow::bail!("group_by field name cannot be empty");
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
    async fn test_group_by_node_simple() {
        let node = GroupByNode::new();
        let context = NodeContext::new("exec-123".to_string(), "node-1".to_string());
        let params = json!({
            "items": [
                {"id": 1, "category": "fruits", "name": "apple"},
                {"id": 2, "category": "vegetables", "name": "carrot"},
                {"id": 3, "category": "fruits", "name": "banana"},
                {"id": 4, "category": "vegetables", "name": "lettuce"}
            ],
            "group_by": "category"
        });

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
        assert_eq!(output.data["group_count"], 2);

        let groups = output.data["groups"].as_object().unwrap();
        assert!(groups.contains_key("fruits"));
        assert!(groups.contains_key("vegetables"));
        assert_eq!(groups["fruits"].as_array().unwrap().len(), 2);
        assert_eq!(groups["vegetables"].as_array().unwrap().len(), 2);
    }

    #[tokio::test]
    async fn test_group_by_node_nested_field() {
        let node = GroupByNode::new();
        let context = NodeContext::new("exec-123".to_string(), "node-1".to_string());
        let params = json!({
            "items": [
                {"id": 1, "user": {"role": "admin"}},
                {"id": 2, "user": {"role": "user"}},
                {"id": 3, "user": {"role": "admin"}}
            ],
            "group_by": "user.role"
        });

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert_eq!(output.data["group_count"], 2);

        let groups = output.data["groups"].as_object().unwrap();
        assert_eq!(groups["admin"].as_array().unwrap().len(), 2);
        assert_eq!(groups["user"].as_array().unwrap().len(), 1);
    }

    #[tokio::test]
    async fn test_group_by_node_with_variable() {
        let node = GroupByNode::new();
        let mut context = NodeContext::new("exec-123".to_string(), "node-1".to_string());

        let items = json!([
            {"id": 1, "status": "active"},
            {"id": 2, "status": "inactive"},
            {"id": 3, "status": "active"},
            {"id": 4, "status": "pending"}
        ]);
        context.set_variable("items".to_string(), items);

        let params = json!({
            "items": "$items",
            "group_by": "status"
        });

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert_eq!(output.data["group_count"], 3);

        let groups = output.data["groups"].as_object().unwrap();
        assert!(groups.contains_key("active"));
        assert!(groups.contains_key("inactive"));
        assert!(groups.contains_key("pending"));
    }

    #[test]
    fn test_group_by_node_validation() {
        let node = GroupByNode::new();

        // Valid parameters
        let params = json!({
            "items": [{"type": "a"}],
            "group_by": "type"
        });
        assert!(node.validate_parameters(&params).is_ok());

        // Invalid - missing group_by
        let params = json!({
            "items": [1, 2, 3]
        });
        assert!(node.validate_parameters(&params).is_err());

        // Valid with variable reference
        let params = json!({
            "items": "$my_items",
            "group_by": "category"
        });
        assert!(node.validate_parameters(&params).is_ok());
    }
}
