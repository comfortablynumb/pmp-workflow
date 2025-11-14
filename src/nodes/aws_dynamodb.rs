use crate::models::{Node, NodeCategory, NodeContext, NodeOutput, NodeSubcategory, NodeType};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

/// AWS DynamoDB advanced operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AwsDynamoDBParams {
    /// Operation to perform
    pub operation: String,
    /// Table name
    pub table_name: Option<String>,
    /// Key for get/delete/update operations
    pub key: Option<Value>,
    /// Item to put (DynamoDB format)
    pub item: Option<Value>,
    /// Attributes to get/update
    pub attributes: Option<Value>,
    /// Condition expression
    pub condition_expression: Option<String>,
    /// Update expression
    pub update_expression: Option<String>,
    /// Filter expression
    pub filter_expression: Option<String>,
    /// Projection expression (attributes to retrieve)
    pub projection_expression: Option<String>,
    /// Expression attribute names
    pub expression_attribute_names: Option<Value>,
    /// Expression attribute values
    pub expression_attribute_values: Option<Value>,
    /// Index name for query/scan
    pub index_name: Option<String>,
    /// Limit number of items
    pub limit: Option<u32>,
    /// Scan index forward (sort order)
    pub scan_forward: Option<bool>,
    /// Consistent read
    pub consistent_read: Option<bool>,
}

/// AWS DynamoDB node for NoSQL database operations
pub struct AwsDynamoDBNode;

impl AwsDynamoDBNode {
    pub fn new() -> Self {
        Self
    }
}

impl Default for AwsDynamoDBNode {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeType for AwsDynamoDBNode {
    fn type_name(&self) -> &str {
        "aws_dynamodb"
    }

    fn category(&self) -> NodeCategory {
        NodeCategory::Action
    }

    fn subcategory(&self) -> NodeSubcategory {
        NodeSubcategory::Database
    }

    fn parameter_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "operation": {
                    "type": "string",
                    "enum": [
                        "put_item",
                        "get_item",
                        "update_item",
                        "delete_item",
                        "query",
                        "scan",
                        "batch_get_item",
                        "batch_write_item",
                        "create_table",
                        "delete_table",
                        "describe_table",
                        "list_tables"
                    ],
                    "description": "DynamoDB operation to perform"
                },
                "table_name": {
                    "type": "string",
                    "description": "DynamoDB table name"
                },
                "key": {
                    "type": "object",
                    "description": "Primary key for item operations (DynamoDB format)"
                },
                "item": {
                    "type": "object",
                    "description": "Item to put (DynamoDB format)"
                },
                "attributes": {
                    "type": "object",
                    "description": "Attributes for operations"
                },
                "condition_expression": {
                    "type": "string",
                    "description": "Condition expression for conditional operations"
                },
                "update_expression": {
                    "type": "string",
                    "description": "Update expression for update operations"
                },
                "filter_expression": {
                    "type": "string",
                    "description": "Filter expression for query/scan"
                },
                "projection_expression": {
                    "type": "string",
                    "description": "Projection expression for attributes to retrieve"
                },
                "expression_attribute_names": {
                    "type": "object",
                    "description": "Expression attribute name mappings"
                },
                "expression_attribute_values": {
                    "type": "object",
                    "description": "Expression attribute value mappings"
                },
                "index_name": {
                    "type": "string",
                    "description": "Index name for query/scan operations"
                },
                "limit": {
                    "type": "integer",
                    "description": "Maximum number of items to return"
                },
                "scan_forward": {
                    "type": "boolean",
                    "description": "Sort order for query results (default: true)"
                },
                "consistent_read": {
                    "type": "boolean",
                    "description": "Use strongly consistent read (default: false)"
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
impl Node for AwsDynamoDBNode {
    async fn execute(
        &self,
        context: &NodeContext,
        parameters: &Value,
    ) -> anyhow::Result<NodeOutput> {
        let params: AwsDynamoDBParams = serde_json::from_value(parameters.clone())?;

        // Validate parameters
        self.validate_parameters(parameters)?;

        // TODO: Implement actual AWS DynamoDB SDK calls
        // This is a placeholder implementation
        match params.operation.as_str() {
            "put_item" => {
                let table_name = params.table_name.as_ref().unwrap();
                let item = params.item.as_ref().unwrap();

                Ok(NodeOutput::success(json!({
                    "success": true,
                    "operation": "put_item",
                    "table_name": table_name,
                    "item": item,
                    "consumed_capacity": {
                        "table_name": table_name,
                        "capacity_units": 1.0
                    },
                    "execution_id": context.execution_id
                })))
            }
            "get_item" => {
                let table_name = params.table_name.as_ref().unwrap();
                let key = params.key.as_ref().unwrap();

                Ok(NodeOutput::success(json!({
                    "success": true,
                    "operation": "get_item",
                    "table_name": table_name,
                    "key": key,
                    "item": {
                        "id": {"S": "12345"},
                        "name": {"S": "Sample Item"},
                        "created_at": {"N": "1640000000"}
                    },
                    "consumed_capacity": {
                        "table_name": table_name,
                        "capacity_units": 0.5
                    }
                })))
            }
            "update_item" => {
                let table_name = params.table_name.as_ref().unwrap();
                let key = params.key.as_ref().unwrap();
                let update_expression = params.update_expression.as_ref().unwrap();

                Ok(NodeOutput::success(json!({
                    "success": true,
                    "operation": "update_item",
                    "table_name": table_name,
                    "key": key,
                    "update_expression": update_expression,
                    "attributes": {
                        "updated_at": {"N": "1640000000"}
                    },
                    "consumed_capacity": {
                        "table_name": table_name,
                        "capacity_units": 1.0
                    }
                })))
            }
            "delete_item" => {
                let table_name = params.table_name.as_ref().unwrap();
                let key = params.key.as_ref().unwrap();

                Ok(NodeOutput::success(json!({
                    "success": true,
                    "operation": "delete_item",
                    "table_name": table_name,
                    "key": key,
                    "deleted": true,
                    "consumed_capacity": {
                        "table_name": table_name,
                        "capacity_units": 1.0
                    }
                })))
            }
            "query" => {
                let table_name = params.table_name.as_ref().unwrap();
                let default_values = json!({});
                let key_condition = params
                    .expression_attribute_values
                    .as_ref()
                    .unwrap_or(&default_values);

                Ok(NodeOutput::success(json!({
                    "success": true,
                    "operation": "query",
                    "table_name": table_name,
                    "key_condition": key_condition,
                    "items": [
                        {
                            "id": {"S": "1"},
                            "name": {"S": "Item 1"},
                            "value": {"N": "100"}
                        },
                        {
                            "id": {"S": "2"},
                            "name": {"S": "Item 2"},
                            "value": {"N": "200"}
                        }
                    ],
                    "count": 2,
                    "scanned_count": 2,
                    "consumed_capacity": {
                        "table_name": table_name,
                        "capacity_units": 1.0
                    }
                })))
            }
            "scan" => {
                let table_name = params.table_name.as_ref().unwrap();
                let limit = params.limit.unwrap_or(100);

                Ok(NodeOutput::success(json!({
                    "success": true,
                    "operation": "scan",
                    "table_name": table_name,
                    "limit": limit,
                    "items": [
                        {
                            "id": {"S": "1"},
                            "name": {"S": "Item 1"}
                        },
                        {
                            "id": {"S": "2"},
                            "name": {"S": "Item 2"}
                        },
                        {
                            "id": {"S": "3"},
                            "name": {"S": "Item 3"}
                        }
                    ],
                    "count": 3,
                    "scanned_count": 3,
                    "consumed_capacity": {
                        "table_name": table_name,
                        "capacity_units": 1.5
                    }
                })))
            }
            "batch_get_item" => {
                let table_name = params.table_name.as_ref().unwrap();

                Ok(NodeOutput::success(json!({
                    "success": true,
                    "operation": "batch_get_item",
                    "table_name": table_name,
                    "responses": {
                        table_name: [
                            {
                                "id": {"S": "1"},
                                "name": {"S": "Batch Item 1"}
                            },
                            {
                                "id": {"S": "2"},
                                "name": {"S": "Batch Item 2"}
                            }
                        ]
                    },
                    "unprocessed_keys": {},
                    "consumed_capacity": [
                        {
                            "table_name": table_name,
                            "capacity_units": 2.0
                        }
                    ]
                })))
            }
            "batch_write_item" => {
                let table_name = params.table_name.as_ref().unwrap();

                Ok(NodeOutput::success(json!({
                    "success": true,
                    "operation": "batch_write_item",
                    "table_name": table_name,
                    "unprocessed_items": {},
                    "consumed_capacity": [
                        {
                            "table_name": table_name,
                            "capacity_units": 5.0
                        }
                    ]
                })))
            }
            "create_table" => {
                let table_name = params.table_name.as_ref().unwrap();

                Ok(NodeOutput::success(json!({
                    "success": true,
                    "operation": "create_table",
                    "table_name": table_name,
                    "table_description": {
                        "table_name": table_name,
                        "table_status": "CREATING",
                        "creation_date_time": "2025-01-15T10:00:00Z",
                        "table_arn": format!("arn:aws:dynamodb:us-east-1:123456789012:table/{}", table_name)
                    }
                })))
            }
            "delete_table" => {
                let table_name = params.table_name.as_ref().unwrap();

                Ok(NodeOutput::success(json!({
                    "success": true,
                    "operation": "delete_table",
                    "table_name": table_name,
                    "table_description": {
                        "table_name": table_name,
                        "table_status": "DELETING"
                    }
                })))
            }
            "describe_table" => {
                let table_name = params.table_name.as_ref().unwrap();

                Ok(NodeOutput::success(json!({
                    "success": true,
                    "operation": "describe_table",
                    "table_name": table_name,
                    "table": {
                        "table_name": table_name,
                        "table_status": "ACTIVE",
                        "creation_date_time": "2025-01-01T00:00:00Z",
                        "item_count": 1000,
                        "table_size_bytes": 1048576,
                        "provisioned_throughput": {
                            "read_capacity_units": 5,
                            "write_capacity_units": 5
                        }
                    }
                })))
            }
            "list_tables" => Ok(NodeOutput::success(json!({
                "success": true,
                "operation": "list_tables",
                "table_names": [
                    "users",
                    "orders",
                    "products",
                    "sessions"
                ],
                "last_evaluated_table_name": null
            }))),
            _ => anyhow::bail!("Unsupported operation: {}", params.operation),
        }
    }

    fn validate_parameters(&self, parameters: &Value) -> anyhow::Result<()> {
        let params: AwsDynamoDBParams = serde_json::from_value(parameters.clone())?;

        // Validate based on operation
        match params.operation.as_str() {
            "put_item" => {
                if params.table_name.is_none() {
                    anyhow::bail!("put_item operation requires 'table_name' parameter");
                }
                if params.item.is_none() {
                    anyhow::bail!("put_item operation requires 'item' parameter");
                }
            }
            "get_item" => {
                if params.table_name.is_none() {
                    anyhow::bail!("get_item operation requires 'table_name' parameter");
                }
                if params.key.is_none() {
                    anyhow::bail!("get_item operation requires 'key' parameter");
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
            }
            "scan" => {
                if params.table_name.is_none() {
                    anyhow::bail!("scan operation requires 'table_name' parameter");
                }
            }
            "batch_get_item" => {
                if params.table_name.is_none() {
                    anyhow::bail!("batch_get_item operation requires 'table_name' parameter");
                }
            }
            "batch_write_item" => {
                if params.table_name.is_none() {
                    anyhow::bail!("batch_write_item operation requires 'table_name' parameter");
                }
            }
            "create_table" => {
                if params.table_name.is_none() {
                    anyhow::bail!("create_table operation requires 'table_name' parameter");
                }
            }
            "delete_table" => {
                if params.table_name.is_none() {
                    anyhow::bail!("delete_table operation requires 'table_name' parameter");
                }
            }
            "describe_table" => {
                if params.table_name.is_none() {
                    anyhow::bail!("describe_table operation requires 'table_name' parameter");
                }
            }
            "list_tables" => {
                // No required parameters
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
    fn test_aws_dynamodb_node_creation() {
        let node = AwsDynamoDBNode::new();
        assert_eq!(node.type_name(), "aws_dynamodb");
        assert!(matches!(node.category(), NodeCategory::Action));
        assert!(matches!(node.subcategory(), NodeSubcategory::Database));
    }

    #[test]
    fn test_aws_dynamodb_required_credential_type() {
        let node = AwsDynamoDBNode::new();
        assert_eq!(node.required_credential_type(), Some("aws"));
    }

    #[test]
    fn test_put_item_validation() {
        let node = AwsDynamoDBNode::new();

        // Valid put_item
        let valid_params = json!({
            "operation": "put_item",
            "table_name": "Users",
            "item": {
                "userId": {"S": "123"},
                "name": {"S": "John Doe"},
                "email": {"S": "john@example.com"}
            }
        });
        assert!(node.validate_parameters(&valid_params).is_ok());

        // Missing table_name
        let invalid_params = json!({
            "operation": "put_item",
            "item": {"userId": {"S": "123"}}
        });
        assert!(node.validate_parameters(&invalid_params).is_err());

        // Missing item
        let invalid_params = json!({
            "operation": "put_item",
            "table_name": "Users"
        });
        assert!(node.validate_parameters(&invalid_params).is_err());
    }

    #[test]
    fn test_get_item_validation() {
        let node = AwsDynamoDBNode::new();

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
    fn test_update_item_validation() {
        let node = AwsDynamoDBNode::new();

        // Valid update_item
        let valid_params = json!({
            "operation": "update_item",
            "table_name": "Users",
            "key": {"userId": {"S": "123"}},
            "update_expression": "SET #n = :name",
            "expression_attribute_names": {"#n": "name"},
            "expression_attribute_values": {":name": {"S": "Jane Doe"}}
        });
        assert!(node.validate_parameters(&valid_params).is_ok());

        // Missing update_expression
        let invalid_params = json!({
            "operation": "update_item",
            "table_name": "Users",
            "key": {"userId": {"S": "123"}}
        });
        assert!(node.validate_parameters(&invalid_params).is_err());
    }

    #[tokio::test]
    async fn test_query_operation() {
        let node = AwsDynamoDBNode::new();
        let params = json!({
            "operation": "query",
            "table_name": "Users",
            "expression_attribute_values": {":id": {"S": "123"}}
        });
        let context = NodeContext::new("test-exec".to_string(), "test-node".to_string());

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert_eq!(output.data["success"], true);
        assert_eq!(output.data["operation"], "query");
        assert!(output.data["items"].is_array());
        assert_eq!(output.data["count"], 2);
    }

    #[tokio::test]
    async fn test_scan_operation() {
        let node = AwsDynamoDBNode::new();
        let params = json!({
            "operation": "scan",
            "table_name": "Products",
            "limit": 50
        });
        let context = NodeContext::new("test-exec".to_string(), "test-node".to_string());

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert_eq!(output.data["success"], true);
        assert_eq!(output.data["operation"], "scan");
        assert_eq!(output.data["limit"], 50);
        assert!(output.data["items"].is_array());
    }

    #[tokio::test]
    async fn test_delete_item_operation() {
        let node = AwsDynamoDBNode::new();
        let params = json!({
            "operation": "delete_item",
            "table_name": "Users",
            "key": {"userId": {"S": "123"}}
        });
        let context = NodeContext::new("test-exec".to_string(), "test-node".to_string());

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert_eq!(output.data["success"], true);
        assert_eq!(output.data["operation"], "delete_item");
        assert_eq!(output.data["deleted"], true);
    }

    #[tokio::test]
    async fn test_create_table_operation() {
        let node = AwsDynamoDBNode::new();
        let params = json!({
            "operation": "create_table",
            "table_name": "NewTable"
        });
        let context = NodeContext::new("test-exec".to_string(), "test-node".to_string());

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert_eq!(output.data["success"], true);
        assert_eq!(output.data["operation"], "create_table");
        assert_eq!(output.data["table_description"]["table_status"], "CREATING");
    }

    #[tokio::test]
    async fn test_list_tables_operation() {
        let node = AwsDynamoDBNode::new();
        let params = json!({
            "operation": "list_tables"
        });
        let context = NodeContext::new("test-exec".to_string(), "test-node".to_string());

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert_eq!(output.data["success"], true);
        assert_eq!(output.data["operation"], "list_tables");
        assert!(output.data["table_names"].is_array());
    }
}
