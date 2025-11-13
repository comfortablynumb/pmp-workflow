use crate::models::{Node, NodeCategory, NodeContext, NodeOutput, NodeSubcategory, NodeType};
use async_trait::async_trait;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct BedrockParams {
    /// Credentials name to use for AWS Bedrock
    pub credentials_name: String,
    /// Bedrock operation to perform
    pub operation: String,
    /// Model ID to use (e.g., "anthropic.claude-3-sonnet-20240229-v1:0", "amazon.titan-text-express-v1")
    pub model_id: String,
    /// Input prompt or messages
    pub prompt: Option<String>,
    /// Messages for chat-based models (Anthropic Claude format)
    pub messages: Option<Vec<serde_json::Value>>,
    /// System prompt for chat models
    pub system: Option<String>,
    /// Maximum tokens to generate
    pub max_tokens: Option<u32>,
    /// Temperature (0.0 to 1.0 for most models)
    pub temperature: Option<f32>,
    /// Top P sampling (0.0 to 1.0)
    pub top_p: Option<f32>,
    /// Top K sampling
    pub top_k: Option<u32>,
    /// Stop sequences
    pub stop_sequences: Option<Vec<String>>,
    /// Model-specific parameters for Anthropic Claude
    pub anthropic_version: Option<String>,
    /// Model-specific parameters for Amazon Titan
    pub text_generation_config: Option<serde_json::Value>,
    /// Image data for vision models (base64 encoded)
    pub image_data: Option<String>,
    /// Image format
    pub image_format: Option<String>,
    /// Input text for embedding models
    pub input_text: Option<String>,
    /// Embedding input type
    pub input_type: Option<String>,
    /// Additional model-specific parameters
    #[serde(default)]
    pub additional_params: serde_json::Value,
}

/// AWS Bedrock node - performs AWS Bedrock API operations
pub struct BedrockNode;

impl BedrockNode {
    pub fn new() -> Self {
        Self
    }
}

impl Default for BedrockNode {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeType for BedrockNode {
    fn type_name(&self) -> &str {
        "bedrock"
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
                    "description": "Name of the AWS credentials to use for Bedrock",
                    "minLength": 1
                },
                "operation": {
                    "type": "string",
                    "description": "Bedrock operation to perform",
                    "enum": [
                        "invoke_model",
                        "invoke_model_stream",
                        "embed_text"
                    ]
                },
                "model_id": {
                    "type": "string",
                    "description": "Model ID to use",
                    "minLength": 1,
                    "examples": [
                        "anthropic.claude-3-sonnet-20240229-v1:0",
                        "anthropic.claude-3-haiku-20240307-v1:0",
                        "anthropic.claude-v2:1",
                        "amazon.titan-text-express-v1",
                        "amazon.titan-embed-text-v1",
                        "ai21.j2-ultra-v1",
                        "cohere.command-text-v14",
                        "meta.llama2-70b-chat-v1",
                        "stability.stable-diffusion-xl-v1"
                    ]
                },
                "prompt": {
                    "type": "string",
                    "description": "Input prompt for text generation (for non-messages models)"
                },
                "messages": {
                    "type": "array",
                    "description": "Messages for chat-based models (Anthropic Claude format)",
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
                    "description": "System prompt for chat models"
                },
                "max_tokens": {
                    "type": "integer",
                    "description": "Maximum number of tokens to generate",
                    "minimum": 1,
                    "maximum": 200000
                },
                "temperature": {
                    "type": "number",
                    "description": "Sampling temperature (0.0 to 1.0 for most models)",
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
                    "minimum": 0,
                    "maximum": 500
                },
                "stop_sequences": {
                    "type": "array",
                    "description": "Sequences where the model will stop generating",
                    "items": {
                        "type": "string"
                    }
                },
                "anthropic_version": {
                    "type": "string",
                    "description": "Anthropic API version (required for Claude models)",
                    "default": "bedrock-2023-05-31"
                },
                "text_generation_config": {
                    "type": "object",
                    "description": "Text generation configuration for Amazon Titan models",
                    "properties": {
                        "maxTokenCount": {"type": "integer"},
                        "stopSequences": {"type": "array"},
                        "temperature": {"type": "number"},
                        "topP": {"type": "number"}
                    }
                },
                "image_data": {
                    "type": "string",
                    "description": "Base64 encoded image data for vision models"
                },
                "image_format": {
                    "type": "string",
                    "description": "Image format (png, jpeg, gif, webp)",
                    "enum": ["png", "jpeg", "gif", "webp"]
                },
                "input_text": {
                    "type": "string",
                    "description": "Input text for embedding models"
                },
                "input_type": {
                    "type": "string",
                    "description": "Input type for embedding models",
                    "enum": ["search_document", "search_query"]
                },
                "additional_params": {
                    "type": "object",
                    "description": "Additional model-specific parameters"
                }
            },
            "required": ["credentials_name", "operation", "model_id"],
            "additionalProperties": false
        })
    }
}

#[async_trait]
impl Node for BedrockNode {
    async fn execute(
        &self,
        context: &NodeContext,
        parameters: &serde_json::Value,
    ) -> anyhow::Result<NodeOutput> {
        let params: BedrockParams = serde_json::from_value(parameters.clone())?;

        // TODO: In a real implementation:
        // 1. Look up credentials from the credentials table using params.credentials_name
        // 2. Decrypt the credentials data to get AWS access key, secret key, and region
        // 3. Create an AWS Bedrock Runtime client using the credentials
        // 4. Build the request body based on the model provider:
        //
        //    For Anthropic Claude models:
        //    {
        //      "anthropic_version": "bedrock-2023-05-31",
        //      "max_tokens": 1024,
        //      "messages": [{"role": "user", "content": "Hello"}],
        //      "system": "You are a helpful assistant",
        //      "temperature": 1.0,
        //      "top_p": 0.999,
        //      "top_k": 250,
        //      "stop_sequences": ["\n\nHuman:"]
        //    }
        //
        //    For Amazon Titan models:
        //    {
        //      "inputText": "Hello",
        //      "textGenerationConfig": {
        //        "maxTokenCount": 512,
        //        "temperature": 0.7,
        //        "topP": 0.9,
        //        "stopSequences": []
        //      }
        //    }
        //
        //    For AI21 Labs models:
        //    {
        //      "prompt": "Hello",
        //      "maxTokens": 200,
        //      "temperature": 0.7,
        //      "topP": 1,
        //      "stopSequences": []
        //    }
        //
        //    For Cohere models:
        //    {
        //      "prompt": "Hello",
        //      "max_tokens": 400,
        //      "temperature": 0.75,
        //      "p": 0.01,
        //      "k": 0,
        //      "stop_sequences": []
        //    }
        //
        //    For Meta Llama models:
        //    {
        //      "prompt": "Hello",
        //      "max_gen_len": 512,
        //      "temperature": 0.5,
        //      "top_p": 0.9
        //    }
        //
        //    For embedding models:
        //    {
        //      "inputText": "Text to embed"
        //    }
        //
        // 5. Execute the operation based on params.operation:
        //    - invoke_model: Call InvokeModel API
        //    - invoke_model_stream: Call InvokeModelWithResponseStream API
        //    - embed_text: Call InvokeModel with an embedding model
        // 6. Parse the response based on the model provider
        // 7. Return the results

        // For now, we'll return a placeholder response
        let result = serde_json::json!({
            "message": "Bedrock operation executed (placeholder implementation)",
            "credentials_name": &params.credentials_name,
            "operation": &params.operation,
            "model_id": &params.model_id,
            "temperature": params.temperature,
            "max_tokens": params.max_tokens,
            "context_execution_id": &context.execution_id,
            "success": true
        });

        Ok(NodeOutput::success(result))
    }

    fn validate_parameters(&self, parameters: &serde_json::Value) -> anyhow::Result<()> {
        let params: BedrockParams = serde_json::from_value(parameters.clone())
            .map_err(|e| anyhow::anyhow!("Invalid parameters: {}", e))?;

        if params.credentials_name.trim().is_empty() {
            anyhow::bail!("credentials_name cannot be empty");
        }

        if params.model_id.trim().is_empty() {
            anyhow::bail!("model_id cannot be empty");
        }

        // Validate operation
        let valid_operations = ["invoke_model", "invoke_model_stream", "embed_text"];

        if !valid_operations.contains(&params.operation.as_str()) {
            anyhow::bail!(
                "Invalid operation: {}. Must be one of: {}",
                params.operation,
                valid_operations.join(", ")
            );
        }

        // Validate that invoke_model/invoke_model_stream has prompt or messages
        if ["invoke_model", "invoke_model_stream"].contains(&params.operation.as_str())
            && params.prompt.is_none()
            && params.messages.is_none()
        {
            anyhow::bail!(
                "{} operation requires either 'prompt' or 'messages' parameter",
                params.operation
            );
        }

        // Validate that embed_text has input_text
        if params.operation == "embed_text" && params.input_text.is_none() {
            anyhow::bail!("embed_text operation requires 'input_text' parameter");
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

        // Validate that Anthropic Claude models have max_tokens for invoke operations
        if params.model_id.starts_with("anthropic.")
            && ["invoke_model", "invoke_model_stream"].contains(&params.operation.as_str())
            && params.max_tokens.is_none()
        {
            anyhow::bail!("Anthropic Claude models require 'max_tokens' parameter");
        }

        // Validate vision operations
        if params.image_data.is_some() && params.image_format.is_none() {
            anyhow::bail!("image_data requires image_format parameter");
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_bedrock_claude_messages() {
        let node = BedrockNode::new();
        let context = NodeContext::new("exec-123".to_string(), "node-1".to_string());

        let params = serde_json::json!({
            "credentials_name": "my_aws",
            "operation": "invoke_model",
            "model_id": "anthropic.claude-3-sonnet-20240229-v1:0",
            "messages": [
                {"role": "user", "content": "Hello, Claude!"}
            ],
            "max_tokens": 1024,
            "temperature": 0.7,
            "anthropic_version": "bedrock-2023-05-31"
        });

        let result = node.execute(&context, &params).await.unwrap();
        assert!(result.success);
        assert_eq!(result.data["operation"], "invoke_model");
        assert_eq!(
            result.data["model_id"],
            "anthropic.claude-3-sonnet-20240229-v1:0"
        );
    }

    #[tokio::test]
    async fn test_bedrock_titan_text() {
        let node = BedrockNode::new();
        let context = NodeContext::new("exec-123".to_string(), "node-1".to_string());

        let params = serde_json::json!({
            "credentials_name": "my_aws",
            "operation": "invoke_model",
            "model_id": "amazon.titan-text-express-v1",
            "prompt": "Write a short story about AI",
            "text_generation_config": {
                "maxTokenCount": 512,
                "temperature": 0.7,
                "topP": 0.9
            }
        });

        let result = node.execute(&context, &params).await.unwrap();
        assert!(result.success);
        assert_eq!(result.data["operation"], "invoke_model");
        assert_eq!(result.data["model_id"], "amazon.titan-text-express-v1");
    }

    #[tokio::test]
    async fn test_bedrock_embedding() {
        let node = BedrockNode::new();
        let context = NodeContext::new("exec-123".to_string(), "node-1".to_string());

        let params = serde_json::json!({
            "credentials_name": "my_aws",
            "operation": "embed_text",
            "model_id": "amazon.titan-embed-text-v1",
            "input_text": "The quick brown fox jumps over the lazy dog"
        });

        let result = node.execute(&context, &params).await.unwrap();
        assert!(result.success);
        assert_eq!(result.data["operation"], "embed_text");
        assert_eq!(result.data["model_id"], "amazon.titan-embed-text-v1");
    }

    #[tokio::test]
    async fn test_bedrock_streaming() {
        let node = BedrockNode::new();
        let context = NodeContext::new("exec-123".to_string(), "node-1".to_string());

        let params = serde_json::json!({
            "credentials_name": "my_aws",
            "operation": "invoke_model_stream",
            "model_id": "anthropic.claude-3-haiku-20240307-v1:0",
            "messages": [
                {"role": "user", "content": "Tell me a joke"}
            ],
            "max_tokens": 500,
            "temperature": 0.9
        });

        let result = node.execute(&context, &params).await.unwrap();
        assert!(result.success);
        assert_eq!(result.data["operation"], "invoke_model_stream");
    }

    #[tokio::test]
    async fn test_bedrock_with_system_prompt() {
        let node = BedrockNode::new();
        let context = NodeContext::new("exec-123".to_string(), "node-1".to_string());

        let params = serde_json::json!({
            "credentials_name": "my_aws",
            "operation": "invoke_model",
            "model_id": "anthropic.claude-v2:1",
            "messages": [
                {"role": "user", "content": "What's the weather like?"}
            ],
            "system": "You are a helpful weather assistant",
            "max_tokens": 200,
            "temperature": 0.5,
            "top_p": 0.9,
            "stop_sequences": ["\n\nHuman:"]
        });

        let result = node.execute(&context, &params).await.unwrap();
        assert!(result.success);
    }

    #[test]
    fn test_bedrock_validation() {
        let node = BedrockNode::new();

        // Valid Claude invocation
        let valid_claude = serde_json::json!({
            "credentials_name": "my_aws",
            "operation": "invoke_model",
            "model_id": "anthropic.claude-3-sonnet-20240229-v1:0",
            "messages": [{"role": "user", "content": "Hi"}],
            "max_tokens": 1024
        });
        assert!(node.validate_parameters(&valid_claude).is_ok());

        // Valid embedding
        let valid_embed = serde_json::json!({
            "credentials_name": "my_aws",
            "operation": "embed_text",
            "model_id": "amazon.titan-embed-text-v1",
            "input_text": "Hello world"
        });
        assert!(node.validate_parameters(&valid_embed).is_ok());

        // Invalid: invoke_model without prompt or messages
        let invalid_invoke = serde_json::json!({
            "credentials_name": "my_aws",
            "operation": "invoke_model",
            "model_id": "anthropic.claude-v2:1",
            "max_tokens": 1024
        });
        assert!(node.validate_parameters(&invalid_invoke).is_err());

        // Invalid: embed_text without input_text
        let invalid_embed = serde_json::json!({
            "credentials_name": "my_aws",
            "operation": "embed_text",
            "model_id": "amazon.titan-embed-text-v1"
        });
        assert!(node.validate_parameters(&invalid_embed).is_err());

        // Invalid: Claude model without max_tokens
        let invalid_claude = serde_json::json!({
            "credentials_name": "my_aws",
            "operation": "invoke_model",
            "model_id": "anthropic.claude-v2:1",
            "messages": [{"role": "user", "content": "Hi"}]
        });
        assert!(node.validate_parameters(&invalid_claude).is_err());

        // Invalid: temperature out of range
        let invalid_temp = serde_json::json!({
            "credentials_name": "my_aws",
            "operation": "invoke_model",
            "model_id": "anthropic.claude-v2:1",
            "messages": [{"role": "user", "content": "Hi"}],
            "max_tokens": 1024,
            "temperature": 1.5
        });
        assert!(node.validate_parameters(&invalid_temp).is_err());

        // Invalid: image_data without image_format
        let invalid_vision = serde_json::json!({
            "credentials_name": "my_aws",
            "operation": "invoke_model",
            "model_id": "anthropic.claude-3-sonnet-20240229-v1:0",
            "messages": [{"role": "user", "content": "Describe"}],
            "max_tokens": 1024,
            "image_data": "base64data"
        });
        assert!(node.validate_parameters(&invalid_vision).is_err());

        // Invalid: unknown operation
        let invalid_op = serde_json::json!({
            "credentials_name": "my_aws",
            "operation": "invalid_op",
            "model_id": "anthropic.claude-v2:1"
        });
        assert!(node.validate_parameters(&invalid_op).is_err());
    }

    #[test]
    fn test_bedrock_node_type() {
        let node = BedrockNode::new();
        assert_eq!(node.type_name(), "bedrock");
        assert_eq!(node.category(), NodeCategory::Action);
        assert_eq!(node.subcategory(), NodeSubcategory::AI);
    }

    #[test]
    fn test_bedrock_parameter_schema() {
        let node = BedrockNode::new();
        let schema = node.parameter_schema();

        assert_eq!(schema["type"], "object");
        assert!(schema["properties"]["credentials_name"].is_object());
        assert!(schema["properties"]["operation"].is_object());
        assert!(schema["properties"]["model_id"].is_object());
        assert!(schema["properties"]["operation"]["enum"].is_array());
        assert_eq!(schema["required"].as_array().unwrap().len(), 3);
    }
}
