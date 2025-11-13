use crate::models::{Node, NodeCategory, NodeContext, NodeOutput, NodeSubcategory, NodeType};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

/// Filter node that filters arrays based on conditions
#[derive(Clone)]
pub struct FilterNode {}

#[derive(Debug, Serialize, Deserialize)]
struct FilterParams {
    /// Array to filter (can be a reference like $items or direct array)
    items: Value,
    /// Condition for filtering (field comparisons)
    condition: Value,
}

#[derive(Debug, Serialize, Deserialize)]
struct Condition {
    /// Field name to check
    field: String,
    /// Operator: equals, not_equals, greater_than, less_than, contains, exists
    operator: String,
    /// Value to compare against (optional for exists)
    value: Option<Value>,
}

impl FilterNode {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for FilterNode {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeType for FilterNode {
    fn type_name(&self) -> &str {
        "filter"
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
                    "description": "Array to filter or variable reference (e.g., $items)",
                    "oneOf": [
                        {"type": "array"},
                        {"type": "string", "pattern": "^\\$[a-zA-Z_][a-zA-Z0-9_]*$"}
                    ]
                },
                "condition": {
                    "type": "object",
                    "description": "Filter condition",
                    "properties": {
                        "field": {
                            "type": "string",
                            "description": "Field name to check (supports nested fields with dot notation)"
                        },
                        "operator": {
                            "type": "string",
                            "enum": ["equals", "not_equals", "greater_than", "less_than", "contains", "exists"],
                            "description": "Comparison operator"
                        },
                        "value": {
                            "description": "Value to compare against (not required for 'exists' operator)"
                        }
                    },
                    "required": ["field", "operator"]
                }
            },
            "required": ["items", "condition"]
        })
    }
}

#[async_trait]
impl Node for FilterNode {
    async fn execute(
        &self,
        context: &NodeContext,
        parameters: &serde_json::Value,
    ) -> anyhow::Result<NodeOutput> {
        let params: FilterParams = serde_json::from_value(parameters.clone())?;

        // Get the array to filter
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

        let condition: Condition = serde_json::from_value(params.condition)?;

        // Filter the array based on condition
        let filtered: Vec<Value> = items_array
            .into_iter()
            .filter(|item| match_condition(item, &condition))
            .collect();

        let result = json!({
            "filtered_items": filtered,
            "count": filtered.len()
        });

        Ok(NodeOutput::success(result))
    }

    fn validate_parameters(&self, parameters: &Value) -> anyhow::Result<()> {
        let params: FilterParams = serde_json::from_value(parameters.clone())
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

        // Validate condition
        let condition: Condition = serde_json::from_value(params.condition)
            .map_err(|e| anyhow::anyhow!("Invalid condition: {}", e))?;

        let valid_operators = [
            "equals",
            "not_equals",
            "greater_than",
            "less_than",
            "contains",
            "exists",
        ];
        if !valid_operators.contains(&condition.operator.as_str()) {
            anyhow::bail!(
                "Invalid operator: {}. Must be one of: {:?}",
                condition.operator,
                valid_operators
            );
        }

        // Value is required for all operators except 'exists'
        if condition.operator != "exists" && condition.value.is_none() {
            anyhow::bail!("Value is required for operator '{}'", condition.operator);
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

/// Helper function to check if an item matches the condition
fn match_condition(item: &Value, condition: &Condition) -> bool {
    let field_value = get_field_value(item, &condition.field);

    match condition.operator.as_str() {
        "exists" => field_value.is_some(),
        "equals" => {
            if let (Some(fv), Some(cv)) = (field_value, &condition.value) {
                fv == *cv
            } else {
                false
            }
        }
        "not_equals" => {
            if let (Some(fv), Some(cv)) = (field_value, &condition.value) {
                fv != *cv
            } else {
                false
            }
        }
        "greater_than" => {
            if let (Some(fv), Some(cv)) = (field_value, &condition.value) {
                if let (Some(fv_num), Some(cv_num)) = (fv.as_f64(), cv.as_f64()) {
                    fv_num > cv_num
                } else {
                    false
                }
            } else {
                false
            }
        }
        "less_than" => {
            if let (Some(fv), Some(cv)) = (field_value, &condition.value) {
                if let (Some(fv_num), Some(cv_num)) = (fv.as_f64(), cv.as_f64()) {
                    fv_num < cv_num
                } else {
                    false
                }
            } else {
                false
            }
        }
        "contains" => {
            if let (Some(fv), Some(cv)) = (field_value, &condition.value) {
                if let (Some(fv_str), Some(cv_str)) = (fv.as_str(), cv.as_str()) {
                    fv_str.contains(cv_str)
                } else if let (Some(fv_arr), _) = (fv.as_array(), cv) {
                    fv_arr.contains(cv)
                } else {
                    false
                }
            } else {
                false
            }
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_filter_node_with_equals() {
        let node = FilterNode::new();
        let context = NodeContext::new("exec-123".to_string(), "node-1".to_string());
        let params = json!({
            "items": [
                {"id": 1, "status": "active"},
                {"id": 2, "status": "inactive"},
                {"id": 3, "status": "active"}
            ],
            "condition": {
                "field": "status",
                "operator": "equals",
                "value": "active"
            }
        });

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
        assert_eq!(output.data["count"], 2);
        assert_eq!(output.data["filtered_items"][0]["id"], 1);
        assert_eq!(output.data["filtered_items"][1]["id"], 3);
    }

    #[tokio::test]
    async fn test_filter_node_with_greater_than() {
        let node = FilterNode::new();
        let context = NodeContext::new("exec-123".to_string(), "node-1".to_string());
        let params = json!({
            "items": [
                {"id": 1, "price": 10},
                {"id": 2, "price": 25},
                {"id": 3, "price": 15}
            ],
            "condition": {
                "field": "price",
                "operator": "greater_than",
                "value": 12
            }
        });

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert_eq!(output.data["count"], 2);
        assert_eq!(output.data["filtered_items"][0]["id"], 2);
        assert_eq!(output.data["filtered_items"][1]["id"], 3);
    }

    #[tokio::test]
    async fn test_filter_node_with_variable_reference() {
        let node = FilterNode::new();
        let mut context = NodeContext::new("exec-123".to_string(), "node-1".to_string());

        let items = json!([
            {"name": "Alice", "age": 30},
            {"name": "Bob", "age": 25},
            {"name": "Charlie", "age": 35}
        ]);
        context.set_variable("users".to_string(), items);

        let params = json!({
            "items": "$users",
            "condition": {
                "field": "age",
                "operator": "greater_than",
                "value": 28
            }
        });

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert_eq!(output.data["count"], 2);
    }

    #[test]
    fn test_filter_node_validation() {
        let node = FilterNode::new();

        // Valid parameters
        let params = json!({
            "items": [1, 2, 3],
            "condition": {
                "field": "status",
                "operator": "equals",
                "value": "active"
            }
        });
        assert!(node.validate_parameters(&params).is_ok());

        // Invalid - missing value for non-exists operator
        let params = json!({
            "items": [1, 2, 3],
            "condition": {
                "field": "status",
                "operator": "equals"
            }
        });
        assert!(node.validate_parameters(&params).is_err());

        // Valid - exists operator without value
        let params = json!({
            "items": [1, 2, 3],
            "condition": {
                "field": "status",
                "operator": "exists"
            }
        });
        assert!(node.validate_parameters(&params).is_ok());
    }
}
