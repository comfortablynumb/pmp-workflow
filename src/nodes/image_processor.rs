use crate::models::{Node, NodeCategory, NodeContext, NodeOutput, NodeSubcategory, NodeType};
use async_trait::async_trait;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ImageProcessorParams {
    /// Image processing operation to perform
    pub operation: String,
    /// Input image file path
    pub input_path: Option<String>,
    /// Output image file path
    pub output_path: Option<String>,
    /// Width in pixels
    pub width: Option<u32>,
    /// Height in pixels
    pub height: Option<u32>,
    /// Image quality (1-100)
    pub quality: Option<u8>,
    /// Output format (png, jpg, webp, gif)
    pub format: Option<String>,
    /// Rotation degrees (90, 180, 270)
    pub rotation_degrees: Option<u32>,
    /// Filter type (blur, sharpen, grayscale, sepia)
    pub filter_type: Option<String>,
    /// Watermark text
    pub watermark_text: Option<String>,
}

/// Image Processor node - processes and manipulates images
pub struct ImageProcessorNode;

impl ImageProcessorNode {
    pub fn new() -> Self {
        Self
    }
}

impl Default for ImageProcessorNode {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeType for ImageProcessorNode {
    fn type_name(&self) -> &str {
        "image_processor"
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
                    "description": "Image processing operation to perform",
                    "enum": [
                        "resize",
                        "crop",
                        "rotate",
                        "optimize",
                        "convert_format",
                        "apply_filter",
                        "add_watermark"
                    ]
                },
                "input_path": {
                    "type": "string",
                    "description": "Path to the input image file"
                },
                "output_path": {
                    "type": "string",
                    "description": "Path for the output image file"
                },
                "width": {
                    "type": "integer",
                    "description": "Width in pixels",
                    "minimum": 1,
                    "maximum": 10000
                },
                "height": {
                    "type": "integer",
                    "description": "Height in pixels",
                    "minimum": 1,
                    "maximum": 10000
                },
                "quality": {
                    "type": "integer",
                    "description": "Image quality (1-100, higher is better)",
                    "minimum": 1,
                    "maximum": 100
                },
                "format": {
                    "type": "string",
                    "description": "Output image format",
                    "enum": ["png", "jpg", "jpeg", "webp", "gif"]
                },
                "rotation_degrees": {
                    "type": "integer",
                    "description": "Rotation angle in degrees",
                    "enum": [90, 180, 270]
                },
                "filter_type": {
                    "type": "string",
                    "description": "Filter to apply to the image",
                    "enum": ["blur", "sharpen", "grayscale", "sepia", "brighten", "contrast"]
                },
                "watermark_text": {
                    "type": "string",
                    "description": "Text to add as watermark"
                }
            },
            "required": ["operation"],
            "additionalProperties": false
        })
    }

    fn required_credential_type(&self) -> Option<&str> {
        None // No credentials needed for image processing
    }
}

#[async_trait]
impl Node for ImageProcessorNode {
    async fn execute(
        &self,
        context: &NodeContext,
        parameters: &serde_json::Value,
    ) -> anyhow::Result<NodeOutput> {
        let params: ImageProcessorParams = serde_json::from_value(parameters.clone())?;

        // TODO: In a real implementation:
        // 1. Load the image from input_path
        // 2. Execute the operation based on params.operation:
        //    - resize: Resize image to specified width/height
        //    - crop: Crop image to specified dimensions
        //    - rotate: Rotate image by specified degrees
        //    - optimize: Optimize image file size while maintaining quality
        //    - convert_format: Convert image to different format
        //    - apply_filter: Apply visual filter (blur, sharpen, etc.)
        //    - add_watermark: Add text watermark to image
        // 3. Save the processed image to output_path
        // 4. Return metadata about the processed image

        // For now, we'll return a placeholder response
        let result = serde_json::json!({
            "message": "Image processing executed (placeholder implementation)",
            "operation": &params.operation,
            "input_path": params.input_path,
            "output_path": params.output_path,
            "width": params.width,
            "height": params.height,
            "context_execution_id": &context.execution_id,
            "success": true
        });

        Ok(NodeOutput::success(result))
    }

    fn validate_parameters(&self, parameters: &serde_json::Value) -> anyhow::Result<()> {
        let params: ImageProcessorParams = serde_json::from_value(parameters.clone())
            .map_err(|e| anyhow::anyhow!("Invalid parameters: {}", e))?;

        // Validate operation
        let valid_operations = [
            "resize",
            "crop",
            "rotate",
            "optimize",
            "convert_format",
            "apply_filter",
            "add_watermark",
        ];

        if !valid_operations.contains(&params.operation.as_str()) {
            anyhow::bail!(
                "Invalid operation: {}. Must be one of: {}",
                params.operation,
                valid_operations.join(", ")
            );
        }

        // All operations require input_path
        if params.input_path.is_none() {
            anyhow::bail!("'input_path' parameter is required");
        }

        // Validate resize and crop operations
        let dimension_ops = ["resize", "crop"];
        if dimension_ops.contains(&params.operation.as_str())
            && params.width.is_none()
            && params.height.is_none()
        {
            anyhow::bail!(
                "{} operation requires at least 'width' or 'height' parameter",
                params.operation
            );
        }

        // Validate rotate operation
        if params.operation == "rotate" {
            if params.rotation_degrees.is_none() {
                anyhow::bail!("rotate operation requires 'rotation_degrees' parameter");
            }
            if let Some(degrees) = params.rotation_degrees
                && ![90, 180, 270].contains(&degrees)
            {
                anyhow::bail!("rotation_degrees must be 90, 180, or 270");
            }
        }

        // Validate convert_format operation
        if params.operation == "convert_format" && params.format.is_none() {
            anyhow::bail!("convert_format operation requires 'format' parameter");
        }

        // Validate apply_filter operation
        if params.operation == "apply_filter" && params.filter_type.is_none() {
            anyhow::bail!("apply_filter operation requires 'filter_type' parameter");
        }

        // Validate add_watermark operation
        if params.operation == "add_watermark" && params.watermark_text.is_none() {
            anyhow::bail!("add_watermark operation requires 'watermark_text' parameter");
        }

        // Validate quality range
        if let Some(quality) = params.quality
            && !(1..=100).contains(&quality)
        {
            anyhow::bail!("quality must be between 1 and 100");
        }

        // Validate dimensions
        if let Some(width) = params.width
            && !(1..=10000).contains(&width)
        {
            anyhow::bail!("width must be between 1 and 10000");
        }
        if let Some(height) = params.height
            && !(1..=10000).contains(&height)
        {
            anyhow::bail!("height must be between 1 and 10000");
        }

        // Validate format
        if let Some(ref format) = params.format {
            let valid_formats = ["png", "jpg", "jpeg", "webp", "gif"];
            if !valid_formats.contains(&format.as_str()) {
                anyhow::bail!(
                    "Invalid format: {}. Must be one of: {}",
                    format,
                    valid_formats.join(", ")
                );
            }
        }

        // Validate filter_type
        if let Some(ref filter) = params.filter_type {
            let valid_filters = [
                "blur",
                "sharpen",
                "grayscale",
                "sepia",
                "brighten",
                "contrast",
            ];
            if !valid_filters.contains(&filter.as_str()) {
                anyhow::bail!(
                    "Invalid filter_type: {}. Must be one of: {}",
                    filter,
                    valid_filters.join(", ")
                );
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
    fn test_image_processor_node_type() {
        let node = ImageProcessorNode::new();
        assert_eq!(node.type_name(), "image_processor");
        assert!(matches!(node.category(), NodeCategory::Action));
        assert!(matches!(node.subcategory(), NodeSubcategory::General));
        assert_eq!(node.required_credential_type(), None);
    }

    #[test]
    fn test_image_processor_parameter_schema() {
        let node = ImageProcessorNode::new();
        let schema = node.parameter_schema();

        assert_eq!(schema["type"], "object");
        assert!(schema["properties"]["operation"].is_object());
        assert!(schema["properties"]["input_path"].is_object());
    }

    #[tokio::test]
    async fn test_image_processor_resize() {
        let node = ImageProcessorNode::new();
        let context = NodeContext::new(Uuid::new_v4().to_string(), "test-workflow".to_string());

        let params = json!({
            "operation": "resize",
            "input_path": "/tmp/image.jpg",
            "output_path": "/tmp/resized.jpg",
            "width": 800,
            "height": 600
        });

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
    }

    #[tokio::test]
    async fn test_image_processor_apply_filter() {
        let node = ImageProcessorNode::new();
        let context = NodeContext::new(Uuid::new_v4().to_string(), "test-workflow".to_string());

        let params = json!({
            "operation": "apply_filter",
            "input_path": "/tmp/image.jpg",
            "output_path": "/tmp/filtered.jpg",
            "filter_type": "grayscale"
        });

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
    }

    #[test]
    fn test_image_processor_validation() {
        let node = ImageProcessorNode::new();

        // Valid resize
        let valid_params = json!({
            "operation": "resize",
            "input_path": "/tmp/image.jpg",
            "width": 800
        });
        assert!(node.validate_parameters(&valid_params).is_ok());

        // Missing input_path
        let invalid_params = json!({
            "operation": "resize",
            "width": 800
        });
        assert!(node.validate_parameters(&invalid_params).is_err());

        // Invalid rotation_degrees
        let invalid_params = json!({
            "operation": "rotate",
            "input_path": "/tmp/image.jpg",
            "rotation_degrees": 45
        });
        assert!(node.validate_parameters(&invalid_params).is_err());

        // Missing filter_type for apply_filter
        let invalid_params = json!({
            "operation": "apply_filter",
            "input_path": "/tmp/image.jpg"
        });
        assert!(node.validate_parameters(&invalid_params).is_err());

        // Invalid quality
        let invalid_params = json!({
            "operation": "optimize",
            "input_path": "/tmp/image.jpg",
            "quality": 150
        });
        assert!(node.validate_parameters(&invalid_params).is_err());
    }
}
