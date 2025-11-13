use crate::models::{Node, NodeCategory, NodeContext, NodeOutput, NodeSubcategory, NodeType};
use async_trait::async_trait;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct HttpWebhookParams {
    /// Optional credentials name (for authenticated requests)
    pub credentials_name: Option<String>,
    /// Target URL
    pub url: String,
    /// HTTP method
    pub method: String,
    /// HTTP headers
    pub headers: Option<serde_json::Value>,
    /// Request body
    pub body: Option<serde_json::Value>,
    /// Query parameters
    pub query_params: Option<serde_json::Value>,
    /// Request timeout in seconds
    pub timeout_seconds: Option<u64>,
}

/// HTTP Webhook node - sends HTTP requests to external services
pub struct HttpWebhookNode;

impl HttpWebhookNode {
    pub fn new() -> Self {
        Self
    }
}

impl Default for HttpWebhookNode {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeType for HttpWebhookNode {
    fn type_name(&self) -> &str {
        "http_webhook"
    }

    fn category(&self) -> NodeCategory {
        NodeCategory::Action
    }

    fn subcategory(&self) -> NodeSubcategory {
        NodeSubcategory::General
    }

    fn parameter_schema(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "credentials_name": {
                    "type": "string",
                    "description": "Optional name of credentials to use for authentication"
                },
                "url": {
                    "type": "string",
                    "description": "Target URL for the HTTP request",
                    "format": "uri",
                    "minLength": 1
                },
                "method": {
                    "type": "string",
                    "description": "HTTP method to use",
                    "enum": ["send_get", "send_post", "send_put", "send_patch", "send_delete"]
                },
                "headers": {
                    "type": "object",
                    "description": "HTTP headers to include in the request"
                },
                "body": {
                    "description": "Request body (for POST, PUT, PATCH methods)"
                },
                "query_params": {
                    "type": "object",
                    "description": "Query parameters to append to the URL"
                },
                "timeout_seconds": {
                    "type": "integer",
                    "description": "Request timeout in seconds",
                    "minimum": 1,
                    "maximum": 300,
                    "default": 30
                }
            },
            "required": ["url", "method"],
            "additionalProperties": false
        })
    }

    fn required_credential_type(&self) -> Option<&str> {
        None // Credentials are optional
    }
}

#[async_trait]
impl Node for HttpWebhookNode {
    async fn execute(
        &self,
        context: &NodeContext,
        parameters: &serde_json::Value,
    ) -> anyhow::Result<NodeOutput> {
        let params: HttpWebhookParams = serde_json::from_value(parameters.clone())?;

        // TODO: In a real implementation:
        // 1. If credentials_name is provided, look up and decrypt credentials
        // 2. Build the HTTP request with the specified method, headers, and body
        // 3. Add query parameters to the URL if provided
        // 4. Set request timeout based on timeout_seconds parameter
        // 5. Execute the HTTP request based on params.method:
        //    - send_get: HTTP GET request
        //    - send_post: HTTP POST request
        //    - send_put: HTTP PUT request
        //    - send_patch: HTTP PATCH request
        //    - send_delete: HTTP DELETE request
        // 6. Return the response (status code, headers, body)

        // For now, we'll return a placeholder response
        let result = serde_json::json!({
            "message": "HTTP webhook executed (placeholder implementation)",
            "url": &params.url,
            "method": &params.method,
            "credentials_name": params.credentials_name,
            "timeout_seconds": params.timeout_seconds.unwrap_or(30),
            "context_execution_id": &context.execution_id,
            "success": true
        });

        Ok(NodeOutput::success(result))
    }

    fn validate_parameters(&self, parameters: &serde_json::Value) -> anyhow::Result<()> {
        let params: HttpWebhookParams = serde_json::from_value(parameters.clone())
            .map_err(|e| anyhow::anyhow!("Invalid parameters: {}", e))?;

        if params.url.trim().is_empty() {
            anyhow::bail!("url cannot be empty");
        }

        // Validate URL format (basic check)
        if !params.url.starts_with("http://") && !params.url.starts_with("https://") {
            anyhow::bail!("url must start with http:// or https://");
        }

        // Validate method
        let valid_methods = [
            "send_get",
            "send_post",
            "send_put",
            "send_patch",
            "send_delete",
        ];

        if !valid_methods.contains(&params.method.as_str()) {
            anyhow::bail!(
                "Invalid method: {}. Must be one of: {}",
                params.method,
                valid_methods.join(", ")
            );
        }

        // Validate timeout range
        if let Some(timeout) = params.timeout_seconds
            && !(1..=300).contains(&timeout)
        {
            anyhow::bail!("timeout_seconds must be between 1 and 300");
        }

        // Validate that body methods have appropriate content
        let body_methods = ["send_post", "send_put", "send_patch"];
        if body_methods.contains(&params.method.as_str()) && params.body.is_none() {
            // Warning: body is often expected for these methods but not strictly required
            // We'll allow it but could log a warning in real implementation
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use uuid::Uuid;

    #[test]
    fn test_http_webhook_node_type() {
        let node = HttpWebhookNode::new();
        assert_eq!(node.type_name(), "http_webhook");
        assert!(matches!(node.category(), NodeCategory::Action));
        assert!(matches!(node.subcategory(), NodeSubcategory::General));
        assert_eq!(node.required_credential_type(), None);
    }

    #[test]
    fn test_http_webhook_parameter_schema() {
        let node = HttpWebhookNode::new();
        let schema = node.parameter_schema();

        assert_eq!(schema["type"], "object");
        assert!(schema["properties"]["url"].is_object());
        assert!(schema["properties"]["method"].is_object());
        assert!(schema["properties"]["headers"].is_object());
    }

    #[tokio::test]
    async fn test_http_webhook_get() {
        let node = HttpWebhookNode::new();
        let context = NodeContext::new(Uuid::new_v4().to_string(), "test-workflow".to_string());

        let params = json!({
            "url": "https://api.example.com/data",
            "method": "send_get",
            "headers": {
                "Content-Type": "application/json"
            }
        });

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
    }

    #[tokio::test]
    async fn test_http_webhook_post() {
        let node = HttpWebhookNode::new();
        let context = NodeContext::new(Uuid::new_v4().to_string(), "test-workflow".to_string());

        let params = json!({
            "url": "https://api.example.com/webhook",
            "method": "send_post",
            "headers": {
                "Content-Type": "application/json"
            },
            "body": {
                "event": "test",
                "data": {"key": "value"}
            },
            "timeout_seconds": 60
        });

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
    }

    #[test]
    fn test_http_webhook_validation() {
        let node = HttpWebhookNode::new();

        // Valid GET request
        let valid_params = json!({
            "url": "https://api.example.com/test",
            "method": "send_get"
        });
        assert!(node.validate_parameters(&valid_params).is_ok());

        // Invalid URL (missing protocol)
        let invalid_params = json!({
            "url": "api.example.com",
            "method": "send_get"
        });
        assert!(node.validate_parameters(&invalid_params).is_err());

        // Invalid method
        let invalid_params = json!({
            "url": "https://api.example.com/test",
            "method": "send_options"
        });
        assert!(node.validate_parameters(&invalid_params).is_err());

        // Invalid timeout
        let invalid_params = json!({
            "url": "https://api.example.com/test",
            "method": "send_get",
            "timeout_seconds": 500
        });
        assert!(node.validate_parameters(&invalid_params).is_err());
    }
}
