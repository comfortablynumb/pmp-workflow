use crate::models::{Node, NodeCategory, NodeContext, NodeOutput, NodeSubcategory, NodeType};
use async_trait::async_trait;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CohereParams {
    /// Credentials name to use for Cohere API
    pub credentials_name: String,
    /// Cohere operation to perform
    pub operation: String,
    /// Model to use
    pub model: Option<String>,
    /// Prompt for generation
    pub prompt: Option<String>,
    /// Messages for chat completion
    pub messages: Option<Vec<serde_json::Value>>,
    /// Temperature (0.0 to 5.0)
    pub temperature: Option<f32>,
    /// Maximum tokens to generate
    pub max_tokens: Option<u32>,
    /// Number of generations
    pub k: Option<u32>,
    /// Top P sampling (0.0 to 1.0)
    pub p: Option<f32>,
    /// Frequency penalty (0.0 to 1.0)
    pub frequency_penalty: Option<f32>,
    /// Presence penalty (0.0 to 1.0)
    pub presence_penalty: Option<f32>,
    /// Stop sequences
    pub stop_sequences: Option<Vec<String>>,
    /// Return likelihoods
    pub return_likelihoods: Option<String>,
    /// Texts for embedding, classification, or reranking
    pub texts: Option<Vec<String>>,
    /// Truncate mode (NONE, START, END)
    pub truncate: Option<String>,
}

/// Cohere node - performs Cohere API operations
pub struct CohereNode;

impl CohereNode {
    pub fn new() -> Self {
        Self
    }
}

impl Default for CohereNode {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeType for CohereNode {
    fn type_name(&self) -> &str {
        "cohere"
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
                    "description": "Name of the Cohere API credentials to use",
                    "minLength": 1
                },
                "operation": {
                    "type": "string",
                    "description": "Cohere operation to perform",
                    "enum": [
                        "generate",
                        "chat",
                        "embed",
                        "classify",
                        "summarize",
                        "rerank",
                        "detect_language",
                        "tokenize",
                        "detokenize"
                    ]
                },
                "model": {
                    "type": "string",
                    "description": "Model to use",
                    "enum": [
                        "command",
                        "command-light",
                        "command-nightly",
                        "command-r",
                        "command-r-plus",
                        "embed-english-v3.0",
                        "embed-multilingual-v3.0"
                    ]
                },
                "prompt": {
                    "type": "string",
                    "description": "Text prompt for generation operations"
                },
                "messages": {
                    "type": "array",
                    "description": "Messages for chat completion (array of {role, content})",
                    "items": {
                        "type": "object",
                        "properties": {
                            "role": {
                                "type": "string",
                                "enum": ["user", "assistant", "system", "chatbot"]
                            },
                            "content": {
                                "type": "string"
                            }
                        },
                        "required": ["role", "content"]
                    }
                },
                "temperature": {
                    "type": "number",
                    "description": "Sampling temperature (0.0 to 5.0). Higher values make output more random",
                    "minimum": 0.0,
                    "maximum": 5.0,
                    "default": 0.75
                },
                "max_tokens": {
                    "type": "integer",
                    "description": "Maximum number of tokens to generate",
                    "minimum": 1
                },
                "k": {
                    "type": "integer",
                    "description": "Number of generations to return",
                    "minimum": 1,
                    "maximum": 5,
                    "default": 1
                },
                "p": {
                    "type": "number",
                    "description": "Top P sampling (0.0 to 1.0)",
                    "minimum": 0.0,
                    "maximum": 1.0
                },
                "frequency_penalty": {
                    "type": "number",
                    "description": "Frequency penalty (0.0 to 1.0)",
                    "minimum": 0.0,
                    "maximum": 1.0,
                    "default": 0.0
                },
                "presence_penalty": {
                    "type": "number",
                    "description": "Presence penalty (0.0 to 1.0)",
                    "minimum": 0.0,
                    "maximum": 1.0,
                    "default": 0.0
                },
                "stop_sequences": {
                    "type": "array",
                    "description": "Custom stop sequences",
                    "items": {
                        "type": "string"
                    }
                },
                "return_likelihoods": {
                    "type": "string",
                    "description": "Return likelihoods for tokens",
                    "enum": ["GENERATION", "ALL", "NONE"],
                    "default": "NONE"
                },
                "texts": {
                    "type": "array",
                    "description": "Array of texts for embedding, classification, or reranking",
                    "items": {
                        "type": "string"
                    }
                },
                "truncate": {
                    "type": "string",
                    "description": "How to truncate text if it exceeds token limit",
                    "enum": ["NONE", "START", "END"],
                    "default": "END"
                }
            },
            "required": ["credentials_name", "operation"],
            "additionalProperties": false
        })
    }

    fn required_credential_type(&self) -> Option<&str> {
        Some("cohere_api")
    }
}

#[async_trait]
impl Node for CohereNode {
    async fn execute(
        &self,
        context: &NodeContext,
        parameters: &serde_json::Value,
    ) -> anyhow::Result<NodeOutput> {
        let params: CohereParams = serde_json::from_value(parameters.clone())?;

        // TODO: In a real implementation:
        // 1. Look up credentials from the credentials table using params.credentials_name
        // 2. Decrypt the credentials data to get the API key
        // 3. Create a Cohere client using the API key
        // 4. Execute the operation based on params.operation:
        //    - generate: Generate text from prompt
        //    - chat: Create a chat completion
        //    - embed: Generate embeddings from texts
        //    - classify: Classify texts into categories
        //    - summarize: Summarize input text
        //    - rerank: Rerank documents based on query
        //    - detect_language: Detect language of texts
        //    - tokenize: Convert text to tokens
        //    - detokenize: Convert tokens to text
        // 5. Apply all the optional parameters (temperature, max_tokens, etc.)
        // 6. Return the API response

        // For now, we'll return a placeholder response based on the operation
        let result = match params.operation.as_str() {
            "generate" => {
                serde_json::json!({
                    "message": "Cohere text generation executed (placeholder implementation)",
                    "credentials_name": &params.credentials_name,
                    "operation": &params.operation,
                    "model": params.model.as_deref().unwrap_or("command"),
                    "prompt": params.prompt,
                    "temperature": params.temperature,
                    "max_tokens": params.max_tokens,
                    "generations": [{
                        "id": "cohere-gen-123",
                        "text": "This is a generated response from Cohere. In a real implementation, this would contain the actual AI-generated text based on the provided prompt.",
                        "finish_reason": "COMPLETE"
                    }],
                    "context_execution_id": &context.execution_id,
                    "success": true
                })
            }
            "chat" => {
                serde_json::json!({
                    "message": "Cohere chat completion executed (placeholder implementation)",
                    "credentials_name": &params.credentials_name,
                    "operation": &params.operation,
                    "model": params.model.as_deref().unwrap_or("command"),
                    "temperature": params.temperature,
                    "max_tokens": params.max_tokens,
                    "text": "This is a chat response from Cohere. In a real implementation, this would contain the actual conversation response.",
                    "generation_id": "cohere-chat-456",
                    "finish_reason": "COMPLETE",
                    "context_execution_id": &context.execution_id,
                    "success": true
                })
            }
            "embed" => {
                let texts_count = params.texts.as_ref().map(|t| t.len()).unwrap_or(0);
                serde_json::json!({
                    "message": "Cohere embeddings generated (placeholder implementation)",
                    "credentials_name": &params.credentials_name,
                    "operation": &params.operation,
                    "model": params.model.as_deref().unwrap_or("embed-english-v3.0"),
                    "embeddings": vec![vec![0.123; 1024]; texts_count],
                    "context_execution_id": &context.execution_id,
                    "success": true
                })
            }
            "classify" => {
                serde_json::json!({
                    "message": "Cohere classification executed (placeholder implementation)",
                    "credentials_name": &params.credentials_name,
                    "operation": &params.operation,
                    "model": params.model.as_deref().unwrap_or("command"),
                    "classifications": [{
                        "input": "example text",
                        "prediction": "positive",
                        "confidence": 0.95
                    }],
                    "context_execution_id": &context.execution_id,
                    "success": true
                })
            }
            "summarize" => {
                serde_json::json!({
                    "message": "Cohere summarization executed (placeholder implementation)",
                    "credentials_name": &params.credentials_name,
                    "operation": &params.operation,
                    "model": params.model.as_deref().unwrap_or("command"),
                    "summary": "This is a summary generated by Cohere. In a real implementation, this would contain an actual summary of the input text.",
                    "id": "cohere-sum-789",
                    "context_execution_id": &context.execution_id,
                    "success": true
                })
            }
            "rerank" => {
                serde_json::json!({
                    "message": "Cohere reranking executed (placeholder implementation)",
                    "credentials_name": &params.credentials_name,
                    "operation": &params.operation,
                    "model": params.model.as_deref().unwrap_or("command"),
                    "results": [{
                        "index": 0,
                        "relevance_score": 0.98
                    }],
                    "context_execution_id": &context.execution_id,
                    "success": true
                })
            }
            "detect_language" => {
                serde_json::json!({
                    "message": "Cohere language detection executed (placeholder implementation)",
                    "credentials_name": &params.credentials_name,
                    "operation": &params.operation,
                    "results": [{
                        "language_code": "en",
                        "language_name": "English"
                    }],
                    "context_execution_id": &context.execution_id,
                    "success": true
                })
            }
            "tokenize" => {
                serde_json::json!({
                    "message": "Cohere tokenization executed (placeholder implementation)",
                    "credentials_name": &params.credentials_name,
                    "operation": &params.operation,
                    "tokens": [1234, 5678, 9012],
                    "token_strings": ["Hello", " world", "!"],
                    "context_execution_id": &context.execution_id,
                    "success": true
                })
            }
            "detokenize" => {
                serde_json::json!({
                    "message": "Cohere detokenization executed (placeholder implementation)",
                    "credentials_name": &params.credentials_name,
                    "operation": &params.operation,
                    "text": "Hello world!",
                    "context_execution_id": &context.execution_id,
                    "success": true
                })
            }
            _ => {
                anyhow::bail!("Unsupported operation: {}", params.operation);
            }
        };

        Ok(NodeOutput::success(result))
    }

    fn validate_parameters(&self, parameters: &serde_json::Value) -> anyhow::Result<()> {
        let params: CohereParams = serde_json::from_value(parameters.clone())
            .map_err(|e| anyhow::anyhow!("Invalid parameters: {}", e))?;

        if params.credentials_name.trim().is_empty() {
            anyhow::bail!("credentials_name cannot be empty");
        }

        // Validate operation
        let valid_operations = [
            "generate",
            "chat",
            "embed",
            "classify",
            "summarize",
            "rerank",
            "detect_language",
            "tokenize",
            "detokenize",
        ];

        if !valid_operations.contains(&params.operation.as_str()) {
            anyhow::bail!(
                "Invalid operation: {}. Must be one of: {}",
                params.operation,
                valid_operations.join(", ")
            );
        }

        // Validate operation-specific requirements
        match params.operation.as_str() {
            "generate" => {
                if params.prompt.is_none() {
                    anyhow::bail!("'prompt' parameter is required for generate operation");
                }
            }
            "chat" => {
                if params.messages.is_none() {
                    anyhow::bail!("'messages' parameter is required for chat operation");
                }
            }
            "embed" => {
                if params.texts.is_none() {
                    anyhow::bail!("'texts' parameter is required for embed operation");
                }
            }
            "classify" => {
                if params.texts.is_none() {
                    anyhow::bail!("'texts' parameter is required for classify operation");
                }
            }
            "summarize" => {
                if params.prompt.is_none() {
                    anyhow::bail!("'prompt' parameter is required for summarize operation");
                }
            }
            "rerank" => {
                if params.texts.is_none() {
                    anyhow::bail!("'texts' parameter is required for rerank operation");
                }
                if params.prompt.is_none() {
                    anyhow::bail!("'prompt' parameter (query) is required for rerank operation");
                }
            }
            "detect_language" => {
                if params.texts.is_none() {
                    anyhow::bail!("'texts' parameter is required for detect_language operation");
                }
            }
            "tokenize" => {
                if params.prompt.is_none() {
                    anyhow::bail!("'prompt' parameter (text) is required for tokenize operation");
                }
            }
            "detokenize" => {
                // detokenize would typically require token IDs, but we'll use texts for simplicity
                if params.texts.is_none() {
                    anyhow::bail!(
                        "'texts' parameter (token IDs) is required for detokenize operation"
                    );
                }
            }
            _ => {}
        }

        // Validate temperature range
        if let Some(temp) = params.temperature
            && !(0.0..=5.0).contains(&temp)
        {
            anyhow::bail!("temperature must be between 0.0 and 5.0");
        }

        // Validate p range
        if let Some(p) = params.p
            && !(0.0..=1.0).contains(&p)
        {
            anyhow::bail!("p must be between 0.0 and 1.0");
        }

        // Validate k range
        if let Some(k) = params.k
            && !(1..=5).contains(&k)
        {
            anyhow::bail!("k must be between 1 and 5");
        }

        // Validate frequency_penalty range
        if let Some(freq_penalty) = params.frequency_penalty
            && !(0.0..=1.0).contains(&freq_penalty)
        {
            anyhow::bail!("frequency_penalty must be between 0.0 and 1.0");
        }

        // Validate presence_penalty range
        if let Some(pres_penalty) = params.presence_penalty
            && !(0.0..=1.0).contains(&pres_penalty)
        {
            anyhow::bail!("presence_penalty must be between 0.0 and 1.0");
        }

        // Validate return_likelihoods
        if let Some(return_likelihoods) = &params.return_likelihoods {
            let valid_values = ["GENERATION", "ALL", "NONE"];
            if !valid_values.contains(&return_likelihoods.as_str()) {
                anyhow::bail!(
                    "return_likelihoods must be one of: {}",
                    valid_values.join(", ")
                );
            }
        }

        // Validate truncate
        if let Some(truncate) = &params.truncate {
            let valid_values = ["NONE", "START", "END"];
            if !valid_values.contains(&truncate.as_str()) {
                anyhow::bail!("truncate must be one of: {}", valid_values.join(", "));
            }
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
    fn test_cohere_node_type() {
        let node = CohereNode::new();
        assert_eq!(node.type_name(), "cohere");
        assert!(matches!(node.category(), NodeCategory::Action));
        assert!(matches!(node.subcategory(), NodeSubcategory::AI));
        assert_eq!(node.required_credential_type(), Some("cohere_api"));
    }

    #[test]
    fn test_cohere_parameter_schema() {
        let node = CohereNode::new();
        let schema = node.parameter_schema();

        assert_eq!(schema["type"], "object");
        assert!(schema["properties"]["credentials_name"].is_object());
        assert!(schema["properties"]["operation"].is_object());
        assert!(schema["properties"]["model"].is_object());
        assert!(schema["properties"]["prompt"].is_object());
        assert!(schema["properties"]["messages"].is_object());
        assert!(schema["properties"]["texts"].is_object());

        // Verify operation enum
        let operations = schema["properties"]["operation"]["enum"]
            .as_array()
            .unwrap();
        assert!(operations.contains(&json!("generate")));
        assert!(operations.contains(&json!("chat")));
        assert!(operations.contains(&json!("embed")));
        assert!(operations.contains(&json!("classify")));
        assert!(operations.contains(&json!("summarize")));
        assert!(operations.contains(&json!("rerank")));
        assert!(operations.contains(&json!("detect_language")));
        assert!(operations.contains(&json!("tokenize")));
        assert!(operations.contains(&json!("detokenize")));

        // Verify model enum
        let models = schema["properties"]["model"]["enum"].as_array().unwrap();
        assert!(models.contains(&json!("command")));
        assert!(models.contains(&json!("command-light")));
        assert!(models.contains(&json!("command-r")));
        assert!(models.contains(&json!("embed-english-v3.0")));
    }

    #[tokio::test]
    async fn test_cohere_generate() {
        let node = CohereNode::new();
        let context = NodeContext::new(Uuid::new_v4().to_string(), "test-workflow".to_string());

        let params = json!({
            "credentials_name": "my_cohere_creds",
            "operation": "generate",
            "model": "command",
            "prompt": "Write a short story about a robot",
            "temperature": 0.7,
            "max_tokens": 100
        });

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
        assert_eq!(output.data["operation"], "generate");
        assert!(output.data["generations"].is_array());
    }

    #[tokio::test]
    async fn test_cohere_chat() {
        let node = CohereNode::new();
        let context = NodeContext::new(Uuid::new_v4().to_string(), "test-workflow".to_string());

        let params = json!({
            "credentials_name": "my_cohere_creds",
            "operation": "chat",
            "model": "command-r",
            "messages": [
                {"role": "user", "content": "Hello, how are you?"}
            ],
            "temperature": 0.8
        });

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
        assert_eq!(output.data["operation"], "chat");
        assert!(output.data["text"].is_string());
    }

    #[tokio::test]
    async fn test_cohere_embed() {
        let node = CohereNode::new();
        let context = NodeContext::new(Uuid::new_v4().to_string(), "test-workflow".to_string());

        let params = json!({
            "credentials_name": "my_cohere_creds",
            "operation": "embed",
            "model": "embed-english-v3.0",
            "texts": ["Hello world", "How are you?"]
        });

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
        assert_eq!(output.data["operation"], "embed");
        assert!(output.data["embeddings"].is_array());
    }

    #[tokio::test]
    async fn test_cohere_classify() {
        let node = CohereNode::new();
        let context = NodeContext::new(Uuid::new_v4().to_string(), "test-workflow".to_string());

        let params = json!({
            "credentials_name": "my_cohere_creds",
            "operation": "classify",
            "model": "command",
            "texts": ["This product is amazing!", "I hate this service."]
        });

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
        assert_eq!(output.data["operation"], "classify");
        assert!(output.data["classifications"].is_array());
    }

    #[tokio::test]
    async fn test_cohere_summarize() {
        let node = CohereNode::new();
        let context = NodeContext::new(Uuid::new_v4().to_string(), "test-workflow".to_string());

        let params = json!({
            "credentials_name": "my_cohere_creds",
            "operation": "summarize",
            "model": "command",
            "prompt": "This is a long text that needs to be summarized. It contains many details about various topics."
        });

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
        assert_eq!(output.data["operation"], "summarize");
        assert!(output.data["summary"].is_string());
    }

    #[test]
    fn test_cohere_validation() {
        let node = CohereNode::new();

        // Valid generate parameters
        let valid_params = json!({
            "credentials_name": "my_cohere_creds",
            "operation": "generate",
            "model": "command",
            "prompt": "Test prompt"
        });
        assert!(node.validate_parameters(&valid_params).is_ok());

        // Missing credentials_name
        let invalid_params = json!({
            "operation": "generate",
            "model": "command",
            "prompt": "Test"
        });
        assert!(node.validate_parameters(&invalid_params).is_err());

        // Invalid operation
        let invalid_params = json!({
            "credentials_name": "my_creds",
            "operation": "invalid_operation",
            "model": "command"
        });
        assert!(node.validate_parameters(&invalid_params).is_err());

        // Invalid temperature
        let invalid_params = json!({
            "credentials_name": "my_creds",
            "operation": "generate",
            "model": "command",
            "prompt": "Test",
            "temperature": 6.0
        });
        assert!(node.validate_parameters(&invalid_params).is_err());

        // Invalid k
        let invalid_params = json!({
            "credentials_name": "my_creds",
            "operation": "generate",
            "model": "command",
            "prompt": "Test",
            "k": 10
        });
        assert!(node.validate_parameters(&invalid_params).is_err());

        // Missing required prompt for generate
        let invalid_params = json!({
            "credentials_name": "my_creds",
            "operation": "generate",
            "model": "command"
        });
        assert!(node.validate_parameters(&invalid_params).is_err());

        // Missing required messages for chat
        let invalid_params = json!({
            "credentials_name": "my_creds",
            "operation": "chat",
            "model": "command"
        });
        assert!(node.validate_parameters(&invalid_params).is_err());

        // Missing required texts for embed
        let invalid_params = json!({
            "credentials_name": "my_creds",
            "operation": "embed",
            "model": "embed-english-v3.0"
        });
        assert!(node.validate_parameters(&invalid_params).is_err());
    }
}
