use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Status of a workflow execution
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq, Eq)]
#[sqlx(type_name = "execution_status", rename_all = "lowercase")]
pub enum ExecutionStatus {
    Running,
    Success,
    Failed,
    Cancelled,
}

impl std::fmt::Display for ExecutionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExecutionStatus::Running => write!(f, "running"),
            ExecutionStatus::Success => write!(f, "success"),
            ExecutionStatus::Failed => write!(f, "failed"),
            ExecutionStatus::Cancelled => write!(f, "cancelled"),
        }
    }
}

impl std::convert::TryFrom<String> for ExecutionStatus {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "running" => Ok(ExecutionStatus::Running),
            "success" => Ok(ExecutionStatus::Success),
            "failed" => Ok(ExecutionStatus::Failed),
            "cancelled" => Ok(ExecutionStatus::Cancelled),
            _ => Err(format!("Invalid execution status: {}", value)),
        }
    }
}

/// Represents a workflow execution
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct WorkflowExecution {
    pub id: Uuid,
    pub workflow_id: Uuid,
    #[sqlx(try_from = "String")]
    pub status: ExecutionStatus,
    pub started_at: DateTime<Utc>,
    pub finished_at: Option<DateTime<Utc>>,
    #[serde(default = "Utc::now")]
    pub updated_at: DateTime<Utc>,
    #[sqlx(json)]
    pub input_data: Option<serde_json::Value>,
    #[sqlx(json)]
    pub output_data: Option<serde_json::Value>,
    pub error: Option<String>,
}

impl WorkflowExecution {
    pub fn new(workflow_id: Uuid, input_data: Option<serde_json::Value>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            workflow_id,
            status: ExecutionStatus::Running,
            started_at: now,
            finished_at: None,
            updated_at: now,
            input_data,
            output_data: None,
            error: None,
        }
    }
}

/// Represents a node execution within a workflow execution
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct NodeExecution {
    pub id: Uuid,
    pub execution_id: Uuid,
    pub node_id: String,
    #[sqlx(try_from = "String")]
    pub status: ExecutionStatus,
    pub started_at: DateTime<Utc>,
    pub finished_at: Option<DateTime<Utc>>,
    #[serde(default = "Utc::now")]
    pub updated_at: DateTime<Utc>,
    #[sqlx(json)]
    pub input_data: Option<serde_json::Value>,
    #[sqlx(json)]
    pub output_data: Option<serde_json::Value>,
    pub error: Option<String>,
}

impl NodeExecution {
    pub fn new(execution_id: Uuid, node_id: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            execution_id,
            node_id,
            status: ExecutionStatus::Running,
            started_at: now,
            finished_at: None,
            updated_at: now,
            input_data: None,
            output_data: None,
            error: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execution_status_display() {
        assert_eq!(ExecutionStatus::Running.to_string(), "running");
        assert_eq!(ExecutionStatus::Success.to_string(), "success");
        assert_eq!(ExecutionStatus::Failed.to_string(), "failed");
        assert_eq!(ExecutionStatus::Cancelled.to_string(), "cancelled");
    }

    #[test]
    fn test_execution_status_try_from() {
        assert!(matches!(
            ExecutionStatus::try_from("running".to_string()),
            Ok(ExecutionStatus::Running)
        ));
        assert!(matches!(
            ExecutionStatus::try_from("success".to_string()),
            Ok(ExecutionStatus::Success)
        ));
        assert!(matches!(
            ExecutionStatus::try_from("failed".to_string()),
            Ok(ExecutionStatus::Failed)
        ));
        assert!(matches!(
            ExecutionStatus::try_from("cancelled".to_string()),
            Ok(ExecutionStatus::Cancelled)
        ));
        assert!(ExecutionStatus::try_from("invalid".to_string()).is_err());
    }

    #[test]
    fn test_workflow_execution_new() {
        let workflow_id = Uuid::new_v4();
        let input = Some(serde_json::json!({"key": "value"}));
        let execution = WorkflowExecution::new(workflow_id, input.clone());

        assert_eq!(execution.workflow_id, workflow_id);
        assert!(matches!(execution.status, ExecutionStatus::Running));
        assert_eq!(execution.input_data, input);
        assert!(execution.output_data.is_none());
        assert!(execution.error.is_none());
        assert!(execution.finished_at.is_none());

        // Verify updated_at is set
        let now = Utc::now();
        let diff = (now - execution.updated_at).num_seconds().abs();
        assert!(diff < 2, "updated_at should be close to current time");
    }

    #[test]
    fn test_node_execution_new() {
        let execution_id = Uuid::new_v4();
        let node_id = "test-node".to_string();
        let node_exec = NodeExecution::new(execution_id, node_id.clone());

        assert_eq!(node_exec.execution_id, execution_id);
        assert_eq!(node_exec.node_id, node_id);
        assert!(matches!(node_exec.status, ExecutionStatus::Running));
        assert!(node_exec.input_data.is_none());
        assert!(node_exec.output_data.is_none());
        assert!(node_exec.error.is_none());
        assert!(node_exec.finished_at.is_none());

        // Verify updated_at is set
        let now = Utc::now();
        let diff = (now - node_exec.updated_at).num_seconds().abs();
        assert!(diff < 2, "updated_at should be close to current time");
    }

    #[test]
    fn test_workflow_execution_tracks_updates() {
        let workflow_id = Uuid::new_v4();
        let input = Some(serde_json::json!({"test": "data"}));
        let execution = WorkflowExecution::new(workflow_id, input);

        // Verify that updated_at is initially set to current time
        let now = Utc::now();
        assert!(execution.updated_at <= now);

        // Verify started_at and updated_at are close
        let diff = (execution.updated_at - execution.started_at)
            .num_seconds()
            .abs();
        assert!(diff < 1, "updated_at and started_at should be very close");
    }

    #[test]
    fn test_node_execution_tracks_updates() {
        let execution_id = Uuid::new_v4();
        let node_id = "test-node".to_string();
        let node_exec = NodeExecution::new(execution_id, node_id);

        // Verify that updated_at is initially set to current time
        let now = Utc::now();
        assert!(node_exec.updated_at <= now);

        // Verify started_at and updated_at are close
        let diff = (node_exec.updated_at - node_exec.started_at)
            .num_seconds()
            .abs();
        assert!(diff < 1, "updated_at and started_at should be very close");
    }
}
