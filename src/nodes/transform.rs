use crate::models::{Node, NodeContext, NodeOutput};
use async_trait::async_trait;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct TransformParams {
    /// JSON path expression or field mapping
    expression: Option<String>,
    /// Direct JSON transformation
    template: Option<serde_json::Value>,
}

/// Transform node - transforms data
pub struct TransformNode;

#[async_trait]
impl Node for TransformNode {
    fn node_type(&self) -> &str {
        "transform"
    }

    async fn execute(
        &self,
        context: &NodeContext,
        parameters: &serde_json::Value,
    ) -> anyhow::Result<NodeOutput> {
        let params: TransformParams = serde_json::from_value(parameters.clone())?;
        let input = context
            .get_main_input()
            .cloned()
            .unwrap_or(serde_json::json!({}));

        // If template is provided, use it
        if let Some(template) = params.template {
            // Simple template substitution
            let result = substitute_template(&template, &input, context)?;
            return Ok(NodeOutput::success(result));
        }

        // If expression is provided, extract field
        if let Some(expression) = params.expression {
            let result = extract_field(&input, &expression)?;
            return Ok(NodeOutput::success(result));
        }

        // If no transformation specified, pass through
        Ok(NodeOutput::success(input))
    }
}

/// Extract a field from JSON using a simple path expression
fn extract_field(data: &serde_json::Value, path: &str) -> anyhow::Result<serde_json::Value> {
    let parts: Vec<&str> = path.split('.').collect();
    let mut current = data;

    for part in parts {
        if part.is_empty() {
            continue;
        }

        // Handle array indexing
        if let Some(idx_str) = part.strip_prefix('[').and_then(|s| s.strip_suffix(']')) {
            if let Ok(idx) = idx_str.parse::<usize>() {
                current = current
                    .get(idx)
                    .ok_or_else(|| anyhow::anyhow!("Array index out of bounds: {}", idx))?;
                continue;
            }
        }

        // Regular field access
        current = current
            .get(part)
            .ok_or_else(|| anyhow::anyhow!("Field not found: {}", part))?;
    }

    Ok(current.clone())
}

/// Substitute template values with data from input
fn substitute_template(
    template: &serde_json::Value,
    input: &serde_json::Value,
    context: &NodeContext,
) -> anyhow::Result<serde_json::Value> {
    match template {
        serde_json::Value::String(s) => {
            // Check for variable substitution syntax: {{path}}
            if s.starts_with("{{") && s.ends_with("}}") {
                let path = s[2..s.len() - 2].trim();

                // Check if it's a variable reference
                if let Some(var_name) = path.strip_prefix("$") {
                    if let Some(value) = context.get_variable(var_name) {
                        return Ok(value.clone());
                    }
                }

                // Otherwise, extract from input
                extract_field(input, path)
            } else {
                Ok(serde_json::Value::String(s.clone()))
            }
        }
        serde_json::Value::Object(obj) => {
            let mut result = serde_json::Map::new();
            for (key, value) in obj {
                result.insert(key.clone(), substitute_template(value, input, context)?);
            }
            Ok(serde_json::Value::Object(result))
        }
        serde_json::Value::Array(arr) => {
            let mut result = Vec::new();
            for value in arr {
                result.push(substitute_template(value, input, context)?);
            }
            Ok(serde_json::Value::Array(result))
        }
        other => Ok(other.clone()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_field() {
        let data = serde_json::json!({
            "user": {
                "name": "John",
                "age": 30
            }
        });

        let result = extract_field(&data, "user.name").unwrap();
        assert_eq!(result, serde_json::json!("John"));

        let result = extract_field(&data, "user.age").unwrap();
        assert_eq!(result, serde_json::json!(30));
    }

    #[test]
    fn test_substitute_template() {
        let input = serde_json::json!({
            "name": "Alice",
            "age": 25
        });

        let template = serde_json::json!({
            "userName": "{{name}}",
            "userAge": "{{age}}"
        });

        let context = NodeContext::new("test-exec".to_string(), "test-node".to_string());
        let result = substitute_template(&template, &input, &context).unwrap();

        assert_eq!(
            result,
            serde_json::json!({
                "userName": "Alice",
                "userAge": 25
            })
        );
    }
}
