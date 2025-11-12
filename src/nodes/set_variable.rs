use crate::models::{Node, NodeCategory, NodeContext, NodeOutput, NodeType};
use async_trait::async_trait;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct SetVariableParams {
    /// Variable name
    name: String,
    /// Value to set (can reference input fields)
    value: serde_json::Value,
}

/// Set Variable node - sets workflow variables
pub struct SetVariableNode;

impl NodeType for SetVariableNode {
    fn type_name(&self) -> &str {
        "set_variable"
    }

    fn category(&self) -> NodeCategory {
        NodeCategory::Action
    }

    fn parameter_schema(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "name": {
                    "type": "string",
                    "description": "Variable name to set",
                    "minLength": 1
                },
                "value": {
                    "description": "Value to set. Can be a literal value or use {{field}} syntax to reference input fields"
                }
            },
            "required": ["name", "value"],
            "additionalProperties": false
        })
    }
}

#[async_trait]
impl Node for SetVariableNode {
    async fn execute(
        &self,
        context: &NodeContext,
        parameters: &serde_json::Value,
    ) -> anyhow::Result<NodeOutput> {
        let params: SetVariableParams = serde_json::from_value(parameters.clone())?;
        let input = context
            .get_main_input()
            .cloned()
            .unwrap_or(serde_json::json!({}));

        // Resolve value (could be a template)
        let resolved_value = resolve_value(&params.value, &input)?;

        // Note: In a real implementation, we would update the context here
        // For now, we'll return the variable information
        Ok(NodeOutput::success(serde_json::json!({
            "variable": params.name,
            "value": resolved_value,
            "input": input
        })))
    }

    fn validate_parameters(&self, parameters: &serde_json::Value) -> anyhow::Result<()> {
        let params: SetVariableParams = serde_json::from_value(parameters.clone())?;

        if params.name.is_empty() {
            anyhow::bail!("Variable name cannot be empty");
        }

        Ok(())
    }
}

fn resolve_value(
    value: &serde_json::Value,
    input: &serde_json::Value,
) -> anyhow::Result<serde_json::Value> {
    match value {
        serde_json::Value::String(s) => {
            // Check for field reference syntax: {{field}}
            if s.starts_with("{{") && s.ends_with("}}") {
                let path = s[2..s.len() - 2].trim();
                extract_field(input, path)
            } else {
                Ok(serde_json::Value::String(s.clone()))
            }
        }
        other => Ok(other.clone()),
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
