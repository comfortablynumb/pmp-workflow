use crate::models::{Node, NodeCategory, NodeContext, NodeOutput, NodeSubcategory, NodeType};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::json;

/// MySQL database node for executing queries and managing connections
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MySQLNode {
    #[serde(skip)]
    _private: (),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MySQLNodeParams {
    /// The operation to perform
    pub operation: String,

    // Query operations
    /// SQL query to execute
    pub query: Option<String>,
    /// Query parameters (for prepared statements)
    pub parameters: Option<Vec<serde_json::Value>>,

    // Table operations
    /// Table name
    pub table: Option<String>,
    /// Database name
    pub database: Option<String>,

    // Insert/Update operations
    /// Data to insert/update (JSON object or array of objects)
    pub data: Option<serde_json::Value>,
    /// Columns for insert (if not using data object)
    pub columns: Option<Vec<String>>,
    /// Values for insert (if not using data object)
    pub values: Option<Vec<Vec<serde_json::Value>>>,

    // Update/Delete conditions
    /// WHERE clause conditions
    pub where_clause: Option<String>,
    /// WHERE parameters
    pub where_params: Option<Vec<serde_json::Value>>,

    // Transaction operations
    /// Transaction ID (for multi-statement transactions)
    pub transaction_id: Option<String>,

    // Advanced options
    /// Fetch size/limit
    pub limit: Option<i32>,
    /// Offset for pagination
    pub offset: Option<i32>,
    /// Return affected rows count
    pub return_count: Option<bool>,
    /// Timeout in seconds
    pub timeout: Option<i32>,
}

impl MySQLNode {
    pub fn new() -> Self {
        Self { _private: () }
    }
}

impl Default for MySQLNode {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl NodeType for MySQLNode {
    fn type_name(&self) -> &str {
        "mysql"
    }

    fn category(&self) -> NodeCategory {
        NodeCategory::Action
    }

    fn subcategory(&self) -> NodeSubcategory {
        NodeSubcategory::Database
    }

    fn required_credential_type(&self) -> Option<&str> {
        Some("mysql_connection")
    }

    fn parameter_schema(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "required": ["operation"],
            "properties": {
                "operation": {
                    "type": "string",
                    "enum": [
                        "execute_query",
                        "select",
                        "insert",
                        "update",
                        "delete",
                        "create_table",
                        "drop_table",
                        "alter_table",
                        "truncate_table",
                        "create_database",
                        "drop_database",
                        "list_databases",
                        "list_tables",
                        "describe_table",
                        "begin_transaction",
                        "commit",
                        "rollback",
                        "execute_stored_procedure",
                        "create_index",
                        "drop_index"
                    ],
                    "description": "The database operation to perform"
                },
                "query": {
                    "type": "string",
                    "description": "SQL query to execute"
                },
                "parameters": {
                    "type": "array",
                    "description": "Query parameters for prepared statements"
                },
                "table": {
                    "type": "string",
                    "description": "Table name"
                },
                "database": {
                    "type": "string",
                    "description": "Database name"
                },
                "data": {
                    "description": "Data to insert/update (object or array of objects)"
                },
                "columns": {
                    "type": "array",
                    "items": {"type": "string"},
                    "description": "Column names for insert"
                },
                "values": {
                    "type": "array",
                    "description": "Values for insert operation"
                },
                "where_clause": {
                    "type": "string",
                    "description": "WHERE clause for update/delete"
                },
                "where_params": {
                    "type": "array",
                    "description": "Parameters for WHERE clause"
                },
                "transaction_id": {
                    "type": "string",
                    "description": "Transaction ID for multi-statement transactions"
                },
                "limit": {
                    "type": "integer",
                    "description": "Limit number of results"
                },
                "offset": {
                    "type": "integer",
                    "description": "Offset for pagination"
                },
                "return_count": {
                    "type": "boolean",
                    "description": "Return affected rows count"
                },
                "timeout": {
                    "type": "integer",
                    "description": "Query timeout in seconds"
                }
            }
        })
    }

}

#[async_trait]
impl Node for MySQLNode {
    async fn execute(
        &self,
        context: &NodeContext,
        parameters: &serde_json::Value,
    ) -> anyhow::Result<NodeOutput> {
        let _params: MySQLNodeParams = serde_json::from_value(parameters.clone())?;

        // Mock implementation - in a real implementation, this would connect to MySQL and execute queries
        let result = json!({
            "success": true,
            "message": "MySQL operation would be executed here",
            "execution_id": &context.execution_id,
            "rows_affected": 0
        });

        Ok(NodeOutput::success(result))
    }

    fn validate_parameters(&self, parameters: &serde_json::Value) -> anyhow::Result<()> {
        let params: MySQLNodeParams = serde_json::from_value(parameters.clone())?;

        // Validate based on operation
        match params.operation.as_str() {
            "execute_query" => {
                if params.query.is_none() {
                    anyhow::bail!("execute_query operation requires 'query' parameter");
                }
            }
            "select" => {
                if params.query.is_none() && params.table.is_none() {
                    anyhow::bail!("select operation requires 'query' or 'table' parameter");
                }
            }
            "insert" => {
                if params.table.is_none() {
                    anyhow::bail!("insert operation requires 'table' parameter");
                }
                if params.data.is_none() && (params.columns.is_none() || params.values.is_none()) {
                    anyhow::bail!("insert operation requires 'data' or both 'columns' and 'values' parameters");
                }
            }
            "update" => {
                if params.table.is_none() {
                    anyhow::bail!("update operation requires 'table' parameter");
                }
                if params.data.is_none() && params.query.is_none() {
                    anyhow::bail!("update operation requires 'data' or 'query' parameter");
                }
            }
            "delete" => {
                if params.table.is_none() {
                    anyhow::bail!("delete operation requires 'table' parameter");
                }
            }
            "create_table" | "drop_table" | "alter_table" | "truncate_table" | "describe_table" => {
                if params.table.is_none() && params.query.is_none() {
                    anyhow::bail!("{} operation requires 'table' or 'query' parameter", params.operation);
                }
            }
            "create_database" | "drop_database" => {
                if params.database.is_none() && params.query.is_none() {
                    anyhow::bail!("{} operation requires 'database' or 'query' parameter", params.operation);
                }
            }
            "execute_stored_procedure" => {
                if params.query.is_none() {
                    anyhow::bail!("execute_stored_procedure operation requires 'query' parameter (procedure name)");
                }
            }
            "create_index" | "drop_index" => {
                if params.query.is_none() {
                    anyhow::bail!("{} operation requires 'query' parameter", params.operation);
                }
            }
            "list_databases" | "list_tables" | "begin_transaction" | "commit" | "rollback" => {
                // No required parameters for these operations
            }
            _ => {
                anyhow::bail!("Unknown operation: {}", params.operation);
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mysql_node_creation() {
        let node = MySQLNode::new();
        assert_eq!(node.type_name(), "mysql");
        assert!(matches!(node.category(), NodeCategory::Action));
        assert!(matches!(node.subcategory(), NodeSubcategory::Database));
    }

    #[test]
    fn test_mysql_required_credential_type() {
        let node = MySQLNode::new();
        assert_eq!(node.required_credential_type(), Some("mysql_connection"));
    }

    #[test]
    fn test_execute_query_validation() {
        let node = MySQLNode::new();

        // Valid query
        let valid_params = json!({
            "operation": "execute_query",
            "query": "SELECT * FROM users WHERE id = ?"
        });
        assert!(node.validate_parameters(&valid_params).is_ok());

        // Missing query
        let invalid_params = json!({
            "operation": "execute_query"
        });
        assert!(node.validate_parameters(&invalid_params).is_err());
    }

    #[test]
    fn test_insert_validation() {
        let node = MySQLNode::new();

        // Valid insert with data
        let valid_params = json!({
            "operation": "insert",
            "table": "users",
            "data": {"name": "John", "email": "john@example.com"}
        });
        assert!(node.validate_parameters(&valid_params).is_ok());

        // Valid insert with columns and values
        let valid_params2 = json!({
            "operation": "insert",
            "table": "users",
            "columns": ["name", "email"],
            "values": [["John", "john@example.com"]]
        });
        assert!(node.validate_parameters(&valid_params2).is_ok());

        // Missing table
        let invalid_params = json!({
            "operation": "insert",
            "data": {"name": "John"}
        });
        assert!(node.validate_parameters(&invalid_params).is_err());
    }

    #[test]
    fn test_update_validation() {
        let node = MySQLNode::new();

        // Valid update
        let valid_params = json!({
            "operation": "update",
            "table": "users",
            "data": {"name": "Jane"},
            "where_clause": "id = ?"
        });
        assert!(node.validate_parameters(&valid_params).is_ok());
    }

    #[test]
    fn test_delete_validation() {
        let node = MySQLNode::new();

        // Valid delete
        let valid_params = json!({
            "operation": "delete",
            "table": "users",
            "where_clause": "id = ?"
        });
        assert!(node.validate_parameters(&valid_params).is_ok());
    }

    #[tokio::test]
    async fn test_mysql_execute() {
        let node = MySQLNode::new();
        let params = json!({
            "operation": "select",
            "table": "users",
            "limit": 10
        });
        let context = NodeContext::new("test-exec".to_string(), "test-node".to_string());

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
    }
}
