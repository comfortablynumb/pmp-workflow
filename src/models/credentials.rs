use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Type of credentials
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "credentials_type", rename_all = "lowercase")]
pub enum CredentialsType {
    /// Database connection credentials (PostgreSQL, MySQL, etc.)
    Database,
    /// API key for third-party services
    ApiKey,
    /// OAuth 2.0 credentials
    OAuth,
    /// Basic authentication (username/password)
    BasicAuth,
    /// Custom credentials
    Custom,
}

/// Credentials entity for storing sensitive information
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Credentials {
    /// Unique identifier
    pub id: Uuid,
    /// Human-readable name
    pub name: String,
    /// Type of credentials
    #[sqlx(try_from = "String")]
    pub credentials_type: CredentialsType,
    /// Encrypted credentials data (JSON)
    /// Should contain connection string, API keys, tokens, etc.
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
        credentials_type: CredentialsType,
        encrypted_data: Vec<u8>,
        description: Option<String>,
    ) -> Self {
        let now = chrono::Utc::now();
        Self {
            id: Uuid::new_v4(),
            name,
            credentials_type,
            encrypted_data,
            description,
            created_at: now,
            updated_at: now,
        }
    }
}

// Helper for sqlx type conversion
impl TryFrom<String> for CredentialsType {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "database" => Ok(CredentialsType::Database),
            "apikey" => Ok(CredentialsType::ApiKey),
            "oauth" => Ok(CredentialsType::OAuth),
            "basicauth" => Ok(CredentialsType::BasicAuth),
            "custom" => Ok(CredentialsType::Custom),
            _ => Err(format!("Unknown credentials type: {}", value)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_credentials_new() {
        let creds = Credentials::new(
            "My Database".to_string(),
            CredentialsType::Database,
            vec![1, 2, 3, 4],
            Some("Test database credentials".to_string()),
        );

        assert_eq!(creds.name, "My Database");
        assert_eq!(creds.credentials_type, CredentialsType::Database);
        assert_eq!(creds.encrypted_data, vec![1, 2, 3, 4]);
        assert_eq!(
            creds.description,
            Some("Test database credentials".to_string())
        );
    }

    #[test]
    fn test_credentials_type_conversion() {
        assert_eq!(
            CredentialsType::try_from("database".to_string()).unwrap(),
            CredentialsType::Database
        );
        assert_eq!(
            CredentialsType::try_from("APIKEY".to_string()).unwrap(),
            CredentialsType::ApiKey
        );
        assert_eq!(
            CredentialsType::try_from("OAuth".to_string()).unwrap(),
            CredentialsType::OAuth
        );
        assert_eq!(
            CredentialsType::try_from("basicauth".to_string()).unwrap(),
            CredentialsType::BasicAuth
        );
        assert_eq!(
            CredentialsType::try_from("custom".to_string()).unwrap(),
            CredentialsType::Custom
        );

        assert!(CredentialsType::try_from("unknown".to_string()).is_err());
    }

    #[test]
    fn test_credentials_type_serialization() {
        let db_type = CredentialsType::Database;
        let json = serde_json::to_string(&db_type).unwrap();
        assert_eq!(json, "\"Database\"");

        let deserialized: CredentialsType = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, CredentialsType::Database);
    }
}
