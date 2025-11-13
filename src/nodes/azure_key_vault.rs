use crate::models::{Node, NodeCategory, NodeContext, NodeOutput, NodeSubcategory, NodeType};
use anyhow::Result;
use async_trait::async_trait;
use serde::Deserialize;
use serde_json::{json, Value};

/// Azure Key Vault integration for secret, key, and certificate management
#[derive(Debug, Clone, Deserialize)]
pub struct AzureKeyVaultParams {
    /// Operation to perform
    pub operation: String,
    /// Vault URL (e.g., https://myvault.vault.azure.net/)
    pub vault_url: Option<String>,
    /// Secret name
    pub secret_name: Option<String>,
    /// Secret value
    pub secret_value: Option<String>,
    /// Key name
    pub key_name: Option<String>,
    /// Certificate name
    pub certificate_name: Option<String>,
    /// Version of secret/key/certificate (optional, uses latest if not specified)
    pub version: Option<String>,
    /// Whether the secret/key/certificate is enabled
    pub enabled: Option<bool>,
    /// Expiration date (ISO 8601 format)
    pub expires_on: Option<String>,
    /// Not before date (ISO 8601 format)
    pub not_before: Option<String>,
    /// Tags for the secret/key/certificate
    pub tags: Option<Value>,
    /// Content type for secrets
    pub content_type: Option<String>,
}

pub struct AzureKeyVaultNode;

impl AzureKeyVaultNode {
    pub fn new() -> Self {
        Self
    }
}

impl NodeType for AzureKeyVaultNode {
    fn type_name(&self) -> &str {
        "azure_key_vault"
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
                        "set_secret",
                        "get_secret",
                        "delete_secret",
                        "list_secrets",
                        "list_secret_versions",
                        "create_key",
                        "get_key",
                        "delete_key",
                        "list_keys",
                        "import_certificate",
                        "get_certificate",
                        "delete_certificate",
                        "list_certificates"
                    ],
                    "description": "Operation to perform"
                },
                "vault_url": {
                    "type": "string",
                    "description": "Azure Key Vault URL (e.g., https://myvault.vault.azure.net/)"
                },
                "secret_name": {
                    "type": "string",
                    "description": "Name of the secret"
                },
                "secret_value": {
                    "type": "string",
                    "description": "Value of the secret"
                },
                "key_name": {
                    "type": "string",
                    "description": "Name of the key"
                },
                "certificate_name": {
                    "type": "string",
                    "description": "Name of the certificate"
                },
                "version": {
                    "type": "string",
                    "description": "Version of the secret/key/certificate (optional)"
                },
                "enabled": {
                    "type": "boolean",
                    "description": "Whether the secret/key/certificate is enabled"
                },
                "expires_on": {
                    "type": "string",
                    "description": "Expiration date in ISO 8601 format"
                },
                "not_before": {
                    "type": "string",
                    "description": "Not before date in ISO 8601 format"
                },
                "tags": {
                    "type": "object",
                    "description": "Tags for the resource"
                },
                "content_type": {
                    "type": "string",
                    "description": "Content type for secrets (e.g., application/json, text/plain)"
                }
            },
            "required": ["operation"]
        })
    }

    fn required_credential_type(&self) -> Option<&str> {
        Some("azure")
    }
}

#[async_trait]
impl Node for AzureKeyVaultNode {
    async fn execute(
        &self,
        context: &NodeContext,
        parameters: &serde_json::Value,
    ) -> Result<NodeOutput> {
        let params: AzureKeyVaultParams = serde_json::from_value(parameters.clone())?;

        // Validate required parameters
        self.validate_parameters(parameters)?;

        // TODO: Implement actual Azure Key Vault SDK calls
        // This is a placeholder implementation
        match params.operation.as_str() {
            "set_secret" => {
                let secret_name = params.secret_name.as_ref().unwrap();
                let secret_value = params.secret_value.as_ref().unwrap();
                let vault_url = params
                    .vault_url
                    .as_deref()
                    .unwrap_or("https://myvault.vault.azure.net/");

                Ok(NodeOutput::success(json!({
                    "operation": "set_secret",
                    "vault_url": vault_url,
                    "name": secret_name,
                    "id": format!("{}/secrets/{}", vault_url, secret_name),
                    "version": "abc123def456",
                    "enabled": params.enabled.unwrap_or(true),
                    "created_on": "2025-01-15T10:30:00Z",
                    "updated_on": "2025-01-15T10:30:00Z",
                    "content_type": params.content_type.as_deref().unwrap_or("text/plain"),
                    "tags": params.tags.clone().unwrap_or_else(|| json!({}))
                })))
            }
            "get_secret" => {
                let secret_name = params.secret_name.as_ref().unwrap();
                let vault_url = params
                    .vault_url
                    .as_deref()
                    .unwrap_or("https://myvault.vault.azure.net/");

                Ok(NodeOutput::success(json!({
                    "operation": "get_secret",
                    "vault_url": vault_url,
                    "name": secret_name,
                    "id": format!("{}/secrets/{}/abc123def456", vault_url, secret_name),
                    "value": "secret-value-placeholder",
                    "version": params.version.as_deref().unwrap_or("abc123def456"),
                    "enabled": true,
                    "created_on": "2025-01-15T10:30:00Z",
                    "updated_on": "2025-01-15T10:30:00Z",
                    "content_type": "text/plain",
                    "tags": {
                        "environment": "production",
                        "owner": "devops"
                    }
                })))
            }
            "delete_secret" => {
                let secret_name = params.secret_name.as_ref().unwrap();
                let vault_url = params
                    .vault_url
                    .as_deref()
                    .unwrap_or("https://myvault.vault.azure.net/");

                Ok(NodeOutput::success(json!({
                    "operation": "delete_secret",
                    "vault_url": vault_url,
                    "name": secret_name,
                    "id": format!("{}/secrets/{}", vault_url, secret_name),
                    "deleted_date": "2025-01-15T10:30:00Z",
                    "scheduled_purge_date": "2025-04-15T10:30:00Z",
                    "recovery_id": format!("{}/deletedsecrets/{}", vault_url, secret_name)
                })))
            }
            "list_secrets" => {
                let vault_url = params
                    .vault_url
                    .as_deref()
                    .unwrap_or("https://myvault.vault.azure.net/");

                Ok(NodeOutput::success(json!({
                    "operation": "list_secrets",
                    "vault_url": vault_url,
                    "secrets": [
                        {
                            "name": "database-password",
                            "id": format!("{}/secrets/database-password", vault_url),
                            "enabled": true,
                            "created_on": "2025-01-10T08:00:00Z",
                            "updated_on": "2025-01-14T15:30:00Z",
                            "tags": {
                                "environment": "production"
                            }
                        },
                        {
                            "name": "api-key",
                            "id": format!("{}/secrets/api-key", vault_url),
                            "enabled": true,
                            "created_on": "2025-01-12T10:00:00Z",
                            "updated_on": "2025-01-12T10:00:00Z",
                            "tags": {
                                "service": "external-api"
                            }
                        }
                    ],
                    "count": 2
                })))
            }
            "list_secret_versions" => {
                let secret_name = params.secret_name.as_ref().unwrap();
                let vault_url = params
                    .vault_url
                    .as_deref()
                    .unwrap_or("https://myvault.vault.azure.net/");

                Ok(NodeOutput::success(json!({
                    "operation": "list_secret_versions",
                    "vault_url": vault_url,
                    "name": secret_name,
                    "versions": [
                        {
                            "id": format!("{}/secrets/{}/abc123def456", vault_url, secret_name),
                            "version": "abc123def456",
                            "enabled": true,
                            "created_on": "2025-01-15T10:30:00Z",
                            "updated_on": "2025-01-15T10:30:00Z"
                        },
                        {
                            "id": format!("{}/secrets/{}/def456ghi789", vault_url, secret_name),
                            "version": "def456ghi789",
                            "enabled": false,
                            "created_on": "2025-01-10T08:00:00Z",
                            "updated_on": "2025-01-10T08:00:00Z"
                        }
                    ],
                    "count": 2
                })))
            }
            "create_key" => {
                let key_name = params.key_name.as_ref().unwrap();
                let vault_url = params
                    .vault_url
                    .as_deref()
                    .unwrap_or("https://myvault.vault.azure.net/");

                Ok(NodeOutput::success(json!({
                    "operation": "create_key",
                    "vault_url": vault_url,
                    "name": key_name,
                    "id": format!("{}/keys/{}", vault_url, key_name),
                    "key_type": "RSA",
                    "key_size": 2048,
                    "version": "xyz789abc123",
                    "enabled": params.enabled.unwrap_or(true),
                    "created_on": "2025-01-15T10:30:00Z",
                    "updated_on": "2025-01-15T10:30:00Z",
                    "key_operations": ["encrypt", "decrypt", "sign", "verify", "wrapKey", "unwrapKey"],
                    "tags": params.tags.clone().unwrap_or_else(|| json!({}))
                })))
            }
            "get_key" => {
                let key_name = params.key_name.as_ref().unwrap();
                let vault_url = params
                    .vault_url
                    .as_deref()
                    .unwrap_or("https://myvault.vault.azure.net/");

                Ok(NodeOutput::success(json!({
                    "operation": "get_key",
                    "vault_url": vault_url,
                    "name": key_name,
                    "id": format!("{}/keys/{}/xyz789abc123", vault_url, key_name),
                    "key_type": "RSA",
                    "key_size": 2048,
                    "version": params.version.as_deref().unwrap_or("xyz789abc123"),
                    "enabled": true,
                    "created_on": "2025-01-15T10:30:00Z",
                    "updated_on": "2025-01-15T10:30:00Z",
                    "key_operations": ["encrypt", "decrypt", "sign", "verify"],
                    "tags": {
                        "purpose": "encryption"
                    }
                })))
            }
            "delete_key" => {
                let key_name = params.key_name.as_ref().unwrap();
                let vault_url = params
                    .vault_url
                    .as_deref()
                    .unwrap_or("https://myvault.vault.azure.net/");

                Ok(NodeOutput::success(json!({
                    "operation": "delete_key",
                    "vault_url": vault_url,
                    "name": key_name,
                    "id": format!("{}/keys/{}", vault_url, key_name),
                    "deleted_date": "2025-01-15T10:30:00Z",
                    "scheduled_purge_date": "2025-04-15T10:30:00Z",
                    "recovery_id": format!("{}/deletedkeys/{}", vault_url, key_name)
                })))
            }
            "list_keys" => {
                let vault_url = params
                    .vault_url
                    .as_deref()
                    .unwrap_or("https://myvault.vault.azure.net/");

                Ok(NodeOutput::success(json!({
                    "operation": "list_keys",
                    "vault_url": vault_url,
                    "keys": [
                        {
                            "name": "encryption-key",
                            "id": format!("{}/keys/encryption-key", vault_url),
                            "key_type": "RSA",
                            "enabled": true,
                            "created_on": "2025-01-10T08:00:00Z",
                            "updated_on": "2025-01-10T08:00:00Z",
                            "tags": {
                                "purpose": "data-encryption"
                            }
                        },
                        {
                            "name": "signing-key",
                            "id": format!("{}/keys/signing-key", vault_url),
                            "key_type": "EC",
                            "enabled": true,
                            "created_on": "2025-01-12T10:00:00Z",
                            "updated_on": "2025-01-12T10:00:00Z",
                            "tags": {
                                "purpose": "digital-signature"
                            }
                        }
                    ],
                    "count": 2
                })))
            }
            "import_certificate" => {
                let certificate_name = params.certificate_name.as_ref().unwrap();
                let vault_url = params
                    .vault_url
                    .as_deref()
                    .unwrap_or("https://myvault.vault.azure.net/");

                Ok(NodeOutput::success(json!({
                    "operation": "import_certificate",
                    "vault_url": vault_url,
                    "name": certificate_name,
                    "id": format!("{}/certificates/{}", vault_url, certificate_name),
                    "version": "cert123abc456",
                    "enabled": params.enabled.unwrap_or(true),
                    "created_on": "2025-01-15T10:30:00Z",
                    "updated_on": "2025-01-15T10:30:00Z",
                    "thumbprint": "A1B2C3D4E5F6G7H8I9J0K1L2M3N4O5P6Q7R8S9T0",
                    "subject": "CN=example.com",
                    "issuer": "CN=DigiCert",
                    "expires_on": "2026-01-15T10:30:00Z",
                    "tags": params.tags.clone().unwrap_or_else(|| json!({}))
                })))
            }
            "get_certificate" => {
                let certificate_name = params.certificate_name.as_ref().unwrap();
                let vault_url = params
                    .vault_url
                    .as_deref()
                    .unwrap_or("https://myvault.vault.azure.net/");

                Ok(NodeOutput::success(json!({
                    "operation": "get_certificate",
                    "vault_url": vault_url,
                    "name": certificate_name,
                    "id": format!("{}/certificates/{}/cert123abc456", vault_url, certificate_name),
                    "version": params.version.as_deref().unwrap_or("cert123abc456"),
                    "enabled": true,
                    "created_on": "2025-01-15T10:30:00Z",
                    "updated_on": "2025-01-15T10:30:00Z",
                    "thumbprint": "A1B2C3D4E5F6G7H8I9J0K1L2M3N4O5P6Q7R8S9T0",
                    "subject": "CN=example.com",
                    "issuer": "CN=DigiCert",
                    "expires_on": "2026-01-15T10:30:00Z",
                    "not_before": "2025-01-15T10:30:00Z",
                    "certificate_type": "application/x-pkcs12",
                    "tags": {
                        "domain": "example.com",
                        "type": "ssl"
                    }
                })))
            }
            "delete_certificate" => {
                let certificate_name = params.certificate_name.as_ref().unwrap();
                let vault_url = params
                    .vault_url
                    .as_deref()
                    .unwrap_or("https://myvault.vault.azure.net/");

                Ok(NodeOutput::success(json!({
                    "operation": "delete_certificate",
                    "vault_url": vault_url,
                    "name": certificate_name,
                    "id": format!("{}/certificates/{}", vault_url, certificate_name),
                    "deleted_date": "2025-01-15T10:30:00Z",
                    "scheduled_purge_date": "2025-04-15T10:30:00Z",
                    "recovery_id": format!("{}/deletedcertificates/{}", vault_url, certificate_name)
                })))
            }
            "list_certificates" => {
                let vault_url = params
                    .vault_url
                    .as_deref()
                    .unwrap_or("https://myvault.vault.azure.net/");

                Ok(NodeOutput::success(json!({
                    "operation": "list_certificates",
                    "vault_url": vault_url,
                    "certificates": [
                        {
                            "name": "ssl-certificate",
                            "id": format!("{}/certificates/ssl-certificate", vault_url),
                            "enabled": true,
                            "created_on": "2025-01-10T08:00:00Z",
                            "updated_on": "2025-01-10T08:00:00Z",
                            "subject": "CN=example.com",
                            "thumbprint": "A1B2C3D4E5F6G7H8I9J0K1L2M3N4O5P6Q7R8S9T0",
                            "expires_on": "2026-01-10T08:00:00Z",
                            "tags": {
                                "domain": "example.com"
                            }
                        },
                        {
                            "name": "client-certificate",
                            "id": format!("{}/certificates/client-certificate", vault_url),
                            "enabled": true,
                            "created_on": "2025-01-12T10:00:00Z",
                            "updated_on": "2025-01-12T10:00:00Z",
                            "subject": "CN=client.example.com",
                            "thumbprint": "B2C3D4E5F6G7H8I9J0K1L2M3N4O5P6Q7R8S9T0U1",
                            "expires_on": "2026-01-12T10:00:00Z",
                            "tags": {
                                "type": "client-auth"
                            }
                        }
                    ],
                    "count": 2
                })))
            }
            _ => anyhow::bail!("Unsupported operation: {}", params.operation),
        }
    }

    fn validate_parameters(&self, parameters: &serde_json::Value) -> Result<()> {
        let params: AzureKeyVaultParams = serde_json::from_value(parameters.clone())?;

        // Operations that require secret_name
        let secret_name_required_ops = vec![
            "set_secret",
            "get_secret",
            "delete_secret",
            "list_secret_versions",
        ];

        if secret_name_required_ops.contains(&params.operation.as_str())
            && params.secret_name.is_none()
        {
            anyhow::bail!(
                "{} operation requires 'secret_name' parameter",
                params.operation
            );
        }

        // set_secret requires secret_value
        if params.operation == "set_secret" && params.secret_value.is_none() {
            anyhow::bail!("set_secret operation requires 'secret_value' parameter");
        }

        // Operations that require key_name
        let key_name_required_ops = vec!["create_key", "get_key", "delete_key"];

        if key_name_required_ops.contains(&params.operation.as_str()) && params.key_name.is_none() {
            anyhow::bail!(
                "{} operation requires 'key_name' parameter",
                params.operation
            );
        }

        // Operations that require certificate_name
        let certificate_name_required_ops = vec![
            "import_certificate",
            "get_certificate",
            "delete_certificate",
        ];

        if certificate_name_required_ops.contains(&params.operation.as_str())
            && params.certificate_name.is_none()
        {
            anyhow::bail!(
                "{} operation requires 'certificate_name' parameter",
                params.operation
            );
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_azure_key_vault_set_secret() {
        let node = AzureKeyVaultNode::new();
        let context = NodeContext::new(Uuid::new_v4().to_string(), "test-workflow".to_string());

        let params = json!({
            "operation": "set_secret",
            "vault_url": "https://myvault.vault.azure.net/",
            "secret_name": "database-password",
            "secret_value": "super-secret-password",
            "content_type": "text/plain",
            "enabled": true,
            "tags": {
                "environment": "production"
            }
        });

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
        assert_eq!(output.data["operation"], "set_secret");
        assert_eq!(output.data["name"], "database-password");
        assert!(output.data["version"].is_string());
    }

    #[tokio::test]
    async fn test_azure_key_vault_get_secret() {
        let node = AzureKeyVaultNode::new();
        let context = NodeContext::new(Uuid::new_v4().to_string(), "test-workflow".to_string());

        let params = json!({
            "operation": "get_secret",
            "vault_url": "https://myvault.vault.azure.net/",
            "secret_name": "database-password"
        });

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
        assert_eq!(output.data["operation"], "get_secret");
        assert!(output.data["value"].is_string());
        assert!(output.data["tags"].is_object());
    }

    #[tokio::test]
    async fn test_azure_key_vault_list_secrets() {
        let node = AzureKeyVaultNode::new();
        let context = NodeContext::new(Uuid::new_v4().to_string(), "test-workflow".to_string());

        let params = json!({
            "operation": "list_secrets",
            "vault_url": "https://myvault.vault.azure.net/"
        });

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
        assert!(output.data["secrets"].is_array());
        assert_eq!(output.data["count"], 2);
    }

    #[tokio::test]
    async fn test_azure_key_vault_create_key() {
        let node = AzureKeyVaultNode::new();
        let context = NodeContext::new(Uuid::new_v4().to_string(), "test-workflow".to_string());

        let params = json!({
            "operation": "create_key",
            "vault_url": "https://myvault.vault.azure.net/",
            "key_name": "encryption-key",
            "enabled": true,
            "tags": {
                "purpose": "data-encryption"
            }
        });

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
        assert_eq!(output.data["operation"], "create_key");
        assert_eq!(output.data["name"], "encryption-key");
        assert_eq!(output.data["key_type"], "RSA");
    }

    #[tokio::test]
    async fn test_azure_key_vault_import_certificate() {
        let node = AzureKeyVaultNode::new();
        let context = NodeContext::new(Uuid::new_v4().to_string(), "test-workflow".to_string());

        let params = json!({
            "operation": "import_certificate",
            "vault_url": "https://myvault.vault.azure.net/",
            "certificate_name": "ssl-certificate",
            "enabled": true,
            "tags": {
                "domain": "example.com"
            }
        });

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
        assert_eq!(output.data["operation"], "import_certificate");
        assert_eq!(output.data["name"], "ssl-certificate");
        assert!(output.data["thumbprint"].is_string());
    }

    #[tokio::test]
    async fn test_azure_key_vault_validation_missing_secret_name() {
        let node = AzureKeyVaultNode::new();
        let context = NodeContext::new(Uuid::new_v4().to_string(), "test-workflow".to_string());

        let params = json!({
            "operation": "get_secret",
            "vault_url": "https://myvault.vault.azure.net/"
        });

        let result = node.execute(&context, &params).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_azure_key_vault_validation_missing_secret_value() {
        let node = AzureKeyVaultNode::new();
        let context = NodeContext::new(Uuid::new_v4().to_string(), "test-workflow".to_string());

        let params = json!({
            "operation": "set_secret",
            "vault_url": "https://myvault.vault.azure.net/",
            "secret_name": "test-secret"
        });

        let result = node.execute(&context, &params).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_azure_key_vault_validation_missing_key_name() {
        let node = AzureKeyVaultNode::new();
        let context = NodeContext::new(Uuid::new_v4().to_string(), "test-workflow".to_string());

        let params = json!({
            "operation": "create_key",
            "vault_url": "https://myvault.vault.azure.net/"
        });

        let result = node.execute(&context, &params).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_azure_key_vault_validation_missing_certificate_name() {
        let node = AzureKeyVaultNode::new();
        let context = NodeContext::new(Uuid::new_v4().to_string(), "test-workflow".to_string());

        let params = json!({
            "operation": "import_certificate",
            "vault_url": "https://myvault.vault.azure.net/"
        });

        let result = node.execute(&context, &params).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_azure_key_vault_list_secret_versions() {
        let node = AzureKeyVaultNode::new();
        let context = NodeContext::new(Uuid::new_v4().to_string(), "test-workflow".to_string());

        let params = json!({
            "operation": "list_secret_versions",
            "vault_url": "https://myvault.vault.azure.net/",
            "secret_name": "database-password"
        });

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
        assert_eq!(output.data["operation"], "list_secret_versions");
        assert!(output.data["versions"].is_array());
        assert_eq!(output.data["count"], 2);
    }
}
