use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Credentials entity for storing sensitive information
/// Now references a CredentialType which defines the schema
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Credentials {
    /// Unique identifier
    pub id: Uuid,
    /// Human-readable name
    pub name: String,
    /// Foreign key to credential_types table
    pub credential_type_id: Uuid,
    /// Encrypted credentials data (JSON)
    /// Should conform to the credential_type's json_schema
    pub encrypted_data: Vec<u8>,
    /// Optional description
    pub description: Option<String>,
    /// When the credentials were created
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// When the credentials were last updated
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl Credentials {
    /// Create new credentials
    pub fn new(
        name: String,
        credential_type_id: Uuid,
        encrypted_data: Vec<u8>,
        description: Option<String>,
    ) -> Self {
        let now = chrono::Utc::now();
        Self {
            id: Uuid::new_v4(),
            name,
            credential_type_id,
            encrypted_data,
            description,
            created_at: now,
            updated_at: now,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_credentials_new() {
        let credential_type_id = Uuid::new_v4();
        let creds = Credentials::new(
            "My Database".to_string(),
            credential_type_id,
            vec![1, 2, 3, 4],
            Some("Test database credentials".to_string()),
        );

        assert_eq!(creds.name, "My Database");
        assert_eq!(creds.credential_type_id, credential_type_id);
        assert_eq!(creds.encrypted_data, vec![1, 2, 3, 4]);
        assert_eq!(
            creds.description,
            Some("Test database credentials".to_string())
        );
    }
}
