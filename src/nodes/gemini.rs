use crate::models::{Node, NodeCategory, NodeContext, NodeOutput, NodeSubcategory, NodeType};
use async_trait::async_trait;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GeminiParams {
    /// Credentials name to use for Google Gemini API
    pub credentials_name: String,
    /// Gemini operation to perform
    pub operation: String,
    /// Model to use (e.g., "gemini-pro", "gemini-pro-vision", "embedding-001")
    pub model: String,
    /// Input text or prompt
    pub prompt: Option<String>,
    /// Contents for chat/multi-turn conversations (array of {role, parts})
    pub contents: Option<Vec<serde_json::Value>>,
    /// System instruction for the model
    pub system_instruction: Option<String>,
    /// Maximum tokens to generate
    pub max_output_tokens: Option<u32>,
    /// Temperature (0.0 to 2.0)
    pub temperature: Option<f32>,
    /// Top P sampling (0.0 to 1.0)
    pub top_p: Option<f32>,
    /// Top K sampling
    pub top_k: Option<u32>,
    /// Stop sequences
    pub stop_sequences: Option<Vec<String>>,
    /// Safety settings
    pub safety_settings: Option<Vec<serde_json::Value>>,
    /// Generation config
    pub generation_config: Option<serde_json::Value>,
    /// Image data for vision operations (base64 encoded)
    pub image_data: Option<String>,
    /// Image MIME type
    pub image_mime_type: Option<String>,
    /// Task type for embeddings
    pub task_type: Option<String>,
    /// Additional parameters
    #[serde(default)]
    pub additional_params: serde_json::Value,
}

/// Gemini node - performs Google Gemini API operations
pub struct GeminiNode;

impl GeminiNode {
    pub fn new() -> Self {
        Self
    }
}

impl Default for GeminiNode {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeType for GeminiNode {
    fn type_name(&self) -> &str {
        "gemini"
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
                    "description": "Name of the Google Gemini API credentials to use",
                    "minLength": 1
                },
                "operation": {
                    "type": "string",
                    "description": "Gemini operation to perform",
                    "enum": [
                        "generate_content",
                        "generate_content_stream",
                        "chat",
                        "count_tokens",
                        "embed_content",
                        "batch_embed_content"
                    ]
                },
                "model": {
                    "type": "string",
                    "description": "Model to use (e.g., 'gemini-pro', 'gemini-pro-vision', 'embedding-001')",
                    "minLength": 1,
                    "examples": ["gemini-pro", "gemini-pro-vision", "gemini-1.5-pro", "gemini-1.5-flash"]
                },
                "prompt": {
                    "type": "string",
                    "description": "Input prompt for single-turn generation"
                },
                "contents": {
                    "type": "array",
                    "description": "Contents for multi-turn conversations (array of {role, parts})",
                    "items": {
                        "type": "object",
                        "properties": {
                            "role": {
                                "type": "string",
                                "enum": ["user", "model"]
                            },
                            "parts": {
                                "type": "array",
                                "items": {
                                    "type": "object"
                                }
                            }
                        }
                    }
                },
                "system_instruction": {
                    "type": "string",
                    "description": "System instruction to guide model behavior"
                },
                "max_output_tokens": {
                    "type": "integer",
                    "description": "Maximum number of tokens to generate",
                    "minimum": 1,
                    "maximum": 8192
                },
                "temperature": {
                    "type": "number",
                    "description": "Sampling temperature (0.0 to 2.0). Higher values make output more random",
                    "minimum": 0.0,
                    "maximum": 2.0,
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
                    "minimum": 1,
                    "maximum": 100
                },
                "stop_sequences": {
                    "type": "array",
                    "description": "Sequences where the API will stop generating",
                    "items": {
                        "type": "string"
                    },
                    "maxItems": 5
                },
                "safety_settings": {
                    "type": "array",
                    "description": "Safety settings to control harmful content filtering",
                    "items": {
                        "type": "object",
                        "properties": {
                            "category": {
                                "type": "string",
                                "enum": [
                                    "HARM_CATEGORY_HARASSMENT",
                                    "HARM_CATEGORY_HATE_SPEECH",
                                    "HARM_CATEGORY_SEXUALLY_EXPLICIT",
                                    "HARM_CATEGORY_DANGEROUS_CONTENT"
                                ]
                            },
                            "threshold": {
                                "type": "string",
                                "enum": [
                                    "BLOCK_NONE",
                                    "BLOCK_LOW_AND_ABOVE",
                                    "BLOCK_MEDIUM_AND_ABOVE",
                                    "BLOCK_ONLY_HIGH"
                                ]
                            }
                        }
                    }
                },
                "generation_config": {
                    "type": "object",
                    "description": "Additional generation configuration"
                },
                "image_data": {
                    "type": "string",
                    "description": "Base64 encoded image data for vision operations"
                },
                "image_mime_type": {
                    "type": "string",
                    "description": "MIME type of the image (e.g., 'image/png', 'image/jpeg')",
                    "enum": ["image/png", "image/jpeg", "image/webp", "image/heic", "image/heif"]
                },
                "task_type": {
                    "type": "string",
                    "description": "Task type for embedding operations",
                    "enum": [
                        "RETRIEVAL_QUERY",
                        "RETRIEVAL_DOCUMENT",
                        "SEMANTIC_SIMILARITY",
                        "CLASSIFICATION",
                        "CLUSTERING"
                    ]
                },
                "additional_params": {
                    "type": "object",
                    "description": "Additional parameters to pass to the API"
                }
            },
            "required": ["credentials_name", "operation", "model"],
            "additionalProperties": false
        })
    }
}

#[async_trait]
impl Node for GeminiNode {
    async fn execute(
        &self,
        context: &NodeContext,
        parameters: &serde_json::Value,
    ) -> anyhow::Result<NodeOutput> {
        let params: GeminiParams = serde_json::from_value(parameters.clone())?;

        // TODO: In a real implementation:
        // 1. Look up credentials from the credentials table using params.credentials_name
        // 2. Decrypt the credentials data to get the API key
        // 3. Make HTTP requests to the Gemini API using reqwest
        // 4. Execute the operation based on params.operation:
        //    - generate_content: Single-turn text generation
        //    - generate_content_stream: Streaming text generation
        //    - chat: Multi-turn conversation
        //    - count_tokens: Count tokens in input
        //    - embed_content: Generate embeddings for text
        //    - batch_embed_content: Generate embeddings for multiple texts
        // 5. Apply all the optional parameters (temperature, max_output_tokens, etc.)
        // 6. Handle vision operations if image_data is provided
        // 7. Return the API response
        //
        // API endpoint format: https://generativelanguage.googleapis.com/v1beta/models/{model}:{operation}?key={api_key}

        // For now, we'll return a placeholder response
        let result = serde_json::json!({
            "message": "Gemini operation executed (placeholder implementation)",
            "credentials_name": &params.credentials_name,
            "operation": &params.operation,
            "model": &params.model,
            "temperature": params.temperature,
            "max_output_tokens": params.max_output_tokens,
            "context_execution_id": &context.execution_id,
            "success": true
        });

        Ok(NodeOutput::success(result))
    }

    fn validate_parameters(&self, parameters: &serde_json::Value) -> anyhow::Result<()> {
        let params: GeminiParams = serde_json::from_value(parameters.clone())
            .map_err(|e| anyhow::anyhow!("Invalid parameters: {}", e))?;

        if params.credentials_name.trim().is_empty() {
            anyhow::bail!("credentials_name cannot be empty");
        }

        if params.model.trim().is_empty() {
            anyhow::bail!("model cannot be empty");
        }

        // Validate operation
        let valid_operations = [
            "generate_content",
            "generate_content_stream",
            "chat",
            "count_tokens",
            "embed_content",
            "batch_embed_content",
        ];

        if !valid_operations.contains(&params.operation.as_str()) {
            anyhow::bail!(
                "Invalid operation: {}. Must be one of: {}",
                params.operation,
                valid_operations.join(", ")
            );
        }

        // Validate that generate_content has prompt or contents
        if ["generate_content", "generate_content_stream"].contains(&params.operation.as_str())
            && params.prompt.is_none()
            && params.contents.is_none()
        {
            anyhow::bail!(
                "{} operation requires either 'prompt' or 'contents' parameter",
                params.operation
            );
        }

        // Validate that chat has contents
        if params.operation == "chat" && params.contents.is_none() {
            anyhow::bail!("chat operation requires 'contents' parameter");
        }

        // Validate that embed operations have prompt or contents
        if ["embed_content", "batch_embed_content"].contains(&params.operation.as_str())
            && params.prompt.is_none()
            && params.contents.is_none()
        {
            anyhow::bail!(
                "{} operation requires either 'prompt' or 'contents' parameter",
                params.operation
            );
        }

        // Validate temperature range
        if let Some(temp) = params.temperature
            && !(0.0..=2.0).contains(&temp)
        {
            anyhow::bail!("temperature must be between 0.0 and 2.0");
        }

        // Validate top_p range
        if let Some(top_p) = params.top_p
            && !(0.0..=1.0).contains(&top_p)
        {
            anyhow::bail!("top_p must be between 0.0 and 1.0");
        }

        // Validate top_k range
        if let Some(top_k) = params.top_k
            && !(1..=100).contains(&top_k)
        {
            anyhow::bail!("top_k must be between 1 and 100");
        }

        // Validate stop_sequences
        if let Some(ref stop) = params.stop_sequences
            && stop.len() > 5
        {
            anyhow::bail!("stop_sequences can have at most 5 sequences");
        }

        // Validate vision operations
        if params.image_data.is_some() && params.image_mime_type.is_none() {
            anyhow::bail!("image_data requires image_mime_type parameter");
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_gemini_generate_content() {
        let node = GeminiNode::new();
        let context = NodeContext::new("exec-123".to_string(), "node-1".to_string());

        let params = serde_json::json!({
            "credentials_name": "my_gemini",
            "operation": "generate_content",
            "model": "gemini-pro",
            "prompt": "Write a short poem about coding",
            "temperature": 0.8,
            "max_output_tokens": 200
        });

        let result = node.execute(&context, &params).await.unwrap();
        assert!(result.success);
        assert_eq!(result.data["operation"], "generate_content");
        assert_eq!(result.data["model"], "gemini-pro");
    }

    #[tokio::test]
    async fn test_gemini_chat() {
        let node = GeminiNode::new();
        let context = NodeContext::new("exec-123".to_string(), "node-1".to_string());

        let params = serde_json::json!({
            "credentials_name": "my_gemini",
            "operation": "chat",
            "model": "gemini-pro",
            "contents": [
                {
                    "role": "user",
                    "parts": [{"text": "Hello! How are you?"}]
                },
                {
                    "role": "model",
                    "parts": [{"text": "I'm doing well, thank you! How can I help you?"}]
                },
                {
                    "role": "user",
                    "parts": [{"text": "Tell me about AI"}]
                }
            ],
            "temperature": 0.7
        });

        let result = node.execute(&context, &params).await.unwrap();
        assert!(result.success);
        assert_eq!(result.data["operation"], "chat");
    }

    #[tokio::test]
    async fn test_gemini_vision() {
        let node = GeminiNode::new();
        let context = NodeContext::new("exec-123".to_string(), "node-1".to_string());

        let params = serde_json::json!({
            "credentials_name": "my_gemini",
            "operation": "generate_content",
            "model": "gemini-pro-vision",
            "prompt": "Describe this image",
            "image_data": "base64_encoded_image_data_here",
            "image_mime_type": "image/jpeg"
        });

        let result = node.execute(&context, &params).await.unwrap();
        assert!(result.success);
        assert_eq!(result.data["operation"], "generate_content");
        assert_eq!(result.data["model"], "gemini-pro-vision");
    }

    #[tokio::test]
    async fn test_gemini_embedding() {
        let node = GeminiNode::new();
        let context = NodeContext::new("exec-123".to_string(), "node-1".to_string());

        let params = serde_json::json!({
            "credentials_name": "my_gemini",
            "operation": "embed_content",
            "model": "embedding-001",
            "prompt": "The quick brown fox jumps over the lazy dog",
            "task_type": "RETRIEVAL_DOCUMENT"
        });

        let result = node.execute(&context, &params).await.unwrap();
        assert!(result.success);
        assert_eq!(result.data["operation"], "embed_content");
    }

    #[tokio::test]
    async fn test_gemini_with_safety_settings() {
        let node = GeminiNode::new();
        let context = NodeContext::new("exec-123".to_string(), "node-1".to_string());

        let params = serde_json::json!({
            "credentials_name": "my_gemini",
            "operation": "generate_content",
            "model": "gemini-pro",
            "prompt": "Tell me a story",
            "safety_settings": [
                {
                    "category": "HARM_CATEGORY_HARASSMENT",
                    "threshold": "BLOCK_MEDIUM_AND_ABOVE"
                },
                {
                    "category": "HARM_CATEGORY_HATE_SPEECH",
                    "threshold": "BLOCK_MEDIUM_AND_ABOVE"
                }
            ]
        });

        let result = node.execute(&context, &params).await.unwrap();
        assert!(result.success);
    }

    #[test]
    fn test_gemini_validation() {
        let node = GeminiNode::new();

        // Valid generate_content with prompt
        let valid_generate = serde_json::json!({
            "credentials_name": "my_gemini",
            "operation": "generate_content",
            "model": "gemini-pro",
            "prompt": "Hello"
        });
        assert!(node.validate_parameters(&valid_generate).is_ok());

        // Valid chat with contents
        let valid_chat = serde_json::json!({
            "credentials_name": "my_gemini",
            "operation": "chat",
            "model": "gemini-pro",
            "contents": [{"role": "user", "parts": [{"text": "Hi"}]}]
        });
        assert!(node.validate_parameters(&valid_chat).is_ok());

        // Invalid: generate_content without prompt or contents
        let invalid_generate = serde_json::json!({
            "credentials_name": "my_gemini",
            "operation": "generate_content",
            "model": "gemini-pro"
        });
        assert!(node.validate_parameters(&invalid_generate).is_err());

        // Invalid: chat without contents
        let invalid_chat = serde_json::json!({
            "credentials_name": "my_gemini",
            "operation": "chat",
            "model": "gemini-pro"
        });
        assert!(node.validate_parameters(&invalid_chat).is_err());

        // Invalid: temperature out of range
        let invalid_temp = serde_json::json!({
            "credentials_name": "my_gemini",
            "operation": "generate_content",
            "model": "gemini-pro",
            "prompt": "Hello",
            "temperature": 3.0
        });
        assert!(node.validate_parameters(&invalid_temp).is_err());

        // Invalid: image_data without image_mime_type
        let invalid_vision = serde_json::json!({
            "credentials_name": "my_gemini",
            "operation": "generate_content",
            "model": "gemini-pro-vision",
            "prompt": "Describe",
            "image_data": "base64data"
        });
        assert!(node.validate_parameters(&invalid_vision).is_err());

        // Invalid: unknown operation
        let invalid_op = serde_json::json!({
            "credentials_name": "my_gemini",
            "operation": "invalid_op",
            "model": "gemini-pro"
        });
        assert!(node.validate_parameters(&invalid_op).is_err());
    }

    #[test]
    fn test_gemini_node_type() {
        let node = GeminiNode::new();
        assert_eq!(node.type_name(), "gemini");
        assert_eq!(node.category(), NodeCategory::Action);
        assert_eq!(node.subcategory(), NodeSubcategory::AI);
    }

    #[test]
    fn test_gemini_parameter_schema() {
        let node = GeminiNode::new();
        let schema = node.parameter_schema();

        assert_eq!(schema["type"], "object");
        assert!(schema["properties"]["credentials_name"].is_object());
        assert!(schema["properties"]["operation"].is_object());
        assert!(schema["properties"]["model"].is_object());
        assert!(schema["properties"]["operation"]["enum"].is_array());
        assert_eq!(schema["required"].as_array().unwrap().len(), 3);
    }
}
