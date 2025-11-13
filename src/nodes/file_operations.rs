use crate::models::{Node, NodeCategory, NodeContext, NodeOutput, NodeSubcategory, NodeType};
use async_trait::async_trait;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct FileOperationsParams {
    /// File operation to perform
    pub operation: String,
    /// File path (for read, write, append, delete operations)
    pub file_path: Option<String>,
    /// Content to write (for write, append operations)
    pub content: Option<String>,
    /// Source path (for copy, move operations)
    pub source_path: Option<String>,
    /// Destination path (for copy, move operations)
    pub destination_path: Option<String>,
    /// Directory path (for list_directory, create_directory operations)
    pub directory_path: Option<String>,
}

/// File Operations node - performs file system operations
pub struct FileOperationsNode;

impl FileOperationsNode {
    pub fn new() -> Self {
        Self
    }
}

impl Default for FileOperationsNode {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeType for FileOperationsNode {
    fn type_name(&self) -> &str {
        "file_operations"
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
                    "description": "File operation to perform",
                    "enum": [
                        "read_file",
                        "write_file",
                        "append_file",
                        "delete_file",
                        "copy_file",
                        "move_file",
                        "list_directory",
                        "create_directory"
                    ]
                },
                "file_path": {
                    "type": "string",
                    "description": "Path to the file"
                },
                "content": {
                    "type": "string",
                    "description": "Content to write or append"
                },
                "source_path": {
                    "type": "string",
                    "description": "Source path for copy or move operations"
                },
                "destination_path": {
                    "type": "string",
                    "description": "Destination path for copy or move operations"
                },
                "directory_path": {
                    "type": "string",
                    "description": "Directory path for list or create operations"
                }
            },
            "required": ["operation"],
            "additionalProperties": false
        })
    }

    fn required_credential_type(&self) -> Option<&str> {
        None // No credentials needed for file operations
    }
}

#[async_trait]
impl Node for FileOperationsNode {
    async fn execute(
        &self,
        context: &NodeContext,
        parameters: &serde_json::Value,
    ) -> anyhow::Result<NodeOutput> {
        let params: FileOperationsParams = serde_json::from_value(parameters.clone())?;

        // TODO: In a real implementation:
        // 1. Execute the file operation based on params.operation:
        //    - read_file: Read file contents
        //    - write_file: Write content to file (overwrite)
        //    - append_file: Append content to file
        //    - delete_file: Delete a file
        //    - copy_file: Copy file from source to destination
        //    - move_file: Move/rename file from source to destination
        //    - list_directory: List files in directory
        //    - create_directory: Create a new directory
        // 2. Handle errors appropriately (file not found, permission denied, etc.)
        // 3. Return operation result

        // For now, we'll return a placeholder response
        let result = serde_json::json!({
            "message": "File operation executed (placeholder implementation)",
            "operation": &params.operation,
            "file_path": params.file_path,
            "source_path": params.source_path,
            "destination_path": params.destination_path,
            "directory_path": params.directory_path,
            "context_execution_id": &context.execution_id,
            "success": true
        });

        Ok(NodeOutput::success(result))
    }

    fn validate_parameters(&self, parameters: &serde_json::Value) -> anyhow::Result<()> {
        let params: FileOperationsParams = serde_json::from_value(parameters.clone())
            .map_err(|e| anyhow::anyhow!("Invalid parameters: {}", e))?;

        // Validate operation
        let valid_operations = [
            "read_file",
            "write_file",
            "append_file",
            "delete_file",
            "copy_file",
            "move_file",
            "list_directory",
            "create_directory",
        ];

        if !valid_operations.contains(&params.operation.as_str()) {
            anyhow::bail!(
                "Invalid operation: {}. Must be one of: {}",
                params.operation,
                valid_operations.join(", ")
            );
        }

        // Validate file_path for single file operations
        let file_path_ops = ["read_file", "write_file", "append_file", "delete_file"];
        if file_path_ops.contains(&params.operation.as_str()) && params.file_path.is_none() {
            anyhow::bail!(
                "{} operation requires 'file_path' parameter",
                params.operation
            );
        }

        // Validate content for write and append operations
        let content_ops = ["write_file", "append_file"];
        if content_ops.contains(&params.operation.as_str()) && params.content.is_none() {
            anyhow::bail!(
                "{} operation requires 'content' parameter",
                params.operation
            );
        }

        // Validate copy and move operations
        let copy_move_ops = ["copy_file", "move_file"];
        if copy_move_ops.contains(&params.operation.as_str()) {
            if params.source_path.is_none() {
                anyhow::bail!(
                    "{} operation requires 'source_path' parameter",
                    params.operation
                );
            }
            if params.destination_path.is_none() {
                anyhow::bail!(
                    "{} operation requires 'destination_path' parameter",
                    params.operation
                );
            }
        }

        // Validate directory operations
        let dir_ops = ["list_directory", "create_directory"];
        if dir_ops.contains(&params.operation.as_str()) && params.directory_path.is_none() {
            anyhow::bail!(
                "{} operation requires 'directory_path' parameter",
                params.operation
            );
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
    fn test_file_operations_node_type() {
        let node = FileOperationsNode::new();
        assert_eq!(node.type_name(), "file_operations");
        assert!(matches!(node.category(), NodeCategory::Action));
        assert!(matches!(node.subcategory(), NodeSubcategory::General));
        assert_eq!(node.required_credential_type(), None);
    }

    #[test]
    fn test_file_operations_parameter_schema() {
        let node = FileOperationsNode::new();
        let schema = node.parameter_schema();

        assert_eq!(schema["type"], "object");
        assert!(schema["properties"]["operation"].is_object());
        assert!(schema["properties"]["file_path"].is_object());
    }

    #[tokio::test]
    async fn test_file_operations_read_file() {
        let node = FileOperationsNode::new();
        let context = NodeContext::new(Uuid::new_v4().to_string(), "test-workflow".to_string());

        let params = json!({
            "operation": "read_file",
            "file_path": "/tmp/test.txt"
        });

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
    }

    #[tokio::test]
    async fn test_file_operations_write_file() {
        let node = FileOperationsNode::new();
        let context = NodeContext::new(Uuid::new_v4().to_string(), "test-workflow".to_string());

        let params = json!({
            "operation": "write_file",
            "file_path": "/tmp/test.txt",
            "content": "Hello, World!"
        });

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
    }

    #[test]
    fn test_file_operations_validation() {
        let node = FileOperationsNode::new();

        // Valid read_file
        let valid_params = json!({
            "operation": "read_file",
            "file_path": "/tmp/test.txt"
        });
        assert!(node.validate_parameters(&valid_params).is_ok());

        // Missing file_path for read_file
        let invalid_params = json!({
            "operation": "read_file"
        });
        assert!(node.validate_parameters(&invalid_params).is_err());

        // Missing content for write_file
        let invalid_params = json!({
            "operation": "write_file",
            "file_path": "/tmp/test.txt"
        });
        assert!(node.validate_parameters(&invalid_params).is_err());

        // Missing paths for copy_file
        let invalid_params = json!({
            "operation": "copy_file",
            "source_path": "/tmp/source.txt"
        });
        assert!(node.validate_parameters(&invalid_params).is_err());

        // Invalid operation
        let invalid_params = json!({
            "operation": "invalid_operation"
        });
        assert!(node.validate_parameters(&invalid_params).is_err());
    }
}
