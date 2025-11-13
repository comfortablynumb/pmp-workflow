use crate::models::{Node, NodeCategory, NodeContext, NodeOutput, NodeSubcategory, NodeType};
use async_trait::async_trait;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct JsonXmlConverterParams {
    /// Conversion/validation operation to perform
    pub operation: String,
    /// Input data (JSON or XML string)
    pub input_data: Option<String>,
    /// Output file path
    pub output_path: Option<String>,
    /// Whether to pretty print the output
    pub pretty_print: Option<bool>,
    /// Root element name for XML (when converting from JSON)
    pub root_element_name: Option<String>,
}

/// JSON/XML Converter node - converts and validates JSON and XML data
pub struct JsonXmlConverterNode;

impl JsonXmlConverterNode {
    pub fn new() -> Self {
        Self
    }
}

impl Default for JsonXmlConverterNode {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeType for JsonXmlConverterNode {
    fn type_name(&self) -> &str {
        "json_xml_converter"
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
                    "description": "Conversion or validation operation to perform",
                    "enum": [
                        "json_to_xml",
                        "xml_to_json",
                        "validate_json",
                        "validate_xml",
                        "format_json",
                        "format_xml"
                    ]
                },
                "input_data": {
                    "type": "string",
                    "description": "Input data as JSON or XML string"
                },
                "output_path": {
                    "type": "string",
                    "description": "Optional output file path to save the result"
                },
                "pretty_print": {
                    "type": "boolean",
                    "description": "Whether to format output with indentation",
                    "default": true
                },
                "root_element_name": {
                    "type": "string",
                    "description": "Root element name for XML output (when converting from JSON)",
                    "default": "root"
                }
            },
            "required": ["operation", "input_data"],
            "additionalProperties": false
        })
    }

    fn required_credential_type(&self) -> Option<&str> {
        None // No credentials needed for JSON/XML operations
    }
}

#[async_trait]
impl Node for JsonXmlConverterNode {
    async fn execute(
        &self,
        context: &NodeContext,
        parameters: &serde_json::Value,
    ) -> anyhow::Result<NodeOutput> {
        let params: JsonXmlConverterParams = serde_json::from_value(parameters.clone())?;

        // TODO: In a real implementation:
        // 1. Execute the operation based on params.operation:
        //    - json_to_xml: Parse JSON and convert to XML format
        //    - xml_to_json: Parse XML and convert to JSON format
        //    - validate_json: Validate JSON syntax
        //    - validate_xml: Validate XML syntax
        //    - format_json: Format/pretty-print JSON
        //    - format_xml: Format/pretty-print XML
        // 2. Apply pretty_print setting if requested
        // 3. For json_to_xml, use root_element_name for the XML root
        // 4. Save to output_path if provided
        // 5. Return the converted/formatted/validated data

        // For now, we'll return a placeholder response
        let result = serde_json::json!({
            "message": "JSON/XML conversion executed (placeholder implementation)",
            "operation": &params.operation,
            "pretty_print": params.pretty_print.unwrap_or(true),
            "root_element_name": params.root_element_name.as_ref().unwrap_or(&"root".to_string()),
            "output_path": params.output_path,
            "context_execution_id": &context.execution_id,
            "success": true
        });

        Ok(NodeOutput::success(result))
    }

    fn validate_parameters(&self, parameters: &serde_json::Value) -> anyhow::Result<()> {
        let params: JsonXmlConverterParams = serde_json::from_value(parameters.clone())
            .map_err(|e| anyhow::anyhow!("Invalid parameters: {}", e))?;

        // Validate operation
        let valid_operations = [
            "json_to_xml",
            "xml_to_json",
            "validate_json",
            "validate_xml",
            "format_json",
            "format_xml",
        ];

        if !valid_operations.contains(&params.operation.as_str()) {
            anyhow::bail!(
                "Invalid operation: {}. Must be one of: {}",
                params.operation,
                valid_operations.join(", ")
            );
        }

        // Validate input_data is provided
        if params.input_data.is_none() {
            anyhow::bail!("'input_data' parameter is required");
        }

        // Validate input_data is not empty
        if let Some(ref input) = params.input_data
            && input.trim().is_empty()
        {
            anyhow::bail!("'input_data' cannot be empty");
        }

        // For json_to_xml, validate root_element_name if provided
        if params.operation == "json_to_xml"
            && let Some(ref root) = params.root_element_name
            && root.trim().is_empty()
        {
            anyhow::bail!("'root_element_name' cannot be empty");
        }
        // Basic validation: must start with letter or underscore
        if params.operation == "json_to_xml"
            && let Some(ref root) = params.root_element_name
            && !root.chars().next().unwrap().is_alphabetic()
            && !root.starts_with('_')
        {
            anyhow::bail!("'root_element_name' must start with a letter or underscore");
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
    fn test_json_xml_converter_node_type() {
        let node = JsonXmlConverterNode::new();
        assert_eq!(node.type_name(), "json_xml_converter");
        assert!(matches!(node.category(), NodeCategory::Action));
        assert!(matches!(node.subcategory(), NodeSubcategory::General));
        assert_eq!(node.required_credential_type(), None);
    }

    #[test]
    fn test_json_xml_converter_parameter_schema() {
        let node = JsonXmlConverterNode::new();
        let schema = node.parameter_schema();

        assert_eq!(schema["type"], "object");
        assert!(schema["properties"]["operation"].is_object());
        assert!(schema["properties"]["input_data"].is_object());
    }

    #[tokio::test]
    async fn test_json_xml_converter_json_to_xml() {
        let node = JsonXmlConverterNode::new();
        let context = NodeContext::new(Uuid::new_v4().to_string(), "test-workflow".to_string());

        let params = json!({
            "operation": "json_to_xml",
            "input_data": r#"{"name": "John", "age": 30}"#,
            "pretty_print": true,
            "root_element_name": "person"
        });

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
    }

    #[tokio::test]
    async fn test_json_xml_converter_validate_json() {
        let node = JsonXmlConverterNode::new();
        let context = NodeContext::new(Uuid::new_v4().to_string(), "test-workflow".to_string());

        let params = json!({
            "operation": "validate_json",
            "input_data": r#"{"valid": true}"#
        });

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
    }

    #[test]
    fn test_json_xml_converter_validation() {
        let node = JsonXmlConverterNode::new();

        // Valid json_to_xml
        let valid_params = json!({
            "operation": "json_to_xml",
            "input_data": r#"{"test": "data"}"#,
            "root_element_name": "root"
        });
        assert!(node.validate_parameters(&valid_params).is_ok());

        // Missing input_data
        let invalid_params = json!({
            "operation": "json_to_xml"
        });
        assert!(node.validate_parameters(&invalid_params).is_err());

        // Empty input_data
        let invalid_params = json!({
            "operation": "validate_json",
            "input_data": "   "
        });
        assert!(node.validate_parameters(&invalid_params).is_err());

        // Invalid root_element_name (starts with number)
        let invalid_params = json!({
            "operation": "json_to_xml",
            "input_data": r#"{"test": "data"}"#,
            "root_element_name": "1root"
        });
        assert!(node.validate_parameters(&invalid_params).is_err());

        // Invalid operation
        let invalid_params = json!({
            "operation": "invalid_operation",
            "input_data": "test"
        });
        assert!(node.validate_parameters(&invalid_params).is_err());
    }
}
