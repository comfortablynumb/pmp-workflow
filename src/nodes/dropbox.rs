use crate::models::{Node, NodeCategory, NodeContext, NodeOutput, NodeSubcategory, NodeType};
use async_trait::async_trait;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DropboxParams {
    /// Credentials name to use for Dropbox API
    pub credentials_name: String,
    /// Dropbox operation to perform
    pub operation: String,
    /// File path in Dropbox
    pub path: Option<String>,
    /// Source path for move/copy
    pub from_path: Option<String>,
    /// Destination path for move/copy
    pub to_path: Option<String>,
    /// File content (base64 encoded)
    pub content: Option<String>,
    /// Local file path for uploads/downloads
    pub local_path: Option<String>,
    /// Search query
    pub query: Option<String>,
    /// Maximum results
    pub max_results: Option<u32>,
    /// File mode: "add", "overwrite", "update"
    pub mode: Option<String>,
    /// Revision ID for updates
    pub rev: Option<String>,
    /// Share link settings
    pub link_password: Option<String>,
    /// Link expiration date
    pub expires: Option<String>,
    /// Recursive operation
    pub recursive: Option<bool>,
    /// Include deleted files
    pub include_deleted: Option<bool>,
    /// Additional parameters
    #[serde(default)]
    pub additional_params: serde_json::Value,
}

/// Dropbox node - performs Dropbox API operations
pub struct DropboxNode;

impl DropboxNode {
    pub fn new() -> Self {
        Self
    }
}

impl Default for DropboxNode {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeType for DropboxNode {
    fn type_name(&self) -> &str {
        "dropbox"
    }

    fn category(&self) -> NodeCategory {
        NodeCategory::Action
    }

    fn subcategory(&self) -> NodeSubcategory {
        NodeSubcategory::Storage
    }

    fn required_credential_type(&self) -> Option<&str> {
        Some("dropbox_oauth")
    }

    fn parameter_schema(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "credentials_name": {
                    "type": "string",
                    "description": "Name of the Dropbox credentials to use",
                    "minLength": 1
                },
                "operation": {
                    "type": "string",
                    "description": "Dropbox operation to perform",
                    "enum": [
                        "upload_file",
                        "download_file",
                        "delete_file",
                        "delete_folder",
                        "list_folder",
                        "create_folder",
                        "move_file",
                        "copy_file",
                        "get_metadata",
                        "search_files",
                        "create_shared_link",
                        "list_shared_links",
                        "revoke_shared_link",
                        "get_account_info",
                        "get_space_usage"
                    ]
                },
                "path": {
                    "type": "string",
                    "description": "File or folder path in Dropbox (starts with /)"
                },
                "from_path": {
                    "type": "string",
                    "description": "Source path for move/copy operations"
                },
                "to_path": {
                    "type": "string",
                    "description": "Destination path for move/copy operations"
                },
                "content": {
                    "type": "string",
                    "description": "File content (base64 encoded)"
                },
                "local_path": {
                    "type": "string",
                    "description": "Local file path for uploads/downloads"
                },
                "query": {
                    "type": "string",
                    "description": "Search query"
                },
                "max_results": {
                    "type": "integer",
                    "description": "Maximum number of results",
                    "minimum": 1,
                    "maximum": 1000,
                    "default": 100
                },
                "mode": {
                    "type": "string",
                    "description": "Upload mode",
                    "enum": ["add", "overwrite", "update"]
                },
                "rev": {
                    "type": "string",
                    "description": "Revision ID for update mode"
                },
                "link_password": {
                    "type": "string",
                    "description": "Password for shared link"
                },
                "expires": {
                    "type": "string",
                    "description": "Link expiration date (ISO 8601)"
                },
                "recursive": {
                    "type": "boolean",
                    "description": "Recursive operation",
                    "default": false
                },
                "include_deleted": {
                    "type": "boolean",
                    "description": "Include deleted files",
                    "default": false
                },
                "additional_params": {
                    "type": "object",
                    "description": "Additional parameters to pass to the API"
                }
            },
            "required": ["credentials_name", "operation"],
            "additionalProperties": false
        })
    }
}

#[async_trait]
impl Node for DropboxNode {
    async fn execute(
        &self,
        context: &NodeContext,
        parameters: &serde_json::Value,
    ) -> anyhow::Result<NodeOutput> {
        let params: DropboxParams = serde_json::from_value(parameters.clone())?;

        // TODO: In a real implementation:
        // 1. Look up credentials from credentials table
        // 2. Decrypt OAuth2 token
        // 3. Create Dropbox API client
        // 4. Execute operations via Dropbox API v2:
        //
        //    Content API: https://content.dropboxapi.com/2/
        //    RPC API: https://api.dropboxapi.com/2/
        //    Authorization: Bearer {access_token}
        //
        //    upload_file: POST /files/upload (Content API)
        //    download_file: POST /files/download (Content API)
        //    delete_file: POST /files/delete_v2
        //    list_folder: POST /files/list_folder
        //    create_folder: POST /files/create_folder_v2
        //    move_file: POST /files/move_v2
        //    copy_file: POST /files/copy_v2
        //    get_metadata: POST /files/get_metadata
        //    search_files: POST /files/search_v2
        //    create_shared_link: POST /sharing/create_shared_link_with_settings
        //    get_account_info: POST /users/get_current_account
        //
        // 5. Handle OAuth token refresh
        // 6. Handle pagination with cursors
        // 7. Return results

        let result = serde_json::json!({
            "message": "Dropbox operation executed (placeholder)",
            "credentials_name": &params.credentials_name,
            "operation": &params.operation,
            "path": params.path,
            "context_execution_id": &context.execution_id,
            "success": true
        });

        Ok(NodeOutput::success(result))
    }

    fn validate_parameters(&self, parameters: &serde_json::Value) -> anyhow::Result<()> {
        let params: DropboxParams = serde_json::from_value(parameters.clone())
            .map_err(|e| anyhow::anyhow!("Invalid parameters: {}", e))?;

        if params.credentials_name.trim().is_empty() {
            anyhow::bail!("credentials_name cannot be empty");
        }

        let valid_operations = [
            "upload_file",
            "download_file",
            "delete_file",
            "delete_folder",
            "list_folder",
            "create_folder",
            "move_file",
            "copy_file",
            "get_metadata",
            "search_files",
            "create_shared_link",
            "list_shared_links",
            "revoke_shared_link",
            "get_account_info",
            "get_space_usage",
        ];

        if !valid_operations.contains(&params.operation.as_str()) {
            anyhow::bail!(
                "Invalid operation: {}. Must be one of: {}",
                params.operation,
                valid_operations.join(", ")
            );
        }

        // Validate path-based operations
        let path_ops = [
            "upload_file",
            "download_file",
            "delete_file",
            "delete_folder",
            "list_folder",
            "create_folder",
            "get_metadata",
            "create_shared_link",
        ];

        if path_ops.contains(&params.operation.as_str()) && params.path.is_none() {
            anyhow::bail!("{} operation requires 'path' parameter", params.operation);
        }

        // Validate move/copy operations
        if ["move_file", "copy_file"].contains(&params.operation.as_str()) {
            if params.from_path.is_none() {
                anyhow::bail!("{} operation requires 'from_path' parameter", params.operation);
            }
            if params.to_path.is_none() {
                anyhow::bail!("{} operation requires 'to_path' parameter", params.operation);
            }
        }

        // Validate upload requires content or local_path
        if params.operation == "upload_file"
            && params.content.is_none()
            && params.local_path.is_none()
        {
            anyhow::bail!("upload_file operation requires 'content' or 'local_path' parameter");
        }

        // Validate search requires query
        if params.operation == "search_files" && params.query.is_none() {
            anyhow::bail!("search_files operation requires 'query' parameter");
        }

        // Validate max_results range
        if let Some(max_results) = params.max_results {
            if !(1..=1000).contains(&max_results) {
                anyhow::bail!("max_results must be between 1 and 1000");
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_dropbox_upload() {
        let node = DropboxNode::new();
        let context = NodeContext::new("exec-123".to_string(), "node-1".to_string());

        let params = serde_json::json!({
            "credentials_name": "my_dropbox",
            "operation": "upload_file",
            "path": "/documents/report.pdf",
            "content": "base64_encoded_content",
            "mode": "add"
        });

        let result = node.execute(&context, &params).await.unwrap();
        assert!(result.success);
        assert_eq!(result.data["operation"], "upload_file");
    }

    #[tokio::test]
    async fn test_dropbox_search() {
        let node = DropboxNode::new();
        let context = NodeContext::new("exec-123".to_string(), "node-1".to_string());

        let params = serde_json::json!({
            "credentials_name": "my_dropbox",
            "operation": "search_files",
            "query": "invoice",
            "max_results": 50
        });

        let result = node.execute(&context, &params).await.unwrap();
        assert!(result.success);
        assert_eq!(result.data["operation"], "search_files");
    }

    #[test]
    fn test_dropbox_validation() {
        let node = DropboxNode::new();

        // Valid upload
        let valid = serde_json::json!({
            "credentials_name": "my_dropbox",
            "operation": "upload_file",
            "path": "/file.txt",
            "content": "data"
        });
        assert!(node.validate_parameters(&valid).is_ok());

        // Invalid: upload without path
        let invalid = serde_json::json!({
            "credentials_name": "my_dropbox",
            "operation": "upload_file",
            "content": "data"
        });
        assert!(node.validate_parameters(&invalid).is_err());

        // Invalid: move without from_path
        let invalid2 = serde_json::json!({
            "credentials_name": "my_dropbox",
            "operation": "move_file",
            "to_path": "/new.txt"
        });
        assert!(node.validate_parameters(&invalid2).is_err());
    }

    #[test]
    fn test_dropbox_node_type() {
        let node = DropboxNode::new();
        assert_eq!(node.type_name(), "dropbox");
        assert_eq!(node.category(), NodeCategory::Action);
        assert_eq!(node.subcategory(), NodeSubcategory::Storage);
        assert_eq!(node.required_credential_type(), Some("dropbox_oauth"));
    }
}
