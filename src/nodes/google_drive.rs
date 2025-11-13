use crate::models::{Node, NodeCategory, NodeContext, NodeOutput, NodeSubcategory, NodeType};
use async_trait::async_trait;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GoogleDriveParams {
    /// Credentials name to use for Google Drive API
    pub credentials_name: String,
    /// Google Drive operation to perform
    pub operation: String,
    /// File ID
    pub file_id: Option<String>,
    /// Folder ID
    pub folder_id: Option<String>,
    /// File name
    pub name: Option<String>,
    /// File content (base64 encoded)
    pub content: Option<String>,
    /// Local file path for uploads/downloads
    pub file_path: Option<String>,
    /// MIME type
    pub mime_type: Option<String>,
    /// Parent folder IDs
    pub parents: Option<Vec<String>>,
    /// File description
    pub description: Option<String>,
    /// Search query
    pub query: Option<String>,
    /// Share with email addresses
    pub share_with: Option<Vec<String>>,
    /// Permission role: "reader", "writer", "commenter", "owner"
    pub role: Option<String>,
    /// Permission type: "user", "group", "domain", "anyone"
    pub permission_type: Option<String>,
    /// Results per page (1-1000)
    pub page_size: Option<u32>,
    /// Page token for pagination
    pub page_token: Option<String>,
    /// Include trashed files
    pub include_trashed: Option<bool>,
    /// Additional parameters
    #[serde(default)]
    pub additional_params: serde_json::Value,
}

/// Google Drive node - performs Google Drive API operations
pub struct GoogleDriveNode;

impl GoogleDriveNode {
    pub fn new() -> Self {
        Self
    }
}

impl Default for GoogleDriveNode {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeType for GoogleDriveNode {
    fn type_name(&self) -> &str {
        "google_drive"
    }

    fn category(&self) -> NodeCategory {
        NodeCategory::Action
    }

    fn subcategory(&self) -> NodeSubcategory {
        NodeSubcategory::Storage
    }

    fn required_credential_type(&self) -> Option<&str> {
        Some("google_drive_oauth")
    }

    fn parameter_schema(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "credentials_name": {
                    "type": "string",
                    "description": "Name of the Google Drive credentials to use",
                    "minLength": 1
                },
                "operation": {
                    "type": "string",
                    "description": "Google Drive operation to perform",
                    "enum": [
                        "upload_file",
                        "download_file",
                        "delete_file",
                        "get_file",
                        "list_files",
                        "search_files",
                        "create_folder",
                        "move_file",
                        "copy_file",
                        "rename_file",
                        "share_file",
                        "update_permissions",
                        "list_permissions",
                        "delete_permission",
                        "export_file",
                        "get_file_metadata",
                        "update_file_metadata"
                    ]
                },
                "file_id": {
                    "type": "string",
                    "description": "Google Drive file ID"
                },
                "folder_id": {
                    "type": "string",
                    "description": "Google Drive folder ID"
                },
                "name": {
                    "type": "string",
                    "description": "File or folder name"
                },
                "content": {
                    "type": "string",
                    "description": "File content (base64 encoded)"
                },
                "file_path": {
                    "type": "string",
                    "description": "Local file path for uploads/downloads"
                },
                "mime_type": {
                    "type": "string",
                    "description": "MIME type of the file"
                },
                "parents": {
                    "type": "array",
                    "description": "Parent folder IDs",
                    "items": {
                        "type": "string"
                    }
                },
                "description": {
                    "type": "string",
                    "description": "File description"
                },
                "query": {
                    "type": "string",
                    "description": "Search query (Google Drive query syntax)"
                },
                "share_with": {
                    "type": "array",
                    "description": "Email addresses to share with",
                    "items": {
                        "type": "string"
                    }
                },
                "role": {
                    "type": "string",
                    "description": "Permission role",
                    "enum": ["reader", "writer", "commenter", "owner"]
                },
                "permission_type": {
                    "type": "string",
                    "description": "Permission type",
                    "enum": ["user", "group", "domain", "anyone"]
                },
                "page_size": {
                    "type": "integer",
                    "description": "Results per page (1-1000)",
                    "minimum": 1,
                    "maximum": 1000,
                    "default": 100
                },
                "page_token": {
                    "type": "string",
                    "description": "Page token for pagination"
                },
                "include_trashed": {
                    "type": "boolean",
                    "description": "Include trashed files in results",
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
impl Node for GoogleDriveNode {
    async fn execute(
        &self,
        context: &NodeContext,
        parameters: &serde_json::Value,
    ) -> anyhow::Result<NodeOutput> {
        let params: GoogleDriveParams = serde_json::from_value(parameters.clone())?;

        // TODO: In a real implementation:
        // 1. Look up credentials from the credentials table
        // 2. Decrypt OAuth2 credentials (access_token, refresh_token)
        // 3. Create Google Drive API client
        // 4. Execute operations via Drive API v3:
        //
        //    API Base: https://www.googleapis.com/drive/v3
        //    Authorization: Bearer {access_token}
        //
        //    upload_file: POST /upload/drive/v3/files (multipart)
        //    download_file: GET /drive/v3/files/{fileId}?alt=media
        //    delete_file: DELETE /drive/v3/files/{fileId}
        //    get_file: GET /drive/v3/files/{fileId}
        //    list_files: GET /drive/v3/files
        //    search_files: GET /drive/v3/files?q={query}
        //    create_folder: POST /drive/v3/files (mimeType: application/vnd.google-apps.folder)
        //    move_file: PATCH /drive/v3/files/{fileId} (update parents)
        //    copy_file: POST /drive/v3/files/{fileId}/copy
        //    share_file: POST /drive/v3/files/{fileId}/permissions
        //    list_permissions: GET /drive/v3/files/{fileId}/permissions
        //    export_file: GET /drive/v3/files/{fileId}/export
        //
        // 5. Handle OAuth token refresh
        // 6. Handle pagination with pageToken
        // 7. Return results

        let result = serde_json::json!({
            "message": "Google Drive operation executed (placeholder)",
            "credentials_name": &params.credentials_name,
            "operation": &params.operation,
            "file_id": params.file_id,
            "context_execution_id": &context.execution_id,
            "success": true
        });

        Ok(NodeOutput::success(result))
    }

    fn validate_parameters(&self, parameters: &serde_json::Value) -> anyhow::Result<()> {
        let params: GoogleDriveParams = serde_json::from_value(parameters.clone())
            .map_err(|e| anyhow::anyhow!("Invalid parameters: {}", e))?;

        if params.credentials_name.trim().is_empty() {
            anyhow::bail!("credentials_name cannot be empty");
        }

        let valid_operations = [
            "upload_file",
            "download_file",
            "delete_file",
            "get_file",
            "list_files",
            "search_files",
            "create_folder",
            "move_file",
            "copy_file",
            "rename_file",
            "share_file",
            "update_permissions",
            "list_permissions",
            "delete_permission",
            "export_file",
            "get_file_metadata",
            "update_file_metadata",
        ];

        if !valid_operations.contains(&params.operation.as_str()) {
            anyhow::bail!(
                "Invalid operation: {}. Must be one of: {}",
                params.operation,
                valid_operations.join(", ")
            );
        }

        // Validate file operations require file_id
        let file_ops = [
            "download_file",
            "delete_file",
            "get_file",
            "move_file",
            "copy_file",
            "rename_file",
            "share_file",
            "update_permissions",
            "list_permissions",
            "delete_permission",
            "export_file",
            "get_file_metadata",
            "update_file_metadata",
        ];

        if file_ops.contains(&params.operation.as_str()) && params.file_id.is_none() {
            anyhow::bail!("{} operation requires 'file_id' parameter", params.operation);
        }

        // Validate upload requires name and content or file_path
        if params.operation == "upload_file" {
            if params.name.is_none() {
                anyhow::bail!("upload_file operation requires 'name' parameter");
            }
            if params.content.is_none() && params.file_path.is_none() {
                anyhow::bail!("upload_file operation requires 'content' or 'file_path' parameter");
            }
        }

        // Validate create_folder requires name
        if params.operation == "create_folder" && params.name.is_none() {
            anyhow::bail!("create_folder operation requires 'name' parameter");
        }

        // Validate search_files requires query
        if params.operation == "search_files" && params.query.is_none() {
            anyhow::bail!("search_files operation requires 'query' parameter");
        }

        // Validate share_file requires share_with or permission_type
        if params.operation == "share_file"
            && params.share_with.is_none()
            && params.permission_type.is_none()
        {
            anyhow::bail!(
                "share_file operation requires 'share_with' or 'permission_type' parameter"
            );
        }

        // Validate page_size range
        if let Some(page_size) = params.page_size
            && !(1..=1000).contains(&page_size)
        {
            anyhow::bail!("page_size must be between 1 and 1000");
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_google_drive_upload() {
        let node = GoogleDriveNode::new();
        let context = NodeContext::new("exec-123".to_string(), "node-1".to_string());

        let params = serde_json::json!({
            "credentials_name": "my_drive",
            "operation": "upload_file",
            "name": "document.pdf",
            "content": "base64_encoded_content",
            "mime_type": "application/pdf",
            "parents": ["folder_id_123"]
        });

        let result = node.execute(&context, &params).await.unwrap();
        assert!(result.success);
        assert_eq!(result.data["operation"], "upload_file");
    }

    #[tokio::test]
    async fn test_google_drive_search() {
        let node = GoogleDriveNode::new();
        let context = NodeContext::new("exec-123".to_string(), "node-1".to_string());

        let params = serde_json::json!({
            "credentials_name": "my_drive",
            "operation": "search_files",
            "query": "name contains 'report' and mimeType='application/pdf'",
            "page_size": 50
        });

        let result = node.execute(&context, &params).await.unwrap();
        assert!(result.success);
        assert_eq!(result.data["operation"], "search_files");
    }

    #[test]
    fn test_google_drive_validation() {
        let node = GoogleDriveNode::new();

        // Valid upload
        let valid_upload = serde_json::json!({
            "credentials_name": "my_drive",
            "operation": "upload_file",
            "name": "file.txt",
            "content": "data"
        });
        assert!(node.validate_parameters(&valid_upload).is_ok());

        // Invalid: upload without name
        let invalid = serde_json::json!({
            "credentials_name": "my_drive",
            "operation": "upload_file",
            "content": "data"
        });
        assert!(node.validate_parameters(&invalid).is_err());

        // Invalid: download without file_id
        let invalid2 = serde_json::json!({
            "credentials_name": "my_drive",
            "operation": "download_file"
        });
        assert!(node.validate_parameters(&invalid2).is_err());
    }

    #[test]
    fn test_google_drive_node_type() {
        let node = GoogleDriveNode::new();
        assert_eq!(node.type_name(), "google_drive");
        assert_eq!(node.category(), NodeCategory::Action);
        assert_eq!(node.subcategory(), NodeSubcategory::Storage);
        assert_eq!(node.required_credential_type(), Some("google_drive_oauth"));
    }
}
