use crate::models::{Node, NodeCategory, NodeContext, NodeOutput, NodeType};
use async_trait::async_trait;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct ConditionalParams {
    /// Field to check
    field: String,
    /// Operator: eq, ne, gt, lt, gte, lte, contains
    operator: String,
    /// Value to compare against
    value: serde_json::Value,
}

/// Conditional node - evaluates conditions and returns result
pub struct ConditionalNode;

impl NodeType for ConditionalNode {
    fn type_name(&self) -> &str {
        "conditional"
    }

    fn category(&self) -> NodeCategory {
        NodeCategory::Action
    }

    fn parameter_schema(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "field": {
                    "type": "string",
                    "description": "Field path to check (e.g., 'value', 'user.age')"
                },
                "operator": {
                    "type": "string",
                    "description": "Comparison operator",
                    "enum": ["eq", "ne", "gt", "lt", "gte", "lte", "contains"]
                },
                "value": {
                    "description": "Value to compare against"
                }
            },
            "required": ["field", "operator", "value"],
            "additionalProperties": false
        })
    }
}

#[async_trait]
impl Node for ConditionalNode {
    async fn execute(
        &self,
        context: &NodeContext,
        parameters: &serde_json::Value,
    ) -> anyhow::Result<NodeOutput> {
        let params: ConditionalParams = serde_json::from_value(parameters.clone())?;
        let input = context
            .get_main_input()
            .cloned()
            .unwrap_or(serde_json::json!({}));

        // Extract the field value
        let field_value = extract_field(&input, &params.field)?;

        // Evaluate the condition
        let result = evaluate_condition(&field_value, &params.operator, &params.value)?;

        // Return result with condition outcome
        Ok(NodeOutput::success(serde_json::json!({
            "condition": result,
            "input": input
        })))
    }

    fn validate_parameters(&self, parameters: &serde_json::Value) -> anyhow::Result<()> {
        let params: ConditionalParams = serde_json::from_value(parameters.clone())?;

        let valid_operators = ["eq", "ne", "gt", "lt", "gte", "lte", "contains"];
        if !valid_operators.contains(&params.operator.as_str()) {
            anyhow::bail!(
                "Invalid operator: {}. Valid operators are: {:?}",
                params.operator,
                valid_operators
            );
        }

        Ok(())
    }
}

fn extract_field(data: &serde_json::Value, path: &str) -> anyhow::Result<serde_json::Value> {
    let parts: Vec<&str> = path.split('.').collect();
    let mut current = data;

    for part in parts {
        if part.is_empty() {
            continue;
        }
        current = current
            .get(part)
            .ok_or_else(|| anyhow::anyhow!("Field not found: {}", part))?;
    }

    Ok(current.clone())
}

fn evaluate_condition(
    field_value: &serde_json::Value,
    operator: &str,
    expected: &serde_json::Value,
) -> anyhow::Result<bool> {
    match operator {
        "eq" => Ok(field_value == expected),
        "ne" => Ok(field_value != expected),
        "gt" => {
            if let (Some(a), Some(b)) = (field_value.as_f64(), expected.as_f64()) {
                Ok(a > b)
            } else {
                anyhow::bail!("Cannot compare non-numeric values with gt operator")
            }
        }
        "lt" => {
            if let (Some(a), Some(b)) = (field_value.as_f64(), expected.as_f64()) {
                Ok(a < b)
            } else {
                anyhow::bail!("Cannot compare non-numeric values with lt operator")
            }
        }
        "gte" => {
            if let (Some(a), Some(b)) = (field_value.as_f64(), expected.as_f64()) {
                Ok(a >= b)
            } else {
                anyhow::bail!("Cannot compare non-numeric values with gte operator")
            }
        }
        "lte" => {
            if let (Some(a), Some(b)) = (field_value.as_f64(), expected.as_f64()) {
                Ok(a <= b)
            } else {
                anyhow::bail!("Cannot compare non-numeric values with lte operator")
            }
        }
        "contains" => {
            if let (Some(haystack), Some(needle)) = (field_value.as_str(), expected.as_str()) {
                Ok(haystack.contains(needle))
            } else {
                anyhow::bail!("Contains operator requires string values")
            }
        }
        _ => anyhow::bail!("Unknown operator: {}", operator),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluate_condition() {
        // Test equality
        assert!(evaluate_condition(&serde_json::json!(5), "eq", &serde_json::json!(5)).unwrap());
        assert!(!evaluate_condition(&serde_json::json!(5), "eq", &serde_json::json!(10)).unwrap());

        // Test greater than
        assert!(evaluate_condition(&serde_json::json!(10), "gt", &serde_json::json!(5)).unwrap());
        assert!(!evaluate_condition(&serde_json::json!(5), "gt", &serde_json::json!(10)).unwrap());
    }

    #[test]
    fn test_string_contains() {
        assert!(evaluate_condition(
            &serde_json::json!("hello world"),
            "contains",
            &serde_json::json!("world")
        )
        .unwrap());

        assert!(!evaluate_condition(
            &serde_json::json!("hello world"),
            "contains",
            &serde_json::json!("foo")
        )
        .unwrap());
    }
}
