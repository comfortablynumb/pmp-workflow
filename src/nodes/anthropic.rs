use crate::models::{Node, NodeCategory, NodeContext, NodeOutput, NodeSubcategory, NodeType};
use async_trait::async_trait;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AnthropicParams {
    /// Credentials name to use for Anthropic API
    pub credentials_name: String,
    /// Anthropic operation to perform
    pub operation: String,
    /// Model to use (e.g., "claude-3-5-sonnet-20241022", "claude-3-opus-20240229")
    pub model: String,
    /// Messages for conversation (array of {role, content})
    pub messages: Option<Vec<serde_json::Value>>,
    /// System prompt
    pub system: Option<String>,
    /// Maximum tokens to generate
    pub max_tokens: Option<u32>,
    /// Temperature (0.0 to 1.0)
    pub temperature: Option<f32>,
    /// Top P sampling (0.0 to 1.0)
    pub top_p: Option<f32>,
    /// Top K sampling
    pub top_k: Option<u32>,
    /// Stop sequences
    pub stop_sequences: Option<Vec<String>>,
    /// Stream response
    pub stream: Option<bool>,
    /// Additional metadata
    #[serde(default)]
    pub metadata: serde_json::Value,
}

/// Anthropic Claude node - performs Anthropic Claude API operations
pub struct AnthropicNode;

impl AnthropicNode {
    pub fn new() -> Self {
        Self
    }
}

impl Default for AnthropicNode {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeType for AnthropicNode {
    fn type_name(&self) -> &str {
        "anthropic"
    }

    fn category(&self) -> NodeCategory {
        NodeCategory::Action
    }

    fn subcategory(&self) -> NodeSubcategory {
        NodeSubcategory::AI
    }

    fn parameter_schema(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "credentials_name": {
                    "type": "string",
                    "description": "Name of the Anthropic API credentials to use",
                    "minLength": 1
                },
                "operation": {
                    "type": "string",
                    "description": "Anthropic operation to perform",
                    "enum": [
                        "messages",
                        "streaming_messages",
                        "message_batches"
                    ]
                },
                "model": {
                    "type": "string",
                    "description": "Model to use (e.g., 'claude-3-5-sonnet-20241022', 'claude-3-opus-20240229', 'claude-3-haiku-20240307')",
                    "minLength": 1
                },
                "messages": {
                    "type": "array",
                    "description": "Messages for conversation (array of {role, content})",
                    "items": {
                        "type": "object",
                        "properties": {
                            "role": {
                                "type": "string",
                                "enum": ["user", "assistant"]
                            },
                            "content": {
                                "oneOf": [
                                    {"type": "string"},
                                    {"type": "array"}
                                ]
                            }
                        },
                        "required": ["role", "content"]
                    }
                },
                "system": {
                    "type": "string",
                    "description": "System prompt to guide Claude's behavior"
                },
                "max_tokens": {
                    "type": "integer",
                    "description": "Maximum number of tokens to generate",
                    "minimum": 1,
                    "maximum": 8192
                },
                "temperature": {
                    "type": "number",
                    "description": "Sampling temperature (0.0 to 1.0). Higher values make output more random",
                    "minimum": 0.0,
                    "maximum": 1.0,
                    "default": 1.0
                },
                "top_p": {
                    "type": "number",
                    "description": "Nucleus sampling parameter (0.0 to 1.0)",
                    "minimum": 0.0,
                    "maximum": 1.0
                },
                "top_k": {
                    "type": "integer",
                    "description": "Top-k sampling parameter",
                    "minimum": 0
                },
                "stop_sequences": {
                    "type": "array",
                    "description": "Custom stop sequences",
                    "items": {
                        "type": "string"
                    }
                },
                "stream": {
                    "type": "boolean",
                    "description": "Stream response incrementally",
                    "default": false
                },
                "metadata": {
                    "type": "object",
                    "description": "Additional metadata for the request"
                }
            },
            "required": ["credentials_name", "operation", "model", "messages", "max_tokens"],
            "additionalProperties": false
        })
    }

    fn required_credential_type(&self) -> Option<&str> {
        Some("anthropic_api")
    }
}

#[async_trait]
impl Node for AnthropicNode {
    async fn execute(
        &self,
        context: &NodeContext,
        parameters: &serde_json::Value,
    ) -> anyhow::Result<NodeOutput> {
        let params: AnthropicParams = serde_json::from_value(parameters.clone())?;

        // TODO: In a real implementation:
        // 1. Look up credentials from the credentials table using params.credentials_name
        // 2. Decrypt the credentials data to get the API key
        // 3. Create an Anthropic client using the API key
        // 4. Execute the operation based on params.operation:
        //    - messages: Create a message with the Messages API
        //    - streaming_messages: Stream a message response
        //    - message_batches: Create multiple messages in batch
        // 5. Apply all the optional parameters (temperature, max_tokens, system, etc.)
        // 6. Return the API response

        // For now, we'll return a placeholder response
        let result = serde_json::json!({
            "message": "Anthropic Claude operation executed (placeholder implementation)",
            "credentials_name": &params.credentials_name,
            "operation": &params.operation,
            "model": &params.model,
            "temperature": params.temperature,
            "max_tokens": params.max_tokens,
            "system": params.system,
            "context_execution_id": &context.execution_id,
            "success": true
        });

        Ok(NodeOutput::success(result))
    }

    fn validate_parameters(&self, parameters: &serde_json::Value) -> anyhow::Result<()> {
        let params: AnthropicParams = serde_json::from_value(parameters.clone())
            .map_err(|e| anyhow::anyhow!("Invalid parameters: {}", e))?;

        if params.credentials_name.trim().is_empty() {
            anyhow::bail!("credentials_name cannot be empty");
        }

        if params.model.trim().is_empty() {
            anyhow::bail!("model cannot be empty");
        }

        // Validate operation
        let valid_operations = ["messages", "streaming_messages", "message_batches"];

        if !valid_operations.contains(&params.operation.as_str()) {
            anyhow::bail!(
                "Invalid operation: {}. Must be one of: {}",
                params.operation,
                valid_operations.join(", ")
            );
        }

        // Validate that messages are provided
        if params.messages.is_none() {
            anyhow::bail!("'messages' parameter is required");
        }

        // Validate max_tokens is provided
        if params.max_tokens.is_none() {
            anyhow::bail!("'max_tokens' parameter is required");
        }

        // Validate temperature range
        if let Some(temp) = params.temperature
            && !(0.0..=1.0).contains(&temp)
        {
            anyhow::bail!("temperature must be between 0.0 and 1.0");
        }

        // Validate top_p range
        if let Some(top_p) = params.top_p
            && !(0.0..=1.0).contains(&top_p)
        {
            anyhow::bail!("top_p must be between 0.0 and 1.0");
        }

        // Validate max_tokens range
        if let Some(max_tokens) = params.max_tokens
            && !(1..=8192).contains(&max_tokens)
        {
            anyhow::bail!("max_tokens must be between 1 and 8192");
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
    fn test_anthropic_node_type() {
        let node = AnthropicNode::new();
        assert_eq!(node.type_name(), "anthropic");
        assert!(matches!(node.category(), NodeCategory::Action));
        assert!(matches!(node.subcategory(), NodeSubcategory::AI));
        assert_eq!(node.required_credential_type(), Some("anthropic_api"));
    }

    #[test]
    fn test_anthropic_parameter_schema() {
        let node = AnthropicNode::new();
        let schema = node.parameter_schema();

        assert_eq!(schema["type"], "object");
        assert!(schema["properties"]["credentials_name"].is_object());
        assert!(schema["properties"]["operation"].is_object());
        assert!(schema["properties"]["model"].is_object());
        assert!(schema["properties"]["messages"].is_object());
    }

    #[tokio::test]
    async fn test_anthropic_messages() {
        let node = AnthropicNode::new();
        let context = NodeContext::new(Uuid::new_v4().to_string(), "test-workflow".to_string());

        let params = json!({
            "credentials_name": "my_anthropic_creds",
            "operation": "messages",
            "model": "claude-3-5-sonnet-20241022",
            "messages": [
                {"role": "user", "content": "Hello, Claude!"}
            ],
            "max_tokens": 1024,
            "temperature": 0.7
        });

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
    }

    #[test]
    fn test_anthropic_validation() {
        let node = AnthropicNode::new();

        // Valid parameters
        let valid_params = json!({
            "credentials_name": "my_anthropic_creds",
            "operation": "messages",
            "model": "claude-3-opus-20240229",
            "messages": [
                {"role": "user", "content": "Test"}
            ],
            "max_tokens": 500
        });
        assert!(node.validate_parameters(&valid_params).is_ok());

        // Missing credentials_name
        let invalid_params = json!({
            "operation": "messages",
            "model": "claude-3-opus-20240229",
            "messages": [],
            "max_tokens": 500
        });
        assert!(node.validate_parameters(&invalid_params).is_err());

        // Invalid operation
        let invalid_params = json!({
            "credentials_name": "my_creds",
            "operation": "invalid_operation",
            "model": "claude-3-opus-20240229",
            "messages": [],
            "max_tokens": 500
        });
        assert!(node.validate_parameters(&invalid_params).is_err());

        // Invalid temperature
        let invalid_params = json!({
            "credentials_name": "my_creds",
            "operation": "messages",
            "model": "claude-3-opus-20240229",
            "messages": [],
            "max_tokens": 500,
            "temperature": 2.0
        });
        assert!(node.validate_parameters(&invalid_params).is_err());
    }
}
