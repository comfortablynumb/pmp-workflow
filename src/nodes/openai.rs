use crate::models::{Node, NodeCategory, NodeContext, NodeOutput, NodeSubcategory, NodeType};
use async_trait::async_trait;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct OpenAIParams {
    /// Credentials name to use for OpenAI API
    pub credentials_name: String,
    /// OpenAI operation to perform
    pub operation: String,
    /// Model to use (e.g., "gpt-4", "gpt-3.5-turbo", "text-embedding-ada-002")
    pub model: String,
    /// Input text or messages for the operation
    pub input: Option<serde_json::Value>,
    /// Messages for chat completions (array of {role, content})
    pub messages: Option<Vec<serde_json::Value>>,
    /// Maximum tokens to generate
    pub max_tokens: Option<u32>,
    /// Temperature (0.0 to 2.0)
    pub temperature: Option<f32>,
    /// Top P sampling (0.0 to 1.0)
    pub top_p: Option<f32>,
    /// Frequency penalty (-2.0 to 2.0)
    pub frequency_penalty: Option<f32>,
    /// Presence penalty (-2.0 to 2.0)
    pub presence_penalty: Option<f32>,
    /// Number of completions to generate
    pub n: Option<u32>,
    /// Stop sequences
    pub stop: Option<Vec<String>>,
    /// Image generation specific params
    pub size: Option<String>,
    pub quality: Option<String>,
    pub style: Option<String>,
    /// Audio specific params
    pub file_path: Option<String>,
    pub language: Option<String>,
    pub prompt: Option<String>,
    /// Response format
    pub response_format: Option<String>,
    /// Additional parameters
    #[serde(default)]
    pub additional_params: serde_json::Value,
}

/// OpenAI node - performs OpenAI API operations
pub struct OpenAINode;

impl OpenAINode {
    pub fn new() -> Self {
        Self
    }
}

impl Default for OpenAINode {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeType for OpenAINode {
    fn type_name(&self) -> &str {
        "openai"
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
                    "description": "Name of the OpenAI API credentials to use",
                    "minLength": 1
                },
                "operation": {
                    "type": "string",
                    "description": "OpenAI operation to perform",
                    "enum": [
                        "chat_completion",
                        "text_completion",
                        "embedding",
                        "image_generation",
                        "image_edit",
                        "image_variation",
                        "audio_transcription",
                        "audio_translation",
                        "moderation"
                    ]
                },
                "model": {
                    "type": "string",
                    "description": "Model to use (e.g., 'gpt-4', 'gpt-3.5-turbo', 'text-embedding-ada-002')",
                    "minLength": 1
                },
                "input": {
                    "description": "Input text for operations like embedding, moderation, or text completion"
                },
                "messages": {
                    "type": "array",
                    "description": "Messages for chat completion (array of {role, content})",
                    "items": {
                        "type": "object",
                        "properties": {
                            "role": {
                                "type": "string",
                                "enum": ["system", "user", "assistant", "function"]
                            },
                            "content": {
                                "type": "string"
                            }
                        },
                        "required": ["role", "content"]
                    }
                },
                "max_tokens": {
                    "type": "integer",
                    "description": "Maximum number of tokens to generate",
                    "minimum": 1,
                    "maximum": 128000
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
                    "maximum": 1.0,
                    "default": 1.0
                },
                "frequency_penalty": {
                    "type": "number",
                    "description": "Penalize tokens based on frequency (-2.0 to 2.0)",
                    "minimum": -2.0,
                    "maximum": 2.0,
                    "default": 0.0
                },
                "presence_penalty": {
                    "type": "number",
                    "description": "Penalize tokens based on presence (-2.0 to 2.0)",
                    "minimum": -2.0,
                    "maximum": 2.0,
                    "default": 0.0
                },
                "n": {
                    "type": "integer",
                    "description": "Number of completions to generate",
                    "minimum": 1,
                    "maximum": 10,
                    "default": 1
                },
                "stop": {
                    "type": "array",
                    "description": "Up to 4 sequences where the API will stop generating",
                    "items": {
                        "type": "string"
                    },
                    "maxItems": 4
                },
                "size": {
                    "type": "string",
                    "description": "Image size for image generation",
                    "enum": ["256x256", "512x512", "1024x1024", "1792x1024", "1024x1792"]
                },
                "quality": {
                    "type": "string",
                    "description": "Image quality for DALL-E 3",
                    "enum": ["standard", "hd"]
                },
                "style": {
                    "type": "string",
                    "description": "Image style for DALL-E 3",
                    "enum": ["vivid", "natural"]
                },
                "file_path": {
                    "type": "string",
                    "description": "File path for audio operations"
                },
                "language": {
                    "type": "string",
                    "description": "Language for audio transcription (ISO-639-1)"
                },
                "prompt": {
                    "type": "string",
                    "description": "Prompt for image or audio operations"
                },
                "response_format": {
                    "type": "string",
                    "description": "Response format",
                    "enum": ["json", "text", "srt", "vtt", "verbose_json"]
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
impl Node for OpenAINode {
    async fn execute(
        &self,
        context: &NodeContext,
        parameters: &serde_json::Value,
    ) -> anyhow::Result<NodeOutput> {
        let params: OpenAIParams = serde_json::from_value(parameters.clone())?;

        // TODO: In a real implementation:
        // 1. Look up credentials from the credentials table using params.credentials_name
        // 2. Decrypt the credentials data to get the API key
        // 3. Create an OpenAI client using the API key
        // 4. Execute the operation based on params.operation:
        //    - chat_completion: Create chat completion with messages
        //    - text_completion: Create text completion
        //    - embedding: Create embeddings from input text
        //    - image_generation: Generate image from prompt using DALL-E
        //    - image_edit: Edit image with prompt
        //    - image_variation: Create variations of an image
        //    - audio_transcription: Transcribe audio using Whisper
        //    - audio_translation: Translate audio using Whisper
        //    - moderation: Check content for policy violations
        // 5. Apply all the optional parameters (temperature, max_tokens, etc.)
        // 6. Return the API response

        // For now, we'll return a placeholder response
        let result = serde_json::json!({
            "message": "OpenAI operation executed (placeholder implementation)",
            "credentials_name": &params.credentials_name,
            "operation": &params.operation,
            "model": &params.model,
            "temperature": params.temperature,
            "max_tokens": params.max_tokens,
            "context_execution_id": &context.execution_id,
            "success": true
        });

        Ok(NodeOutput::success(result))
    }

    fn validate_parameters(&self, parameters: &serde_json::Value) -> anyhow::Result<()> {
        let params: OpenAIParams = serde_json::from_value(parameters.clone())
            .map_err(|e| anyhow::anyhow!("Invalid parameters: {}", e))?;

        if params.credentials_name.trim().is_empty() {
            anyhow::bail!("credentials_name cannot be empty");
        }

        if params.model.trim().is_empty() {
            anyhow::bail!("model cannot be empty");
        }

        // Validate operation
        let valid_operations = [
            "chat_completion",
            "text_completion",
            "embedding",
            "image_generation",
            "image_edit",
            "image_variation",
            "audio_transcription",
            "audio_translation",
            "moderation",
        ];

        if !valid_operations.contains(&params.operation.as_str()) {
            anyhow::bail!(
                "Invalid operation: {}. Must be one of: {}",
                params.operation,
                valid_operations.join(", ")
            );
        }

        // Validate that chat_completion has messages
        if params.operation == "chat_completion" && params.messages.is_none() {
            anyhow::bail!("chat_completion operation requires 'messages' parameter");
        }

        // Validate that text_completion, embedding, moderation have input
        if ["text_completion", "embedding", "moderation"].contains(&params.operation.as_str())
            && params.input.is_none()
        {
            anyhow::bail!("{} operation requires 'input' parameter", params.operation);
        }

        // Validate that image_generation has prompt
        if params.operation == "image_generation" && params.prompt.is_none() {
            anyhow::bail!("image_generation operation requires 'prompt' parameter");
        }

        // Validate that audio operations have file_path
        if ["audio_transcription", "audio_translation"].contains(&params.operation.as_str())
            && params.file_path.is_none()
        {
            anyhow::bail!(
                "{} operation requires 'file_path' parameter",
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

        // Validate frequency_penalty range
        if let Some(penalty) = params.frequency_penalty
            && !(-2.0..=2.0).contains(&penalty)
        {
            anyhow::bail!("frequency_penalty must be between -2.0 and 2.0");
        }

        // Validate presence_penalty range
        if let Some(penalty) = params.presence_penalty
            && !(-2.0..=2.0).contains(&penalty)
        {
            anyhow::bail!("presence_penalty must be between -2.0 and 2.0");
        }

        // Validate n range
        if let Some(n) = params.n
            && !(1..=10).contains(&n)
        {
            anyhow::bail!("n must be between 1 and 10");
        }

        // Validate stop sequences
        if let Some(ref stop) = params.stop
            && stop.len() > 4
        {
            anyhow::bail!("stop can have at most 4 sequences");
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_openai_chat_completion() {
        let node = OpenAINode::new();
        let context = NodeContext::new("exec-123".to_string(), "node-1".to_string());

        let params = serde_json::json!({
            "credentials_name": "my_openai",
            "operation": "chat_completion",
            "model": "gpt-4",
            "messages": [
                {"role": "system", "content": "You are a helpful assistant."},
                {"role": "user", "content": "Hello!"}
            ],
            "temperature": 0.7,
            "max_tokens": 100
        });

        let result = node.execute(&context, &params).await.unwrap();
        assert!(result.success);
        assert_eq!(result.data["operation"], "chat_completion");
        assert_eq!(result.data["model"], "gpt-4");
    }

    #[tokio::test]
    async fn test_openai_text_completion() {
        let node = OpenAINode::new();
        let context = NodeContext::new("exec-123".to_string(), "node-1".to_string());

        let params = serde_json::json!({
            "credentials_name": "my_openai",
            "operation": "text_completion",
            "model": "gpt-3.5-turbo-instruct",
            "input": "Once upon a time",
            "max_tokens": 50
        });

        let result = node.execute(&context, &params).await.unwrap();
        assert!(result.success);
        assert_eq!(result.data["operation"], "text_completion");
    }

    #[tokio::test]
    async fn test_openai_embedding() {
        let node = OpenAINode::new();
        let context = NodeContext::new("exec-123".to_string(), "node-1".to_string());

        let params = serde_json::json!({
            "credentials_name": "my_openai",
            "operation": "embedding",
            "model": "text-embedding-ada-002",
            "input": "The quick brown fox jumps over the lazy dog"
        });

        let result = node.execute(&context, &params).await.unwrap();
        assert!(result.success);
        assert_eq!(result.data["operation"], "embedding");
    }

    #[tokio::test]
    async fn test_openai_image_generation() {
        let node = OpenAINode::new();
        let context = NodeContext::new("exec-123".to_string(), "node-1".to_string());

        let params = serde_json::json!({
            "credentials_name": "my_openai",
            "operation": "image_generation",
            "model": "dall-e-3",
            "prompt": "A serene landscape with mountains",
            "size": "1024x1024",
            "quality": "hd",
            "style": "vivid"
        });

        let result = node.execute(&context, &params).await.unwrap();
        assert!(result.success);
        assert_eq!(result.data["operation"], "image_generation");
    }

    #[tokio::test]
    async fn test_openai_audio_transcription() {
        let node = OpenAINode::new();
        let context = NodeContext::new("exec-123".to_string(), "node-1".to_string());

        let params = serde_json::json!({
            "credentials_name": "my_openai",
            "operation": "audio_transcription",
            "model": "whisper-1",
            "file_path": "/path/to/audio.mp3",
            "language": "en"
        });

        let result = node.execute(&context, &params).await.unwrap();
        assert!(result.success);
        assert_eq!(result.data["operation"], "audio_transcription");
    }

    #[test]
    fn test_openai_validation() {
        let node = OpenAINode::new();

        // Valid chat completion
        let valid_chat = serde_json::json!({
            "credentials_name": "my_openai",
            "operation": "chat_completion",
            "model": "gpt-4",
            "messages": [{"role": "user", "content": "Hi"}]
        });
        assert!(node.validate_parameters(&valid_chat).is_ok());

        // Invalid: chat_completion without messages
        let invalid_chat = serde_json::json!({
            "credentials_name": "my_openai",
            "operation": "chat_completion",
            "model": "gpt-4"
        });
        assert!(node.validate_parameters(&invalid_chat).is_err());

        // Invalid: text_completion without input
        let invalid_text = serde_json::json!({
            "credentials_name": "my_openai",
            "operation": "text_completion",
            "model": "gpt-3.5-turbo"
        });
        assert!(node.validate_parameters(&invalid_text).is_err());

        // Invalid: temperature out of range
        let invalid_temp = serde_json::json!({
            "credentials_name": "my_openai",
            "operation": "chat_completion",
            "model": "gpt-4",
            "messages": [{"role": "user", "content": "Hi"}],
            "temperature": 3.0
        });
        assert!(node.validate_parameters(&invalid_temp).is_err());

        // Invalid: unknown operation
        let invalid_op = serde_json::json!({
            "credentials_name": "my_openai",
            "operation": "invalid_op",
            "model": "gpt-4"
        });
        assert!(node.validate_parameters(&invalid_op).is_err());
    }

    #[test]
    fn test_openai_node_type() {
        let node = OpenAINode::new();
        assert_eq!(node.type_name(), "openai");
        assert_eq!(node.category(), NodeCategory::Action);
        assert_eq!(node.subcategory(), NodeSubcategory::AI);
    }

    #[test]
    fn test_openai_parameter_schema() {
        let node = OpenAINode::new();
        let schema = node.parameter_schema();

        assert_eq!(schema["type"], "object");
        assert!(schema["properties"]["credentials_name"].is_object());
        assert!(schema["properties"]["operation"].is_object());
        assert!(schema["properties"]["model"].is_object());
        assert!(schema["properties"]["operation"]["enum"].is_array());
        assert_eq!(schema["required"].as_array().unwrap().len(), 3);
    }
}
