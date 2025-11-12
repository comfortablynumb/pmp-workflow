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

/// Trait that all node types must implement
#[async_trait]
pub trait Node: Send + Sync {
    /// Get the node type identifier
    fn node_type(&self) -> &str;

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
