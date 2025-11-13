use crate::models::{Node, NodeCategory, NodeContext, NodeOutput, NodeSubcategory, NodeType};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

/// Map node that transforms each item in an array
#[derive(Clone)]
pub struct MapNode {}

#[derive(Debug, Serialize, Deserialize)]
struct MapParams {
    /// Array to transform (can be a reference like $items or direct array)
    items: Value,
    /// Transform template or expression
    transform: Value,
}

impl MapNode {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for MapNode {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeType for MapNode {
    fn type_name(&self) -> &str {
        "map"
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
                    "description": "Array to transform or variable reference (e.g., $items)",
                    "oneOf": [
                        {"type": "array"},
                        {"type": "string", "pattern": "^\\$[a-zA-Z_][a-zA-Z0-9_]*$"}
                    ]
                },
                "transform": {
                    "description": "Template object for transformation. Use {{field}} for field substitution from each item",
                    "oneOf": [
                        {"type": "object"},
                        {"type": "string"}
                    ]
                }
            },
            "required": ["items", "transform"]
        })
    }
}

#[async_trait]
impl Node for MapNode {
    async fn execute(
        &self,
        context: &NodeContext,
        parameters: &serde_json::Value,
    ) -> anyhow::Result<NodeOutput> {
        let params: MapParams = serde_json::from_value(parameters.clone())?;

        // Get the array to transform
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

        // Transform each item
        let transformed: Result<Vec<Value>, anyhow::Error> = items_array
            .into_iter()
            .map(|item| transform_item(&item, &params.transform))
            .collect();

        let transformed = transformed?;

        let result = json!({
            "mapped_items": transformed,
            "count": transformed.len()
        });

        Ok(NodeOutput::success(result))
    }

    fn validate_parameters(&self, parameters: &Value) -> anyhow::Result<()> {
        let params: MapParams = serde_json::from_value(parameters.clone())
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

        // Validate transform is provided
        if params.transform.is_null() {
            anyhow::bail!("transform must be provided");
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

/// Transform a single item using the template
fn transform_item(item: &Value, template: &Value) -> anyhow::Result<Value> {
    match template {
        Value::String(s) => {
            // Check for template substitution syntax: {{field}}
            if s.starts_with("{{") && s.ends_with("}}") {
                let field = s[2..s.len() - 2].trim();
                get_field_value(item, field)
                    .ok_or_else(|| anyhow::anyhow!("Field '{}' not found in item", field))
            } else {
                // Return the string as-is
                Ok(Value::String(s.clone()))
            }
        }
        Value::Object(obj) => {
            let mut result = serde_json::Map::new();
            for (key, value) in obj {
                result.insert(key.clone(), transform_item(item, value)?);
            }
            Ok(Value::Object(result))
        }
        Value::Array(arr) => {
            let mut result = Vec::new();
            for value in arr {
                result.push(transform_item(item, value)?);
            }
            Ok(Value::Array(result))
        }
        other => Ok(other.clone()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_map_node_with_object_transform() {
        let node = MapNode::new();
        let context = NodeContext::new("exec-123".to_string(), "node-1".to_string());
        let params = json!({
            "items": [
                {"id": 1, "name": "Alice", "age": 30},
                {"id": 2, "name": "Bob", "age": 25}
            ],
            "transform": {
                "user_id": "{{id}}",
                "user_name": "{{name}}"
            }
        });

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
        assert_eq!(output.data["count"], 2);
        assert_eq!(output.data["mapped_items"][0]["user_id"], 1);
        assert_eq!(output.data["mapped_items"][0]["user_name"], "Alice");
        assert_eq!(output.data["mapped_items"][1]["user_id"], 2);
        assert_eq!(output.data["mapped_items"][1]["user_name"], "Bob");
    }

    #[tokio::test]
    async fn test_map_node_with_nested_fields() {
        let node = MapNode::new();
        let context = NodeContext::new("exec-123".to_string(), "node-1".to_string());
        let params = json!({
            "items": [
                {"id": 1, "user": {"name": "Alice", "email": "alice@example.com"}},
                {"id": 2, "user": {"name": "Bob", "email": "bob@example.com"}}
            ],
            "transform": {
                "id": "{{id}}",
                "name": "{{user.name}}",
                "email": "{{user.email}}"
            }
        });

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert_eq!(output.data["count"], 2);
        assert_eq!(output.data["mapped_items"][0]["name"], "Alice");
        assert_eq!(output.data["mapped_items"][0]["email"], "alice@example.com");
    }

    #[tokio::test]
    async fn test_map_node_with_variable_reference() {
        let node = MapNode::new();
        let mut context = NodeContext::new("exec-123".to_string(), "node-1".to_string());

        let items = json!([
            {"price": 100, "quantity": 2},
            {"price": 50, "quantity": 3}
        ]);
        context.set_variable("orders".to_string(), items);

        let params = json!({
            "items": "$orders",
            "transform": {
                "price": "{{price}}",
                "quantity": "{{quantity}}"
            }
        });

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert_eq!(output.data["count"], 2);
        assert_eq!(output.data["mapped_items"][0]["price"], 100);
    }

    #[test]
    fn test_map_node_validation() {
        let node = MapNode::new();

        // Valid parameters
        let params = json!({
            "items": [1, 2, 3],
            "transform": {"value": "{{id}}"}
        });
        assert!(node.validate_parameters(&params).is_ok());

        // Invalid - items not array or variable
        let params = json!({
            "items": 123,
            "transform": {"value": "{{id}}"}
        });
        assert!(node.validate_parameters(&params).is_err());

        // Invalid - missing transform
        let params = json!({
            "items": [1, 2, 3]
        });
        assert!(node.validate_parameters(&params).is_err());
    }
}
