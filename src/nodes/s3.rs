use crate::models::{Node, NodeCategory, NodeContext, NodeOutput, NodeSubcategory, NodeType};
use async_trait::async_trait;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct S3Params {
    /// Credentials name to use for AWS S3 connection
    pub credentials_name: String,
    /// S3 operation to perform
    pub operation: String,
    /// S3 bucket name
    pub bucket: String,
    /// Object key (path within bucket)
    pub key: String,
    /// File content for upload (base64 encoded or JSON)
    pub content: Option<serde_json::Value>,
    /// Content type for upload operations
    pub content_type: Option<String>,
    /// Local file path for download operations
    pub local_path: Option<String>,
    /// Prefix for list operations
    pub prefix: Option<String>,
    /// Maximum number of objects to list
    pub max_keys: Option<i32>,
}

/// S3 node - performs AWS S3 operations
pub struct S3Node;

impl S3Node {
    pub fn new() -> Self {
        Self
    }
}

impl Default for S3Node {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeType for S3Node {
    fn type_name(&self) -> &str {
        "s3"
    }

    fn category(&self) -> NodeCategory {
        NodeCategory::Action
    }

    fn subcategory(&self) -> NodeSubcategory {
        NodeSubcategory::Storage
    }

    fn parameter_schema(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "credentials_name": {
                    "type": "string",
                    "description": "Name of the AWS credentials to use",
                    "minLength": 1
                },
                "operation": {
                    "type": "string",
                    "description": "S3 operation to perform",
                    "enum": ["upload", "download", "delete", "list", "exists", "get_metadata", "copy", "move"]
                },
                "bucket": {
                    "type": "string",
                    "description": "S3 bucket name",
                    "minLength": 1
                },
                "key": {
                    "type": "string",
                    "description": "Object key (path within bucket)",
                    "minLength": 1
                },
                "content": {
                    "description": "File content for upload operations (base64 encoded string or JSON)"
                },
                "content_type": {
                    "type": "string",
                    "description": "Content type for upload operations (e.g., 'application/json', 'text/plain')",
                    "default": "application/octet-stream"
                },
                "local_path": {
                    "type": "string",
                    "description": "Local file path for download operations"
                },
                "prefix": {
                    "type": "string",
                    "description": "Prefix filter for list operations"
                },
                "max_keys": {
                    "type": "integer",
                    "description": "Maximum number of objects to return in list operations",
                    "minimum": 1,
                    "maximum": 1000,
                    "default": 1000
                }
            },
            "required": ["credentials_name", "operation", "bucket", "key"],
            "additionalProperties": false
        })
    }
}

#[async_trait]
impl Node for S3Node {
    async fn execute(
        &self,
        context: &NodeContext,
        parameters: &serde_json::Value,
    ) -> anyhow::Result<NodeOutput> {
        let params: S3Params = serde_json::from_value(parameters.clone())?;

        // TODO: In a real implementation:
        // 1. Look up credentials from the credentials table using params.credentials_name
        // 2. Decrypt the credentials data (AWS access key, secret key, region)
        // 3. Create an AWS S3 client using the decrypted credentials
        // 4. Execute the S3 operation based on params.operation:
        //    - upload: Put object to S3
        //    - download: Get object from S3
        //    - delete: Delete object from S3
        //    - list: List objects in bucket
        //    - exists: Check if object exists
        //    - get_metadata: Get object metadata (size, content-type, etc.)
        //    - copy: Copy object within S3
        //    - move: Move object within S3 (copy + delete)
        // 5. Return the results

        // For now, we'll return a placeholder response
        let result = serde_json::json!({
            "message": "S3 operation executed (placeholder implementation)",
            "credentials_name": params.credentials_name,
            "operation": params.operation,
            "bucket": params.bucket,
            "key": params.key,
            "content_type": params.content_type,
            "context_execution_id": context.execution_id,
            "success": true
        });

        Ok(NodeOutput::success(result))
    }

    fn validate_parameters(&self, parameters: &serde_json::Value) -> anyhow::Result<()> {
        let params: S3Params = serde_json::from_value(parameters.clone())
            .map_err(|e| anyhow::anyhow!("Invalid parameters: {}", e))?;

        if params.credentials_name.trim().is_empty() {
            anyhow::bail!("credentials_name cannot be empty");
        }

        if params.bucket.trim().is_empty() {
            anyhow::bail!("bucket cannot be empty");
        }

        if params.key.trim().is_empty() {
            anyhow::bail!("key cannot be empty");
        }

        // Validate operation
        let valid_operations = [
            "upload",
            "download",
            "delete",
            "list",
            "exists",
            "get_metadata",
            "copy",
            "move",
        ];

        if !valid_operations.contains(&params.operation.as_str()) {
            anyhow::bail!(
                "Invalid operation: {}. Must be one of: {}",
                params.operation,
                valid_operations.join(", ")
            );
        }

        // Validate that upload operations have content
        if params.operation == "upload" && params.content.is_none() {
            anyhow::bail!("Upload operation requires 'content' parameter");
        }

        // Validate that download operations have local_path
        if params.operation == "download" && params.local_path.is_none() {
            anyhow::bail!("Download operation requires 'local_path' parameter");
        }

        // Validate bucket name (basic S3 bucket naming rules)
        if params.bucket.len() < 3 || params.bucket.len() > 63 {
            anyhow::bail!("Bucket name must be between 3 and 63 characters");
        }

        if !params
            .bucket
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-' || c == '.')
        {
            anyhow::bail!(
                "Bucket name can only contain lowercase letters, numbers, hyphens, and dots"
            );
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_s3_upload_operation() {
        let node = S3Node::new();
        let context = NodeContext::new("exec-123".to_string(), "node-1".to_string());

        let params = serde_json::json!({
            "credentials_name": "my_aws",
            "operation": "upload",
            "bucket": "my-bucket",
            "key": "path/to/file.json",
            "content": {"data": "test"},
            "content_type": "application/json"
        });

        let result = node.execute(&context, &params).await.unwrap();
        assert!(result.success);
        assert_eq!(result.data["operation"], "upload");
        assert_eq!(result.data["bucket"], "my-bucket");
        assert_eq!(result.data["key"], "path/to/file.json");
    }

    #[tokio::test]
    async fn test_s3_download_operation() {
        let node = S3Node::new();
        let context = NodeContext::new("exec-123".to_string(), "node-1".to_string());

        let params = serde_json::json!({
            "credentials_name": "my_aws",
            "operation": "download",
            "bucket": "my-bucket",
            "key": "path/to/file.json",
            "local_path": "/tmp/downloaded.json"
        });

        let result = node.execute(&context, &params).await.unwrap();
        assert!(result.success);
        assert_eq!(result.data["operation"], "download");
    }

    #[tokio::test]
    async fn test_s3_list_operation() {
        let node = S3Node::new();
        let context = NodeContext::new("exec-123".to_string(), "node-1".to_string());

        let params = serde_json::json!({
            "credentials_name": "my_aws",
            "operation": "list",
            "bucket": "my-bucket",
            "key": "",
            "prefix": "path/to/",
            "max_keys": 100
        });

        let result = node.execute(&context, &params).await.unwrap();
        assert!(result.success);
    }

    #[test]
    fn test_s3_validation() {
        let node = S3Node::new();

        // Valid upload operation
        let valid_upload = serde_json::json!({
            "credentials_name": "my_aws",
            "operation": "upload",
            "bucket": "my-bucket",
            "key": "test.json",
            "content": "test data"
        });
        assert!(node.validate_parameters(&valid_upload).is_ok());

        // Invalid: upload without content
        let invalid_upload = serde_json::json!({
            "credentials_name": "my_aws",
            "operation": "upload",
            "bucket": "my-bucket",
            "key": "test.json"
        });
        assert!(node.validate_parameters(&invalid_upload).is_err());

        // Invalid: download without local_path
        let invalid_download = serde_json::json!({
            "credentials_name": "my_aws",
            "operation": "download",
            "bucket": "my-bucket",
            "key": "test.json"
        });
        assert!(node.validate_parameters(&invalid_download).is_err());

        // Invalid: empty bucket
        let invalid_bucket = serde_json::json!({
            "credentials_name": "my_aws",
            "operation": "list",
            "bucket": "",
            "key": ""
        });
        assert!(node.validate_parameters(&invalid_bucket).is_err());

        // Invalid: bucket name too short
        let invalid_bucket_short = serde_json::json!({
            "credentials_name": "my_aws",
            "operation": "list",
            "bucket": "ab",
            "key": ""
        });
        assert!(node.validate_parameters(&invalid_bucket_short).is_err());

        // Invalid: bucket name with uppercase
        let invalid_bucket_case = serde_json::json!({
            "credentials_name": "my_aws",
            "operation": "list",
            "bucket": "MyBucket",
            "key": ""
        });
        assert!(node.validate_parameters(&invalid_bucket_case).is_err());

        // Invalid: unknown operation
        let invalid_op = serde_json::json!({
            "credentials_name": "my_aws",
            "operation": "invalid_op",
            "bucket": "my-bucket",
            "key": "test.json"
        });
        assert!(node.validate_parameters(&invalid_op).is_err());
    }

    #[test]
    fn test_s3_node_type() {
        let node = S3Node::new();
        assert_eq!(node.type_name(), "s3");
        assert_eq!(node.category(), NodeCategory::Action);
        assert_eq!(node.subcategory(), NodeSubcategory::Storage);
    }

    #[test]
    fn test_s3_parameter_schema() {
        let node = S3Node::new();
        let schema = node.parameter_schema();

        assert_eq!(schema["type"], "object");
        assert!(schema["properties"]["credentials_name"].is_object());
        assert!(schema["properties"]["operation"].is_object());
        assert!(schema["properties"]["bucket"].is_object());
        assert!(schema["properties"]["key"].is_object());
        assert!(schema["properties"]["operation"]["enum"].is_array());
        assert_eq!(schema["required"].as_array().unwrap().len(), 4);
    }
}
