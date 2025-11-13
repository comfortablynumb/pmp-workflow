use crate::models::{Node, NodeCategory, NodeContext, NodeOutput, NodeSubcategory, NodeType};
use async_trait::async_trait;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Mock node for testing that returns predefined outputs
#[derive(Clone)]
pub struct MockNode {
    /// Predefined responses keyed by node ID
    responses: Arc<Mutex<HashMap<String, NodeOutput>>>,
    /// Track how many times each node was called
    call_counts: Arc<Mutex<HashMap<String, usize>>>,
}

impl MockNode {
    /// Create a new mock node
    pub fn new() -> Self {
        Self {
            responses: Arc::new(Mutex::new(HashMap::new())),
            call_counts: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Set a mock response for a specific node ID
    pub fn set_response(&self, node_id: &str, output: NodeOutput) {
        self.responses
            .lock()
            .unwrap()
            .insert(node_id.to_string(), output);
    }

    /// Get the number of times a node was called
    pub fn get_call_count(&self, node_id: &str) -> usize {
        self.call_counts
            .lock()
            .unwrap()
            .get(node_id)
            .copied()
            .unwrap_or(0)
    }

    /// Reset all mock data
    pub fn reset(&self) {
        self.responses.lock().unwrap().clear();
        self.call_counts.lock().unwrap().clear();
    }
}

impl Default for MockNode {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeType for MockNode {
    fn type_name(&self) -> &str {
        "mock"
    }

    fn category(&self) -> NodeCategory {
        NodeCategory::Action
    }

    fn subcategory(&self) -> NodeSubcategory {
        NodeSubcategory::General
    }

    fn parameter_schema(&self) -> Value {
        serde_json::json!({
            "type": "object",
            "properties": {}
        })
    }
}

#[async_trait]
impl Node for MockNode {
    async fn execute(
        &self,
        context: &NodeContext,
        _parameters: &serde_json::Value,
    ) -> anyhow::Result<NodeOutput> {
        // Increment call count
        let mut counts = self.call_counts.lock().unwrap();
        let count = counts.entry(context.node_id.clone()).or_insert(0);
        *count += 1;
        drop(counts);

        // Return predefined response if available
        let responses = self.responses.lock().unwrap();
        if let Some(response) = responses.get(&context.node_id) {
            Ok(response.clone())
        } else {
            // Default response
            Ok(NodeOutput::success(serde_json::json!({
                "status": "mocked",
                "node_id": context.node_id
            })))
        }
    }

    fn validate_parameters(&self, _parameters: &Value) -> anyhow::Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mock_node_default_response() {
        let mock = MockNode::new();
        let context = NodeContext::new("exec-123".to_string(), "node-1".to_string());
        let params = serde_json::json!({});

        let result = mock.execute(&context, &params).await.unwrap();
        assert!(result.success);
        assert_eq!(mock.get_call_count("node-1"), 1);
    }

    #[tokio::test]
    async fn test_mock_node_custom_response() {
        let mock = MockNode::new();
        let custom_output = NodeOutput::success(serde_json::json!({"custom": "data"}));
        mock.set_response("node-1", custom_output.clone());

        let context = NodeContext::new("exec-123".to_string(), "node-1".to_string());
        let params = serde_json::json!({});
        let result = mock.execute(&context, &params).await.unwrap();

        assert!(result.success);
        assert_eq!(result.data, serde_json::json!({"custom": "data"}));
    }

    #[tokio::test]
    async fn test_mock_node_call_counting() {
        let mock = MockNode::new();
        let context = NodeContext::new("exec-123".to_string(), "node-1".to_string());
        let params = serde_json::json!({});

        assert_eq!(mock.get_call_count("node-1"), 0);

        mock.execute(&context, &params).await.unwrap();
        assert_eq!(mock.get_call_count("node-1"), 1);

        mock.execute(&context, &params).await.unwrap();
        assert_eq!(mock.get_call_count("node-1"), 2);
    }
}
