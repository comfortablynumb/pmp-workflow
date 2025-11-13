use crate::models::{Node, NodeCategory, NodeContext, NodeOutput, NodeSubcategory, NodeType};
use async_trait::async_trait;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct MockServerParams {
    /// Mock server operation to perform
    pub operation: String,
    /// Port to run the mock server on
    pub port: Option<u16>,
    /// Endpoint path (e.g., "/api/users")
    pub endpoint_path: Option<String>,
    /// Response body to return
    pub response_body: Option<String>,
    /// HTTP status code for response
    pub response_status: Option<u16>,
    /// Response headers as JSON object
    #[serde(default)]
    pub response_headers: serde_json::Value,
    /// Delay in milliseconds before responding
    pub delay_ms: Option<u32>,
    /// Mock server instance ID
    pub mock_id: Option<String>,
}

/// Mock Server node - simulates external APIs for testing workflows
pub struct MockServerNode;

impl MockServerNode {
    pub fn new() -> Self {
        Self
    }
}

impl Default for MockServerNode {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeType for MockServerNode {
    fn type_name(&self) -> &str {
        "mock_server"
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
                "operation": {
                    "type": "string",
                    "description": "Mock server operation to perform",
                    "enum": [
                        "start_mock",
                        "stop_mock",
                        "add_endpoint",
                        "remove_endpoint",
                        "list_endpoints",
                        "get_request_log"
                    ]
                },
                "port": {
                    "type": "integer",
                    "description": "Port to run the mock server on",
                    "minimum": 1024,
                    "maximum": 65535,
                    "default": 8080
                },
                "endpoint_path": {
                    "type": "string",
                    "description": "Endpoint path (e.g., '/api/users', '/health')"
                },
                "response_body": {
                    "type": "string",
                    "description": "Response body to return (JSON string or plain text)"
                },
                "response_status": {
                    "type": "integer",
                    "description": "HTTP status code for response",
                    "minimum": 100,
                    "maximum": 599,
                    "default": 200
                },
                "response_headers": {
                    "type": "object",
                    "description": "Response headers as key-value pairs"
                },
                "delay_ms": {
                    "type": "integer",
                    "description": "Delay in milliseconds before responding",
                    "minimum": 0,
                    "maximum": 60000,
                    "default": 0
                },
                "mock_id": {
                    "type": "string",
                    "description": "Mock server instance ID (for stop_mock and other operations)"
                }
            },
            "required": ["operation"],
            "additionalProperties": false
        })
    }

    fn required_credential_type(&self) -> Option<&str> {
        None
    }
}

#[async_trait]
impl Node for MockServerNode {
    async fn execute(
        &self,
        context: &NodeContext,
        parameters: &serde_json::Value,
    ) -> anyhow::Result<NodeOutput> {
        let params: MockServerParams = serde_json::from_value(parameters.clone())?;

        // TODO: In a real implementation:
        // 1. For start_mock: Start an HTTP mock server on specified port
        // 2. For stop_mock: Stop a running mock server instance
        // 3. For add_endpoint: Add a new endpoint with specified response
        // 4. For remove_endpoint: Remove an endpoint from the mock server
        // 5. For list_endpoints: List all configured endpoints
        // 6. For get_request_log: Return log of all requests received
        // 7. Support configurable response delays for simulating slow APIs
        // 8. Support custom headers in responses
        // 9. Log all incoming requests for debugging
        // 10. Use a library like warp or axum for HTTP server

        // For now, we'll return a placeholder response
        let result = serde_json::json!({
            "message": "Mock server operation executed (placeholder implementation)",
            "operation": &params.operation,
            "port": params.port.unwrap_or(8080),
            "endpoint_path": params.endpoint_path,
            "response_status": params.response_status.unwrap_or(200),
            "delay_ms": params.delay_ms.unwrap_or(0),
            "mock_id": params.mock_id.unwrap_or_else(|| "mock-12345".to_string()),
            "context_execution_id": &context.execution_id,
            "server_status": "running",
            "success": true
        });

        Ok(NodeOutput::success(result))
    }

    fn validate_parameters(&self, parameters: &serde_json::Value) -> anyhow::Result<()> {
        let params: MockServerParams = serde_json::from_value(parameters.clone())
            .map_err(|e| anyhow::anyhow!("Invalid parameters: {}", e))?;

        // Validate operation
        let valid_operations = [
            "start_mock",
            "stop_mock",
            "add_endpoint",
            "remove_endpoint",
            "list_endpoints",
            "get_request_log",
        ];

        if !valid_operations.contains(&params.operation.as_str()) {
            anyhow::bail!(
                "Invalid operation: {}. Must be one of: {}",
                params.operation,
                valid_operations.join(", ")
            );
        }

        // Validate port range
        if let Some(port) = params.port
            && !(1024..=65535).contains(&port)
        {
            anyhow::bail!("port must be between 1024 and 65535");
        }

        // Validate response_status range
        if let Some(status) = params.response_status
            && !(100..=599).contains(&status)
        {
            anyhow::bail!("response_status must be between 100 and 599");
        }

        // Validate delay_ms range
        if let Some(delay) = params.delay_ms
            && !(0..=60000).contains(&delay)
        {
            anyhow::bail!("delay_ms must be between 0 and 60000");
        }

        // start_mock requires port
        if params.operation == "start_mock" && params.port.is_none() {
            anyhow::bail!("start_mock operation requires 'port' parameter");
        }

        // add_endpoint requires endpoint_path and response_body
        if params.operation == "add_endpoint" {
            if params.endpoint_path.is_none() {
                anyhow::bail!("add_endpoint operation requires 'endpoint_path' parameter");
            }
            if params.response_body.is_none() {
                anyhow::bail!("add_endpoint operation requires 'response_body' parameter");
            }
        }

        // Operations that require mock_id
        let requires_mock_id = [
            "stop_mock",
            "add_endpoint",
            "remove_endpoint",
            "list_endpoints",
            "get_request_log",
        ];
        if requires_mock_id.contains(&params.operation.as_str()) && params.mock_id.is_none() {
            anyhow::bail!(
                "{} operation requires 'mock_id' parameter",
                params.operation
            );
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
    fn test_mock_server_node_type() {
        let node = MockServerNode::new();
        assert_eq!(node.type_name(), "mock_server");
        assert!(matches!(node.category(), NodeCategory::Action));
        assert!(matches!(node.subcategory(), NodeSubcategory::General));
        assert_eq!(node.required_credential_type(), None);
    }

    #[test]
    fn test_mock_server_parameter_schema() {
        let node = MockServerNode::new();
        let schema = node.parameter_schema();

        assert_eq!(schema["type"], "object");
        assert!(schema["properties"]["operation"].is_object());
        assert!(schema["properties"]["port"].is_object());
        assert!(schema["properties"]["endpoint_path"].is_object());
    }

    #[tokio::test]
    async fn test_mock_server_start() {
        let node = MockServerNode::new();
        let context = NodeContext::new(Uuid::new_v4().to_string(), "test-workflow".to_string());

        let params = json!({
            "operation": "start_mock",
            "port": 8080
        });

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
    }

    #[tokio::test]
    async fn test_mock_server_add_endpoint() {
        let node = MockServerNode::new();
        let context = NodeContext::new(Uuid::new_v4().to_string(), "test-workflow".to_string());

        let params = json!({
            "operation": "add_endpoint",
            "mock_id": "mock-123",
            "endpoint_path": "/api/users",
            "response_body": "{\"users\": []}",
            "response_status": 200,
            "delay_ms": 100
        });

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
    }

    #[test]
    fn test_mock_server_validation() {
        let node = MockServerNode::new();

        // Valid start_mock
        let valid_params = json!({
            "operation": "start_mock",
            "port": 3000
        });
        assert!(node.validate_parameters(&valid_params).is_ok());

        // Invalid operation
        let invalid_params = json!({
            "operation": "invalid_operation"
        });
        assert!(node.validate_parameters(&invalid_params).is_err());

        // Missing port for start_mock
        let invalid_params = json!({
            "operation": "start_mock"
        });
        assert!(node.validate_parameters(&invalid_params).is_err());

        // Missing endpoint_path for add_endpoint
        let invalid_params = json!({
            "operation": "add_endpoint",
            "mock_id": "mock-123",
            "response_body": "{}"
        });
        assert!(node.validate_parameters(&invalid_params).is_err());

        // Invalid port range
        let invalid_params = json!({
            "operation": "start_mock",
            "port": 100
        });
        assert!(node.validate_parameters(&invalid_params).is_err());
    }
}
