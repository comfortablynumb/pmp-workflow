use crate::models::{Node, NodeCategory, NodeContext, NodeOutput, NodeSubcategory, NodeType};
use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

/// AWS S3 advanced operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AwsS3Params {
    /// Operation to perform
    pub operation: String,
    /// Bucket name
    pub bucket: Option<String>,
    /// Object key/path
    pub key: Option<String>,
    /// Content to upload
    pub content: Option<String>,
    /// Content type (MIME type)
    pub content_type: Option<String>,
    /// Source bucket for copy/replication
    pub source_bucket: Option<String>,
    /// Source key for copy
    pub source_key: Option<String>,
    /// Destination bucket
    pub destination_bucket: Option<String>,
    /// Destination key
    pub destination_key: Option<String>,
    /// Storage class
    pub storage_class: Option<String>,
    /// Server-side encryption
    pub encryption: Option<String>,
    /// ACL (access control list)
    pub acl: Option<String>,
    /// Prefix for listing
    pub prefix: Option<String>,
    /// Max keys to return
    pub max_keys: Option<u32>,
    /// Versioning configuration
    pub versioning: Option<String>,
    /// Lifecycle rules
    pub lifecycle_rules: Option<Value>,
    /// Bucket policy
    pub policy: Option<Value>,
    /// CORS configuration
    pub cors: Option<Value>,
    /// Tags
    pub tags: Option<Value>,
    /// Expiration in days
    pub expiration_days: Option<u32>,
    /// Presigned URL expiration in seconds
    pub expires_in: Option<u32>,
    /// Enable multipart upload
    pub multipart: Option<bool>,
    /// Part size for multipart upload (MB)
    pub part_size: Option<u32>,
}

pub struct AwsS3Node;

impl AwsS3Node {
    pub fn new() -> Self {
        Self
    }
}

impl NodeType for AwsS3Node {
    fn type_name(&self) -> &str {
        "aws_s3"
    }

    fn category(&self) -> NodeCategory {
        NodeCategory::Action
    }

    fn subcategory(&self) -> NodeSubcategory {
        NodeSubcategory::Storage
    }

    fn parameter_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "operation": {
                    "type": "string",
                    "enum": [
                        "upload_object",
                        "download_object",
                        "delete_object",
                        "delete_objects",
                        "copy_object",
                        "list_objects",
                        "list_buckets",
                        "create_bucket",
                        "delete_bucket",
                        "get_object_metadata",
                        "set_object_acl",
                        "get_object_acl",
                        "generate_presigned_url",
                        "multipart_upload",
                        "abort_multipart_upload",
                        "set_bucket_versioning",
                        "get_bucket_versioning",
                        "set_lifecycle_policy",
                        "get_lifecycle_policy",
                        "set_bucket_policy",
                        "get_bucket_policy",
                        "set_cors",
                        "get_cors",
                        "replicate_object",
                        "restore_object"
                    ],
                    "description": "S3 operation to perform"
                },
                "bucket": {
                    "type": "string",
                    "description": "S3 bucket name"
                },
                "key": {
                    "type": "string",
                    "description": "Object key/path"
                },
                "content": {
                    "type": "string",
                    "description": "Content to upload"
                },
                "content_type": {
                    "type": "string",
                    "description": "MIME type (e.g., application/json, image/png)"
                },
                "storage_class": {
                    "type": "string",
                    "enum": [
                        "STANDARD",
                        "REDUCED_REDUNDANCY",
                        "STANDARD_IA",
                        "ONEZONE_IA",
                        "INTELLIGENT_TIERING",
                        "GLACIER",
                        "DEEP_ARCHIVE"
                    ],
                    "description": "Storage class"
                },
                "encryption": {
                    "type": "string",
                    "enum": ["AES256", "aws:kms"],
                    "description": "Server-side encryption"
                },
                "acl": {
                    "type": "string",
                    "enum": [
                        "private",
                        "public-read",
                        "public-read-write",
                        "authenticated-read",
                        "bucket-owner-read",
                        "bucket-owner-full-control"
                    ],
                    "description": "Access control list"
                },
                "prefix": {
                    "type": "string",
                    "description": "Prefix for listing objects"
                },
                "max_keys": {
                    "type": "integer",
                    "description": "Maximum keys to return in list"
                },
                "tags": {
                    "type": "object",
                    "description": "Object tags"
                },
                "expires_in": {
                    "type": "integer",
                    "description": "Presigned URL expiration in seconds"
                }
            },
            "required": ["operation"]
        })
    }

    fn required_credential_type(&self) -> Option<&str> {
        Some("aws")
    }
}

#[async_trait]
impl Node for AwsS3Node {
    async fn execute(&self, context: &NodeContext, parameters: &serde_json::Value) -> Result<NodeOutput> {
        let params: AwsS3Params = serde_json::from_value(parameters.clone())?;

        // Validate required parameters
        self.validate_parameters(parameters)?;

        // TODO: Implement actual AWS S3 SDK calls
        // This is a placeholder implementation
        match params.operation.as_str() {
            "upload_object" => {
                let bucket = params.bucket.as_ref().unwrap();
                let key = params.key.as_ref().unwrap();

                Ok(NodeOutput::success(json!({
                    "success": true,
                    "operation": "upload_object",
                    "bucket": bucket,
                    "key": key,
                    "etag": "d41d8cd98f00b204e9800998ecf8427e",
                    "version_id": "v123456",
                    "size": 1024,
                    "location": format!("s3://{}/{}", bucket, key)
                })))
            }
            "download_object" => {
                let bucket = params.bucket.as_ref().unwrap();
                let key = params.key.as_ref().unwrap();

                Ok(NodeOutput::success(json!({
                    "success": true,
                    "operation": "download_object",
                    "bucket": bucket,
                    "key": key,
                    "content": "Base64 encoded content would go here",
                    "content_type": "application/octet-stream",
                    "size": 1024,
                    "last_modified": "2025-01-15T10:30:00Z",
                    "etag": "d41d8cd98f00b204e9800998ecf8427e"
                })))
            }
            "delete_object" => {
                let bucket = params.bucket.as_ref().unwrap();
                let key = params.key.as_ref().unwrap();

                Ok(NodeOutput::success(json!({
                    "success": true,
                    "operation": "delete_object",
                    "bucket": bucket,
                    "key": key,
                    "deleted": true,
                    "version_id": "v123456"
                })))
            }
            "copy_object" => {
                let bucket = params.bucket.as_ref().unwrap();
                let key = params.key.as_ref().unwrap();
                let source_bucket = params.source_bucket.as_ref().unwrap();
                let source_key = params.source_key.as_ref().unwrap();

                Ok(NodeOutput::success(json!({
                    "success": true,
                    "operation": "copy_object",
                    "source": format!("s3://{}/{}", source_bucket, source_key),
                    "destination": format!("s3://{}/{}", bucket, key),
                    "etag": "d41d8cd98f00b204e9800998ecf8427e",
                    "version_id": "v789012"
                })))
            }
            "list_objects" => {
                let bucket = params.bucket.as_ref().unwrap();
                let prefix = params.prefix.as_deref().unwrap_or("");

                Ok(NodeOutput::success(json!({
                    "success": true,
                    "operation": "list_objects",
                    "bucket": bucket,
                    "prefix": prefix,
                    "contents": [
                        {
                            "key": format!("{}file1.txt", prefix),
                            "size": 1024,
                            "last_modified": "2025-01-15T10:30:00Z",
                            "etag": "abc123",
                            "storage_class": "STANDARD"
                        },
                        {
                            "key": format!("{}file2.json", prefix),
                            "size": 2048,
                            "last_modified": "2025-01-15T11:00:00Z",
                            "etag": "def456",
                            "storage_class": "STANDARD"
                        }
                    ],
                    "is_truncated": false,
                    "key_count": 2
                })))
            }
            "list_buckets" => {
                Ok(NodeOutput::success(json!({
                    "success": true,
                    "operation": "list_buckets",
                    "buckets": [
                        {
                            "name": "my-bucket-1",
                            "creation_date": "2025-01-01T00:00:00Z"
                        },
                        {
                            "name": "my-bucket-2",
                            "creation_date": "2025-01-10T00:00:00Z"
                        }
                    ],
                    "owner": {
                        "id": "abc123",
                        "display_name": "user"
                    }
                })))
            }
            "create_bucket" => {
                let bucket = params.bucket.as_ref().unwrap();

                Ok(NodeOutput::success(json!({
                    "success": true,
                    "operation": "create_bucket",
                    "bucket": bucket,
                    "location": format!("/{}", bucket)
                })))
            }
            "generate_presigned_url" => {
                let bucket = params.bucket.as_ref().unwrap();
                let key = params.key.as_ref().unwrap();
                let expires_in = params.expires_in.unwrap_or(3600);

                Ok(NodeOutput::success(json!({
                    "success": true,
                    "operation": "generate_presigned_url",
                    "bucket": bucket,
                    "key": key,
                    "url": format!("https://{}.s3.amazonaws.com/{}?X-Amz-Algorithm=...", bucket, key),
                    "expires_in": expires_in
                })))
            }
            "multipart_upload" => {
                let bucket = params.bucket.as_ref().unwrap();
                let key = params.key.as_ref().unwrap();

                Ok(NodeOutput::success(json!({
                    "success": true,
                    "operation": "multipart_upload",
                    "bucket": bucket,
                    "key": key,
                    "upload_id": "upload-12345",
                    "parts": 5,
                    "completed": true,
                    "etag": "multipart-etag-abc123"
                })))
            }
            "set_bucket_versioning" => {
                let bucket = params.bucket.as_ref().unwrap();
                let versioning = params.versioning.as_deref().unwrap_or("Enabled");

                Ok(NodeOutput::success(json!({
                    "success": true,
                    "operation": "set_bucket_versioning",
                    "bucket": bucket,
                    "versioning": versioning
                })))
            }
            "get_object_metadata" => {
                let bucket = params.bucket.as_ref().unwrap();
                let key = params.key.as_ref().unwrap();

                Ok(NodeOutput::success(json!({
                    "success": true,
                    "operation": "get_object_metadata",
                    "bucket": bucket,
                    "key": key,
                    "metadata": {
                        "content_type": "application/json",
                        "content_length": 1024,
                        "last_modified": "2025-01-15T10:30:00Z",
                        "etag": "abc123",
                        "version_id": "v123456",
                        "storage_class": "STANDARD",
                        "server_side_encryption": "AES256"
                    }
                })))
            }
            _ => anyhow::bail!("Unsupported operation: {}", params.operation),
        }
    }

    fn validate_parameters(&self, parameters: &serde_json::Value) -> Result<()> {
        let params: AwsS3Params = serde_json::from_value(parameters.clone())?;
        // Operations that require bucket
        let bucket_required_ops = vec![
            "upload_object",
            "download_object",
            "delete_object",
            "copy_object",
            "list_objects",
            "create_bucket",
            "generate_presigned_url",
            "multipart_upload",
            "set_bucket_versioning",
            "get_object_metadata",
        ];

        if bucket_required_ops.contains(&params.operation.as_str())
            && params.bucket.is_none()
        {
            anyhow::bail!("{} operation requires 'bucket' parameter", params.operation);
        }

        // Operations that require key
        let key_required_ops = vec![
            "upload_object",
            "download_object",
            "delete_object",
            "get_object_metadata",
            "generate_presigned_url",
            "multipart_upload",
        ];

        if key_required_ops.contains(&params.operation.as_str()) && params.key.is_none() {
            anyhow::bail!("{} operation requires 'key' parameter", params.operation);
        }

        // Upload requires content
        if params.operation == "upload_object" && params.content.is_none() {
            anyhow::bail!("upload_object operation requires 'content' parameter");
        }

        // Copy requires source
        if params.operation == "copy_object" {
            if params.source_bucket.is_none() {
                anyhow::bail!("copy_object operation requires 'source_bucket' parameter");
            }
            if params.source_key.is_none() {
                anyhow::bail!("copy_object operation requires 'source_key' parameter");
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_s3_upload_object() {
        let node = AwsS3Node::new();
        let params = json!({
            "operation": "upload_object",
            "bucket": "my-bucket",
            "key": "path/to/file.txt",
            "content": "Hello, World!",
            "content_type": "text/plain"
        });

        let context = NodeContext::new(
            Uuid::new_v4().to_string(),
            "test-workflow".to_string(),
        );

        let result = node.execute(&context, &params).await.unwrap();
        assert_eq!(result.data["success"], true);
        assert_eq!(result.data["operation"], "upload_object");
        assert!(result.data["etag"].is_string());
    }

    #[tokio::test]
    async fn test_s3_list_objects() {
        let node = AwsS3Node::new();
        let params = json!({
            "operation": "list_objects",
            "bucket": "my-bucket",
            "prefix": "data/"
        });

        let context = NodeContext::new(
            Uuid::new_v4().to_string(),
            "test-workflow".to_string(),
        );

        let result = node.execute(&context, &params).await.unwrap();
        assert_eq!(result.data["success"], true);
        assert!(result.data["contents"].is_array());
    }

    #[tokio::test]
    async fn test_s3_copy_object() {
        let node = AwsS3Node::new();
        let params = json!({
            "operation": "copy_object",
            "bucket": "destination-bucket",
            "key": "new-file.txt",
            "source_bucket": "source-bucket",
            "source_key": "old-file.txt"
        });

        let context = NodeContext::new(
            Uuid::new_v4().to_string(),
            "test-workflow".to_string(),
        );

        let result = node.execute(&context, &params).await.unwrap();
        assert_eq!(result.data["success"], true);
        assert!(result.data["source"].as_str().unwrap().contains("source-bucket"));
    }

    #[tokio::test]
    async fn test_s3_generate_presigned_url() {
        let node = AwsS3Node::new();
        let params = json!({
            "operation": "generate_presigned_url",
            "bucket": "my-bucket",
            "key": "file.txt",
            "expires_in": 7200
        });

        let context = NodeContext::new(
            Uuid::new_v4().to_string(),
            "test-workflow".to_string(),
        );

        let result = node.execute(&context, &params).await.unwrap();
        assert_eq!(result.data["success"], true);
        assert!(result.data["url"].as_str().unwrap().contains("s3.amazonaws.com"));
    }

    #[tokio::test]
    async fn test_s3_validation_missing_bucket() {
        let node = AwsS3Node::new();
        let params = json!({
            "operation": "upload_object",
            "key": "file.txt",
            "content": "test"
        });

        let context = NodeContext::new(
            Uuid::new_v4().to_string(),
            "test-workflow".to_string(),
        );

        let result = node.execute(&context, &params).await;
        assert!(result.is_err());
    }
}
