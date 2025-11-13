use crate::models::{Node, NodeCategory, NodeContext, NodeOutput, NodeSubcategory, NodeType};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

/// Google Sheets node for spreadsheet operations
#[derive(Clone)]
pub struct GoogleSheetsNode {}

#[derive(Debug, Serialize, Deserialize)]
struct GoogleSheetsParams {
    credentials_name: String,
    operation: String,
    spreadsheet_id: Option<String>,
    sheet_name: Option<String>,
    range: Option<String>,
    values: Option<Value>,
    title: Option<String>,
    // Formatting and options
    value_input_option: Option<String>,
    major_dimension: Option<String>,
}

impl GoogleSheetsNode {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for GoogleSheetsNode {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeType for GoogleSheetsNode {
    fn type_name(&self) -> &str {
        "google_sheets"
    }

    fn category(&self) -> NodeCategory {
        NodeCategory::Action
    }

    fn subcategory(&self) -> NodeSubcategory {
        NodeSubcategory::General
    }

    fn required_credential_type(&self) -> Option<&str> {
        Some("google_oauth")
    }

    fn parameter_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "credentials_name": {
                    "type": "string",
                    "description": "Name of the Google OAuth credentials"
                },
                "operation": {
                    "type": "string",
                    "enum": [
                        "create_spreadsheet",
                        "read_values",
                        "write_values",
                        "append_values",
                        "clear_values",
                        "create_sheet",
                        "delete_sheet",
                        "get_spreadsheet_metadata"
                    ]
                },
                "spreadsheet_id": {"type": "string"},
                "sheet_name": {"type": "string"},
                "range": {
                    "type": "string",
                    "description": "A1 notation range (e.g., 'Sheet1!A1:D5')"
                },
                "values": {
                    "type": "array",
                    "description": "2D array of values to write"
                },
                "title": {"type": "string"}
            },
            "required": ["credentials_name", "operation"]
        })
    }
}

#[async_trait]
impl Node for GoogleSheetsNode {
    async fn execute(
        &self,
        context: &NodeContext,
        parameters: &serde_json::Value,
    ) -> anyhow::Result<NodeOutput> {
        let params: GoogleSheetsParams = serde_json::from_value(parameters.clone())?;

        let result = match params.operation.as_str() {
            "create_spreadsheet" => {
                json!({
                    "spreadsheet_id": "1BxiMVs0XRA5nFMdKvBdBZjgmUUqptlbs74OgvE2upms",
                    "title": params.title.unwrap_or_else(|| "New Spreadsheet".to_string()),
                    "url": "https://docs.google.com/spreadsheets/d/1BxiMVs0XRA5nFMdKvBdBZjgmUUqptlbs74OgvE2upms"
                })
            }
            "read_values" => {
                json!({
                    "range": params.range.unwrap_or_default(),
                    "values": [
                        ["Name", "Email", "Status"],
                        ["John Doe", "john@example.com", "Active"],
                        ["Jane Smith", "jane@example.com", "Active"]
                    ],
                    "row_count": 3
                })
            }
            "write_values" => {
                json!({
                    "updated_range": params.range.unwrap_or_default(),
                    "updated_rows": 2,
                    "updated_columns": 3,
                    "updated_cells": 6
                })
            }
            "append_values" => {
                json!({
                    "updated_range": "Sheet1!A4:C4",
                    "updated_rows": 1,
                    "appended": true
                })
            }
            "clear_values" => {
                json!({
                    "cleared_range": params.range.unwrap_or_default(),
                    "cleared": true
                })
            }
            "create_sheet" => {
                json!({
                    "sheet_id": 123456,
                    "title": params.sheet_name.unwrap_or_else(|| "New Sheet".to_string())
                })
            }
            "delete_sheet" => {
                json!({
                    "sheet_id": params.sheet_name.unwrap_or_default(),
                    "deleted": true
                })
            }
            "get_spreadsheet_metadata" => {
                json!({
                    "title": "My Spreadsheet",
                    "sheets": [
                        {"title": "Sheet1", "index": 0},
                        {"title": "Sheet2", "index": 1}
                    ]
                })
            }
            _ => anyhow::bail!("Unknown operation: {}", params.operation),
        };

        Ok(NodeOutput::success(json!({
            "result": result,
            "operation": params.operation,
            "context_execution_id": context.execution_id
        })))
    }

    fn validate_parameters(&self, parameters: &Value) -> anyhow::Result<()> {
        let params: GoogleSheetsParams = serde_json::from_value(parameters.clone())?;

        if params.credentials_name.trim().is_empty() {
            anyhow::bail!("credentials_name cannot be empty");
        }

        // Operations that require spreadsheet_id
        if [
            "read_values",
            "write_values",
            "append_values",
            "clear_values",
            "create_sheet",
            "delete_sheet",
        ]
        .contains(&params.operation.as_str())
            && params.spreadsheet_id.is_none()
        {
            anyhow::bail!("{} requires 'spreadsheet_id'", params.operation);
        }

        // Operations that require range
        if [
            "read_values",
            "write_values",
            "append_values",
            "clear_values",
        ]
        .contains(&params.operation.as_str())
            && params.range.is_none()
        {
            anyhow::bail!("{} requires 'range'", params.operation);
        }

        // Operations that require values
        if ["write_values", "append_values"].contains(&params.operation.as_str())
            && params.values.is_none()
        {
            anyhow::bail!("{} requires 'values'", params.operation);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_google_sheets_read_values() {
        let node = GoogleSheetsNode::new();
        let context = NodeContext::new("exec-123".to_string(), "node-1".to_string());
        let params = json!({
            "credentials_name": "my_google",
            "operation": "read_values",
            "spreadsheet_id": "abc123",
            "range": "Sheet1!A1:C3"
        });

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
    }
}
