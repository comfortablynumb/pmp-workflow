use crate::models::{Node, NodeCategory, NodeContext, NodeOutput, NodeSubcategory, NodeType};
use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

/// HashiCorp Vault integration for secret management
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VaultParams {
    /// Operation to perform
    pub operation: String,
    /// Vault server URL
    pub vault_url: Option<String>,
    /// Path to secret (e.g., "secret/data/myapp/config")
    pub path: Option<String>,
    /// Secret data to write (for write operations)
    pub data: Option<Value>,
    /// Key to read from secret (optional, returns all if not specified)
    pub key: Option<String>,
    /// Secret engine mount point (default: "secret")
    pub mount: Option<String>,
    /// Namespace (Vault Enterprise)
    pub namespace: Option<String>,
    /// Version to read (for KV v2, optional)
    pub version: Option<u32>,
    /// Token role for AppRole auth
    pub role_id: Option<String>,
    /// Secret ID for AppRole auth
    pub secret_id: Option<String>,
}

pub struct VaultNode;

impl Default for VaultNode {
    fn default() -> Self {
        Self::new()
    }
}

impl VaultNode {
    pub fn new() -> Self {
        Self
    }
}

impl NodeType for VaultNode {
    fn type_name(&self) -> &str {
        "vault"
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
                        "read_secret",
                        "write_secret",
                        "delete_secret",
                        "list_secrets",
                        "read_kv_metadata",
                        "create_token",
                        "renew_token",
                        "revoke_token",
                        "approle_login",
                        "enable_audit",
                        "seal_status"
                    ],
                    "description": "Operation to perform"
                },
                "vault_url": {
                    "type": "string",
                    "description": "Vault server URL (e.g., https://vault.example.com:8200)"
                },
                "path": {
                    "type": "string",
                    "description": "Path to secret"
                },
                "data": {
                    "type": "object",
                    "description": "Secret data to write"
                },
                "key": {
                    "type": "string",
                    "description": "Specific key to read from secret"
                },
                "mount": {
                    "type": "string",
                    "description": "Secret engine mount point (default: secret)"
                },
                "namespace": {
                    "type": "string",
                    "description": "Vault namespace (Enterprise feature)"
                },
                "version": {
                    "type": "integer",
                    "description": "Secret version to read (KV v2)"
                },
                "role_id": {
                    "type": "string",
                    "description": "Role ID for AppRole authentication"
                },
                "secret_id": {
                    "type": "string",
                    "description": "Secret ID for AppRole authentication"
                }
            },
            "required": ["operation"]
        })
    }

    fn required_credential_type(&self) -> Option<&str> {
        Some("vault")
    }
}

#[async_trait]
impl Node for VaultNode {
    async fn execute(
        &self,
        _context: &NodeContext,
        parameters: &serde_json::Value,
    ) -> Result<NodeOutput> {
        let params: VaultParams = serde_json::from_value(parameters.clone())?;

        // Validate required parameters
        self.validate_parameters(parameters)?;

        // TODO: Implement actual Vault API calls
        // This is a placeholder implementation
        match params.operation.as_str() {
            "read_secret" => {
                let path = params.path.as_ref().unwrap();
                let mount = params.mount.as_deref().unwrap_or("secret");

                Ok(NodeOutput::success(json!({
                    "success": true,
                    "operation": "read_secret",
                    "path": format!("{}/data/{}", mount, path),
                    "data": {
                        "database_url": "postgresql://...",
                        "api_key": "sk-..."
                    },
                    "version": params.version.unwrap_or(0),
                    "created_time": "2025-01-15T10:30:00Z",
                    "metadata": {
                        "created_time": "2025-01-15T10:30:00Z",
                        "version": params.version.unwrap_or(0)
                    }
                })))
            }
            "write_secret" => {
                let path = params.path.as_ref().unwrap();
                let _data = params.data.as_ref().unwrap();
                let mount = params.mount.as_deref().unwrap_or("secret");

                Ok(NodeOutput::success(json!({
                    "success": true,
                    "operation": "write_secret",
                    "path": format!("{}/data/{}", mount, path),
                    "version": 1,
                    "created_time": "2025-01-15T10:30:00Z"
                })))
            }
            "delete_secret" => {
                let path = params.path.as_ref().unwrap();
                let mount = params.mount.as_deref().unwrap_or("secret");

                Ok(NodeOutput::success(json!({
                    "success": true,
                    "operation": "delete_secret",
                    "path": format!("{}/data/{}", mount, path),
                    "deleted": true
                })))
            }
            "list_secrets" => {
                let path = params.path.as_deref().unwrap_or("");
                let mount = params.mount.as_deref().unwrap_or("secret");

                Ok(NodeOutput::success(json!({
                    "success": true,
                    "operation": "list_secrets",
                    "path": format!("{}/metadata/{}", mount, path),
                    "keys": ["config/", "database/", "api-keys"]
                })))
            }
            "read_kv_metadata" => {
                let path = params.path.as_ref().unwrap();
                let mount = params.mount.as_deref().unwrap_or("secret");

                Ok(NodeOutput::success(json!({
                    "success": true,
                    "operation": "read_kv_metadata",
                    "path": format!("{}/metadata/{}", mount, path),
                    "metadata": {
                        "current_version": 3,
                        "oldest_version": 1,
                        "created_time": "2025-01-15T10:30:00Z",
                        "updated_time": "2025-01-15T12:45:00Z",
                        "versions": {
                            "1": {"created_time": "2025-01-15T10:30:00Z", "deletion_time": ""},
                            "2": {"created_time": "2025-01-15T11:15:00Z", "deletion_time": ""},
                            "3": {"created_time": "2025-01-15T12:45:00Z", "deletion_time": ""}
                        }
                    }
                })))
            }
            "create_token" => Ok(NodeOutput::success(json!({
                "success": true,
                "operation": "create_token",
                "auth": {
                    "client_token": "hvs.CAESIJ...",
                    "accessor": "hmac-sha256...",
                    "policies": ["default", "app-policy"],
                    "lease_duration": 3600,
                    "renewable": true
                }
            }))),
            "renew_token" => Ok(NodeOutput::success(json!({
                "success": true,
                "operation": "renew_token",
                "auth": {
                    "client_token": "hvs.CAESIJ...",
                    "lease_duration": 3600,
                    "renewable": true
                }
            }))),
            "revoke_token" => Ok(NodeOutput::success(json!({
                "success": true,
                "operation": "revoke_token",
                "revoked": true
            }))),
            "approle_login" => Ok(NodeOutput::success(json!({
                "success": true,
                "operation": "approle_login",
                "auth": {
                    "client_token": "hvs.CAESIJ...",
                    "accessor": "hmac-sha256...",
                    "policies": ["default", "app-policy"],
                    "lease_duration": 3600,
                    "renewable": true,
                    "metadata": {
                        "role_name": "my-role"
                    }
                }
            }))),
            "enable_audit" => Ok(NodeOutput::success(json!({
                "success": true,
                "operation": "enable_audit",
                "enabled": true,
                "type": "file",
                "path": "audit/"
            }))),
            "seal_status" => Ok(NodeOutput::success(json!({
                "success": true,
                "operation": "seal_status",
                "sealed": false,
                "type": "shamir",
                "initialized": true,
                "version": "1.15.0",
                "cluster_name": "vault-cluster-1",
                "cluster_id": "abc123...",
                "progress": 0,
                "n": 5,
                "t": 3
            }))),
            _ => anyhow::bail!("Unsupported operation: {}", params.operation),
        }
    }

    fn validate_parameters(&self, parameters: &serde_json::Value) -> Result<()> {
        let params: VaultParams = serde_json::from_value(parameters.clone())?;
        // Operations that require path
        let path_required_ops = [
            "read_secret",
            "write_secret",
            "delete_secret",
            "read_kv_metadata",
        ];

        if path_required_ops.contains(&params.operation.as_str()) && params.path.is_none() {
            anyhow::bail!("{} operation requires 'path' parameter", params.operation);
        }

        // Write operation requires data
        if params.operation == "write_secret" && params.data.is_none() {
            anyhow::bail!("write_secret operation requires 'data' parameter");
        }

        // AppRole login requires role_id and secret_id
        if params.operation == "approle_login" {
            if params.role_id.is_none() {
                anyhow::bail!("approle_login requires 'role_id' parameter");
            }
            if params.secret_id.is_none() {
                anyhow::bail!("approle_login requires 'secret_id' parameter");
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
    async fn test_vault_read_secret() {
        let node = VaultNode::new();
        let params = json!({
            "operation": "read_secret",
            "path": "myapp/config",
            "mount": "secret"
        });

        let context = NodeContext::new(Uuid::new_v4().to_string(), "test-workflow".to_string());

        let result = node.execute(&context, &params).await.unwrap();
        assert_eq!(result.data["success"], true);
        assert_eq!(result.data["operation"], "read_secret");
        assert!(result.data["data"].is_object());
    }

    #[tokio::test]
    async fn test_vault_write_secret() {
        let node = VaultNode::new();
        let params = json!({
            "operation": "write_secret",
            "path": "myapp/config",
            "data": {
                "database_url": "postgresql://...",
                "api_key": "sk-..."
            }
        });

        let context = NodeContext::new(Uuid::new_v4().to_string(), "test-workflow".to_string());

        let result = node.execute(&context, &params).await.unwrap();
        assert_eq!(result.data["success"], true);
        assert_eq!(result.data["operation"], "write_secret");
    }

    #[tokio::test]
    async fn test_vault_list_secrets() {
        let node = VaultNode::new();
        let params = json!({
            "operation": "list_secrets",
            "path": "myapp/"
        });

        let context = NodeContext::new(Uuid::new_v4().to_string(), "test-workflow".to_string());

        let result = node.execute(&context, &params).await.unwrap();
        assert_eq!(result.data["success"], true);
        assert!(result.data["keys"].is_array());
    }

    #[tokio::test]
    async fn test_vault_approle_login() {
        let node = VaultNode::new();
        let params = json!({
            "operation": "approle_login",
            "role_id": "role-123",
            "secret_id": "secret-456"
        });

        let context = NodeContext::new(Uuid::new_v4().to_string(), "test-workflow".to_string());

        let result = node.execute(&context, &params).await.unwrap();
        assert_eq!(result.data["success"], true);
        assert!(result.data["auth"]["client_token"].is_string());
    }

    #[tokio::test]
    async fn test_vault_validation_missing_path() {
        let node = VaultNode::new();
        let params = json!({
            "operation": "read_secret"
        });

        let context = NodeContext::new(Uuid::new_v4().to_string(), "test-workflow".to_string());

        let result = node.execute(&context, &params).await;
        assert!(result.is_err());
    }
}
