use crate::models::{Node, NodeContext, NodeOutput, NodeType};
use async_trait::async_trait;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct WebhookTriggerParams {
    /// HTTP method (GET, POST, PUT, DELETE, PATCH)
    #[serde(default = "default_method")]
    pub method: String,
    /// Optional description
    pub description: Option<String>,
}

fn default_method() -> String {
    "POST".to_string()
}

/// Webhook Trigger node - starts workflow via HTTP webhook
pub struct WebhookTriggerNode;

impl NodeType for WebhookTriggerNode {
    fn type_name(&self) -> &str {
        "webhook_trigger"
    }

    fn parameter_schema(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "method": {
                    "type": "string",
                    "description": "HTTP method for the webhook endpoint",
                    "enum": ["GET", "POST", "PUT", "DELETE", "PATCH"],
                    "default": "POST"
                },
                "description": {
                    "type": "string",
                    "description": "Description of this webhook trigger"
                }
            },
            "additionalProperties": false
        })
    }
}

#[async_trait]
impl Node for WebhookTriggerNode {
    async fn execute(
        &self,
        context: &NodeContext,
        _parameters: &serde_json::Value,
    ) -> anyhow::Result<NodeOutput> {
        // Webhook trigger passes through the input data from the HTTP request
        let data = context
            .get_main_input()
            .cloned()
            .unwrap_or(serde_json::json!({}));

        Ok(NodeOutput::success(data))
    }

    fn validate_parameters(&self, parameters: &serde_json::Value) -> anyhow::Result<()> {
        let params: WebhookTriggerParams = serde_json::from_value(parameters.clone())?;

        let valid_methods = ["GET", "POST", "PUT", "DELETE", "PATCH"];
        if !valid_methods.contains(&params.method.to_uppercase().as_str()) {
            anyhow::bail!(
                "Invalid HTTP method: {}. Valid methods are: {:?}",
                params.method,
                valid_methods
            );
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_webhook_trigger_execution() {
        let node = WebhookTriggerNode;
        let mut context = NodeContext::new("exec-123".to_string(), "trigger-1".to_string());
        context.add_input(
            "webhook".to_string(),
            serde_json::json!({"user": "test", "action": "create"}),
        );

        let params = serde_json::json!({
            "method": "POST",
            "description": "Test webhook"
        });

        let result = node.execute(&context, &params).await.unwrap();
        assert!(result.success);
        assert_eq!(
            result.data,
            serde_json::json!({"user": "test", "action": "create"})
        );
    }

    #[test]
    fn test_webhook_trigger_validation() {
        let node = WebhookTriggerNode;

        let valid_params = serde_json::json!({
            "method": "POST"
        });
        assert!(node.validate_parameters(&valid_params).is_ok());

        let invalid_params = serde_json::json!({
            "method": "INVALID"
        });
        assert!(node.validate_parameters(&invalid_params).is_err());
    }
}
