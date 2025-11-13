use crate::models::{Node, NodeCategory, NodeContext, NodeOutput, NodeSubcategory, NodeType};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;

/// DynamoDB node for AWS NoSQL database operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DynamoDBNode {
    #[serde(skip)]
    _private: (),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DynamoDBNodeParams {
    /// The operation to perform
    pub operation: String,

    // Table operations
    /// Table name
    pub table_name: Option<String>,

    // Item operations
    /// Item to put/update (DynamoDB item format)
    pub item: Option<serde_json::Value>,
    /// Key for get/delete operations
    pub key: Option<serde_json::Value>,

    // Query and Scan operations
    /// Key condition expression
    pub key_condition_expression: Option<String>,
    /// Filter expression
    pub filter_expression: Option<String>,
    /// Projection expression (attributes to retrieve)
    pub projection_expression: Option<String>,
    /// Expression attribute names
    pub expression_attribute_names: Option<HashMap<String, String>>,
    /// Expression attribute values
    pub expression_attribute_values: Option<HashMap<String, serde_json::Value>>,

    // Update operations
    /// Update expression
    pub update_expression: Option<String>,
    /// Condition expression
    pub condition_expression: Option<String>,

    // Query options
    /// Index name for query/scan
    pub index_name: Option<String>,
    /// Scan index forward (sort order)
    pub scan_index_forward: Option<bool>,
    /// Limit number of items
    pub limit: Option<i32>,
    /// Exclusive start key for pagination
    pub exclusive_start_key: Option<serde_json::Value>,
    /// Consistent read
    pub consistent_read: Option<bool>,

    // Batch operations
    /// Items for batch operations
    pub items: Option<Vec<serde_json::Value>>,
    /// Keys for batch get
    pub keys: Option<Vec<serde_json::Value>>,
    /// Request items for batch operations
    pub request_items: Option<serde_json::Value>,

    // Table management
    /// Key schema for table creation
    pub key_schema: Option<Vec<serde_json::Value>>,
    /// Attribute definitions
    pub attribute_definitions: Option<Vec<serde_json::Value>>,
    /// Provisioned throughput
    pub provisioned_throughput: Option<serde_json::Value>,
    /// Billing mode (PROVISIONED or PAY_PER_REQUEST)
    pub billing_mode: Option<String>,
    /// Global secondary indexes
    pub global_secondary_indexes: Option<Vec<serde_json::Value>>,
    /// Local secondary indexes
    pub local_secondary_indexes: Option<Vec<serde_json::Value>>,
    /// Stream specification
    pub stream_specification: Option<serde_json::Value>,

    // Transaction operations
    /// Transact items
    pub transact_items: Option<Vec<serde_json::Value>>,

    // Advanced options
    /// Return values (NONE, ALL_OLD, UPDATED_OLD, ALL_NEW, UPDATED_NEW)
    pub return_values: Option<String>,
    /// Return consumed capacity
    pub return_consumed_capacity: Option<String>,
    /// Return item collection metrics
    pub return_item_collection_metrics: Option<String>,
    /// Select (for query/scan: ALL_ATTRIBUTES, ALL_PROJECTED_ATTRIBUTES, SPECIFIC_ATTRIBUTES, COUNT)
    pub select: Option<String>,
}

impl DynamoDBNode {
    pub fn new() -> Self {
        Self { _private: () }
    }
}

impl Default for DynamoDBNode {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl NodeType for DynamoDBNode {
    fn type_name(&self) -> &str {
        "dynamodb"
    }

    fn category(&self) -> NodeCategory {
        NodeCategory::Action
    }

    fn subcategory(&self) -> NodeSubcategory {
        NodeSubcategory::Database
    }

    fn required_credential_type(&self) -> Option<&str> {
        Some("aws_credentials")
    }

    fn parameter_schema(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "required": ["operation"],
            "properties": {
                "operation": {
                    "type": "string",
                    "enum": [
                        "get_item",
                        "put_item",
                        "update_item",
                        "delete_item",
                        "query",
                        "scan",
                        "batch_get_item",
                        "batch_write_item",
                        "transact_write_items",
                        "transact_get_items",
                        "create_table",
                        "delete_table",
                        "describe_table",
                        "list_tables",
                        "update_table",
                        "create_backup",
                        "delete_backup",
                        "describe_backup",
                        "list_backups",
                        "restore_table_from_backup"
                    ],
                    "description": "The DynamoDB operation to perform"
                },
                "table_name": {
                    "type": "string",
                    "description": "Table name"
                },
                "item": {
                    "description": "Item to put/update (DynamoDB format)"
                },
                "key": {
                    "description": "Key for get/delete operations"
                },
                "key_condition_expression": {
                    "type": "string",
                    "description": "Key condition expression for query"
                },
                "filter_expression": {
                    "type": "string",
                    "description": "Filter expression"
                },
                "projection_expression": {
                    "type": "string",
                    "description": "Attributes to retrieve"
                },
                "expression_attribute_names": {
                    "type": "object",
                    "description": "Expression attribute name mappings"
                },
                "expression_attribute_values": {
                    "type": "object",
                    "description": "Expression attribute value mappings"
                },
                "update_expression": {
                    "type": "string",
                    "description": "Update expression"
                },
                "condition_expression": {
                    "type": "string",
                    "description": "Condition expression"
                },
                "index_name": {
                    "type": "string",
                    "description": "Index name for query/scan"
                },
                "scan_index_forward": {
                    "type": "boolean",
                    "description": "Sort order for query results"
                },
                "limit": {
                    "type": "integer",
                    "description": "Maximum number of items"
                },
                "exclusive_start_key": {
                    "description": "Exclusive start key for pagination"
                },
                "consistent_read": {
                    "type": "boolean",
                    "description": "Use consistent read"
                },
                "items": {
                    "type": "array",
                    "description": "Items for batch operations"
                },
                "keys": {
                    "type": "array",
                    "description": "Keys for batch get"
                },
                "request_items": {
                    "description": "Request items for batch operations"
                },
                "key_schema": {
                    "type": "array",
                    "description": "Key schema for table creation"
                },
                "attribute_definitions": {
                    "type": "array",
                    "description": "Attribute definitions"
                },
                "provisioned_throughput": {
                    "description": "Provisioned throughput settings"
                },
                "billing_mode": {
                    "type": "string",
                    "enum": ["PROVISIONED", "PAY_PER_REQUEST"],
                    "description": "Billing mode"
                },
                "global_secondary_indexes": {
                    "type": "array",
                    "description": "Global secondary indexes"
                },
                "local_secondary_indexes": {
                    "type": "array",
                    "description": "Local secondary indexes"
                },
                "stream_specification": {
                    "description": "Stream specification"
                },
                "transact_items": {
                    "type": "array",
                    "description": "Transaction items"
                },
                "return_values": {
                    "type": "string",
                    "enum": ["NONE", "ALL_OLD", "UPDATED_OLD", "ALL_NEW", "UPDATED_NEW"],
                    "description": "Return values option"
                },
                "return_consumed_capacity": {
                    "type": "string",
                    "enum": ["INDEXES", "TOTAL", "NONE"],
                    "description": "Return consumed capacity"
                },
                "return_item_collection_metrics": {
                    "type": "string",
                    "enum": ["SIZE", "NONE"],
                    "description": "Return item collection metrics"
                },
                "select": {
                    "type": "string",
                    "enum": ["ALL_ATTRIBUTES", "ALL_PROJECTED_ATTRIBUTES", "SPECIFIC_ATTRIBUTES", "COUNT"],
                    "description": "Attributes to select"
                }
            }
        })
    }
}

#[async_trait]
impl Node for DynamoDBNode {
    async fn execute(
        &self,
        context: &NodeContext,
        parameters: &serde_json::Value,
    ) -> anyhow::Result<NodeOutput> {
        let _params: DynamoDBNodeParams = serde_json::from_value(parameters.clone())?;

        // Mock implementation - in a real implementation, this would use AWS SDK to interact with DynamoDB
        let result = json!({
            "success": true,
            "message": "DynamoDB operation would be executed here",
            "execution_id": &context.execution_id
        });

        Ok(NodeOutput::success(result))
    }

    fn validate_parameters(&self, parameters: &serde_json::Value) -> anyhow::Result<()> {
        let params: DynamoDBNodeParams = serde_json::from_value(parameters.clone())?;

        // Validate based on operation
        match params.operation.as_str() {
            "get_item" => {
                if params.table_name.is_none() {
                    anyhow::bail!("get_item operation requires 'table_name' parameter");
                }
                if params.key.is_none() {
                    anyhow::bail!("get_item operation requires 'key' parameter");
                }
            }
            "put_item" => {
                if params.table_name.is_none() {
                    anyhow::bail!("put_item operation requires 'table_name' parameter");
                }
                if params.item.is_none() {
                    anyhow::bail!("put_item operation requires 'item' parameter");
                }
            }
            "update_item" => {
                if params.table_name.is_none() {
                    anyhow::bail!("update_item operation requires 'table_name' parameter");
                }
                if params.key.is_none() {
                    anyhow::bail!("update_item operation requires 'key' parameter");
                }
                if params.update_expression.is_none() {
                    anyhow::bail!("update_item operation requires 'update_expression' parameter");
                }
            }
            "delete_item" => {
                if params.table_name.is_none() {
                    anyhow::bail!("delete_item operation requires 'table_name' parameter");
                }
                if params.key.is_none() {
                    anyhow::bail!("delete_item operation requires 'key' parameter");
                }
            }
            "query" => {
                if params.table_name.is_none() {
                    anyhow::bail!("query operation requires 'table_name' parameter");
                }
                if params.key_condition_expression.is_none() {
                    anyhow::bail!("query operation requires 'key_condition_expression' parameter");
                }
            }
            "scan" => {
                if params.table_name.is_none() {
                    anyhow::bail!("scan operation requires 'table_name' parameter");
                }
            }
            "batch_get_item" => {
                if params.request_items.is_none()
                    && (params.table_name.is_none() || params.keys.is_none())
                {
                    anyhow::bail!(
                        "batch_get_item operation requires 'request_items' or both 'table_name' and 'keys' parameters"
                    );
                }
            }
            "batch_write_item" => {
                if params.request_items.is_none() {
                    anyhow::bail!("batch_write_item operation requires 'request_items' parameter");
                }
            }
            "transact_write_items" | "transact_get_items" => {
                if params.transact_items.is_none() {
                    anyhow::bail!(
                        "{} operation requires 'transact_items' parameter",
                        params.operation
                    );
                }
            }
            "create_table" => {
                if params.table_name.is_none() {
                    anyhow::bail!("create_table operation requires 'table_name' parameter");
                }
                if params.key_schema.is_none() {
                    anyhow::bail!("create_table operation requires 'key_schema' parameter");
                }
                if params.attribute_definitions.is_none() {
                    anyhow::bail!(
                        "create_table operation requires 'attribute_definitions' parameter"
                    );
                }
            }
            "delete_table" | "describe_table" => {
                if params.table_name.is_none() {
                    anyhow::bail!(
                        "{} operation requires 'table_name' parameter",
                        params.operation
                    );
                }
            }
            "update_table" => {
                if params.table_name.is_none() {
                    anyhow::bail!("update_table operation requires 'table_name' parameter");
                }
            }
            "create_backup" => {
                if params.table_name.is_none() {
                    anyhow::bail!("create_backup operation requires 'table_name' parameter");
                }
            }
            "list_tables" | "list_backups" => {
                // No required parameters
            }
            "delete_backup" | "describe_backup" | "restore_table_from_backup" => {
                // These would require backup-specific parameters
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
    fn test_dynamodb_node_creation() {
        let node = DynamoDBNode::new();
        assert_eq!(node.type_name(), "dynamodb");
        assert!(matches!(node.category(), NodeCategory::Action));
        assert!(matches!(node.subcategory(), NodeSubcategory::Database));
    }

    #[test]
    fn test_dynamodb_required_credential_type() {
        let node = DynamoDBNode::new();
        assert_eq!(node.required_credential_type(), Some("aws_credentials"));
    }

    #[test]
    fn test_get_item_validation() {
        let node = DynamoDBNode::new();

        // Valid get_item
        let valid_params = json!({
            "operation": "get_item",
            "table_name": "Users",
            "key": {"userId": {"S": "123"}}
        });
        assert!(node.validate_parameters(&valid_params).is_ok());

        // Missing key
        let invalid_params = json!({
            "operation": "get_item",
            "table_name": "Users"
        });
        assert!(node.validate_parameters(&invalid_params).is_err());
    }

    #[test]
    fn test_put_item_validation() {
        let node = DynamoDBNode::new();

        // Valid put_item
        let valid_params = json!({
            "operation": "put_item",
            "table_name": "Users",
            "item": {"userId": {"S": "123"}, "name": {"S": "John"}}
        });
        assert!(node.validate_parameters(&valid_params).is_ok());
    }

    #[test]
    fn test_query_validation() {
        let node = DynamoDBNode::new();

        // Valid query
        let valid_params = json!({
            "operation": "query",
            "table_name": "Users",
            "key_condition_expression": "userId = :id",
            "expression_attribute_values": {":id": {"S": "123"}}
        });
        assert!(node.validate_parameters(&valid_params).is_ok());

        // Missing key_condition_expression
        let invalid_params = json!({
            "operation": "query",
            "table_name": "Users"
        });
        assert!(node.validate_parameters(&invalid_params).is_err());
    }

    #[test]
    fn test_update_item_validation() {
        let node = DynamoDBNode::new();

        // Valid update
        let valid_params = json!({
            "operation": "update_item",
            "table_name": "Users",
            "key": {"userId": {"S": "123"}},
            "update_expression": "SET #n = :name",
            "expression_attribute_names": {"#n": "name"},
            "expression_attribute_values": {":name": {"S": "Jane"}}
        });
        assert!(node.validate_parameters(&valid_params).is_ok());
    }

    #[test]
    fn test_create_table_validation() {
        let node = DynamoDBNode::new();

        // Valid create_table
        let valid_params = json!({
            "operation": "create_table",
            "table_name": "Users",
            "key_schema": [{"AttributeName": "userId", "KeyType": "HASH"}],
            "attribute_definitions": [{"AttributeName": "userId", "AttributeType": "S"}],
            "billing_mode": "PAY_PER_REQUEST"
        });
        assert!(node.validate_parameters(&valid_params).is_ok());
    }

    #[tokio::test]
    async fn test_dynamodb_execute() {
        let node = DynamoDBNode::new();
        let params = json!({
            "operation": "scan",
            "table_name": "Users"
        });
        let context = NodeContext::new("test-exec".to_string(), "test-node".to_string());

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
    }
}
