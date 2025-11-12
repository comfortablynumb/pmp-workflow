use crate::models::{Node, NodeContext, NodeOutput};
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

#[async_trait]
impl Node for ConditionalNode {
    fn node_type(&self) -> &str {
        "conditional"
    }

    async fn execute(
        &self,
        context: &NodeContext,
        parameters: &serde_json::Value,
    ) -> anyhow::Result<NodeOutput> {
        let params: ConditionalParams = serde_json::from_value(parameters.clone())?;
        let input = context.get_main_input().cloned().unwrap_or(serde_json::json!({}));

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
            anyhow::bail!("Invalid operator: {}. Valid operators are: {:?}", params.operator, valid_operators);
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
    compare_value: &serde_json::Value,
) -> anyhow::Result<bool> {
    match operator {
        "eq" => Ok(field_value == compare_value),
        "ne" => Ok(field_value != compare_value),
        "gt" => {
            let a = field_value.as_f64().ok_or_else(|| anyhow::anyhow!("Field is not a number"))?;
            let b = compare_value.as_f64().ok_or_else(|| anyhow::anyhow!("Compare value is not a number"))?;
            Ok(a > b)
        }
        "lt" => {
            let a = field_value.as_f64().ok_or_else(|| anyhow::anyhow!("Field is not a number"))?;
            let b = compare_value.as_f64().ok_or_else(|| anyhow::anyhow!("Compare value is not a number"))?;
            Ok(a < b)
        }
        "gte" => {
            let a = field_value.as_f64().ok_or_else(|| anyhow::anyhow!("Field is not a number"))?;
            let b = compare_value.as_f64().ok_or_else(|| anyhow::anyhow!("Compare value is not a number"))?;
            Ok(a >= b)
        }
        "lte" => {
            let a = field_value.as_f64().ok_or_else(|| anyhow::anyhow!("Field is not a number"))?;
            let b = compare_value.as_f64().ok_or_else(|| anyhow::anyhow!("Compare value is not a number"))?;
            Ok(a <= b)
        }
        "contains" => {
            let a = field_value.as_str().ok_or_else(|| anyhow::anyhow!("Field is not a string"))?;
            let b = compare_value.as_str().ok_or_else(|| anyhow::anyhow!("Compare value is not a string"))?;
            Ok(a.contains(b))
        }
        _ => anyhow::bail!("Unknown operator: {}", operator),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluate_condition() {
        let value = serde_json::json!(42);
        let compare = serde_json::json!(40);

        assert!(evaluate_condition(&value, "gt", &compare).unwrap());
        assert!(!evaluate_condition(&value, "lt", &compare).unwrap());
        assert!(!evaluate_condition(&value, "eq", &compare).unwrap());
    }

    #[test]
    fn test_string_contains() {
        let value = serde_json::json!("hello world");
        let compare = serde_json::json!("world");

        assert!(evaluate_condition(&value, "contains", &compare).unwrap());
    }
}
