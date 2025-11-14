use crate::models::{Node, NodeCategory, NodeContext, NodeOutput, NodeSubcategory, NodeType};
use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

/// Mistral AI integration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MistralParams {
    /// Operation to perform
    pub operation: String,
    /// Model to use
    pub model: Option<String>,
    /// Messages for chat completion
    pub messages: Option<Vec<Value>>,
    /// System prompt
    pub system: Option<String>,
    /// Prompt for completion
    pub prompt: Option<String>,
    /// Temperature (0.0 to 1.0)
    pub temperature: Option<f32>,
    /// Top P sampling
    pub top_p: Option<f32>,
    /// Maximum tokens to generate
    pub max_tokens: Option<u32>,
    /// Enable streaming
    pub stream: Option<bool>,
    /// Random seed for reproducibility
    pub random_seed: Option<i64>,
    /// Safe mode (enable safety prompt)
    pub safe_mode: Option<bool>,
}

pub struct MistralNode;

impl Default for MistralNode {
    fn default() -> Self {
        Self::new()
    }
}

impl MistralNode {
    pub fn new() -> Self {
        Self
    }
}

impl NodeType for MistralNode {
    fn type_name(&self) -> &str {
        "mistral"
    }

    fn category(&self) -> NodeCategory {
        NodeCategory::Action
    }

    fn subcategory(&self) -> NodeSubcategory {
        NodeSubcategory::AI
    }

    fn parameter_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "operation": {
                    "type": "string",
                    "enum": [
                        "chat_completion",
                        "completion",
                        "embeddings",
                        "list_models",
                        "get_model"
                    ],
                    "description": "Operation to perform"
                },
                "model": {
                    "type": "string",
                    "enum": [
                        "mistral-large-latest",
                        "mistral-medium-latest",
                        "mistral-small-latest",
                        "mistral-tiny",
                        "mistral-embed"
                    ],
                    "description": "Model to use"
                },
                "messages": {
                    "type": "array",
                    "description": "Array of message objects with role and content"
                },
                "system": {
                    "type": "string",
                    "description": "System prompt"
                },
                "prompt": {
                    "type": "string",
                    "description": "Prompt for completion"
                },
                "temperature": {
                    "type": "number",
                    "minimum": 0.0,
                    "maximum": 1.0,
                    "description": "Sampling temperature"
                },
                "top_p": {
                    "type": "number",
                    "minimum": 0.0,
                    "maximum": 1.0,
                    "description": "Top P sampling"
                },
                "max_tokens": {
                    "type": "integer",
                    "minimum": 1,
                    "description": "Maximum tokens to generate"
                },
                "stream": {
                    "type": "boolean",
                    "description": "Enable streaming"
                },
                "random_seed": {
                    "type": "integer",
                    "description": "Random seed for reproducibility"
                },
                "safe_mode": {
                    "type": "boolean",
                    "description": "Enable safety prompt"
                }
            },
            "required": ["operation"]
        })
    }

    fn required_credential_type(&self) -> Option<&str> {
        Some("mistral_api")
    }
}

#[async_trait]
impl Node for MistralNode {
    async fn execute(
        &self,
        _context: &NodeContext,
        parameters: &serde_json::Value,
    ) -> Result<NodeOutput> {
        let params: MistralParams = serde_json::from_value(parameters.clone())?;

        // Validate required parameters
        self.validate_parameters(parameters)?;

        // TODO: Implement actual Mistral API calls
        // This is a placeholder implementation
        match params.operation.as_str() {
            "chat_completion" => {
                let model = params.model.as_deref().unwrap_or("mistral-large-latest");

                Ok(NodeOutput::success(json!({
                    "success": true,
                    "operation": "chat_completion",
                    "model": model,
                    "choices": [{
                        "index": 0,
                        "message": {
                            "role": "assistant",
                            "content": "This is a response from Mistral AI. In a real implementation, this would contain the actual AI-generated response based on the input messages."
                        },
                        "finish_reason": "stop"
                    }],
                    "usage": {
                        "prompt_tokens": 150,
                        "completion_tokens": 75,
                        "total_tokens": 225
                    },
                    "created": 1705320000,
                    "id": "mistral-12345"
                })))
            }
            "completion" => {
                let model = params.model.as_deref().unwrap_or("mistral-medium-latest");

                Ok(NodeOutput::success(json!({
                    "success": true,
                    "operation": "completion",
                    "model": model,
                    "choices": [{
                        "index": 0,
                        "text": "This is a completion from Mistral AI.",
                        "finish_reason": "stop"
                    }],
                    "usage": {
                        "prompt_tokens": 50,
                        "completion_tokens": 25,
                        "total_tokens": 75
                    },
                    "created": 1705320000,
                    "id": "mistral-67890"
                })))
            }
            "embeddings" => {
                let model = params.model.as_deref().unwrap_or("mistral-embed");

                Ok(NodeOutput::success(json!({
                    "success": true,
                    "operation": "embeddings",
                    "model": model,
                    "data": [{
                        "object": "embedding",
                        "index": 0,
                        "embedding": vec![0.123; 1024] // Placeholder embedding vector
                    }],
                    "usage": {
                        "prompt_tokens": 10,
                        "total_tokens": 10
                    }
                })))
            }
            "list_models" => Ok(NodeOutput::success(json!({
                "success": true,
                "operation": "list_models",
                "data": [
                    {
                        "id": "mistral-large-latest",
                        "object": "model",
                        "created": 1700000000,
                        "owned_by": "mistralai",
                        "capabilities": {
                            "completion_chat": true,
                            "completion_fim": false,
                            "function_calling": true,
                            "fine_tuning": false
                        }
                    },
                    {
                        "id": "mistral-medium-latest",
                        "object": "model",
                        "created": 1700000000,
                        "owned_by": "mistralai",
                        "capabilities": {
                            "completion_chat": true,
                            "completion_fim": false,
                            "function_calling": true,
                            "fine_tuning": false
                        }
                    },
                    {
                        "id": "mistral-small-latest",
                        "object": "model",
                        "created": 1700000000,
                        "owned_by": "mistralai",
                        "capabilities": {
                            "completion_chat": true,
                            "completion_fim": false,
                            "function_calling": true,
                            "fine_tuning": false
                        }
                    }
                ]
            }))),
            "get_model" => {
                let model = params.model.as_deref().unwrap_or("mistral-large-latest");

                Ok(NodeOutput::success(json!({
                    "success": true,
                    "operation": "get_model",
                    "id": model,
                    "object": "model",
                    "created": 1700000000,
                    "owned_by": "mistralai",
                    "capabilities": {
                        "completion_chat": true,
                        "completion_fim": false,
                        "function_calling": true,
                        "fine_tuning": false
                    },
                    "max_context_length": 32768
                })))
            }
            _ => anyhow::bail!("Unsupported operation: {}", params.operation),
        }
    }

    fn validate_parameters(&self, parameters: &serde_json::Value) -> Result<()> {
        let params: MistralParams = serde_json::from_value(parameters.clone())?;
        // Chat completion requires messages
        if params.operation == "chat_completion" && params.messages.is_none() {
            anyhow::bail!("chat_completion operation requires 'messages' parameter");
        }

        // Completion requires prompt
        if params.operation == "completion" && params.prompt.is_none() {
            anyhow::bail!("completion operation requires 'prompt' parameter");
        }

        // Embeddings requires prompt or messages
        if params.operation == "embeddings" && params.prompt.is_none() && params.messages.is_none()
        {
            anyhow::bail!("embeddings operation requires 'prompt' or 'messages' parameter");
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

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_mistral_chat_completion() {
        let node = MistralNode::new();
        let params = json!({
            "operation": "chat_completion",
            "model": "mistral-large-latest",
            "messages": [
                {"role": "user", "content": "Hello, how are you?"}
            ],
            "temperature": 0.7,
            "max_tokens": 100
        });

        let context = NodeContext::new(Uuid::new_v4().to_string(), "test-workflow".to_string());

        let result = node.execute(&context, &params).await.unwrap();
        assert_eq!(result.data["success"], true);
        assert_eq!(result.data["operation"], "chat_completion");
        assert!(result.data["choices"].is_array());
    }

    #[tokio::test]
    async fn test_mistral_completion() {
        let node = MistralNode::new();
        let params = json!({
            "operation": "completion",
            "model": "mistral-medium-latest",
            "prompt": "Once upon a time",
            "temperature": 0.8
        });

        let context = NodeContext::new(Uuid::new_v4().to_string(), "test-workflow".to_string());

        let result = node.execute(&context, &params).await.unwrap();
        assert_eq!(result.data["success"], true);
        assert_eq!(result.data["operation"], "completion");
    }

    #[tokio::test]
    async fn test_mistral_embeddings() {
        let node = MistralNode::new();
        let params = json!({
            "operation": "embeddings",
            "model": "mistral-embed",
            "prompt": "Hello world"
        });

        let context = NodeContext::new(Uuid::new_v4().to_string(), "test-workflow".to_string());

        let result = node.execute(&context, &params).await.unwrap();
        assert_eq!(result.data["success"], true);
        assert!(result.data["data"].is_array());
    }

    #[tokio::test]
    async fn test_mistral_list_models() {
        let node = MistralNode::new();
        let params = json!({
            "operation": "list_models"
        });

        let context = NodeContext::new(Uuid::new_v4().to_string(), "test-workflow".to_string());

        let result = node.execute(&context, &params).await.unwrap();
        assert_eq!(result.data["success"], true);
        assert!(result.data["data"].is_array());
    }

    #[tokio::test]
    async fn test_mistral_validation_missing_messages() {
        let node = MistralNode::new();
        let params = json!({
            "operation": "chat_completion",
            "model": "mistral-large-latest"
        });

        let context = NodeContext::new(Uuid::new_v4().to_string(), "test-workflow".to_string());

        let result = node.execute(&context, &params).await;
        assert!(result.is_err());
    }
}
