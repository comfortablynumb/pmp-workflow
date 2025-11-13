use crate::models::{Node, NodeCategory, NodeContext, NodeOutput, NodeSubcategory, NodeType};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use uuid::Uuid;

/// Wait for webhook node that pauses execution until an external event occurs
#[derive(Clone)]
pub struct WaitWebhookNode {}

#[derive(Debug, Serialize, Deserialize)]
struct WaitWebhookParams {
    /// Unique identifier for this wait point
    wait_id: Option<String>,
    /// Timeout in seconds (max wait time)
    timeout_seconds: Option<u64>,
    /// Expected payload schema (for validation)
    expected_schema: Option<Value>,
    /// Return URL path (defaults to /webhook/resume/{wait_id})
    webhook_path: Option<String>,
}

impl WaitWebhookNode {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for WaitWebhookNode {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeType for WaitWebhookNode {
    fn type_name(&self) -> &str {
        "wait_webhook"
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
                "wait_id": {
                    "type": "string",
                    "description": "Unique identifier for this wait point (auto-generated if not provided)"
                },
                "timeout_seconds": {
                    "type": "integer",
                    "description": "Maximum wait time in seconds (default: 3600)",
                    "minimum": 1,
                    "maximum": 86400
                },
                "expected_schema": {
                    "type": "object",
                    "description": "JSON Schema for validating webhook payload"
                },
                "webhook_path": {
                    "type": "string",
                    "description": "Custom webhook path (defaults to /webhook/resume/{wait_id})"
                }
            }
        })
    }
}

#[async_trait]
impl Node for WaitWebhookNode {
    async fn execute(
        &self,
        context: &NodeContext,
        parameters: &serde_json::Value,
    ) -> anyhow::Result<NodeOutput> {
        let params: WaitWebhookParams = serde_json::from_value(parameters.clone())?;

        // Generate or use provided wait ID
        let wait_id = params.wait_id.unwrap_or_else(|| Uuid::new_v4().to_string());

        // Generate webhook URL
        let webhook_path = params
            .webhook_path
            .unwrap_or_else(|| format!("/webhook/resume/{}", wait_id));

        let timeout_seconds = params.timeout_seconds.unwrap_or(3600); // Default 1 hour

        // In a real implementation, this would:
        // 1. Store the execution state in the database
        // 2. Register the webhook endpoint
        // 3. Pause execution
        // 4. Wait for the webhook to be called
        // 5. Resume execution with the webhook payload

        // For now, we'll simulate the setup
        let result = json!({
            "wait_id": wait_id,
            "webhook_url": webhook_path,
            "timeout_seconds": timeout_seconds,
            "status": "waiting",
            "created_at": chrono::Utc::now().to_rfc3339(),
            "expires_at": (chrono::Utc::now() + chrono::Duration::seconds(timeout_seconds as i64)).to_rfc3339(),
            "execution_id": context.execution_id,
            "node_id": context.node_id
        });

        Ok(NodeOutput::success(result))
    }

    fn validate_parameters(&self, parameters: &Value) -> anyhow::Result<()> {
        let params: WaitWebhookParams = serde_json::from_value(parameters.clone())?;

        // Validate timeout
        if let Some(timeout) = params.timeout_seconds {
            if !(1..=86400).contains(&timeout) {
                anyhow::bail!("timeout_seconds must be between 1 and 86400 (24 hours)");
            }
        }

        // Validate webhook path format
        if let Some(path) = &params.webhook_path {
            if !path.starts_with('/') {
                anyhow::bail!("webhook_path must start with '/'");
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_wait_webhook_node() {
        let node = WaitWebhookNode::new();
        let context = NodeContext::new("exec-123".to_string(), "node-1".to_string());
        let params = json!({
            "timeout_seconds": 300
        });

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
        assert!(output.data["wait_id"].is_string());
        assert!(output.data["webhook_url"].is_string());
    }

    #[tokio::test]
    async fn test_wait_webhook_with_custom_id() {
        let node = WaitWebhookNode::new();
        let context = NodeContext::new("exec-123".to_string(), "node-1".to_string());
        let params = json!({
            "wait_id": "custom-wait-123",
            "webhook_path": "/custom/webhook/path"
        });

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert_eq!(output.data["wait_id"], "custom-wait-123");
        assert_eq!(output.data["webhook_url"], "/custom/webhook/path");
    }

    #[test]
    fn test_wait_webhook_validation() {
        let node = WaitWebhookNode::new();

        // Valid
        let params = json!({"timeout_seconds": 600});
        assert!(node.validate_parameters(&params).is_ok());

        // Invalid timeout
        let params = json!({"timeout_seconds": 100000});
        assert!(node.validate_parameters(&params).is_err());

        // Invalid webhook path
        let params = json!({"webhook_path": "invalid-path"});
        assert!(node.validate_parameters(&params).is_err());
    }
}
