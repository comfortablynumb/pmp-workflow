use crate::models::{Node, NodeCategory, NodeContext, NodeOutput, NodeSubcategory, NodeType};
use async_trait::async_trait;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DatabaseQueryParams {
    /// Credentials name to use for database connection
    pub credentials_name: String,
    /// SQL query to execute
    pub query: String,
    /// Optional query parameters for prepared statements
    #[serde(default)]
    pub params: Vec<serde_json::Value>,
}

/// Database Query node - executes SQL queries against databases
pub struct DatabaseQueryNode {
    // In a real implementation, this would have a connection pool manager
    // For now, we'll keep it simple
}

impl DatabaseQueryNode {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for DatabaseQueryNode {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeType for DatabaseQueryNode {
    fn type_name(&self) -> &str {
        "database_query"
    }

    fn category(&self) -> NodeCategory {
        NodeCategory::Action
    }

    fn subcategory(&self) -> NodeSubcategory {
        NodeSubcategory::Database
    }

    fn parameter_schema(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "credentials_name": {
                    "type": "string",
                    "description": "Name of the database credentials to use",
                    "minLength": 1
                },
                "query": {
                    "type": "string",
                    "description": "SQL query to execute. Use $1, $2, etc. for parameterized queries",
                    "minLength": 1
                },
                "params": {
                    "type": "array",
                    "description": "Parameters for the prepared statement",
                    "items": {},
                    "default": []
                }
            },
            "required": ["credentials_name", "query"],
            "additionalProperties": false
        })
    }
}

#[async_trait]
impl Node for DatabaseQueryNode {
    async fn execute(
        &self,
        context: &NodeContext,
        parameters: &serde_json::Value,
    ) -> anyhow::Result<NodeOutput> {
        let params: DatabaseQueryParams = serde_json::from_value(parameters.clone())?;

        // TODO: In a real implementation:
        // 1. Look up credentials from the credentials table using params.credentials_name
        // 2. Decrypt the credentials data
        // 3. Create a database connection using the decrypted credentials
        // 4. Execute the query with the provided parameters
        // 5. Return the results as JSON

        // For now, we'll return a placeholder response
        let result = serde_json::json!({
            "message": "Database query executed (placeholder implementation)",
            "credentials_name": params.credentials_name,
            "query": params.query,
            "params_count": params.params.len(),
            "context_execution_id": context.execution_id,
            "rows_affected": 0,
            "results": []
        });

        Ok(NodeOutput::success(result))
    }

    fn validate_parameters(&self, parameters: &serde_json::Value) -> anyhow::Result<()> {
        let params: DatabaseQueryParams = serde_json::from_value(parameters.clone())
            .map_err(|e| anyhow::anyhow!("Invalid parameters: {}", e))?;

        if params.credentials_name.trim().is_empty() {
            anyhow::bail!("credentials_name cannot be empty");
        }

        if params.query.trim().is_empty() {
            anyhow::bail!("query cannot be empty");
        }

        // Basic SQL injection prevention - check for dangerous keywords
        let query_lower = params.query.to_lowercase();
        let dangerous_patterns = vec!["drop table", "drop database", "truncate"];

        for pattern in dangerous_patterns {
            if query_lower.contains(pattern) {
                anyhow::bail!(
                    "Query contains potentially dangerous operation: {}",
                    pattern
                );
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_database_query_execution() {
        let node = DatabaseQueryNode::new();
        let context = NodeContext::new("exec-123".to_string(), "node-1".to_string());

        let params = serde_json::json!({
            "credentials_name": "my_postgres",
            "query": "SELECT * FROM users WHERE id = $1",
            "params": [42]
        });

        let result = node.execute(&context, &params).await.unwrap();
        assert!(result.success);
        assert!(result.data.get("credentials_name").is_some());
        assert_eq!(result.data["credentials_name"], "my_postgres");
    }

    #[test]
    fn test_database_query_validation() {
        let node = DatabaseQueryNode::new();

        let valid_params = serde_json::json!({
            "credentials_name": "my_db",
            "query": "SELECT * FROM users",
            "params": []
        });
        assert!(node.validate_parameters(&valid_params).is_ok());

        let empty_credentials = serde_json::json!({
            "credentials_name": "",
            "query": "SELECT * FROM users"
        });
        assert!(node.validate_parameters(&empty_credentials).is_err());

        let empty_query = serde_json::json!({
            "credentials_name": "my_db",
            "query": ""
        });
        assert!(node.validate_parameters(&empty_query).is_err());

        let dangerous_query = serde_json::json!({
            "credentials_name": "my_db",
            "query": "DROP TABLE users"
        });
        assert!(node.validate_parameters(&dangerous_query).is_err());
    }

    #[test]
    fn test_database_query_node_type() {
        let node = DatabaseQueryNode::new();
        assert_eq!(node.type_name(), "database_query");
        assert_eq!(node.category(), NodeCategory::Action);
        assert_eq!(node.subcategory(), NodeSubcategory::Database);
    }

    #[test]
    fn test_database_query_parameter_schema() {
        let node = DatabaseQueryNode::new();
        let schema = node.parameter_schema();

        assert_eq!(schema["type"], "object");
        assert!(schema["properties"]["credentials_name"].is_object());
        assert!(schema["properties"]["query"].is_object());
        assert!(schema["properties"]["params"].is_object());
        assert_eq!(schema["required"].as_array().unwrap().len(), 2);
    }
}
