use crate::models::{Node, NodeCategory, NodeContext, NodeOutput, NodeSubcategory, NodeType};
use async_trait::async_trait;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RedisParams {
    /// Credentials name to use for Redis connection
    pub credentials_name: String,
    /// Redis operation to perform
    pub operation: String,
    /// Key to operate on
    pub key: String,
    /// Value (for SET, SETEX, etc.)
    pub value: Option<serde_json::Value>,
    /// TTL in seconds (for SETEX, EXPIRE)
    pub ttl: Option<u64>,
    /// Additional arguments for complex operations
    #[serde(default)]
    pub args: Vec<String>,
}

/// Redis node - performs Redis operations
pub struct RedisNode;

impl RedisNode {
    pub fn new() -> Self {
        Self
    }
}

impl Default for RedisNode {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeType for RedisNode {
    fn type_name(&self) -> &str {
        "redis"
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
                    "description": "Name of the Redis credentials to use",
                    "minLength": 1
                },
                "operation": {
                    "type": "string",
                    "description": "Redis operation to perform",
                    "enum": ["GET", "SET", "DEL", "EXISTS", "EXPIRE", "TTL", "INCR", "DECR", "SETEX", "HGET", "HSET", "HDEL", "LPUSH", "RPUSH", "LPOP", "RPOP", "LRANGE", "SADD", "SMEMBERS", "SREM"]
                },
                "key": {
                    "type": "string",
                    "description": "Redis key to operate on",
                    "minLength": 1
                },
                "value": {
                    "description": "Value to set (for SET, HSET, etc.). Will be JSON-serialized."
                },
                "ttl": {
                    "type": "integer",
                    "description": "Time-to-live in seconds (for SETEX, EXPIRE)",
                    "minimum": 1
                },
                "args": {
                    "type": "array",
                    "description": "Additional arguments for complex operations",
                    "items": {
                        "type": "string"
                    },
                    "default": []
                }
            },
            "required": ["credentials_name", "operation", "key"],
            "additionalProperties": false
        })
    }
}

#[async_trait]
impl Node for RedisNode {
    async fn execute(
        &self,
        context: &NodeContext,
        parameters: &serde_json::Value,
    ) -> anyhow::Result<NodeOutput> {
        let params: RedisParams = serde_json::from_value(parameters.clone())?;

        // TODO: In a real implementation:
        // 1. Look up credentials from the credentials table using params.credentials_name
        // 2. Decrypt the credentials data
        // 3. Create a Redis connection using the decrypted connection string
        // 4. Execute the Redis operation based on params.operation
        // 5. Return the results

        // For now, we'll return a placeholder response
        let result = serde_json::json!({
            "message": "Redis operation executed (placeholder implementation)",
            "credentials_name": params.credentials_name,
            "operation": params.operation,
            "key": params.key,
            "value": params.value,
            "ttl": params.ttl,
            "context_execution_id": context.execution_id,
            "success": true
        });

        Ok(NodeOutput::success(result))
    }

    fn validate_parameters(&self, parameters: &serde_json::Value) -> anyhow::Result<()> {
        let params: RedisParams = serde_json::from_value(parameters.clone())
            .map_err(|e| anyhow::anyhow!("Invalid parameters: {}", e))?;

        if params.credentials_name.trim().is_empty() {
            anyhow::bail!("credentials_name cannot be empty");
        }

        if params.key.trim().is_empty() {
            anyhow::bail!("key cannot be empty");
        }

        // Validate operation
        let valid_operations = [
            "GET", "SET", "DEL", "EXISTS", "EXPIRE", "TTL", "INCR", "DECR", "SETEX", "HGET",
            "HSET", "HDEL", "LPUSH", "RPUSH", "LPOP", "RPOP", "LRANGE", "SADD", "SMEMBERS", "SREM",
        ];

        if !valid_operations.contains(&params.operation.as_str()) {
            anyhow::bail!(
                "Invalid operation: {}. Must be one of: {}",
                params.operation,
                valid_operations.join(", ")
            );
        }

        // Validate that SET operations have a value
        if ["SET", "SETEX", "HSET", "LPUSH", "RPUSH", "SADD"].contains(&params.operation.as_str())
            && params.value.is_none()
        {
            anyhow::bail!(
                "Operation {} requires a 'value' parameter",
                params.operation
            );
        }

        // Validate that SETEX has a TTL
        if params.operation == "SETEX" && params.ttl.is_none() {
            anyhow::bail!("SETEX operation requires a 'ttl' parameter");
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_redis_get_operation() {
        let node = RedisNode::new();
        let context = NodeContext::new("exec-123".to_string(), "node-1".to_string());

        let params = serde_json::json!({
            "credentials_name": "my_redis",
            "operation": "GET",
            "key": "user:123"
        });

        let result = node.execute(&context, &params).await.unwrap();
        assert!(result.success);
        assert_eq!(result.data["credentials_name"], "my_redis");
        assert_eq!(result.data["operation"], "GET");
        assert_eq!(result.data["key"], "user:123");
    }

    #[tokio::test]
    async fn test_redis_set_operation() {
        let node = RedisNode::new();
        let context = NodeContext::new("exec-123".to_string(), "node-1".to_string());

        let params = serde_json::json!({
            "credentials_name": "my_redis",
            "operation": "SET",
            "key": "user:123",
            "value": {"name": "John", "age": 30}
        });

        let result = node.execute(&context, &params).await.unwrap();
        assert!(result.success);
        assert_eq!(result.data["operation"], "SET");
    }

    #[test]
    fn test_redis_validation() {
        let node = RedisNode::new();

        // Valid GET operation
        let valid_get = serde_json::json!({
            "credentials_name": "my_redis",
            "operation": "GET",
            "key": "test_key"
        });
        assert!(node.validate_parameters(&valid_get).is_ok());

        // Valid SET operation
        let valid_set = serde_json::json!({
            "credentials_name": "my_redis",
            "operation": "SET",
            "key": "test_key",
            "value": "test_value"
        });
        assert!(node.validate_parameters(&valid_set).is_ok());

        // Invalid: SET without value
        let invalid_set = serde_json::json!({
            "credentials_name": "my_redis",
            "operation": "SET",
            "key": "test_key"
        });
        assert!(node.validate_parameters(&invalid_set).is_err());

        // Invalid: Empty credentials_name
        let invalid_creds = serde_json::json!({
            "credentials_name": "",
            "operation": "GET",
            "key": "test_key"
        });
        assert!(node.validate_parameters(&invalid_creds).is_err());

        // Invalid: Unknown operation
        let invalid_op = serde_json::json!({
            "credentials_name": "my_redis",
            "operation": "INVALID_OP",
            "key": "test_key"
        });
        assert!(node.validate_parameters(&invalid_op).is_err());

        // Invalid: SETEX without TTL
        let invalid_setex = serde_json::json!({
            "credentials_name": "my_redis",
            "operation": "SETEX",
            "key": "test_key",
            "value": "test_value"
        });
        assert!(node.validate_parameters(&invalid_setex).is_err());
    }

    #[test]
    fn test_redis_node_type() {
        let node = RedisNode::new();
        assert_eq!(node.type_name(), "redis");
        assert_eq!(node.category(), NodeCategory::Action);
        assert_eq!(node.subcategory(), NodeSubcategory::Database);
    }

    #[test]
    fn test_redis_parameter_schema() {
        let node = RedisNode::new();
        let schema = node.parameter_schema();

        assert_eq!(schema["type"], "object");
        assert!(schema["properties"]["credentials_name"].is_object());
        assert!(schema["properties"]["operation"].is_object());
        assert!(schema["properties"]["key"].is_object());
        assert!(schema["properties"]["operation"]["enum"].is_array());
        assert_eq!(schema["required"].as_array().unwrap().len(), 3);
    }
}
