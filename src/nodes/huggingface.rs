use crate::models::{Node, NodeCategory, NodeContext, NodeOutput, NodeSubcategory, NodeType};
use async_trait::async_trait;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct HuggingFaceParams {
    /// Operation to perform
    pub operation: String,
    /// Model to use
    pub model: Option<String>,
    /// Input data for the model
    pub inputs: Option<serde_json::Value>,
    /// Model parameters
    pub parameters: Option<serde_json::Value>,
    /// Additional options
    pub options: Option<serde_json::Value>,
    /// Task type (for inference API)
    pub task: Option<String>,
    /// Use cached results
    pub use_cache: Option<bool>,
    /// Wait for model to load if not ready
    pub wait_for_model: Option<bool>,
    /// Repository ID (for model operations)
    pub repo_id: Option<String>,
    /// Filename (for model downloads)
    pub filename: Option<String>,
    /// Model revision/version
    pub revision: Option<String>,
}

/// HuggingFace node - performs HuggingFace API operations
pub struct HuggingFaceNode;

impl HuggingFaceNode {
    pub fn new() -> Self {
        Self
    }
}

impl Default for HuggingFaceNode {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeType for HuggingFaceNode {
    fn type_name(&self) -> &str {
        "huggingface"
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
                "operation": {
                    "type": "string",
                    "enum": [
                        "inference",
                        "text_generation",
                        "text_classification",
                        "token_classification",
                        "question_answering",
                        "summarization",
                        "translation",
                        "text_to_image",
                        "image_classification",
                        "object_detection",
                        "speech_recognition",
                        "embeddings",
                        "download_model",
                        "list_models"
                    ],
                    "description": "Operation to perform"
                },
                "model": {
                    "type": "string",
                    "description": "Model to use (e.g., 'gpt2', 'bert-base-uncased', 'facebook/bart-large-cnn')"
                },
                "inputs": {
                    "description": "Input data for the model (string or structured data)",
                    "oneOf": [
                        {"type": "string"},
                        {"type": "object"},
                        {"type": "array"}
                    ]
                },
                "parameters": {
                    "type": "object",
                    "description": "Model-specific parameters (e.g., max_length, temperature)"
                },
                "options": {
                    "type": "object",
                    "description": "Additional API options"
                },
                "task": {
                    "type": "string",
                    "description": "Specific task type for inference"
                },
                "use_cache": {
                    "type": "boolean",
                    "description": "Use cached results if available",
                    "default": true
                },
                "wait_for_model": {
                    "type": "boolean",
                    "description": "Wait for model to load if not ready",
                    "default": false
                },
                "repo_id": {
                    "type": "string",
                    "description": "Repository ID for model operations"
                },
                "filename": {
                    "type": "string",
                    "description": "Filename for model downloads"
                },
                "revision": {
                    "type": "string",
                    "description": "Model revision/version"
                }
            },
            "required": ["operation"],
            "additionalProperties": false
        })
    }

    fn required_credential_type(&self) -> Option<&str> {
        Some("huggingface_api")
    }
}

#[async_trait]
impl Node for HuggingFaceNode {
    async fn execute(
        &self,
        context: &NodeContext,
        parameters: &serde_json::Value,
    ) -> anyhow::Result<NodeOutput> {
        let params: HuggingFaceParams = serde_json::from_value(parameters.clone())?;

        // TODO: In a real implementation:
        // 1. Look up credentials from the credentials table
        // 2. Decrypt the credentials data to get the API key
        // 3. Create a HuggingFace client using the API key
        // 4. Execute the operation based on params.operation
        // 5. Apply all the optional parameters
        // 6. Return the API response

        // For now, we'll return a placeholder response
        let result = match params.operation.as_str() {
            "inference" => {
                let model = params.model.as_deref().unwrap_or("gpt2");

                serde_json::json!({
                    "message": "HuggingFace inference executed (placeholder implementation)",
                    "operation": "inference",
                    "model": model,
                    "outputs": [{
                        "generated_text": "This is a response from HuggingFace inference. In a real implementation, this would contain the actual model-generated output based on the provided inputs."
                    }],
                    "metadata": {
                        "model": model,
                        "task": params.task,
                        "use_cache": params.use_cache.unwrap_or(true),
                        "wait_for_model": params.wait_for_model.unwrap_or(false)
                    },
                    "context_execution_id": &context.execution_id,
                    "success": true
                })
            }
            "text_generation" => {
                let model = params.model.as_deref().unwrap_or("gpt2");

                serde_json::json!({
                    "message": "HuggingFace text generation executed (placeholder implementation)",
                    "operation": "text_generation",
                    "model": model,
                    "generated_text": "This is generated text from HuggingFace. In a real implementation, this would be the actual generated text based on the input prompt.",
                    "metadata": {
                        "model": model,
                        "parameters": params.parameters
                    },
                    "context_execution_id": &context.execution_id,
                    "success": true
                })
            }
            "text_classification" => {
                let model = params
                    .model
                    .as_deref()
                    .unwrap_or("distilbert-base-uncased-finetuned-sst-2-english");

                serde_json::json!({
                    "message": "HuggingFace text classification executed (placeholder implementation)",
                    "operation": "text_classification",
                    "model": model,
                    "predictions": [{
                        "label": "POSITIVE",
                        "score": 0.9998
                    }, {
                        "label": "NEGATIVE",
                        "score": 0.0002
                    }],
                    "context_execution_id": &context.execution_id,
                    "success": true
                })
            }
            "token_classification" => {
                let model = params
                    .model
                    .as_deref()
                    .unwrap_or("dbmdz/bert-large-cased-finetuned-conll03-english");

                serde_json::json!({
                    "message": "HuggingFace token classification executed (placeholder implementation)",
                    "operation": "token_classification",
                    "model": model,
                    "entities": [{
                        "entity": "B-PER",
                        "score": 0.9988,
                        "word": "John",
                        "start": 0,
                        "end": 4
                    }],
                    "context_execution_id": &context.execution_id,
                    "success": true
                })
            }
            "question_answering" => {
                let model = params
                    .model
                    .as_deref()
                    .unwrap_or("distilbert-base-cased-distilled-squad");

                serde_json::json!({
                    "message": "HuggingFace question answering executed (placeholder implementation)",
                    "operation": "question_answering",
                    "model": model,
                    "answer": "This is the answer to your question.",
                    "score": 0.9156,
                    "start": 12,
                    "end": 45,
                    "context_execution_id": &context.execution_id,
                    "success": true
                })
            }
            "summarization" => {
                let model = params.model.as_deref().unwrap_or("facebook/bart-large-cnn");

                serde_json::json!({
                    "message": "HuggingFace summarization executed (placeholder implementation)",
                    "operation": "summarization",
                    "model": model,
                    "summary_text": "This is a summarized version of the input text. In a real implementation, this would be the actual summary generated by the model.",
                    "context_execution_id": &context.execution_id,
                    "success": true
                })
            }
            "translation" => {
                let model = params.model.as_deref().unwrap_or("t5-base");

                serde_json::json!({
                    "message": "HuggingFace translation executed (placeholder implementation)",
                    "operation": "translation",
                    "model": model,
                    "translation_text": "This is the translated text. In a real implementation, this would be the actual translation.",
                    "context_execution_id": &context.execution_id,
                    "success": true
                })
            }
            "text_to_image" => {
                let model = params
                    .model
                    .as_deref()
                    .unwrap_or("stabilityai/stable-diffusion-2");

                serde_json::json!({
                    "message": "HuggingFace text-to-image executed (placeholder implementation)",
                    "operation": "text_to_image",
                    "model": model,
                    "image": "base64_encoded_image_data_placeholder",
                    "metadata": {
                        "width": 512,
                        "height": 512,
                        "model": model
                    },
                    "context_execution_id": &context.execution_id,
                    "success": true
                })
            }
            "image_classification" => {
                let model = params
                    .model
                    .as_deref()
                    .unwrap_or("google/vit-base-patch16-224");

                serde_json::json!({
                    "message": "HuggingFace image classification executed (placeholder implementation)",
                    "operation": "image_classification",
                    "model": model,
                    "predictions": [{
                        "label": "cat",
                        "score": 0.9876
                    }, {
                        "label": "dog",
                        "score": 0.0098
                    }],
                    "context_execution_id": &context.execution_id,
                    "success": true
                })
            }
            "object_detection" => {
                let model = params.model.as_deref().unwrap_or("facebook/detr-resnet-50");

                serde_json::json!({
                    "message": "HuggingFace object detection executed (placeholder implementation)",
                    "operation": "object_detection",
                    "model": model,
                    "detections": [{
                        "label": "person",
                        "score": 0.9912,
                        "box": {
                            "xmin": 100,
                            "ymin": 150,
                            "xmax": 300,
                            "ymax": 450
                        }
                    }],
                    "context_execution_id": &context.execution_id,
                    "success": true
                })
            }
            "speech_recognition" => {
                let model = params
                    .model
                    .as_deref()
                    .unwrap_or("facebook/wav2vec2-base-960h");

                serde_json::json!({
                    "message": "HuggingFace speech recognition executed (placeholder implementation)",
                    "operation": "speech_recognition",
                    "model": model,
                    "text": "This is the transcribed text from the audio input. In a real implementation, this would be the actual transcription.",
                    "context_execution_id": &context.execution_id,
                    "success": true
                })
            }
            "embeddings" => {
                let model = params
                    .model
                    .as_deref()
                    .unwrap_or("sentence-transformers/all-MiniLM-L6-v2");

                serde_json::json!({
                    "message": "HuggingFace embeddings executed (placeholder implementation)",
                    "operation": "embeddings",
                    "model": model,
                    "embeddings": vec![0.123; 384], // Placeholder embedding vector
                    "metadata": {
                        "model": model,
                        "dimension": 384
                    },
                    "context_execution_id": &context.execution_id,
                    "success": true
                })
            }
            "download_model" => {
                let repo_id = params
                    .repo_id
                    .as_ref()
                    .ok_or_else(|| anyhow::anyhow!("repo_id is required for download_model"))?;

                serde_json::json!({
                    "message": "HuggingFace model download initiated (placeholder implementation)",
                    "operation": "download_model",
                    "repo_id": repo_id,
                    "filename": params.filename,
                    "revision": params.revision,
                    "download_url": format!("https://huggingface.co/{}/resolve/main/pytorch_model.bin", repo_id),
                    "context_execution_id": &context.execution_id,
                    "success": true
                })
            }
            "list_models" => {
                serde_json::json!({
                    "message": "HuggingFace models listed (placeholder implementation)",
                    "operation": "list_models",
                    "models": [
                        {
                            "id": "gpt2",
                            "task": "text-generation",
                            "downloads": 1000000,
                            "library": "transformers"
                        },
                        {
                            "id": "bert-base-uncased",
                            "task": "fill-mask",
                            "downloads": 2000000,
                            "library": "transformers"
                        },
                        {
                            "id": "facebook/bart-large-cnn",
                            "task": "summarization",
                            "downloads": 500000,
                            "library": "transformers"
                        },
                        {
                            "id": "stabilityai/stable-diffusion-2",
                            "task": "text-to-image",
                            "downloads": 1500000,
                            "library": "diffusers"
                        },
                        {
                            "id": "sentence-transformers/all-MiniLM-L6-v2",
                            "task": "sentence-similarity",
                            "downloads": 800000,
                            "library": "sentence-transformers"
                        }
                    ],
                    "total": 5,
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
        let params: HuggingFaceParams = serde_json::from_value(parameters.clone())
            .map_err(|e| anyhow::anyhow!("Invalid parameters: {}", e))?;

        // Validate operations that require specific parameters
        match params.operation.as_str() {
            "inference"
            | "text_generation"
            | "text_classification"
            | "token_classification"
            | "summarization"
            | "translation" => {
                if params.inputs.is_none() {
                    anyhow::bail!("{} operation requires 'inputs' parameter", params.operation);
                }
            }
            "question_answering" => {
                if params.inputs.is_none() {
                    anyhow::bail!(
                        "question_answering operation requires 'inputs' parameter with 'question' and 'context'"
                    );
                }
            }
            "text_to_image" => {
                if params.inputs.is_none() {
                    anyhow::bail!(
                        "text_to_image operation requires 'inputs' parameter with text prompt"
                    );
                }
            }
            "image_classification" | "object_detection" => {
                if params.inputs.is_none() {
                    anyhow::bail!(
                        "{} operation requires 'inputs' parameter with image data",
                        params.operation
                    );
                }
            }
            "speech_recognition" => {
                if params.inputs.is_none() {
                    anyhow::bail!(
                        "speech_recognition operation requires 'inputs' parameter with audio data"
                    );
                }
            }
            "embeddings" => {
                if params.inputs.is_none() {
                    anyhow::bail!("embeddings operation requires 'inputs' parameter");
                }
            }
            "download_model" => {
                if params.repo_id.is_none() {
                    anyhow::bail!("download_model operation requires 'repo_id' parameter");
                }
            }
            "list_models" => {
                // No specific parameters required
            }
            _ => {
                anyhow::bail!("Unsupported operation: {}", params.operation);
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
    fn test_huggingface_node_type() {
        let node = HuggingFaceNode::new();
        assert_eq!(node.type_name(), "huggingface");
        assert!(matches!(node.category(), NodeCategory::Action));
        assert!(matches!(node.subcategory(), NodeSubcategory::AI));
        assert_eq!(node.required_credential_type(), Some("huggingface_api"));
    }

    #[test]
    fn test_huggingface_parameter_schema() {
        let node = HuggingFaceNode::new();
        let schema = node.parameter_schema();

        assert_eq!(schema["type"], "object");
        assert!(schema["properties"]["operation"].is_object());
        assert!(schema["properties"]["model"].is_object());
        assert!(schema["properties"]["inputs"].is_object());
    }

    #[tokio::test]
    async fn test_huggingface_text_generation() {
        let node = HuggingFaceNode::new();
        let context = NodeContext::new(Uuid::new_v4().to_string(), "test-workflow".to_string());

        let params = json!({
            "operation": "text_generation",
            "model": "gpt2",
            "inputs": "Once upon a time",
            "parameters": {
                "max_length": 100,
                "temperature": 0.7
            }
        });

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
        assert_eq!(output.data["operation"], "text_generation");
    }

    #[tokio::test]
    async fn test_huggingface_text_classification() {
        let node = HuggingFaceNode::new();
        let context = NodeContext::new(Uuid::new_v4().to_string(), "test-workflow".to_string());

        let params = json!({
            "operation": "text_classification",
            "model": "distilbert-base-uncased-finetuned-sst-2-english",
            "inputs": "I love this product!"
        });

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
        assert_eq!(output.data["operation"], "text_classification");
        assert!(output.data["predictions"].is_array());
    }

    #[tokio::test]
    async fn test_huggingface_embeddings() {
        let node = HuggingFaceNode::new();
        let context = NodeContext::new(Uuid::new_v4().to_string(), "test-workflow".to_string());

        let params = json!({
            "operation": "embeddings",
            "model": "sentence-transformers/all-MiniLM-L6-v2",
            "inputs": "This is a test sentence"
        });

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
        assert!(output.data["embeddings"].is_array());
    }

    #[tokio::test]
    async fn test_huggingface_list_models() {
        let node = HuggingFaceNode::new();
        let context = NodeContext::new(Uuid::new_v4().to_string(), "test-workflow".to_string());

        let params = json!({
            "operation": "list_models"
        });

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
        assert!(output.data["models"].is_array());
    }

    #[test]
    fn test_huggingface_validation() {
        let node = HuggingFaceNode::new();

        // Valid parameters
        let valid_params = json!({
            "operation": "text_generation",
            "model": "gpt2",
            "inputs": "Test prompt"
        });
        assert!(node.validate_parameters(&valid_params).is_ok());

        // Missing inputs for text_generation
        let invalid_params = json!({
            "operation": "text_generation",
            "model": "gpt2"
        });
        assert!(node.validate_parameters(&invalid_params).is_err());

        // Invalid operation
        let invalid_params = json!({
            "operation": "invalid_operation",
            "inputs": "test"
        });
        assert!(node.validate_parameters(&invalid_params).is_err());

        // Valid list_models (no inputs required)
        let valid_params = json!({
            "operation": "list_models"
        });
        assert!(node.validate_parameters(&valid_params).is_ok());

        // Missing repo_id for download_model
        let invalid_params = json!({
            "operation": "download_model"
        });
        assert!(node.validate_parameters(&invalid_params).is_err());
    }
}
