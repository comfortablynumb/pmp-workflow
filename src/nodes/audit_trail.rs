use crate::models::{Node, NodeCategory, NodeContext, NodeOutput, NodeSubcategory, NodeType};
use async_trait::async_trait;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AuditTrailParams {
    /// Audit trail operation to perform
    pub operation: String,
    /// Type of action being audited
    pub action_type: Option<String>,
    /// Resource ID being accessed/modified
    pub resource_id: Option<String>,
    /// User ID performing the action
    pub user_id: Option<String>,
    /// Detailed information about the action
    #[serde(default)]
    pub details: serde_json::Value,
    /// Severity level of the audit event
    pub severity: Option<String>,
    /// Retention period in days
    pub retention_days: Option<u32>,
    /// Query filter for log queries (JSON object)
    #[serde(default)]
    pub query_filter: serde_json::Value,
}

/// Audit Trail node - tracks and audits all workflow actions for compliance
pub struct AuditTrailNode;

impl AuditTrailNode {
    pub fn new() -> Self {
        Self
    }
}

impl Default for AuditTrailNode {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeType for AuditTrailNode {
    fn type_name(&self) -> &str {
        "audit_trail"
    }

    fn category(&self) -> NodeCategory {
        NodeCategory::Action
    }

    fn subcategory(&self) -> NodeSubcategory {
        NodeSubcategory::General
    }

    fn parameter_schema(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "operation": {
                    "type": "string",
                    "description": "Audit trail operation to perform",
                    "enum": [
                        "log_action",
                        "log_access",
                        "log_change",
                        "query_audit_log",
                        "export_audit_log",
                        "compliance_report"
                    ]
                },
                "action_type": {
                    "type": "string",
                    "description": "Type of action (e.g., 'create', 'update', 'delete', 'access', 'login')"
                },
                "resource_id": {
                    "type": "string",
                    "description": "Resource ID being accessed or modified"
                },
                "user_id": {
                    "type": "string",
                    "description": "User ID performing the action"
                },
                "details": {
                    "type": "object",
                    "description": "Detailed information about the action (e.g., changes, IP address, user agent)"
                },
                "severity": {
                    "type": "string",
                    "description": "Severity level of the audit event",
                    "enum": ["low", "medium", "high", "critical"],
                    "default": "medium"
                },
                "retention_days": {
                    "type": "integer",
                    "description": "Retention period in days for the audit log",
                    "minimum": 1,
                    "maximum": 3650,
                    "default": 365
                },
                "query_filter": {
                    "type": "object",
                    "description": "Query filter for log queries (e.g., {\"user_id\": \"user-123\", \"action_type\": \"delete\"})"
                }
            },
            "required": ["operation"],
            "additionalProperties": false
        })
    }

    fn required_credential_type(&self) -> Option<&str> {
        None
    }
}

#[async_trait]
impl Node for AuditTrailNode {
    async fn execute(
        &self,
        context: &NodeContext,
        parameters: &serde_json::Value,
    ) -> anyhow::Result<NodeOutput> {
        let params: AuditTrailParams = serde_json::from_value(parameters.clone())?;

        // TODO: In a real implementation:
        // 1. For log_action: Record a general action in the audit log
        // 2. For log_access: Record resource access events
        // 3. For log_change: Record data modification events with before/after values
        // 4. For query_audit_log: Query audit logs with filters (user, resource, date range)
        // 5. For export_audit_log: Export audit logs in various formats (CSV, JSON, PDF)
        // 6. For compliance_report: Generate compliance reports (SOC2, HIPAA, GDPR, etc.)
        // 7. Store audit logs in immutable storage
        // 8. Include IP address, user agent, timestamp for all events
        // 9. Support retention policies
        // 10. Enable tamper-proof logging with cryptographic signatures

        // For now, we'll return a placeholder response
        let result = serde_json::json!({
            "message": "Audit trail operation executed (placeholder implementation)",
            "operation": &params.operation,
            "action_type": params.action_type,
            "resource_id": params.resource_id,
            "user_id": params.user_id,
            "severity": params.severity.unwrap_or_else(|| "medium".to_string()),
            "retention_days": params.retention_days.unwrap_or(365),
            "details": params.details,
            "context_execution_id": &context.execution_id,
            "audit_log_id": "audit-12345",
            "timestamp": chrono::Utc::now().to_rfc3339(),
            "success": true
        });

        Ok(NodeOutput::success(result))
    }

    fn validate_parameters(&self, parameters: &serde_json::Value) -> anyhow::Result<()> {
        let params: AuditTrailParams = serde_json::from_value(parameters.clone())
            .map_err(|e| anyhow::anyhow!("Invalid parameters: {}", e))?;

        // Validate operation
        let valid_operations = [
            "log_action",
            "log_access",
            "log_change",
            "query_audit_log",
            "export_audit_log",
            "compliance_report",
        ];

        if !valid_operations.contains(&params.operation.as_str()) {
            anyhow::bail!(
                "Invalid operation: {}. Must be one of: {}",
                params.operation,
                valid_operations.join(", ")
            );
        }

        // Validate severity if provided
        if let Some(ref severity) = params.severity {
            let valid_severities = ["low", "medium", "high", "critical"];
            if !valid_severities.contains(&severity.as_str()) {
                anyhow::bail!(
                    "Invalid severity: {}. Must be one of: {}",
                    severity,
                    valid_severities.join(", ")
                );
            }
        }

        // Validate retention_days range
        if let Some(retention) = params.retention_days
            && !(1..=3650).contains(&retention)
        {
            anyhow::bail!("retention_days must be between 1 and 3650");
        }

        // Logging operations require action_type, resource_id, and user_id
        let logging_ops = ["log_action", "log_access", "log_change"];
        if logging_ops.contains(&params.operation.as_str()) {
            if params.action_type.is_none() {
                anyhow::bail!(
                    "{} operation requires 'action_type' parameter",
                    params.operation
                );
            }
            if params.resource_id.is_none() {
                anyhow::bail!(
                    "{} operation requires 'resource_id' parameter",
                    params.operation
                );
            }
            if params.user_id.is_none() {
                anyhow::bail!(
                    "{} operation requires 'user_id' parameter",
                    params.operation
                );
            }
        }

        // query_audit_log requires query_filter
        if params.operation == "query_audit_log" && params.query_filter.is_null() {
            anyhow::bail!("query_audit_log operation requires 'query_filter' parameter");
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use uuid::Uuid;

    #[test]
    fn test_audit_trail_node_type() {
        let node = AuditTrailNode::new();
        assert_eq!(node.type_name(), "audit_trail");
        assert!(matches!(node.category(), NodeCategory::Action));
        assert!(matches!(node.subcategory(), NodeSubcategory::General));
        assert_eq!(node.required_credential_type(), None);
    }

    #[test]
    fn test_audit_trail_parameter_schema() {
        let node = AuditTrailNode::new();
        let schema = node.parameter_schema();

        assert_eq!(schema["type"], "object");
        assert!(schema["properties"]["operation"].is_object());
        assert!(schema["properties"]["action_type"].is_object());
        assert!(schema["properties"]["severity"].is_object());
    }

    #[tokio::test]
    async fn test_audit_trail_log_action() {
        let node = AuditTrailNode::new();
        let context = NodeContext::new(Uuid::new_v4().to_string(), "test-workflow".to_string());

        let params = json!({
            "operation": "log_action",
            "action_type": "create",
            "resource_id": "workflow-123",
            "user_id": "user-456",
            "severity": "medium",
            "details": {
                "workflow_name": "test-workflow",
                "ip_address": "192.168.1.1"
            }
        });

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
    }

    #[tokio::test]
    async fn test_audit_trail_query() {
        let node = AuditTrailNode::new();
        let context = NodeContext::new(Uuid::new_v4().to_string(), "test-workflow".to_string());

        let params = json!({
            "operation": "query_audit_log",
            "query_filter": {
                "user_id": "user-456",
                "action_type": "delete",
                "start_date": "2024-01-01"
            }
        });

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
    }

    #[test]
    fn test_audit_trail_validation() {
        let node = AuditTrailNode::new();

        // Valid log_action
        let valid_params = json!({
            "operation": "log_action",
            "action_type": "update",
            "resource_id": "res-123",
            "user_id": "user-456"
        });
        assert!(node.validate_parameters(&valid_params).is_ok());

        // Invalid operation
        let invalid_params = json!({
            "operation": "invalid_operation"
        });
        assert!(node.validate_parameters(&invalid_params).is_err());

        // Missing action_type for log_action
        let invalid_params = json!({
            "operation": "log_action",
            "resource_id": "res-123",
            "user_id": "user-456"
        });
        assert!(node.validate_parameters(&invalid_params).is_err());

        // Invalid severity
        let invalid_params = json!({
            "operation": "log_action",
            "action_type": "create",
            "resource_id": "res-123",
            "user_id": "user-456",
            "severity": "invalid_severity"
        });
        assert!(node.validate_parameters(&invalid_params).is_err());

        // Missing query_filter for query_audit_log
        let invalid_params = json!({
            "operation": "query_audit_log"
        });
        assert!(node.validate_parameters(&invalid_params).is_err());
    }
}
