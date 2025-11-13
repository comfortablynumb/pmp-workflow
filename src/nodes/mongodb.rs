use crate::models::{Node, NodeCategory, NodeContext, NodeOutput, NodeSubcategory, NodeType};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::json;

/// MongoDB node for NoSQL database operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MongoDBNode {
    #[serde(skip)]
    _private: (),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MongoDBNodeParams {
    /// The operation to perform
    pub operation: String,

    // Collection operations
    /// Collection name
    pub collection: Option<String>,
    /// Database name
    pub database: Option<String>,

    // Document operations
    /// Document to insert/update (JSON object or array)
    pub document: Option<serde_json::Value>,
    /// Documents to insert (array of objects)
    pub documents: Option<Vec<serde_json::Value>>,

    // Query operations
    /// Query filter (MongoDB query object)
    pub filter: Option<serde_json::Value>,
    /// Update operations (MongoDB update object)
    pub update: Option<serde_json::Value>,
    /// Projection (fields to return)
    pub projection: Option<serde_json::Value>,
    /// Sort order
    pub sort: Option<serde_json::Value>,

    // Aggregation operations
    /// Aggregation pipeline
    pub pipeline: Option<Vec<serde_json::Value>>,

    // Options
    /// Limit number of results
    pub limit: Option<i64>,
    /// Skip number of results
    pub skip: Option<i64>,
    /// Upsert flag (insert if not exists)
    pub upsert: Option<bool>,
    /// Update multiple documents
    pub multi: Option<bool>,
    /// Return updated document
    pub return_document: Option<String>, // "before" or "after"

    // Index operations
    /// Index specification
    pub index: Option<serde_json::Value>,
    /// Index name
    pub index_name: Option<String>,
    /// Index options
    pub index_options: Option<serde_json::Value>,

    // Bulk operations
    /// Bulk operations array
    pub operations: Option<Vec<serde_json::Value>>,
    /// Ordered bulk operations
    pub ordered: Option<bool>,
}

impl MongoDBNode {
    pub fn new() -> Self {
        Self { _private: () }
    }
}

impl Default for MongoDBNode {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl NodeType for MongoDBNode {
    fn type_name(&self) -> &str {
        "mongodb"
    }

    fn category(&self) -> NodeCategory {
        NodeCategory::Action
    }

    fn subcategory(&self) -> NodeSubcategory {
        NodeSubcategory::Database
    }

    fn required_credential_type(&self) -> Option<&str> {
        Some("mongodb_connection")
    }

    fn parameter_schema(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "required": ["operation"],
            "properties": {
                "operation": {
                    "type": "string",
                    "enum": [
                        "find",
                        "find_one",
                        "insert_one",
                        "insert_many",
                        "update_one",
                        "update_many",
                        "replace_one",
                        "delete_one",
                        "delete_many",
                        "count_documents",
                        "distinct",
                        "aggregate",
                        "create_index",
                        "drop_index",
                        "list_indexes",
                        "create_collection",
                        "drop_collection",
                        "list_collections",
                        "list_databases",
                        "bulk_write"
                    ],
                    "description": "The MongoDB operation to perform"
                },
                "collection": {
                    "type": "string",
                    "description": "Collection name"
                },
                "database": {
                    "type": "string",
                    "description": "Database name (optional if specified in connection)"
                },
                "document": {
                    "description": "Document to insert/update (JSON object)"
                },
                "documents": {
                    "type": "array",
                    "description": "Array of documents to insert"
                },
                "filter": {
                    "description": "MongoDB query filter"
                },
                "update": {
                    "description": "MongoDB update operations"
                },
                "projection": {
                    "description": "Fields to return"
                },
                "sort": {
                    "description": "Sort order"
                },
                "pipeline": {
                    "type": "array",
                    "description": "Aggregation pipeline stages"
                },
                "limit": {
                    "type": "integer",
                    "description": "Limit number of results"
                },
                "skip": {
                    "type": "integer",
                    "description": "Skip number of results"
                },
                "upsert": {
                    "type": "boolean",
                    "description": "Insert if document doesn't exist"
                },
                "multi": {
                    "type": "boolean",
                    "description": "Update multiple documents"
                },
                "return_document": {
                    "type": "string",
                    "enum": ["before", "after"],
                    "description": "Return document before or after update"
                },
                "index": {
                    "description": "Index specification"
                },
                "index_name": {
                    "type": "string",
                    "description": "Index name"
                },
                "index_options": {
                    "description": "Index options"
                },
                "operations": {
                    "type": "array",
                    "description": "Bulk operations array"
                },
                "ordered": {
                    "type": "boolean",
                    "description": "Execute bulk operations in order"
                }
            }
        })
    }
}

#[async_trait]
impl Node for MongoDBNode {
    async fn execute(
        &self,
        context: &NodeContext,
        parameters: &serde_json::Value,
    ) -> anyhow::Result<NodeOutput> {
        let _params: MongoDBNodeParams = serde_json::from_value(parameters.clone())?;

        // Mock implementation - in a real implementation, this would connect to MongoDB and execute operations
        let result = json!({
            "success": true,
            "message": "MongoDB operation would be executed here",
            "execution_id": &context.execution_id
        });

        Ok(NodeOutput::success(result))
    }

    fn validate_parameters(&self, parameters: &serde_json::Value) -> anyhow::Result<()> {
        let params: MongoDBNodeParams = serde_json::from_value(parameters.clone())?;

        // Validate based on operation
        match params.operation.as_str() {
            "find" | "find_one" | "count_documents" | "distinct" => {
                if params.collection.is_none() {
                    anyhow::bail!(
                        "{} operation requires 'collection' parameter",
                        params.operation
                    );
                }
            }
            "insert_one" => {
                if params.collection.is_none() {
                    anyhow::bail!("insert_one operation requires 'collection' parameter");
                }
                if params.document.is_none() {
                    anyhow::bail!("insert_one operation requires 'document' parameter");
                }
            }
            "insert_many" => {
                if params.collection.is_none() {
                    anyhow::bail!("insert_many operation requires 'collection' parameter");
                }
                if params.documents.is_none() {
                    anyhow::bail!("insert_many operation requires 'documents' parameter");
                }
            }
            "update_one" | "update_many" | "replace_one" => {
                if params.collection.is_none() {
                    anyhow::bail!(
                        "{} operation requires 'collection' parameter",
                        params.operation
                    );
                }
                if params.filter.is_none() {
                    anyhow::bail!("{} operation requires 'filter' parameter", params.operation);
                }
                if params.update.is_none() && params.document.is_none() {
                    anyhow::bail!(
                        "{} operation requires 'update' or 'document' parameter",
                        params.operation
                    );
                }
            }
            "delete_one" | "delete_many" => {
                if params.collection.is_none() {
                    anyhow::bail!(
                        "{} operation requires 'collection' parameter",
                        params.operation
                    );
                }
                if params.filter.is_none() {
                    anyhow::bail!("{} operation requires 'filter' parameter", params.operation);
                }
            }
            "aggregate" => {
                if params.collection.is_none() {
                    anyhow::bail!("aggregate operation requires 'collection' parameter");
                }
                if params.pipeline.is_none() {
                    anyhow::bail!("aggregate operation requires 'pipeline' parameter");
                }
            }
            "create_index" => {
                if params.collection.is_none() {
                    anyhow::bail!("create_index operation requires 'collection' parameter");
                }
                if params.index.is_none() {
                    anyhow::bail!("create_index operation requires 'index' parameter");
                }
            }
            "drop_index" => {
                if params.collection.is_none() {
                    anyhow::bail!("drop_index operation requires 'collection' parameter");
                }
                if params.index_name.is_none() && params.index.is_none() {
                    anyhow::bail!(
                        "drop_index operation requires 'index_name' or 'index' parameter"
                    );
                }
            }
            "list_indexes" | "drop_collection" => {
                if params.collection.is_none() {
                    anyhow::bail!(
                        "{} operation requires 'collection' parameter",
                        params.operation
                    );
                }
            }
            "create_collection" => {
                if params.collection.is_none() {
                    anyhow::bail!("create_collection operation requires 'collection' parameter");
                }
            }
            "bulk_write" => {
                if params.collection.is_none() {
                    anyhow::bail!("bulk_write operation requires 'collection' parameter");
                }
                if params.operations.is_none() {
                    anyhow::bail!("bulk_write operation requires 'operations' parameter");
                }
            }
            "list_collections" | "list_databases" => {
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
    fn test_mongodb_node_creation() {
        let node = MongoDBNode::new();
        assert_eq!(node.type_name(), "mongodb");
        assert!(matches!(node.category(), NodeCategory::Action));
        assert!(matches!(node.subcategory(), NodeSubcategory::Database));
    }

    #[test]
    fn test_mongodb_required_credential_type() {
        let node = MongoDBNode::new();
        assert_eq!(node.required_credential_type(), Some("mongodb_connection"));
    }

    #[test]
    fn test_find_validation() {
        let node = MongoDBNode::new();

        // Valid find
        let valid_params = json!({
            "operation": "find",
            "collection": "users",
            "filter": {"age": {"$gt": 18}}
        });
        assert!(node.validate_parameters(&valid_params).is_ok());

        // Missing collection
        let invalid_params = json!({
            "operation": "find",
            "filter": {"age": {"$gt": 18}}
        });
        assert!(node.validate_parameters(&invalid_params).is_err());
    }

    #[test]
    fn test_insert_one_validation() {
        let node = MongoDBNode::new();

        // Valid insert
        let valid_params = json!({
            "operation": "insert_one",
            "collection": "users",
            "document": {"name": "John", "age": 30}
        });
        assert!(node.validate_parameters(&valid_params).is_ok());

        // Missing document
        let invalid_params = json!({
            "operation": "insert_one",
            "collection": "users"
        });
        assert!(node.validate_parameters(&invalid_params).is_err());
    }

    #[test]
    fn test_update_validation() {
        let node = MongoDBNode::new();

        // Valid update
        let valid_params = json!({
            "operation": "update_one",
            "collection": "users",
            "filter": {"_id": "123"},
            "update": {"$set": {"age": 31}}
        });
        assert!(node.validate_parameters(&valid_params).is_ok());
    }

    #[test]
    fn test_aggregate_validation() {
        let node = MongoDBNode::new();

        // Valid aggregation
        let valid_params = json!({
            "operation": "aggregate",
            "collection": "users",
            "pipeline": [
                {"$match": {"age": {"$gt": 18}}},
                {"$group": {"_id": "$city", "count": {"$sum": 1}}}
            ]
        });
        assert!(node.validate_parameters(&valid_params).is_ok());

        // Missing pipeline
        let invalid_params = json!({
            "operation": "aggregate",
            "collection": "users"
        });
        assert!(node.validate_parameters(&invalid_params).is_err());
    }

    #[tokio::test]
    async fn test_mongodb_execute() {
        let node = MongoDBNode::new();
        let params = json!({
            "operation": "find",
            "collection": "users",
            "filter": {}
        });
        let context = NodeContext::new("test-exec".to_string(), "test-node".to_string());

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
    }
}
