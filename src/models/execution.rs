use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Status of a workflow execution
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
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
    #[sqlx(json)]
    pub input_data: Option<serde_json::Value>,
    #[sqlx(json)]
    pub output_data: Option<serde_json::Value>,
    pub error: Option<String>,
}

impl WorkflowExecution {
    pub fn new(workflow_id: Uuid, input_data: Option<serde_json::Value>) -> Self {
        Self {
            id: Uuid::new_v4(),
            workflow_id,
            status: ExecutionStatus::Running,
            started_at: Utc::now(),
            finished_at: None,
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
    #[sqlx(json)]
    pub input_data: Option<serde_json::Value>,
    #[sqlx(json)]
    pub output_data: Option<serde_json::Value>,
    pub error: Option<String>,
}

impl NodeExecution {
    pub fn new(execution_id: Uuid, node_id: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            execution_id,
            node_id,
            status: ExecutionStatus::Running,
            started_at: Utc::now(),
            finished_at: None,
            input_data: None,
            output_data: None,
            error: None,
        }
    }
}
