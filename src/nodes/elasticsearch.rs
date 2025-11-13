use crate::models::{Node, NodeCategory, NodeContext, NodeOutput, NodeSubcategory, NodeType};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::json;

/// Elasticsearch node for search and analytics operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElasticsearchNode {
    #[serde(skip)]
    _private: (),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElasticsearchNodeParams {
    /// The operation to perform
    pub operation: String,

    // Index operations
    /// Index name
    pub index: Option<String>,
    /// Document ID
    pub id: Option<String>,
    /// Document body
    pub document: Option<serde_json::Value>,

    // Search operations
    /// Search query
    pub query: Option<serde_json::Value>,
    /// Aggregations
    pub aggregations: Option<serde_json::Value>,
    /// Source fields to return
    pub source: Option<serde_json::Value>,
    /// Sort order
    pub sort: Option<Vec<serde_json::Value>>,

    // Pagination
    /// Number of results to return
    pub size: Option<i32>,
    /// Offset for results
    pub from: Option<i32>,
    /// Search after (for deep pagination)
    pub search_after: Option<Vec<serde_json::Value>>,

    // Bulk operations
    /// Bulk operations array
    pub operations: Option<Vec<serde_json::Value>>,

    // Update operations
    /// Script for update
    pub script: Option<serde_json::Value>,
    /// Partial document for update
    pub doc: Option<serde_json::Value>,
    /// Upsert document
    pub upsert: Option<serde_json::Value>,

    // Index settings and mappings
    /// Index settings
    pub settings: Option<serde_json::Value>,
    /// Index mappings
    pub mappings: Option<serde_json::Value>,

    // Reindex operations
    /// Source index
    pub source_index: Option<String>,
    /// Destination index
    pub dest_index: Option<String>,

    // Snapshot operations
    /// Repository name
    pub repository: Option<String>,
    /// Snapshot name
    pub snapshot: Option<String>,

    // Advanced options
    /// Refresh policy (true, false, "wait_for")
    pub refresh: Option<String>,
    /// Routing value
    pub routing: Option<String>,
    /// Timeout
    pub timeout: Option<String>,
    /// Track total hits
    pub track_total_hits: Option<bool>,
}

impl ElasticsearchNode {
    pub fn new() -> Self {
        Self { _private: () }
    }
}

impl Default for ElasticsearchNode {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl NodeType for ElasticsearchNode {
    fn type_name(&self) -> &str {
        "elasticsearch"
    }

    fn category(&self) -> NodeCategory {
        NodeCategory::Action
    }

    fn subcategory(&self) -> NodeSubcategory {
        NodeSubcategory::Database
    }

    fn required_credential_type(&self) -> Option<&str> {
        Some("elasticsearch_connection")
    }

    fn parameter_schema(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "required": ["operation"],
            "properties": {
                "operation": {
                    "type": "string",
                    "enum": [
                        "index",
                        "get",
                        "update",
                        "delete",
                        "search",
                        "bulk",
                        "create_index",
                        "delete_index",
                        "get_index",
                        "update_mapping",
                        "get_mapping",
                        "reindex",
                        "count",
                        "exists",
                        "mget",
                        "msearch",
                        "update_by_query",
                        "delete_by_query",
                        "refresh_index",
                        "flush_index",
                        "clear_cache",
                        "cluster_health",
                        "cluster_stats",
                        "index_stats",
                        "create_snapshot",
                        "restore_snapshot",
                        "delete_snapshot"
                    ],
                    "description": "The Elasticsearch operation to perform"
                },
                "index": {
                    "type": "string",
                    "description": "Index name"
                },
                "id": {
                    "type": "string",
                    "description": "Document ID"
                },
                "document": {
                    "description": "Document body (JSON object)"
                },
                "query": {
                    "description": "Search query (Elasticsearch query DSL)"
                },
                "aggregations": {
                    "description": "Aggregations for search"
                },
                "source": {
                    "description": "Source fields to return"
                },
                "sort": {
                    "type": "array",
                    "description": "Sort order"
                },
                "size": {
                    "type": "integer",
                    "description": "Number of results to return"
                },
                "from": {
                    "type": "integer",
                    "description": "Offset for results"
                },
                "search_after": {
                    "type": "array",
                    "description": "Search after values for deep pagination"
                },
                "operations": {
                    "type": "array",
                    "description": "Bulk operations array"
                },
                "script": {
                    "description": "Script for update operation"
                },
                "doc": {
                    "description": "Partial document for update"
                },
                "upsert": {
                    "description": "Document to insert if not exists"
                },
                "settings": {
                    "description": "Index settings"
                },
                "mappings": {
                    "description": "Index mappings"
                },
                "source_index": {
                    "type": "string",
                    "description": "Source index for reindex"
                },
                "dest_index": {
                    "type": "string",
                    "description": "Destination index for reindex"
                },
                "repository": {
                    "type": "string",
                    "description": "Snapshot repository name"
                },
                "snapshot": {
                    "type": "string",
                    "description": "Snapshot name"
                },
                "refresh": {
                    "type": "string",
                    "enum": ["true", "false", "wait_for"],
                    "description": "Refresh policy"
                },
                "routing": {
                    "type": "string",
                    "description": "Routing value"
                },
                "timeout": {
                    "type": "string",
                    "description": "Request timeout"
                },
                "track_total_hits": {
                    "type": "boolean",
                    "description": "Track total hits accurately"
                }
            }
        })
    }

}

#[async_trait]
impl Node for ElasticsearchNode {
    async fn execute(
        &self,
        context: &NodeContext,
        parameters: &serde_json::Value,
    ) -> anyhow::Result<NodeOutput> {
        let _params: ElasticsearchNodeParams = serde_json::from_value(parameters.clone())?;

        // Mock implementation - in a real implementation, this would connect to Elasticsearch and execute operations
        let result = json!({
            "success": true,
            "message": "Elasticsearch operation would be executed here",
            "execution_id": &context.execution_id
        });

        Ok(NodeOutput::success(result))
    }

    fn validate_parameters(&self, parameters: &serde_json::Value) -> anyhow::Result<()> {
        let params: ElasticsearchNodeParams = serde_json::from_value(parameters.clone())?;

        // Validate based on operation
        match params.operation.as_str() {
            "index" => {
                if params.index.is_none() {
                    anyhow::bail!("index operation requires 'index' parameter");
                }
                if params.document.is_none() {
                    anyhow::bail!("index operation requires 'document' parameter");
                }
            }
            "get" | "delete" | "exists" => {
                if params.index.is_none() {
                    anyhow::bail!("{} operation requires 'index' parameter", params.operation);
                }
                if params.id.is_none() {
                    anyhow::bail!("{} operation requires 'id' parameter", params.operation);
                }
            }
            "update" => {
                if params.index.is_none() {
                    anyhow::bail!("update operation requires 'index' parameter");
                }
                if params.id.is_none() {
                    anyhow::bail!("update operation requires 'id' parameter");
                }
                if params.doc.is_none() && params.script.is_none() {
                    anyhow::bail!("update operation requires 'doc' or 'script' parameter");
                }
            }
            "search" | "count" => {
                if params.index.is_none() {
                    anyhow::bail!("{} operation requires 'index' parameter", params.operation);
                }
            }
            "bulk" => {
                if params.operations.is_none() {
                    anyhow::bail!("bulk operation requires 'operations' parameter");
                }
            }
            "create_index" => {
                if params.index.is_none() {
                    anyhow::bail!("create_index operation requires 'index' parameter");
                }
            }
            "delete_index" | "get_index" | "refresh_index" | "flush_index" | "index_stats" => {
                if params.index.is_none() {
                    anyhow::bail!("{} operation requires 'index' parameter", params.operation);
                }
            }
            "update_mapping" | "get_mapping" => {
                if params.index.is_none() {
                    anyhow::bail!("{} operation requires 'index' parameter", params.operation);
                }
            }
            "reindex" => {
                if params.source_index.is_none() {
                    anyhow::bail!("reindex operation requires 'source_index' parameter");
                }
                if params.dest_index.is_none() {
                    anyhow::bail!("reindex operation requires 'dest_index' parameter");
                }
            }
            "mget" => {
                if params.operations.is_none() && params.index.is_none() {
                    anyhow::bail!("mget operation requires 'operations' or 'index' parameter");
                }
            }
            "msearch" => {
                if params.operations.is_none() {
                    anyhow::bail!("msearch operation requires 'operations' parameter");
                }
            }
            "update_by_query" | "delete_by_query" => {
                if params.index.is_none() {
                    anyhow::bail!("{} operation requires 'index' parameter", params.operation);
                }
                if params.query.is_none() {
                    anyhow::bail!("{} operation requires 'query' parameter", params.operation);
                }
            }
            "create_snapshot" => {
                if params.repository.is_none() {
                    anyhow::bail!("create_snapshot operation requires 'repository' parameter");
                }
                if params.snapshot.is_none() {
                    anyhow::bail!("create_snapshot operation requires 'snapshot' parameter");
                }
            }
            "restore_snapshot" | "delete_snapshot" => {
                if params.repository.is_none() {
                    anyhow::bail!("{} operation requires 'repository' parameter", params.operation);
                }
                if params.snapshot.is_none() {
                    anyhow::bail!("{} operation requires 'snapshot' parameter", params.operation);
                }
            }
            "cluster_health" | "cluster_stats" | "clear_cache" => {
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
    fn test_elasticsearch_node_creation() {
        let node = ElasticsearchNode::new();
        assert_eq!(node.type_name(), "elasticsearch");
        assert!(matches!(node.category(), NodeCategory::Action));
        assert!(matches!(node.subcategory(), NodeSubcategory::Database));
    }

    #[test]
    fn test_elasticsearch_required_credential_type() {
        let node = ElasticsearchNode::new();
        assert_eq!(node.required_credential_type(), Some("elasticsearch_connection"));
    }

    #[test]
    fn test_index_validation() {
        let node = ElasticsearchNode::new();

        // Valid index
        let valid_params = json!({
            "operation": "index",
            "index": "users",
            "document": {"name": "John", "age": 30}
        });
        assert!(node.validate_parameters(&valid_params).is_ok());

        // Missing document
        let invalid_params = json!({
            "operation": "index",
            "index": "users"
        });
        assert!(node.validate_parameters(&invalid_params).is_err());
    }

    #[test]
    fn test_search_validation() {
        let node = ElasticsearchNode::new();

        // Valid search
        let valid_params = json!({
            "operation": "search",
            "index": "users",
            "query": {"match": {"name": "John"}}
        });
        assert!(node.validate_parameters(&valid_params).is_ok());

        // Missing index
        let invalid_params = json!({
            "operation": "search",
            "query": {"match_all": {}}
        });
        assert!(node.validate_parameters(&invalid_params).is_err());
    }

    #[test]
    fn test_update_validation() {
        let node = ElasticsearchNode::new();

        // Valid update with doc
        let valid_params = json!({
            "operation": "update",
            "index": "users",
            "id": "123",
            "doc": {"age": 31}
        });
        assert!(node.validate_parameters(&valid_params).is_ok());

        // Valid update with script
        let valid_params2 = json!({
            "operation": "update",
            "index": "users",
            "id": "123",
            "script": {"source": "ctx._source.age += 1"}
        });
        assert!(node.validate_parameters(&valid_params2).is_ok());
    }

    #[test]
    fn test_reindex_validation() {
        let node = ElasticsearchNode::new();

        // Valid reindex
        let valid_params = json!({
            "operation": "reindex",
            "source_index": "users_old",
            "dest_index": "users_new"
        });
        assert!(node.validate_parameters(&valid_params).is_ok());

        // Missing dest_index
        let invalid_params = json!({
            "operation": "reindex",
            "source_index": "users_old"
        });
        assert!(node.validate_parameters(&invalid_params).is_err());
    }

    #[tokio::test]
    async fn test_elasticsearch_execute() {
        let node = ElasticsearchNode::new();
        let params = json!({
            "operation": "search",
            "index": "users",
            "query": {"match_all": {}}
        });
        let context = NodeContext::new("test-exec".to_string(), "test-node".to_string());

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
    }
}
