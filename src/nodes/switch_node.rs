use crate::models::{Node, NodeCategory, NodeContext, NodeOutput, NodeSubcategory, NodeType};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

/// Switch node for multi-path conditional logic
#[derive(Clone)]
pub struct SwitchNode {}

#[derive(Debug, Serialize, Deserialize)]
struct SwitchCase {
    /// Condition to evaluate (simple comparison or pattern)
    condition: Option<String>,
    /// Value to compare against
    value: Option<Value>,
    /// Output path name
    path: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct SwitchParams {
    /// Variable or expression to switch on
    switch_on: String,
    /// List of cases to evaluate
    cases: Vec<SwitchCase>,
    /// Default path if no cases match
    default: Option<String>,
}

impl SwitchNode {
    pub fn new() -> Self {
        Self {}
    }

    /// Evaluate a switch case
    fn evaluate_case(&self, switch_value: &Value, case: &SwitchCase) -> bool {
        if let Some(case_value) = &case.value {
            // Direct value comparison
            switch_value == case_value
        } else if let Some(condition) = &case.condition {
            // Condition-based evaluation
            match condition.as_str() {
                "exists" => !switch_value.is_null(),
                "null" => switch_value.is_null(),
                "empty" => {
                    switch_value.is_null()
                        || (switch_value.is_string() && switch_value.as_str().unwrap().is_empty())
                        || (switch_value.is_array() && switch_value.as_array().unwrap().is_empty())
                }
                "truthy" => {
                    switch_value.is_boolean() && switch_value.as_bool().unwrap()
                        || (switch_value.is_number() && switch_value.as_f64().unwrap_or(0.0) != 0.0)
                        || (switch_value.is_string() && !switch_value.as_str().unwrap().is_empty())
                }
                _ => false,
            }
        } else {
            false
        }
    }
}

impl Default for SwitchNode {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeType for SwitchNode {
    fn type_name(&self) -> &str {
        "switch"
    }

    fn category(&self) -> NodeCategory {
        NodeCategory::Control
    }

    fn subcategory(&self) -> NodeSubcategory {
        NodeSubcategory::General
    }

    fn parameter_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "switch_on": {
                    "type": "string",
                    "description": "Variable or input to switch on (e.g., $status or input_field)"
                },
                "cases": {
                    "type": "array",
                    "description": "List of cases to evaluate",
                    "items": {
                        "type": "object",
                        "properties": {
                            "condition": {
                                "type": "string",
                                "enum": ["exists", "null", "empty", "truthy"],
                                "description": "Condition to evaluate"
                            },
                            "value": {
                                "description": "Value to compare against (alternative to condition)"
                            },
                            "path": {
                                "type": "string",
                                "description": "Output path name if this case matches"
                            }
                        },
                        "required": ["path"]
                    },
                    "minItems": 1
                },
                "default": {
                    "type": "string",
                    "description": "Default path if no cases match"
                }
            },
            "required": ["switch_on", "cases"]
        })
    }
}

#[async_trait]
impl Node for SwitchNode {
    async fn execute(
        &self,
        context: &NodeContext,
        parameters: &serde_json::Value,
    ) -> anyhow::Result<NodeOutput> {
        let params: SwitchParams = serde_json::from_value(parameters.clone())?;

        // Resolve the switch value
        let switch_value = if params.switch_on.starts_with('$') {
            // Variable reference
            let var_name = params.switch_on.trim_start_matches('$');
            context
                .get_variable(var_name)
                .cloned()
                .unwrap_or(Value::Null)
        } else {
            // Try to get from input
            context
                .get_input(&params.switch_on)
                .cloned()
                .unwrap_or(Value::Null)
        };

        // Evaluate cases in order
        let mut matched_path: Option<String> = None;

        for case in &params.cases {
            if self.evaluate_case(&switch_value, case) {
                matched_path = Some(case.path.clone());
                break;
            }
        }

        // Use default if no case matched
        let selected_path = matched_path
            .or_else(|| params.default.clone())
            .unwrap_or_else(|| "default".to_string());

        let result = json!({
            "switch_value": switch_value,
            "selected_path": selected_path,
            "evaluated_cases": params.cases.len(),
            "has_default": params.default.is_some()
        });

        Ok(NodeOutput::success(result))
    }

    fn validate_parameters(&self, parameters: &Value) -> anyhow::Result<()> {
        let params: SwitchParams = serde_json::from_value(parameters.clone())
            .map_err(|e| anyhow::anyhow!("Invalid parameters: {}", e))?;

        if params.switch_on.trim().is_empty() {
            anyhow::bail!("switch_on cannot be empty");
        }

        if params.cases.is_empty() {
            anyhow::bail!("cases must contain at least one case");
        }

        // Validate each case has either condition or value
        for (idx, case) in params.cases.iter().enumerate() {
            if case.condition.is_none() && case.value.is_none() {
                anyhow::bail!("Case {} must have either 'condition' or 'value'", idx);
            }
            if case.path.trim().is_empty() {
                anyhow::bail!("Case {} path cannot be empty", idx);
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_switch_node_value_matching() {
        let node = SwitchNode::new();
        let mut context = NodeContext::new("exec-123".to_string(), "node-1".to_string());

        context.set_variable("status".to_string(), json!("active"));

        let params = json!({
            "switch_on": "$status",
            "cases": [
                {"value": "active", "path": "active_path"},
                {"value": "inactive", "path": "inactive_path"}
            ],
            "default": "default_path"
        });

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert_eq!(output.data["selected_path"], "active_path");
    }

    #[tokio::test]
    async fn test_switch_node_condition_matching() {
        let node = SwitchNode::new();
        let mut context = NodeContext::new("exec-123".to_string(), "node-1".to_string());

        context.set_variable("user".to_string(), json!({"name": "Alice"}));

        let params = json!({
            "switch_on": "$user",
            "cases": [
                {"condition": "exists", "path": "user_exists"},
                {"condition": "null", "path": "user_null"}
            ]
        });

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert_eq!(output.data["selected_path"], "user_exists");
    }

    #[tokio::test]
    async fn test_switch_node_default_path() {
        let node = SwitchNode::new();
        let mut context = NodeContext::new("exec-123".to_string(), "node-1".to_string());

        context.set_variable("status".to_string(), json!("unknown"));

        let params = json!({
            "switch_on": "$status",
            "cases": [
                {"value": "active", "path": "active_path"}
            ],
            "default": "fallback_path"
        });

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert_eq!(output.data["selected_path"], "fallback_path");
    }

    #[test]
    fn test_switch_node_validation() {
        let node = SwitchNode::new();

        // Valid configuration
        let params = json!({
            "switch_on": "$status",
            "cases": [
                {"value": "active", "path": "path1"}
            ]
        });
        assert!(node.validate_parameters(&params).is_ok());

        // Invalid - empty cases
        let params = json!({
            "switch_on": "$status",
            "cases": []
        });
        assert!(node.validate_parameters(&params).is_err());

        // Invalid - case without condition or value
        let params = json!({
            "switch_on": "$status",
            "cases": [
                {"path": "path1"}
            ]
        });
        assert!(node.validate_parameters(&params).is_err());
    }
}
