use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

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
}

/// Represents a node in the workflow
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeDefinition {
    pub id: String,
    pub node_type: String,
    pub name: String,
    #[serde(default)]
    pub parameters: serde_json::Value,
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
            }],
            edges: vec![],
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
    }
}
