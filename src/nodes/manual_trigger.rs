use crate::models::{Node, NodeContext, NodeOutput, NodeType};
use async_trait::async_trait;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ManualTriggerParams {
    /// Optional description
    pub description: Option<String>,
    /// Optional default input data schema
    pub input_schema: Option<serde_json::Value>,
}

/// Manual Trigger node - starts workflow via manual execution (CLI or API)
pub struct ManualTriggerNode;

impl NodeType for ManualTriggerNode {
    fn type_name(&self) -> &str {
        "manual_trigger"
    }

    fn parameter_schema(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "description": {
                    "type": "string",
                    "description": "Description of this manual trigger"
                },
                "input_schema": {
                    "type": "object",
                    "description": "JSON Schema defining the expected input data when manually triggering this workflow"
                }
            },
            "additionalProperties": false
        })
    }
}

#[async_trait]
impl Node for ManualTriggerNode {
    async fn execute(
        &self,
        context: &NodeContext,
        _parameters: &serde_json::Value,
    ) -> anyhow::Result<NodeOutput> {
        // Manual trigger passes through the input data provided during execution
        let data = context.get_main_input().cloned().unwrap_or_else(|| {
            serde_json::json!({
                "trigger_type": "manual",
                "triggered_at": chrono::Utc::now().to_rfc3339()
            })
        });

        Ok(NodeOutput::success(data))
    }

    fn validate_parameters(&self, parameters: &serde_json::Value) -> anyhow::Result<()> {
        // Try to deserialize to ensure valid structure
        let _params: ManualTriggerParams = serde_json::from_value(parameters.clone())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_manual_trigger_with_input() {
        let node = ManualTriggerNode;
        let mut context = NodeContext::new("exec-123".to_string(), "trigger-1".to_string());
        context.add_input(
            "manual".to_string(),
            serde_json::json!({"user_id": 123, "action": "process"}),
        );

        let params = serde_json::json!({
            "description": "Manual workflow trigger"
        });

        let result = node.execute(&context, &params).await.unwrap();
        assert!(result.success);
        assert_eq!(
            result.data,
            serde_json::json!({"user_id": 123, "action": "process"})
        );
    }

    #[tokio::test]
    async fn test_manual_trigger_without_input() {
        let node = ManualTriggerNode;
        let context = NodeContext::new("exec-123".to_string(), "trigger-1".to_string());

        let params = serde_json::json!({
            "description": "Manual workflow trigger"
        });

        let result = node.execute(&context, &params).await.unwrap();
        assert!(result.success);
        assert!(result.data.get("trigger_type").is_some());
        assert_eq!(result.data["trigger_type"], "manual");
    }

    #[test]
    fn test_manual_trigger_validation() {
        let node = ManualTriggerNode;

        let valid_params = serde_json::json!({
            "description": "Test trigger"
        });
        assert!(node.validate_parameters(&valid_params).is_ok());

        let with_schema = serde_json::json!({
            "description": "Test trigger",
            "input_schema": {
                "type": "object",
                "properties": {
                    "user_id": { "type": "number" }
                }
            }
        });
        assert!(node.validate_parameters(&with_schema).is_ok());

        let empty_params = serde_json::json!({});
        assert!(node.validate_parameters(&empty_params).is_ok());
    }
}
