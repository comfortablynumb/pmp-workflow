use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Execution mode for workflow nodes
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "lowercase")]
pub enum ExecutionMode {
    /// Execute nodes sequentially (default)
    #[default]
    Sequential,
    /// Execute nodes in parallel where possible
    Parallel,
}

/// Represents a workflow definition
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Workflow {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    #[serde(default)]
    pub active: bool,
    #[sqlx(json)]
    pub nodes: serde_json::Value,
    #[sqlx(json)]
    pub edges: serde_json::Value,
    #[serde(default = "Utc::now")]
    pub created_at: DateTime<Utc>,
    #[serde(default = "Utc::now")]
    pub updated_at: DateTime<Utc>,
}

/// Represents a workflow definition in YAML format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowDefinition {
    pub name: String,
    pub description: Option<String>,
    pub nodes: Vec<NodeDefinition>,
    pub edges: Vec<EdgeDefinition>,
    /// Execution mode (sequential or parallel)
    #[serde(default)]
    pub execution_mode: ExecutionMode,
    /// Global timeout in seconds (optional)
    pub timeout_seconds: Option<u64>,
}

/// Represents a node in the workflow
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeDefinition {
    pub id: String,
    pub node_type: String,
    pub name: String,
    #[serde(default)]
    pub parameters: serde_json::Value,
    /// Node-specific timeout in seconds (overrides workflow timeout)
    pub timeout_seconds: Option<u64>,
}

/// Represents an edge connecting two nodes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgeDefinition {
    pub from: String,
    pub to: String,
    #[serde(default)]
    pub from_output: String,
    #[serde(default)]
    pub to_input: String,
}

impl WorkflowDefinition {
    /// Convert to a Workflow entity for database storage
    pub fn to_workflow(&self) -> anyhow::Result<Workflow> {
        Ok(Workflow {
            id: Uuid::new_v4(),
            name: self.name.clone(),
            description: self.description.clone(),
            active: true,
            nodes: serde_json::to_value(&self.nodes)?,
            edges: serde_json::to_value(&self.edges)?,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }
}

impl Workflow {
    /// Convert to WorkflowDefinition for execution
    pub fn to_definition(&self) -> anyhow::Result<WorkflowDefinition> {
        Ok(WorkflowDefinition {
            name: self.name.clone(),
            description: self.description.clone(),
            nodes: serde_json::from_value(self.nodes.clone())?,
            edges: serde_json::from_value(self.edges.clone())?,
            execution_mode: ExecutionMode::Sequential, // Default to sequential
            timeout_seconds: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_workflow_definition_to_workflow() {
        let def = WorkflowDefinition {
            name: "Test Workflow".to_string(),
            description: Some("A test workflow".to_string()),
            nodes: vec![NodeDefinition {
                id: "node1".to_string(),
                node_type: "start".to_string(),
                name: "Start".to_string(),
                parameters: serde_json::json!({}),
                timeout_seconds: None,
            }],
            edges: vec![],
            execution_mode: ExecutionMode::Sequential,
            timeout_seconds: None,
        };

        let workflow = def.to_workflow().unwrap();
        assert_eq!(workflow.name, "Test Workflow");
        assert_eq!(workflow.description, Some("A test workflow".to_string()));
        assert!(workflow.active);
    }

    #[test]
    fn test_workflow_to_definition() {
        let workflow = Workflow {
            id: Uuid::new_v4(),
            name: "Test Workflow".to_string(),
            description: Some("A test workflow".to_string()),
            active: true,
            nodes: serde_json::json!([{
                "id": "node1",
                "node_type": "start",
                "name": "Start",
                "parameters": {}
            }]),
            edges: serde_json::json!([]),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let def = workflow.to_definition().unwrap();
        assert_eq!(def.name, "Test Workflow");
        assert_eq!(def.nodes.len(), 1);
        assert_eq!(def.nodes[0].id, "node1");
        assert_eq!(def.execution_mode, ExecutionMode::Sequential);
        assert_eq!(def.timeout_seconds, None);
    }

    #[test]
    fn test_execution_mode_serialization() {
        // Test sequential mode
        let sequential = ExecutionMode::Sequential;
        let json = serde_json::to_value(&sequential).unwrap();
        assert_eq!(json, serde_json::json!("sequential"));

        // Test parallel mode
        let parallel = ExecutionMode::Parallel;
        let json = serde_json::to_value(&parallel).unwrap();
        assert_eq!(json, serde_json::json!("parallel"));
    }

    #[test]
    fn test_execution_mode_deserialization() {
        // Test sequential mode
        let json = serde_json::json!("sequential");
        let mode: ExecutionMode = serde_json::from_value(json).unwrap();
        assert_eq!(mode, ExecutionMode::Sequential);

        // Test parallel mode
        let json = serde_json::json!("parallel");
        let mode: ExecutionMode = serde_json::from_value(json).unwrap();
        assert_eq!(mode, ExecutionMode::Parallel);
    }

    #[test]
    fn test_execution_mode_default() {
        let mode = ExecutionMode::default();
        assert_eq!(mode, ExecutionMode::Sequential);
    }

    #[test]
    fn test_workflow_with_timeout_configuration() {
        let def = WorkflowDefinition {
            name: "Timeout Workflow".to_string(),
            description: None,
            nodes: vec![
                NodeDefinition {
                    id: "node1".to_string(),
                    node_type: "http_request".to_string(),
                    name: "Fast Node".to_string(),
                    parameters: serde_json::json!({}),
                    timeout_seconds: Some(5),
                },
                NodeDefinition {
                    id: "node2".to_string(),
                    node_type: "transform".to_string(),
                    name: "Regular Node".to_string(),
                    parameters: serde_json::json!({}),
                    timeout_seconds: None,
                },
            ],
            edges: vec![],
            execution_mode: ExecutionMode::Parallel,
            timeout_seconds: Some(30),
        };

        let workflow = def.to_workflow().unwrap();
        let restored_def = workflow.to_definition().unwrap();

        // Nodes are preserved in the JSON
        assert_eq!(restored_def.nodes[0].timeout_seconds, Some(5));
        assert_eq!(restored_def.nodes[1].timeout_seconds, None);

        // Note: execution_mode and timeout_seconds are not stored in the Workflow table,
        // so they default when converting back to WorkflowDefinition
        assert_eq!(restored_def.execution_mode, ExecutionMode::Sequential);
        assert_eq!(restored_def.timeout_seconds, None);
    }

    #[test]
    fn test_parallel_execution_mode() {
        let def = WorkflowDefinition {
            name: "Parallel Workflow".to_string(),
            description: Some("Executes nodes in parallel".to_string()),
            nodes: vec![NodeDefinition {
                id: "node1".to_string(),
                node_type: "start".to_string(),
                name: "Start".to_string(),
                parameters: serde_json::json!({}),
                timeout_seconds: None,
            }],
            edges: vec![],
            execution_mode: ExecutionMode::Parallel,
            timeout_seconds: None,
        };

        assert_eq!(def.execution_mode, ExecutionMode::Parallel);

        // Test serialization
        let json = serde_json::to_value(&def.execution_mode).unwrap();
        assert_eq!(json, "parallel");
    }
}
