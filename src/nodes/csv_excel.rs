use crate::models::{Node, NodeCategory, NodeContext, NodeOutput, NodeSubcategory, NodeType};
use async_trait::async_trait;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CsvExcelParams {
    /// CSV/Excel operation to perform
    pub operation: String,
    /// Input file path
    pub file_path: Option<String>,
    /// Output file path
    pub output_path: Option<String>,
    /// Data to write (array of objects/arrays)
    pub data: Option<serde_json::Value>,
    /// Excel sheet name
    pub sheet_name: Option<String>,
    /// Whether the file/data has headers
    pub has_headers: Option<bool>,
    /// CSV delimiter character
    pub delimiter: Option<String>,
}

/// CSV/Excel Parser node - parses and generates CSV and Excel files
pub struct CsvExcelNode;

impl CsvExcelNode {
    pub fn new() -> Self {
        Self
    }
}

impl Default for CsvExcelNode {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeType for CsvExcelNode {
    fn type_name(&self) -> &str {
        "csv_excel"
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
                    "description": "CSV/Excel operation to perform",
                    "enum": [
                        "parse_csv",
                        "generate_csv",
                        "parse_excel",
                        "generate_excel",
                        "convert_csv_to_excel",
                        "convert_excel_to_csv"
                    ]
                },
                "file_path": {
                    "type": "string",
                    "description": "Path to the input file"
                },
                "output_path": {
                    "type": "string",
                    "description": "Path for the output file"
                },
                "data": {
                    "description": "Data to write (array of objects or arrays)"
                },
                "sheet_name": {
                    "type": "string",
                    "description": "Excel sheet name",
                    "default": "Sheet1"
                },
                "has_headers": {
                    "type": "boolean",
                    "description": "Whether the data has headers",
                    "default": true
                },
                "delimiter": {
                    "type": "string",
                    "description": "CSV delimiter character",
                    "default": ",",
                    "maxLength": 1
                }
            },
            "required": ["operation"],
            "additionalProperties": false
        })
    }

    fn required_credential_type(&self) -> Option<&str> {
        None // No credentials needed for CSV/Excel operations
    }
}

#[async_trait]
impl Node for CsvExcelNode {
    async fn execute(
        &self,
        context: &NodeContext,
        parameters: &serde_json::Value,
    ) -> anyhow::Result<NodeOutput> {
        let params: CsvExcelParams = serde_json::from_value(parameters.clone())?;

        // TODO: In a real implementation:
        // 1. Execute the operation based on params.operation:
        //    - parse_csv: Read CSV file and convert to JSON/array
        //    - generate_csv: Convert JSON/array to CSV file
        //    - parse_excel: Read Excel file and convert to JSON/array
        //    - generate_excel: Convert JSON/array to Excel file
        //    - convert_csv_to_excel: Convert CSV file to Excel format
        //    - convert_excel_to_csv: Convert Excel file to CSV format
        // 2. Handle headers and delimiters appropriately
        // 3. For Excel, handle multiple sheets if needed
        // 4. Return parsed data or confirmation of file generation

        // For now, we'll return a placeholder response
        let result = serde_json::json!({
            "message": "CSV/Excel operation executed (placeholder implementation)",
            "operation": &params.operation,
            "file_path": params.file_path,
            "output_path": params.output_path,
            "sheet_name": params.sheet_name.as_ref().unwrap_or(&"Sheet1".to_string()),
            "has_headers": params.has_headers.unwrap_or(true),
            "context_execution_id": &context.execution_id,
            "success": true
        });

        Ok(NodeOutput::success(result))
    }

    fn validate_parameters(&self, parameters: &serde_json::Value) -> anyhow::Result<()> {
        let params: CsvExcelParams = serde_json::from_value(parameters.clone())
            .map_err(|e| anyhow::anyhow!("Invalid parameters: {}", e))?;

        // Validate operation
        let valid_operations = [
            "parse_csv",
            "generate_csv",
            "parse_excel",
            "generate_excel",
            "convert_csv_to_excel",
            "convert_excel_to_csv",
        ];

        if !valid_operations.contains(&params.operation.as_str()) {
            anyhow::bail!(
                "Invalid operation: {}. Must be one of: {}",
                params.operation,
                valid_operations.join(", ")
            );
        }

        // Validate parse operations require file_path
        let parse_ops = [
            "parse_csv",
            "parse_excel",
            "convert_csv_to_excel",
            "convert_excel_to_csv",
        ];
        if parse_ops.contains(&params.operation.as_str()) && params.file_path.is_none() {
            anyhow::bail!(
                "{} operation requires 'file_path' parameter",
                params.operation
            );
        }

        // Validate generate operations require data
        let generate_ops = ["generate_csv", "generate_excel"];
        if generate_ops.contains(&params.operation.as_str()) && params.data.is_none() {
            anyhow::bail!("{} operation requires 'data' parameter", params.operation);
        }

        // Validate conversion operations require output_path
        let convert_ops = ["convert_csv_to_excel", "convert_excel_to_csv"];
        if convert_ops.contains(&params.operation.as_str()) && params.output_path.is_none() {
            anyhow::bail!(
                "{} operation requires 'output_path' parameter",
                params.operation
            );
        }

        // Validate delimiter
        if let Some(ref delimiter) = params.delimiter
            && delimiter.len() != 1
        {
            anyhow::bail!("delimiter must be a single character");
        }

        // Validate data is an array for generate operations
        if generate_ops.contains(&params.operation.as_str())
            && let Some(ref data) = params.data
            && !data.is_array()
        {
            anyhow::bail!("data must be an array");
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
    fn test_csv_excel_node_type() {
        let node = CsvExcelNode::new();
        assert_eq!(node.type_name(), "csv_excel");
        assert!(matches!(node.category(), NodeCategory::Action));
        assert!(matches!(node.subcategory(), NodeSubcategory::General));
        assert_eq!(node.required_credential_type(), None);
    }

    #[test]
    fn test_csv_excel_parameter_schema() {
        let node = CsvExcelNode::new();
        let schema = node.parameter_schema();

        assert_eq!(schema["type"], "object");
        assert!(schema["properties"]["operation"].is_object());
        assert!(schema["properties"]["file_path"].is_object());
    }

    #[tokio::test]
    async fn test_csv_excel_parse_csv() {
        let node = CsvExcelNode::new();
        let context = NodeContext::new(Uuid::new_v4().to_string(), "test-workflow".to_string());

        let params = json!({
            "operation": "parse_csv",
            "file_path": "/tmp/data.csv",
            "has_headers": true,
            "delimiter": ","
        });

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
    }

    #[tokio::test]
    async fn test_csv_excel_generate_excel() {
        let node = CsvExcelNode::new();
        let context = NodeContext::new(Uuid::new_v4().to_string(), "test-workflow".to_string());

        let params = json!({
            "operation": "generate_excel",
            "data": [
                {"name": "John", "age": 30},
                {"name": "Jane", "age": 25}
            ],
            "output_path": "/tmp/output.xlsx",
            "sheet_name": "Users"
        });

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
    }

    #[test]
    fn test_csv_excel_validation() {
        let node = CsvExcelNode::new();

        // Valid parse_csv
        let valid_params = json!({
            "operation": "parse_csv",
            "file_path": "/tmp/data.csv"
        });
        assert!(node.validate_parameters(&valid_params).is_ok());

        // Missing file_path for parse_csv
        let invalid_params = json!({
            "operation": "parse_csv"
        });
        assert!(node.validate_parameters(&invalid_params).is_err());

        // Missing data for generate_excel
        let invalid_params = json!({
            "operation": "generate_excel",
            "output_path": "/tmp/output.xlsx"
        });
        assert!(node.validate_parameters(&invalid_params).is_err());

        // Invalid delimiter (too long)
        let invalid_params = json!({
            "operation": "parse_csv",
            "file_path": "/tmp/data.csv",
            "delimiter": "abc"
        });
        assert!(node.validate_parameters(&invalid_params).is_err());

        // Invalid operation
        let invalid_params = json!({
            "operation": "invalid_operation"
        });
        assert!(node.validate_parameters(&invalid_params).is_err());
    }
}
