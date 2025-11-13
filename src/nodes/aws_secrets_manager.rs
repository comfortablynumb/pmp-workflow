use crate::models::{Node, NodeCategory, NodeContext, NodeOutput, NodeSubcategory, NodeType};
use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

/// AWS Secrets Manager integration for secret management
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AwsSecretsManagerParams {
    /// Operation to perform
    pub operation: String,
    /// Secret name/identifier
    pub secret_name: Option<String>,
    /// Secret value (string or JSON object)
    pub secret_value: Option<Value>,
    /// Description of the secret
    pub description: Option<String>,
    /// KMS key ID for encryption
    pub kms_key_id: Option<String>,
    /// Version ID for retrieval
    pub version_id: Option<String>,
    /// Version stage (e.g., AWSCURRENT, AWSPENDING)
    pub version_stage: Option<String>,
    /// Tags for the secret
    pub tags: Option<Value>,
    /// ARN of Lambda function for rotation
    pub rotation_lambda_arn: Option<String>,
    /// Number of days between automatic rotations
    pub rotation_days: Option<u32>,
}

pub struct AwsSecretsManagerNode;

impl AwsSecretsManagerNode {
    pub fn new() -> Self {
        Self
    }
}

impl NodeType for AwsSecretsManagerNode {
    fn type_name(&self) -> &str {
        "aws_secrets_manager"
    }

    fn category(&self) -> NodeCategory {
        NodeCategory::Action
    }

    fn subcategory(&self) -> NodeSubcategory {
        NodeSubcategory::General
    }

    fn parameter_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "operation": {
                    "type": "string",
                    "enum": [
                        "create_secret",
                        "get_secret_value",
                        "update_secret",
                        "delete_secret",
                        "list_secrets",
                        "describe_secret",
                        "rotate_secret",
                        "put_secret_value",
                        "restore_secret",
                        "tag_resource"
                    ],
                    "description": "Operation to perform"
                },
                "secret_name": {
                    "type": "string",
                    "description": "Name or ARN of the secret"
                },
                "secret_value": {
                    "description": "Secret value (string or JSON object)"
                },
                "description": {
                    "type": "string",
                    "description": "Description of the secret"
                },
                "kms_key_id": {
                    "type": "string",
                    "description": "KMS key ID for encryption"
                },
                "version_id": {
                    "type": "string",
                    "description": "Version ID to retrieve"
                },
                "version_stage": {
                    "type": "string",
                    "description": "Version stage (AWSCURRENT, AWSPENDING, etc.)"
                },
                "tags": {
                    "type": "object",
                    "description": "Tags for the secret"
                },
                "rotation_lambda_arn": {
                    "type": "string",
                    "description": "ARN of Lambda function for rotation"
                },
                "rotation_days": {
                    "type": "integer",
                    "minimum": 1,
                    "description": "Days between automatic rotations"
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
impl Node for AwsSecretsManagerNode {
    async fn execute(&self, context: &NodeContext, parameters: &serde_json::Value) -> Result<NodeOutput> {
        let params: AwsSecretsManagerParams = serde_json::from_value(parameters.clone())?;

        // Validate required parameters
        self.validate_parameters(parameters)?;

        // TODO: Implement actual AWS Secrets Manager SDK calls
        // This is a placeholder implementation
        match params.operation.as_str() {
            "create_secret" => {
                let secret_name = params.secret_name.as_ref().unwrap();
                let secret_value = params.secret_value.as_ref().unwrap();

                Ok(NodeOutput::success(json!({
                    "success": true,
                    "operation": "create_secret",
                    "secret_name": secret_name,
                    "arn": format!("arn:aws:secretsmanager:us-east-1:123456789012:secret:{}-AbCdEf", secret_name),
                    "version_id": "a1b2c3d4-5678-90ab-cdef-123456789012",
                    "created_date": "2025-01-15T10:30:00Z"
                })))
            }
            "get_secret_value" => {
                let secret_name = params.secret_name.as_ref().unwrap();
                let version_stage = params.version_stage.as_deref().unwrap_or("AWSCURRENT");

                Ok(NodeOutput::success(json!({
                    "success": true,
                    "operation": "get_secret_value",
                    "secret_name": secret_name,
                    "arn": format!("arn:aws:secretsmanager:us-east-1:123456789012:secret:{}-AbCdEf", secret_name),
                    "secret_string": "{\"username\":\"admin\",\"password\":\"secret123\"}",
                    "version_id": "a1b2c3d4-5678-90ab-cdef-123456789012",
                    "version_stages": [version_stage],
                    "created_date": "2025-01-15T10:30:00Z"
                })))
            }
            "update_secret" => {
                let secret_name = params.secret_name.as_ref().unwrap();
                let secret_value = params.secret_value.as_ref().unwrap();

                Ok(NodeOutput::success(json!({
                    "success": true,
                    "operation": "update_secret",
                    "secret_name": secret_name,
                    "arn": format!("arn:aws:secretsmanager:us-east-1:123456789012:secret:{}-AbCdEf", secret_name),
                    "version_id": "b2c3d4e5-6789-01bc-def0-234567890123",
                    "updated_date": "2025-01-15T12:00:00Z"
                })))
            }
            "delete_secret" => {
                let secret_name = params.secret_name.as_ref().unwrap();

                Ok(NodeOutput::success(json!({
                    "success": true,
                    "operation": "delete_secret",
                    "secret_name": secret_name,
                    "arn": format!("arn:aws:secretsmanager:us-east-1:123456789012:secret:{}-AbCdEf", secret_name),
                    "deletion_date": "2025-02-14T10:30:00Z",
                    "scheduled_deletion": true
                })))
            }
            "list_secrets" => Ok(NodeOutput::success(json!({
                "success": true,
                "operation": "list_secrets",
                "secrets": [
                    {
                        "name": "database/production",
                        "arn": "arn:aws:secretsmanager:us-east-1:123456789012:secret:database/production-AbCdEf",
                        "description": "Production database credentials",
                        "created_date": "2025-01-10T08:00:00Z",
                        "last_accessed_date": "2025-01-15T10:30:00Z",
                        "last_changed_date": "2025-01-12T14:20:00Z",
                        "last_rotated_date": "2025-01-12T14:20:00Z",
                        "rotation_enabled": true,
                        "rotation_lambda_arn": "arn:aws:lambda:us-east-1:123456789012:function:rotate-secret"
                    },
                    {
                        "name": "api/keys/external",
                        "arn": "arn:aws:secretsmanager:us-east-1:123456789012:secret:api/keys/external-GhIjKl",
                        "description": "External API keys",
                        "created_date": "2025-01-08T09:00:00Z",
                        "last_accessed_date": "2025-01-15T09:15:00Z",
                        "last_changed_date": "2025-01-08T09:00:00Z",
                        "rotation_enabled": false
                    }
                ],
                "next_token": null
            }))),
            "describe_secret" => {
                let secret_name = params.secret_name.as_ref().unwrap();

                Ok(NodeOutput::success(json!({
                    "success": true,
                    "operation": "describe_secret",
                    "secret_name": secret_name,
                    "arn": format!("arn:aws:secretsmanager:us-east-1:123456789012:secret:{}-AbCdEf", secret_name),
                    "description": "Database credentials for production environment",
                    "kms_key_id": "arn:aws:kms:us-east-1:123456789012:key/12345678-1234-1234-1234-123456789012",
                    "rotation_enabled": true,
                    "rotation_lambda_arn": "arn:aws:lambda:us-east-1:123456789012:function:rotate-secret",
                    "rotation_rules": {
                        "automatically_after_days": 30
                    },
                    "last_rotated_date": "2025-01-10T10:30:00Z",
                    "last_changed_date": "2025-01-10T10:30:00Z",
                    "last_accessed_date": "2025-01-15T10:30:00Z",
                    "created_date": "2025-01-01T00:00:00Z",
                    "version_ids_to_stages": {
                        "a1b2c3d4-5678-90ab-cdef-123456789012": ["AWSCURRENT"],
                        "b2c3d4e5-6789-01bc-def0-234567890123": ["AWSPREVIOUS"]
                    },
                    "tags": [
                        {"key": "Environment", "value": "Production"},
                        {"key": "Application", "value": "MyApp"}
                    ]
                })))
            }
            "rotate_secret" => {
                let secret_name = params.secret_name.as_ref().unwrap();
                let rotation_lambda_arn = params.rotation_lambda_arn.as_ref().unwrap();

                Ok(NodeOutput::success(json!({
                    "success": true,
                    "operation": "rotate_secret",
                    "secret_name": secret_name,
                    "arn": format!("arn:aws:secretsmanager:us-east-1:123456789012:secret:{}-AbCdEf", secret_name),
                    "version_id": "c3d4e5f6-7890-12cd-ef01-345678901234",
                    "rotation_lambda_arn": rotation_lambda_arn,
                    "rotation_initiated": "2025-01-15T10:30:00Z"
                })))
            }
            "put_secret_value" => {
                let secret_name = params.secret_name.as_ref().unwrap();
                let secret_value = params.secret_value.as_ref().unwrap();

                Ok(NodeOutput::success(json!({
                    "success": true,
                    "operation": "put_secret_value",
                    "secret_name": secret_name,
                    "arn": format!("arn:aws:secretsmanager:us-east-1:123456789012:secret:{}-AbCdEf", secret_name),
                    "version_id": "d4e5f6g7-8901-23de-f012-456789012345",
                    "version_stages": ["AWSCURRENT"]
                })))
            }
            "restore_secret" => {
                let secret_name = params.secret_name.as_ref().unwrap();

                Ok(NodeOutput::success(json!({
                    "success": true,
                    "operation": "restore_secret",
                    "secret_name": secret_name,
                    "arn": format!("arn:aws:secretsmanager:us-east-1:123456789012:secret:{}-AbCdEf", secret_name),
                    "restored": true
                })))
            }
            "tag_resource" => {
                let secret_name = params.secret_name.as_ref().unwrap();
                let tags = params.tags.as_ref().unwrap();

                Ok(NodeOutput::success(json!({
                    "success": true,
                    "operation": "tag_resource",
                    "secret_name": secret_name,
                    "arn": format!("arn:aws:secretsmanager:us-east-1:123456789012:secret:{}-AbCdEf", secret_name),
                    "tags_applied": tags
                })))
            }
            _ => anyhow::bail!("Unsupported operation: {}", params.operation),
        }
    }

    fn validate_parameters(&self, parameters: &serde_json::Value) -> Result<()> {
        let params: AwsSecretsManagerParams = serde_json::from_value(parameters.clone())?;
        // Operations that require secret_name
        let secret_name_required_ops = vec![
            "create_secret",
            "get_secret_value",
            "update_secret",
            "delete_secret",
            "describe_secret",
            "rotate_secret",
            "put_secret_value",
            "restore_secret",
            "tag_resource",
        ];

        if secret_name_required_ops.contains(&params.operation.as_str())
            && params.secret_name.is_none()
        {
            anyhow::bail!(
                "{} operation requires 'secret_name' parameter",
                params.operation
            );
        }

        // create_secret and put_secret_value require secret_value
        if (params.operation == "create_secret" || params.operation == "put_secret_value")
            && params.secret_value.is_none()
        {
            anyhow::bail!(
                "{} operation requires 'secret_value' parameter",
                params.operation
            );
        }

        // rotate_secret requires rotation_lambda_arn
        if params.operation == "rotate_secret" && params.rotation_lambda_arn.is_none() {
            anyhow::bail!("rotate_secret operation requires 'rotation_lambda_arn' parameter");
        }

        // update_secret requires either secret_value, description, or kms_key_id
        if params.operation == "update_secret"
            && params.secret_value.is_none()
            && params.description.is_none()
            && params.kms_key_id.is_none()
        {
            anyhow::bail!(
                "update_secret operation requires at least one of: 'secret_value', 'description', or 'kms_key_id'"
            );
        }

        // tag_resource requires tags
        if params.operation == "tag_resource" && params.tags.is_none() {
            anyhow::bail!("tag_resource operation requires 'tags' parameter");
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_create_secret() {
        let node = AwsSecretsManagerNode::new();
        let params = json!({
            "operation": "create_secret",
            "secret_name": "my-database-secret",
            "secret_value": {
                "username": "admin",
                "password": "secure_password_123"
            },
            "description": "Database credentials",
            "tags": {
                "Environment": "Production",
                "Application": "MyApp"
            }
        });

        let context = NodeContext::new(Uuid::new_v4().to_string(), "test-workflow".to_string());

        let result = node.execute(&context, &params).await.unwrap();
        assert_eq!(result.data["success"], true);
        assert_eq!(result.data["operation"], "create_secret");
        assert_eq!(result.data["secret_name"], "my-database-secret");
        assert!(result.data["arn"].is_string());
        assert!(result.data["version_id"].is_string());
    }

    #[tokio::test]
    async fn test_get_secret_value() {
        let node = AwsSecretsManagerNode::new();
        let params = json!({
            "operation": "get_secret_value",
            "secret_name": "my-database-secret",
            "version_stage": "AWSCURRENT"
        });

        let context = NodeContext::new(Uuid::new_v4().to_string(), "test-workflow".to_string());

        let result = node.execute(&context, &params).await.unwrap();
        assert_eq!(result.data["success"], true);
        assert_eq!(result.data["operation"], "get_secret_value");
        assert!(result.data["secret_string"].is_string());
        assert!(result.data["version_id"].is_string());
    }

    #[tokio::test]
    async fn test_list_secrets() {
        let node = AwsSecretsManagerNode::new();
        let params = json!({
            "operation": "list_secrets"
        });

        let context = NodeContext::new(Uuid::new_v4().to_string(), "test-workflow".to_string());

        let result = node.execute(&context, &params).await.unwrap();
        assert_eq!(result.data["success"], true);
        assert_eq!(result.data["operation"], "list_secrets");
        assert!(result.data["secrets"].is_array());
        assert!(result.data["secrets"].as_array().unwrap().len() > 0);
    }

    #[tokio::test]
    async fn test_rotate_secret() {
        let node = AwsSecretsManagerNode::new();
        let params = json!({
            "operation": "rotate_secret",
            "secret_name": "my-database-secret",
            "rotation_lambda_arn": "arn:aws:lambda:us-east-1:123456789012:function:rotate-secret",
            "rotation_days": 30
        });

        let context = NodeContext::new(Uuid::new_v4().to_string(), "test-workflow".to_string());

        let result = node.execute(&context, &params).await.unwrap();
        assert_eq!(result.data["success"], true);
        assert_eq!(result.data["operation"], "rotate_secret");
        assert!(result.data["version_id"].is_string());
        assert_eq!(
            result.data["rotation_lambda_arn"],
            "arn:aws:lambda:us-east-1:123456789012:function:rotate-secret"
        );
    }

    #[tokio::test]
    async fn test_validation_missing_secret_name() {
        let node = AwsSecretsManagerNode::new();
        let params = json!({
            "operation": "get_secret_value"
        });

        let context = NodeContext::new(Uuid::new_v4().to_string(), "test-workflow".to_string());

        let result = node.execute(&context, &params).await;
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("requires 'secret_name' parameter")
        );
    }
}
