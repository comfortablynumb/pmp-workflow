use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;
use uuid::Uuid;

/// Type of action being audited
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "audit_action", rename_all = "snake_case")]
pub enum AuditAction {
    /// Workflow actions
    WorkflowCreated,
    WorkflowUpdated,
    WorkflowDeleted,
    WorkflowExecuted,

    /// Execution actions
    ExecutionStarted,
    ExecutionCompleted,
    ExecutionFailed,
    ExecutionCancelled,

    /// Node execution actions
    NodeExecutionStarted,
    NodeExecutionCompleted,
    NodeExecutionFailed,

    /// Credential actions
    CredentialCreated,
    CredentialUpdated,
    CredentialDeleted,
    CredentialAccessed,

    /// Role and permission actions
    RoleCreated,
    RoleUpdated,
    RoleDeleted,
    RoleAssigned,
    RoleRevoked,
    PermissionGranted,
    PermissionRevoked,

    /// System actions
    SystemConfigChanged,
    UserLogin,
    UserLogout,
    UnauthorizedAccess,

    /// Custom action
    Custom,
}

impl std::fmt::Display for AuditAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuditAction::WorkflowCreated => write!(f, "workflow_created"),
            AuditAction::WorkflowUpdated => write!(f, "workflow_updated"),
            AuditAction::WorkflowDeleted => write!(f, "workflow_deleted"),
            AuditAction::WorkflowExecuted => write!(f, "workflow_executed"),
            AuditAction::ExecutionStarted => write!(f, "execution_started"),
            AuditAction::ExecutionCompleted => write!(f, "execution_completed"),
            AuditAction::ExecutionFailed => write!(f, "execution_failed"),
            AuditAction::ExecutionCancelled => write!(f, "execution_cancelled"),
            AuditAction::NodeExecutionStarted => write!(f, "node_execution_started"),
            AuditAction::NodeExecutionCompleted => write!(f, "node_execution_completed"),
            AuditAction::NodeExecutionFailed => write!(f, "node_execution_failed"),
            AuditAction::CredentialCreated => write!(f, "credential_created"),
            AuditAction::CredentialUpdated => write!(f, "credential_updated"),
            AuditAction::CredentialDeleted => write!(f, "credential_deleted"),
            AuditAction::CredentialAccessed => write!(f, "credential_accessed"),
            AuditAction::RoleCreated => write!(f, "role_created"),
            AuditAction::RoleUpdated => write!(f, "role_updated"),
            AuditAction::RoleDeleted => write!(f, "role_deleted"),
            AuditAction::RoleAssigned => write!(f, "role_assigned"),
            AuditAction::RoleRevoked => write!(f, "role_revoked"),
            AuditAction::PermissionGranted => write!(f, "permission_granted"),
            AuditAction::PermissionRevoked => write!(f, "permission_revoked"),
            AuditAction::SystemConfigChanged => write!(f, "system_config_changed"),
            AuditAction::UserLogin => write!(f, "user_login"),
            AuditAction::UserLogout => write!(f, "user_logout"),
            AuditAction::UnauthorizedAccess => write!(f, "unauthorized_access"),
            AuditAction::Custom => write!(f, "custom"),
        }
    }
}

/// Severity level of audit event
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "audit_severity", rename_all = "lowercase")]
pub enum AuditSeverity {
    Info,
    Warning,
    Error,
    Critical,
}

impl std::fmt::Display for AuditSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuditSeverity::Info => write!(f, "info"),
            AuditSeverity::Warning => write!(f, "warning"),
            AuditSeverity::Error => write!(f, "error"),
            AuditSeverity::Critical => write!(f, "critical"),
        }
    }
}

/// Result of the audited action
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "audit_result", rename_all = "lowercase")]
pub enum AuditResult {
    Success,
    Failure,
    Partial,
}

impl std::fmt::Display for AuditResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuditResult::Success => write!(f, "success"),
            AuditResult::Failure => write!(f, "failure"),
            AuditResult::Partial => write!(f, "partial"),
        }
    }
}

/// Enhanced audit log entry
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AuditLog {
    pub id: Uuid,
    pub action: String,
    pub severity: String,
    pub result: String,

    /// User who performed the action
    pub user_id: String,

    /// IP address of the request
    pub ip_address: Option<String>,

    /// User agent string
    pub user_agent: Option<String>,

    /// Resource type (workflow, credential, etc.)
    pub resource_type: Option<String>,

    /// Resource ID
    pub resource_id: Option<Uuid>,

    /// Workflow ID (if applicable)
    pub workflow_id: Option<Uuid>,

    /// Execution ID (if applicable)
    pub execution_id: Option<Uuid>,

    /// Description of the action
    pub description: String,

    /// Additional metadata as JSON
    #[sqlx(json)]
    pub metadata: Option<Value>,

    /// Error message (if action failed)
    pub error: Option<String>,

    /// Duration of the action in milliseconds
    pub duration_ms: Option<i64>,

    /// Timestamp when the action occurred
    pub timestamp: DateTime<Utc>,
}

impl AuditLog {
    pub fn new(action: AuditAction, user_id: String, description: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            action: action.to_string(),
            severity: AuditSeverity::Info.to_string(),
            result: AuditResult::Success.to_string(),
            user_id,
            ip_address: None,
            user_agent: None,
            resource_type: None,
            resource_id: None,
            workflow_id: None,
            execution_id: None,
            description,
            metadata: None,
            error: None,
            duration_ms: None,
            timestamp: Utc::now(),
        }
    }

    pub fn with_severity(mut self, severity: AuditSeverity) -> Self {
        self.severity = severity.to_string();
        self
    }

    pub fn with_result(mut self, result: AuditResult) -> Self {
        self.result = result.to_string();
        self
    }

    pub fn with_ip_address(mut self, ip_address: String) -> Self {
        self.ip_address = Some(ip_address);
        self
    }

    pub fn with_user_agent(mut self, user_agent: String) -> Self {
        self.user_agent = Some(user_agent);
        self
    }

    pub fn with_resource(mut self, resource_type: String, resource_id: Uuid) -> Self {
        self.resource_type = Some(resource_type);
        self.resource_id = Some(resource_id);
        self
    }

    pub fn with_workflow_id(mut self, workflow_id: Uuid) -> Self {
        self.workflow_id = Some(workflow_id);
        self
    }

    pub fn with_execution_id(mut self, execution_id: Uuid) -> Self {
        self.execution_id = Some(execution_id);
        self
    }

    pub fn with_metadata(mut self, metadata: Value) -> Self {
        self.metadata = Some(metadata);
        self
    }

    pub fn with_error(mut self, error: String) -> Self {
        self.error = Some(error);
        self
    }

    pub fn with_duration_ms(mut self, duration_ms: i64) -> Self {
        self.duration_ms = Some(duration_ms);
        self
    }
}

/// Audit log query filters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLogFilter {
    pub user_id: Option<String>,
    pub action: Option<AuditAction>,
    pub severity: Option<AuditSeverity>,
    pub result: Option<AuditResult>,
    pub resource_type: Option<String>,
    pub resource_id: Option<Uuid>,
    pub workflow_id: Option<Uuid>,
    pub execution_id: Option<Uuid>,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

impl Default for AuditLogFilter {
    fn default() -> Self {
        Self {
            user_id: None,
            action: None,
            severity: None,
            result: None,
            resource_type: None,
            resource_id: None,
            workflow_id: None,
            execution_id: None,
            start_time: None,
            end_time: None,
            limit: Some(100),
            offset: Some(0),
        }
    }
}

/// Audit statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditStats {
    pub total_events: i64,
    pub events_by_action: Vec<(String, i64)>,
    pub events_by_severity: Vec<(String, i64)>,
    pub events_by_result: Vec<(String, i64)>,
    pub events_by_user: Vec<(String, i64)>,
    pub success_rate: f64,
    pub average_duration_ms: Option<f64>,
}

impl Default for AuditStats {
    fn default() -> Self {
        Self {
            total_events: 0,
            events_by_action: Vec::new(),
            events_by_severity: Vec::new(),
            events_by_result: Vec::new(),
            events_by_user: Vec::new(),
            success_rate: 0.0,
            average_duration_ms: None,
        }
    }
}

/// Audit trail for a specific workflow execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionAuditTrail {
    pub execution_id: Uuid,
    pub workflow_id: Uuid,
    pub started_by: String,
    pub started_at: DateTime<Utc>,
    pub finished_at: Option<DateTime<Utc>>,
    pub total_duration_ms: Option<i64>,
    pub events: Vec<AuditLog>,
    pub node_executions: Vec<NodeAuditTrail>,
}

/// Audit trail for a specific node execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeAuditTrail {
    pub node_id: String,
    pub node_type: String,
    pub started_at: DateTime<Utc>,
    pub finished_at: Option<DateTime<Utc>>,
    pub duration_ms: Option<i64>,
    pub status: String,
    pub input_size: Option<usize>,
    pub output_size: Option<usize>,
    pub error: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_audit_action_display() {
        assert_eq!(AuditAction::WorkflowCreated.to_string(), "workflow_created");
        assert_eq!(
            AuditAction::ExecutionStarted.to_string(),
            "execution_started"
        );
        assert_eq!(
            AuditAction::UnauthorizedAccess.to_string(),
            "unauthorized_access"
        );
    }

    #[test]
    fn test_audit_severity_display() {
        assert_eq!(AuditSeverity::Info.to_string(), "info");
        assert_eq!(AuditSeverity::Critical.to_string(), "critical");
    }

    #[test]
    fn test_audit_result_display() {
        assert_eq!(AuditResult::Success.to_string(), "success");
        assert_eq!(AuditResult::Failure.to_string(), "failure");
    }

    #[test]
    fn test_audit_log_creation() {
        let log = AuditLog::new(
            AuditAction::WorkflowCreated,
            "user1".to_string(),
            "Created new workflow".to_string(),
        );

        assert_eq!(log.action, "workflow_created");
        assert_eq!(log.user_id, "user1");
        assert_eq!(log.description, "Created new workflow");
        assert_eq!(log.severity, "info");
        assert_eq!(log.result, "success");
    }

    #[test]
    fn test_audit_log_builder() {
        let workflow_id = Uuid::new_v4();
        let metadata = json!({
            "workflow_name": "test-workflow",
            "version": "1.0"
        });

        let log = AuditLog::new(
            AuditAction::WorkflowExecuted,
            "user1".to_string(),
            "Executed workflow".to_string(),
        )
        .with_severity(AuditSeverity::Info)
        .with_result(AuditResult::Success)
        .with_ip_address("192.168.1.1".to_string())
        .with_user_agent("Mozilla/5.0".to_string())
        .with_resource("workflow".to_string(), workflow_id)
        .with_workflow_id(workflow_id)
        .with_metadata(metadata.clone())
        .with_duration_ms(1500);

        assert_eq!(log.severity, "info");
        assert_eq!(log.result, "success");
        assert_eq!(log.ip_address, Some("192.168.1.1".to_string()));
        assert_eq!(log.user_agent, Some("Mozilla/5.0".to_string()));
        assert_eq!(log.resource_type, Some("workflow".to_string()));
        assert_eq!(log.resource_id, Some(workflow_id));
        assert_eq!(log.workflow_id, Some(workflow_id));
        assert_eq!(log.metadata, Some(metadata));
        assert_eq!(log.duration_ms, Some(1500));
    }

    #[test]
    fn test_audit_log_with_error() {
        let log = AuditLog::new(
            AuditAction::ExecutionFailed,
            "user1".to_string(),
            "Workflow execution failed".to_string(),
        )
        .with_severity(AuditSeverity::Error)
        .with_result(AuditResult::Failure)
        .with_error("Database connection timeout".to_string());

        assert_eq!(log.severity, "error");
        assert_eq!(log.result, "failure");
        assert_eq!(log.error, Some("Database connection timeout".to_string()));
    }

    #[test]
    fn test_audit_log_filter_default() {
        let filter = AuditLogFilter::default();
        assert_eq!(filter.limit, Some(100));
        assert_eq!(filter.offset, Some(0));
        assert!(filter.user_id.is_none());
        assert!(filter.action.is_none());
    }

    #[test]
    fn test_audit_stats_default() {
        let stats = AuditStats::default();
        assert_eq!(stats.total_events, 0);
        assert_eq!(stats.success_rate, 0.0);
        assert!(stats.events_by_action.is_empty());
    }

    #[test]
    fn test_execution_audit_trail() {
        let execution_id = Uuid::new_v4();
        let workflow_id = Uuid::new_v4();

        let trail = ExecutionAuditTrail {
            execution_id,
            workflow_id,
            started_by: "user1".to_string(),
            started_at: Utc::now(),
            finished_at: None,
            total_duration_ms: None,
            events: vec![],
            node_executions: vec![],
        };

        assert_eq!(trail.execution_id, execution_id);
        assert_eq!(trail.workflow_id, workflow_id);
        assert_eq!(trail.started_by, "user1");
    }

    #[test]
    fn test_node_audit_trail() {
        let trail = NodeAuditTrail {
            node_id: "node-1".to_string(),
            node_type: "http".to_string(),
            started_at: Utc::now(),
            finished_at: None,
            duration_ms: Some(250),
            status: "success".to_string(),
            input_size: Some(1024),
            output_size: Some(2048),
            error: None,
        };

        assert_eq!(trail.node_id, "node-1");
        assert_eq!(trail.node_type, "http");
        assert_eq!(trail.duration_ms, Some(250));
        assert_eq!(trail.input_size, Some(1024));
        assert_eq!(trail.output_size, Some(2048));
    }
}
