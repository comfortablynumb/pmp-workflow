use crate::models::{Node, NodeCategory, NodeContext, NodeOutput, NodeSubcategory, NodeType};
use async_trait::async_trait;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PdfGeneratorParams {
    /// PDF generation operation to perform
    pub operation: String,
    /// Content (HTML, Markdown, or template data)
    pub content: Option<String>,
    /// Template name (for template-based generation)
    pub template_name: Option<String>,
    /// Output file path
    pub output_path: Option<String>,
    /// Page size (A4, Letter, Legal, etc.)
    pub page_size: Option<String>,
    /// Page orientation (portrait, landscape)
    pub orientation: Option<String>,
    /// Page margin (in mm)
    pub margin: Option<f64>,
    /// PDF file paths (for merge operation)
    pub pdf_paths: Option<Vec<String>>,
}

/// PDF Generator node - generates and manipulates PDF documents
pub struct PdfGeneratorNode;

impl PdfGeneratorNode {
    pub fn new() -> Self {
        Self
    }
}

impl Default for PdfGeneratorNode {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeType for PdfGeneratorNode {
    fn type_name(&self) -> &str {
        "pdf_generator"
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
                    "description": "PDF generation operation to perform",
                    "enum": [
                        "generate_from_html",
                        "generate_from_markdown",
                        "generate_from_template",
                        "merge_pdfs"
                    ]
                },
                "content": {
                    "type": "string",
                    "description": "Content to convert to PDF (HTML or Markdown)"
                },
                "template_name": {
                    "type": "string",
                    "description": "Template name for template-based generation"
                },
                "output_path": {
                    "type": "string",
                    "description": "Output file path for the generated PDF"
                },
                "page_size": {
                    "type": "string",
                    "description": "Page size",
                    "enum": ["A4", "Letter", "Legal", "A3", "A5"],
                    "default": "A4"
                },
                "orientation": {
                    "type": "string",
                    "description": "Page orientation",
                    "enum": ["portrait", "landscape"],
                    "default": "portrait"
                },
                "margin": {
                    "type": "number",
                    "description": "Page margin in millimeters",
                    "minimum": 0,
                    "maximum": 100,
                    "default": 10
                },
                "pdf_paths": {
                    "type": "array",
                    "description": "Array of PDF file paths to merge",
                    "items": {
                        "type": "string"
                    }
                }
            },
            "required": ["operation"],
            "additionalProperties": false
        })
    }

    fn required_credential_type(&self) -> Option<&str> {
        None // No credentials needed for PDF generation
    }
}

#[async_trait]
impl Node for PdfGeneratorNode {
    async fn execute(
        &self,
        context: &NodeContext,
        parameters: &serde_json::Value,
    ) -> anyhow::Result<NodeOutput> {
        let params: PdfGeneratorParams = serde_json::from_value(parameters.clone())?;

        // TODO: In a real implementation:
        // 1. Execute the PDF operation based on params.operation:
        //    - generate_from_html: Convert HTML to PDF using a library like wkhtmltopdf or headless Chrome
        //    - generate_from_markdown: Convert Markdown to HTML first, then to PDF
        //    - generate_from_template: Load template, render with data, then convert to PDF
        //    - merge_pdfs: Merge multiple PDF files into one
        // 2. Apply page settings (size, orientation, margins)
        // 3. Save to output_path
        // 4. Return the file path and metadata

        // For now, we'll return a placeholder response
        let result = serde_json::json!({
            "message": "PDF generation executed (placeholder implementation)",
            "operation": &params.operation,
            "output_path": params.output_path,
            "page_size": params.page_size.as_ref().unwrap_or(&"A4".to_string()),
            "orientation": params.orientation.as_ref().unwrap_or(&"portrait".to_string()),
            "context_execution_id": &context.execution_id,
            "success": true
        });

        Ok(NodeOutput::success(result))
    }

    fn validate_parameters(&self, parameters: &serde_json::Value) -> anyhow::Result<()> {
        let params: PdfGeneratorParams = serde_json::from_value(parameters.clone())
            .map_err(|e| anyhow::anyhow!("Invalid parameters: {}", e))?;

        // Validate operation
        let valid_operations = [
            "generate_from_html",
            "generate_from_markdown",
            "generate_from_template",
            "merge_pdfs",
        ];

        if !valid_operations.contains(&params.operation.as_str()) {
            anyhow::bail!(
                "Invalid operation: {}. Must be one of: {}",
                params.operation,
                valid_operations.join(", ")
            );
        }

        // Validate content for HTML and Markdown operations
        let content_ops = ["generate_from_html", "generate_from_markdown"];
        if content_ops.contains(&params.operation.as_str()) && params.content.is_none() {
            anyhow::bail!(
                "{} operation requires 'content' parameter",
                params.operation
            );
        }

        // Validate template_name for template operation
        if params.operation == "generate_from_template" && params.template_name.is_none() {
            anyhow::bail!("generate_from_template operation requires 'template_name' parameter");
        }

        // Validate pdf_paths for merge operation
        if params.operation == "merge_pdfs" {
            if params.pdf_paths.is_none() {
                anyhow::bail!("merge_pdfs operation requires 'pdf_paths' parameter");
            }
            if let Some(ref paths) = params.pdf_paths
                && paths.len() < 2
            {
                anyhow::bail!("merge_pdfs operation requires at least 2 PDF files");
            }
        }

        // Validate page_size
        if let Some(ref page_size) = params.page_size {
            let valid_sizes = ["A4", "Letter", "Legal", "A3", "A5"];
            if !valid_sizes.contains(&page_size.as_str()) {
                anyhow::bail!(
                    "Invalid page_size: {}. Must be one of: {}",
                    page_size,
                    valid_sizes.join(", ")
                );
            }
        }

        // Validate orientation
        if let Some(ref orientation) = params.orientation {
            let valid_orientations = ["portrait", "landscape"];
            if !valid_orientations.contains(&orientation.as_str()) {
                anyhow::bail!(
                    "Invalid orientation: {}. Must be one of: {}",
                    orientation,
                    valid_orientations.join(", ")
                );
            }
        }

        // Validate margin
        if let Some(margin) = params.margin
            && !(0.0..=100.0).contains(&margin)
        {
            anyhow::bail!("margin must be between 0 and 100 mm");
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
    fn test_pdf_generator_node_type() {
        let node = PdfGeneratorNode::new();
        assert_eq!(node.type_name(), "pdf_generator");
        assert!(matches!(node.category(), NodeCategory::Action));
        assert!(matches!(node.subcategory(), NodeSubcategory::General));
        assert_eq!(node.required_credential_type(), None);
    }

    #[test]
    fn test_pdf_generator_parameter_schema() {
        let node = PdfGeneratorNode::new();
        let schema = node.parameter_schema();

        assert_eq!(schema["type"], "object");
        assert!(schema["properties"]["operation"].is_object());
        assert!(schema["properties"]["content"].is_object());
    }

    #[tokio::test]
    async fn test_pdf_generator_from_html() {
        let node = PdfGeneratorNode::new();
        let context = NodeContext::new(Uuid::new_v4().to_string(), "test-workflow".to_string());

        let params = json!({
            "operation": "generate_from_html",
            "content": "<html><body><h1>Test PDF</h1></body></html>",
            "output_path": "/tmp/test.pdf",
            "page_size": "A4",
            "orientation": "portrait"
        });

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
    }

    #[tokio::test]
    async fn test_pdf_generator_merge() {
        let node = PdfGeneratorNode::new();
        let context = NodeContext::new(Uuid::new_v4().to_string(), "test-workflow".to_string());

        let params = json!({
            "operation": "merge_pdfs",
            "pdf_paths": ["/tmp/file1.pdf", "/tmp/file2.pdf"],
            "output_path": "/tmp/merged.pdf"
        });

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
    }

    #[test]
    fn test_pdf_generator_validation() {
        let node = PdfGeneratorNode::new();

        // Valid generate_from_html
        let valid_params = json!({
            "operation": "generate_from_html",
            "content": "<html><body>Test</body></html>",
            "output_path": "/tmp/test.pdf"
        });
        assert!(node.validate_parameters(&valid_params).is_ok());

        // Missing content for generate_from_html
        let invalid_params = json!({
            "operation": "generate_from_html",
            "output_path": "/tmp/test.pdf"
        });
        assert!(node.validate_parameters(&invalid_params).is_err());

        // Invalid page_size
        let invalid_params = json!({
            "operation": "generate_from_html",
            "content": "test",
            "page_size": "B4"
        });
        assert!(node.validate_parameters(&invalid_params).is_err());

        // merge_pdfs with less than 2 files
        let invalid_params = json!({
            "operation": "merge_pdfs",
            "pdf_paths": ["/tmp/file1.pdf"]
        });
        assert!(node.validate_parameters(&invalid_params).is_err());
    }
}
