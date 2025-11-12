use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Result of a node execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeOutput {
    pub success: bool,
    pub data: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl NodeOutput {
    pub fn success(data: serde_json::Value) -> Self {
        Self {
            success: true,
            data,
            error: None,
        }
    }

    pub fn error(message: String) -> Self {
        Self {
            success: false,
            data: serde_json::Value::Null,
            error: Some(message),
        }
    }
}

/// Context passed to nodes during execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeContext {
    /// Workflow execution ID
    pub execution_id: String,
    /// Node ID
    pub node_id: String,
    /// Input data from previous nodes
    pub inputs: HashMap<String, serde_json::Value>,
    /// Global workflow variables
    pub variables: HashMap<String, serde_json::Value>,
}

impl NodeContext {
    pub fn new(execution_id: String, node_id: String) -> Self {
        Self {
            execution_id,
            node_id,
            inputs: HashMap::new(),
            variables: HashMap::new(),
        }
    }

    /// Get input data from a specific source
    pub fn get_input(&self, key: &str) -> Option<&serde_json::Value> {
        self.inputs.get(key)
    }

    /// Get the main input (when there's only one)
    pub fn get_main_input(&self) -> Option<&serde_json::Value> {
        self.inputs.values().next()
    }

    /// Add input data
    pub fn add_input(&mut self, key: String, value: serde_json::Value) {
        self.inputs.insert(key, value);
    }

    /// Get a workflow variable
    pub fn get_variable(&self, key: &str) -> Option<&serde_json::Value> {
        self.variables.get(key)
    }

    /// Set a workflow variable
    pub fn set_variable(&mut self, key: String, value: serde_json::Value) {
        self.variables.insert(key, value);
    }
}

/// Node category - determines if a node is a trigger or action
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NodeCategory {
    /// Trigger nodes - can be used as the first node in a workflow
    /// Examples: manual_trigger, webhook_trigger, schedule_trigger
    Trigger,
    /// Action nodes - execute actions within a workflow
    /// Examples: http_request, transform, conditional
    Action,
}

/// Node subcategory - provides additional classification within a category
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NodeSubcategory {
    /// Webhook-based triggers
    Webhook,
    /// Schedule/cron-based triggers
    Schedule,
    /// Manual/on-demand triggers
    Manual,
    /// HTTP/API related actions
    Http,
    /// Data transformation actions
    Transform,
    /// Logic/conditional actions
    Logic,
    /// Database operations
    Database,
    /// Workflow execution actions
    Workflow,
    /// Variable management actions
    Variable,
    /// General/uncategorized
    General,
}

/// Trait defining a node type with metadata
pub trait NodeType {
    /// Get the node type identifier (e.g., "http_request", "transform")
    fn type_name(&self) -> &str;

    /// Get the category of this node (Trigger or Action)
    fn category(&self) -> NodeCategory;

    /// Get the subcategory of this node
    fn subcategory(&self) -> NodeSubcategory {
        NodeSubcategory::General
    }

    /// Get the JSON schema for the node's parameters
    /// Returns a JSON Schema object describing the expected parameters
    fn parameter_schema(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {},
            "additionalProperties": true
        })
    }
}

/// Trait that all nodes must implement for execution
#[async_trait]
pub trait Node: NodeType + Send + Sync {
    /// Execute the node with the given context and parameters
    async fn execute(
        &self,
        context: &NodeContext,
        parameters: &serde_json::Value,
    ) -> anyhow::Result<NodeOutput>;

    /// Validate node parameters (optional)
    fn validate_parameters(&self, _parameters: &serde_json::Value) -> anyhow::Result<()> {
        Ok(())
    }
}

/// Factory for creating node instances
pub type NodeFactory = Box<dyn Fn() -> Box<dyn Node> + Send + Sync>;

/// Registry for node types
pub struct NodeRegistry {
    factories: HashMap<String, NodeFactory>,
}

impl NodeRegistry {
    pub fn new() -> Self {
        Self {
            factories: HashMap::new(),
        }
    }

    /// Register a new node type
    pub fn register<F>(&mut self, node_type: &str, factory: F)
    where
        F: Fn() -> Box<dyn Node> + Send + Sync + 'static,
    {
        self.factories
            .insert(node_type.to_string(), Box::new(factory));
    }

    /// Create a node instance by type
    pub fn create(&self, node_type: &str) -> anyhow::Result<Box<dyn Node>> {
        self.factories
            .get(node_type)
            .ok_or_else(|| anyhow::anyhow!("Unknown node type: {}", node_type))
            .map(|factory| factory())
    }

    /// Get all registered node types
    pub fn get_types(&self) -> Vec<String> {
        self.factories.keys().cloned().collect()
    }
}

impl Default for NodeRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_context_new() {
        let context = NodeContext::new("exec-123".to_string(), "node-1".to_string());
        assert_eq!(context.execution_id, "exec-123");
        assert_eq!(context.node_id, "node-1");
        assert!(context.inputs.is_empty());
        assert!(context.variables.is_empty());
    }

    #[test]
    fn test_node_context_input_operations() {
        let mut context = NodeContext::new("exec-123".to_string(), "node-1".to_string());

        context.add_input("key1".to_string(), serde_json::json!({"value": 42}));

        assert!(context.get_input("key1").is_some());
        assert!(context.get_input("key2").is_none());
        assert_eq!(
            context.get_input("key1").unwrap(),
            &serde_json::json!({"value": 42})
        );

        let main_input = context.get_main_input();
        assert!(main_input.is_some());
    }

    #[test]
    fn test_node_context_variable_operations() {
        let mut context = NodeContext::new("exec-123".to_string(), "node-1".to_string());

        context.set_variable("var1".to_string(), serde_json::json!("test"));

        assert!(context.get_variable("var1").is_some());
        assert_eq!(
            context.get_variable("var1").unwrap(),
            &serde_json::json!("test")
        );
        assert!(context.get_variable("var2").is_none());
    }

    #[test]
    fn test_node_output_success() {
        let output = NodeOutput::success(serde_json::json!({"result": "ok"}));
        assert!(output.success);
        assert!(output.error.is_none());
        assert_eq!(output.data, serde_json::json!({"result": "ok"}));
    }

    #[test]
    fn test_node_output_error() {
        let output = NodeOutput::error("Something went wrong".to_string());
        assert!(!output.success);
        assert_eq!(output.error, Some("Something went wrong".to_string()));
        assert_eq!(output.data, serde_json::Value::Null);
    }

    struct TestNode;

    impl NodeType for TestNode {
        fn type_name(&self) -> &str {
            "test_node"
        }

        fn category(&self) -> NodeCategory {
            NodeCategory::Action
        }
    }

    #[async_trait]
    impl Node for TestNode {
        async fn execute(
            &self,
            _context: &NodeContext,
            _parameters: &serde_json::Value,
        ) -> anyhow::Result<NodeOutput> {
            Ok(NodeOutput::success(serde_json::json!({"test": true})))
        }
    }

    #[test]
    fn test_node_registry() {
        let mut registry = NodeRegistry::new();

        registry.register("test_node", || Box::new(TestNode));

        let types = registry.get_types();
        assert_eq!(types.len(), 1);
        assert!(types.contains(&"test_node".to_string()));

        let node = registry.create("test_node");
        assert!(node.is_ok());
        assert_eq!(node.unwrap().type_name(), "test_node");

        let invalid = registry.create("invalid");
        assert!(invalid.is_err());
    }

    #[test]
    fn test_node_category_enum() {
        // Test that categories are different
        assert_ne!(NodeCategory::Trigger, NodeCategory::Action);

        // Test Debug trait
        assert_eq!(format!("{:?}", NodeCategory::Trigger), "Trigger");
        assert_eq!(format!("{:?}", NodeCategory::Action), "Action");

        // Test Copy trait
        let cat = NodeCategory::Trigger;
        let cat2 = cat;
        assert_eq!(cat, cat2);
    }

    #[test]
    fn test_node_category_assignment() {
        // Test that test node has correct category
        let node = TestNode;
        assert_eq!(node.category(), NodeCategory::Action);
    }

    // Test trigger node
    struct TestTriggerNode;

    impl NodeType for TestTriggerNode {
        fn type_name(&self) -> &str {
            "test_trigger"
        }

        fn category(&self) -> NodeCategory {
            NodeCategory::Trigger
        }
    }

    #[async_trait]
    impl Node for TestTriggerNode {
        async fn execute(
            &self,
            _context: &NodeContext,
            _parameters: &serde_json::Value,
        ) -> anyhow::Result<NodeOutput> {
            Ok(NodeOutput::success(serde_json::json!({"test": true})))
        }
    }

    #[test]
    fn test_trigger_vs_action_categories() {
        let action_node = TestNode;
        let trigger_node = TestTriggerNode;

        // Verify categories
        assert_eq!(action_node.category(), NodeCategory::Action);
        assert_eq!(trigger_node.category(), NodeCategory::Trigger);
        assert_ne!(action_node.category(), trigger_node.category());
    }

    #[test]
    fn test_node_category_serialization() {
        use serde_json;

        // Test serialization
        let trigger = NodeCategory::Trigger;
        let action = NodeCategory::Action;

        let trigger_json = serde_json::to_string(&trigger).unwrap();
        let action_json = serde_json::to_string(&action).unwrap();

        assert_eq!(trigger_json, "\"Trigger\"");
        assert_eq!(action_json, "\"Action\"");

        // Test deserialization
        let deserialized_trigger: NodeCategory = serde_json::from_str(&trigger_json).unwrap();
        let deserialized_action: NodeCategory = serde_json::from_str(&action_json).unwrap();

        assert_eq!(deserialized_trigger, NodeCategory::Trigger);
        assert_eq!(deserialized_action, NodeCategory::Action);
    }
}
