use crate::models::{Node, NodeCategory, NodeContext, NodeOutput, NodeSubcategory, NodeType};
use async_trait::async_trait;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct FtpParams {
    /// Credentials name to use for FTP/SFTP
    pub credentials_name: String,
    /// FTP operation to perform
    pub operation: String,
    /// Remote file path
    pub remote_path: Option<String>,
    /// Local file path
    pub local_path: Option<String>,
    /// File content (base64 encoded)
    pub content: Option<String>,
    /// Directory path
    pub directory: Option<String>,
    /// Rename from path
    pub from_path: Option<String>,
    /// Rename to path
    pub to_path: Option<String>,
    /// Recursive operation
    pub recursive: Option<bool>,
    /// Transfer mode: "binary" or "ascii"
    pub mode: Option<String>,
    /// Create parent directories
    pub create_parents: Option<bool>,
    /// Additional parameters
    #[serde(default)]
    pub additional_params: serde_json::Value,
}

/// FTP/SFTP node - performs FTP and SFTP file transfer operations
pub struct FtpNode;

impl FtpNode {
    pub fn new() -> Self {
        Self
    }
}

impl Default for FtpNode {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeType for FtpNode {
    fn type_name(&self) -> &str {
        "ftp"
    }

    fn category(&self) -> NodeCategory {
        NodeCategory::Action
    }

    fn subcategory(&self) -> NodeSubcategory {
        NodeSubcategory::Storage
    }

    fn required_credential_type(&self) -> Option<&str> {
        Some("ftp_credentials")
    }

    fn parameter_schema(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "credentials_name": {
                    "type": "string",
                    "description": "Name of the FTP/SFTP credentials to use",
                    "minLength": 1
                },
                "operation": {
                    "type": "string",
                    "description": "FTP/SFTP operation to perform",
                    "enum": [
                        "upload_file",
                        "download_file",
                        "delete_file",
                        "list_directory",
                        "create_directory",
                        "delete_directory",
                        "rename_file",
                        "move_file",
                        "get_file_info",
                        "file_exists",
                        "directory_exists"
                    ]
                },
                "remote_path": {
                    "type": "string",
                    "description": "Remote file path on FTP/SFTP server"
                },
                "local_path": {
                    "type": "string",
                    "description": "Local file path"
                },
                "content": {
                    "type": "string",
                    "description": "File content (base64 encoded)"
                },
                "directory": {
                    "type": "string",
                    "description": "Directory path"
                },
                "from_path": {
                    "type": "string",
                    "description": "Source path for rename/move operations"
                },
                "to_path": {
                    "type": "string",
                    "description": "Destination path for rename/move operations"
                },
                "recursive": {
                    "type": "boolean",
                    "description": "Recursive operation for directories",
                    "default": false
                },
                "mode": {
                    "type": "string",
                    "description": "Transfer mode",
                    "enum": ["binary", "ascii"],
                    "default": "binary"
                },
                "create_parents": {
                    "type": "boolean",
                    "description": "Create parent directories if they don't exist",
                    "default": false
                },
                "additional_params": {
                    "type": "object",
                    "description": "Additional parameters to pass to the client"
                }
            },
            "required": ["credentials_name", "operation"],
            "additionalProperties": false
        })
    }
}

#[async_trait]
impl Node for FtpNode {
    async fn execute(
        &self,
        context: &NodeContext,
        parameters: &serde_json::Value,
    ) -> anyhow::Result<NodeOutput> {
        let params: FtpParams = serde_json::from_value(parameters.clone())?;

        // TODO: In a real implementation:
        // 1. Look up credentials from credentials table
        // 2. Decrypt credentials (host, port, username, password, protocol)
        // 3. Determine protocol (FTP, FTPS, SFTP) from credentials
        // 4. Create appropriate client:
        //
        //    For FTP/FTPS: Use suppaftp crate or ftp crate
        //    - Connect to host:port
        //    - Login with username/password
        //    - Set transfer mode (binary/ascii)
        //    - Execute operations:
        //      upload_file: PUT file
        //      download_file: GET file
        //      delete_file: DELE file
        //      list_directory: LIST or NLST directory
        //      create_directory: MKD directory
        //      delete_directory: RMD directory
        //      rename_file: RNFR/RNTO
        //
        //    For SFTP: Use ssh2 crate or async-ssh2-tokio
        //    - Connect via SSH
        //    - Authenticate with username/password or key
        //    - Open SFTP channel
        //    - Execute operations:
        //      upload_file: sftp.write_file()
        //      download_file: sftp.read_file()
        //      delete_file: sftp.unlink()
        //      list_directory: sftp.readdir()
        //      create_directory: sftp.mkdir()
        //      rename_file: sftp.rename()
        //
        // 5. Handle errors and connection cleanup
        // 6. Return results

        let result = serde_json::json!({
            "message": "FTP/SFTP operation executed (placeholder)",
            "credentials_name": &params.credentials_name,
            "operation": &params.operation,
            "remote_path": params.remote_path,
            "context_execution_id": &context.execution_id,
            "success": true
        });

        Ok(NodeOutput::success(result))
    }

    fn validate_parameters(&self, parameters: &serde_json::Value) -> anyhow::Result<()> {
        let params: FtpParams = serde_json::from_value(parameters.clone())
            .map_err(|e| anyhow::anyhow!("Invalid parameters: {}", e))?;

        if params.credentials_name.trim().is_empty() {
            anyhow::bail!("credentials_name cannot be empty");
        }

        let valid_operations = [
            "upload_file",
            "download_file",
            "delete_file",
            "list_directory",
            "create_directory",
            "delete_directory",
            "rename_file",
            "move_file",
            "get_file_info",
            "file_exists",
            "directory_exists",
        ];

        if !valid_operations.contains(&params.operation.as_str()) {
            anyhow::bail!(
                "Invalid operation: {}. Must be one of: {}",
                params.operation,
                valid_operations.join(", ")
            );
        }

        // Validate file operations require remote_path
        let remote_path_ops = [
            "download_file",
            "delete_file",
            "get_file_info",
            "file_exists",
        ];

        if remote_path_ops.contains(&params.operation.as_str()) && params.remote_path.is_none() {
            anyhow::bail!("{} operation requires 'remote_path' parameter", params.operation);
        }

        // Validate upload requires remote_path and (content or local_path)
        if params.operation == "upload_file" {
            if params.remote_path.is_none() {
                anyhow::bail!("upload_file operation requires 'remote_path' parameter");
            }
            if params.content.is_none() && params.local_path.is_none() {
                anyhow::bail!(
                    "upload_file operation requires 'content' or 'local_path' parameter"
                );
            }
        }

        // Validate directory operations
        if ["list_directory", "create_directory", "delete_directory", "directory_exists"]
            .contains(&params.operation.as_str())
            && params.directory.is_none()
        {
            anyhow::bail!("{} operation requires 'directory' parameter", params.operation);
        }

        // Validate rename/move operations
        if ["rename_file", "move_file"].contains(&params.operation.as_str()) {
            if params.from_path.is_none() {
                anyhow::bail!("{} operation requires 'from_path' parameter", params.operation);
            }
            if params.to_path.is_none() {
                anyhow::bail!("{} operation requires 'to_path' parameter", params.operation);
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_ftp_upload() {
        let node = FtpNode::new();
        let context = NodeContext::new("exec-123".to_string(), "node-1".to_string());

        let params = serde_json::json!({
            "credentials_name": "my_ftp",
            "operation": "upload_file",
            "remote_path": "/uploads/document.pdf",
            "content": "base64_encoded_content",
            "mode": "binary"
        });

        let result = node.execute(&context, &params).await.unwrap();
        assert!(result.success);
        assert_eq!(result.data["operation"], "upload_file");
    }

    #[tokio::test]
    async fn test_ftp_list_directory() {
        let node = FtpNode::new();
        let context = NodeContext::new("exec-123".to_string(), "node-1".to_string());

        let params = serde_json::json!({
            "credentials_name": "my_ftp",
            "operation": "list_directory",
            "directory": "/files"
        });

        let result = node.execute(&context, &params).await.unwrap();
        assert!(result.success);
        assert_eq!(result.data["operation"], "list_directory");
    }

    #[test]
    fn test_ftp_validation() {
        let node = FtpNode::new();

        // Valid upload
        let valid = serde_json::json!({
            "credentials_name": "my_ftp",
            "operation": "upload_file",
            "remote_path": "/file.txt",
            "content": "data"
        });
        assert!(node.validate_parameters(&valid).is_ok());

        // Invalid: upload without remote_path
        let invalid = serde_json::json!({
            "credentials_name": "my_ftp",
            "operation": "upload_file",
            "content": "data"
        });
        assert!(node.validate_parameters(&invalid).is_err());

        // Invalid: rename without to_path
        let invalid2 = serde_json::json!({
            "credentials_name": "my_ftp",
            "operation": "rename_file",
            "from_path": "/old.txt"
        });
        assert!(node.validate_parameters(&invalid2).is_err());
    }

    #[test]
    fn test_ftp_node_type() {
        let node = FtpNode::new();
        assert_eq!(node.type_name(), "ftp");
        assert_eq!(node.category(), NodeCategory::Action);
        assert_eq!(node.subcategory(), NodeSubcategory::Storage);
        assert_eq!(node.required_credential_type(), Some("ftp_credentials"));
    }
}
