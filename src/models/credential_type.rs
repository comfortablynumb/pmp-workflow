use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// CredentialType defines the schema and requirements for a type of credential
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CredentialType {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    /// JSON Schema that defines the structure of the credential data
    pub json_schema: serde_json::Value,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl CredentialType {
    /// Create a new CredentialType
    pub fn new(name: String, description: Option<String>, json_schema: serde_json::Value) -> Self {
        let now = chrono::Utc::now();
        Self {
            id: Uuid::new_v4(),
            name,
            description,
            json_schema,
            created_at: now,
            updated_at: now,
        }
    }

    /// Validate that credential data conforms to this type's schema
    pub fn validate_credential_data(&self, data: &serde_json::Value) -> anyhow::Result<()> {
        // TODO: In a real implementation, use a JSON Schema validator crate
        // like `jsonschema` to validate the data against self.json_schema
        // For now, we'll do basic validation

        if !data.is_object() {
            anyhow::bail!("Credential data must be a JSON object");
        }

        // Basic validation: check that all required fields are present
        if let Some(required) = self.json_schema.get("required")
            && let Some(required_fields) = required.as_array()
        {
            for field in required_fields {
                if let Some(field_name) = field.as_str()
                    && data.get(field_name).is_none()
                {
                    anyhow::bail!("Missing required field: {}", field_name);
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_credential_type_new() {
        let schema = serde_json::json!({
            "type": "object",
            "properties": {
                "api_key": {
                    "type": "string",
                    "description": "API key for authentication"
                }
            },
            "required": ["api_key"]
        });

        let cred_type = CredentialType::new(
            "github_token".to_string(),
            Some("GitHub Personal Access Token".to_string()),
            schema.clone(),
        );

        assert_eq!(cred_type.name, "github_token");
        assert_eq!(cred_type.description, Some("GitHub Personal Access Token".to_string()));
        assert_eq!(cred_type.json_schema, schema);
    }

    #[test]
    fn test_validate_credential_data_valid() {
        let schema = serde_json::json!({
            "type": "object",
            "properties": {
                "api_key": {
                    "type": "string"
                }
            },
            "required": ["api_key"]
        });

        let cred_type = CredentialType::new(
            "test_type".to_string(),
            None,
            schema,
        );

        let valid_data = serde_json::json!({
            "api_key": "ghp_1234567890"
        });

        assert!(cred_type.validate_credential_data(&valid_data).is_ok());
    }

    #[test]
    fn test_validate_credential_data_missing_required() {
        let schema = serde_json::json!({
            "type": "object",
            "properties": {
                "api_key": {
                    "type": "string"
                }
            },
            "required": ["api_key"]
        });

        let cred_type = CredentialType::new(
            "test_type".to_string(),
            None,
            schema,
        );

        let invalid_data = serde_json::json!({
            "other_field": "value"
        });

        assert!(cred_type.validate_credential_data(&invalid_data).is_err());
    }

    #[test]
    fn test_validate_credential_data_not_object() {
        let schema = serde_json::json!({
            "type": "object"
        });

        let cred_type = CredentialType::new(
            "test_type".to_string(),
            None,
            schema,
        );

        let invalid_data = serde_json::json!("not an object");

        assert!(cred_type.validate_credential_data(&invalid_data).is_err());
    }
}
