use crate::models::{Node, NodeCategory, NodeContext, NodeOutput, NodeSubcategory, NodeType};
use async_trait::async_trait;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PagerDutyParams {
    /// Credentials name to use for PagerDuty API
    pub credentials_name: String,
    /// PagerDuty operation to perform
    pub operation: String,
    /// Incident ID
    pub incident_id: Option<String>,
    /// Incident title
    pub title: Option<String>,
    /// Incident description
    pub description: Option<String>,
    /// Service ID
    pub service_id: Option<String>,
    /// Urgency level (high, low)
    pub urgency: Option<String>,
    /// Escalation policy ID
    pub escalation_policy_id: Option<String>,
    /// Incident status
    pub status: Option<String>,
}

/// PagerDuty node - performs PagerDuty incident management operations
pub struct PagerDutyNode;

impl PagerDutyNode {
    pub fn new() -> Self {
        Self
    }
}

impl Default for PagerDutyNode {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeType for PagerDutyNode {
    fn type_name(&self) -> &str {
        "pagerduty"
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
                "credentials_name": {
                    "type": "string",
                    "description": "Name of the PagerDuty API credentials to use",
                    "minLength": 1
                },
                "operation": {
                    "type": "string",
                    "description": "PagerDuty operation to perform",
                    "enum": [
                        "create_incident",
                        "update_incident",
                        "resolve_incident",
                        "trigger_incident",
                        "acknowledge_incident",
                        "list_incidents",
                        "get_oncall"
                    ]
                },
                "incident_id": {
                    "type": "string",
                    "description": "Incident ID for update, resolve, or acknowledge operations"
                },
                "title": {
                    "type": "string",
                    "description": "Incident title"
                },
                "description": {
                    "type": "string",
                    "description": "Incident description"
                },
                "service_id": {
                    "type": "string",
                    "description": "Service ID for incident creation"
                },
                "urgency": {
                    "type": "string",
                    "description": "Urgency level",
                    "enum": ["high", "low"]
                },
                "escalation_policy_id": {
                    "type": "string",
                    "description": "Escalation policy ID"
                },
                "status": {
                    "type": "string",
                    "description": "Incident status",
                    "enum": ["triggered", "acknowledged", "resolved"]
                }
            },
            "required": ["credentials_name", "operation"],
            "additionalProperties": false
        })
    }

    fn required_credential_type(&self) -> Option<&str> {
        Some("pagerduty_api")
    }
}

#[async_trait]
impl Node for PagerDutyNode {
    async fn execute(
        &self,
        context: &NodeContext,
        parameters: &serde_json::Value,
    ) -> anyhow::Result<NodeOutput> {
        let params: PagerDutyParams = serde_json::from_value(parameters.clone())?;

        // TODO: In a real implementation:
        // 1. Look up credentials from the credentials table using params.credentials_name
        // 2. Decrypt the credentials data to get the PagerDuty API key
        // 3. Create a PagerDuty client using the API key
        // 4. Execute the operation based on params.operation:
        //    - create_incident: Create a new incident
        //    - update_incident: Update incident details
        //    - resolve_incident: Resolve an incident
        //    - trigger_incident: Trigger an incident
        //    - acknowledge_incident: Acknowledge an incident
        //    - list_incidents: List all incidents
        //    - get_oncall: Get current on-call schedule
        // 5. Return the API response

        // For now, we'll return a placeholder response
        let result = serde_json::json!({
            "message": "PagerDuty operation executed (placeholder implementation)",
            "credentials_name": &params.credentials_name,
            "operation": &params.operation,
            "incident_id": params.incident_id,
            "title": params.title,
            "service_id": params.service_id,
            "context_execution_id": &context.execution_id,
            "success": true
        });

        Ok(NodeOutput::success(result))
    }

    fn validate_parameters(&self, parameters: &serde_json::Value) -> anyhow::Result<()> {
        let params: PagerDutyParams = serde_json::from_value(parameters.clone())
            .map_err(|e| anyhow::anyhow!("Invalid parameters: {}", e))?;

        if params.credentials_name.trim().is_empty() {
            anyhow::bail!("credentials_name cannot be empty");
        }

        // Validate operation
        let valid_operations = [
            "create_incident",
            "update_incident",
            "resolve_incident",
            "trigger_incident",
            "acknowledge_incident",
            "list_incidents",
            "get_oncall",
        ];

        if !valid_operations.contains(&params.operation.as_str()) {
            anyhow::bail!(
                "Invalid operation: {}. Must be one of: {}",
                params.operation,
                valid_operations.join(", ")
            );
        }

        // Validate create_incident and trigger_incident
        let create_ops = ["create_incident", "trigger_incident"];
        if create_ops.contains(&params.operation.as_str()) {
            if params.title.is_none() {
                anyhow::bail!("{} operation requires 'title' parameter", params.operation);
            }
            if params.service_id.is_none() {
                anyhow::bail!(
                    "{} operation requires 'service_id' parameter",
                    params.operation
                );
            }
        }

        // Validate operations requiring incident_id
        let incident_id_ops = [
            "update_incident",
            "resolve_incident",
            "acknowledge_incident",
        ];
        if incident_id_ops.contains(&params.operation.as_str()) && params.incident_id.is_none() {
            anyhow::bail!(
                "{} operation requires 'incident_id' parameter",
                params.operation
            );
        }

        // Validate urgency value
        if let Some(ref urgency) = params.urgency
            && urgency != "high"
            && urgency != "low"
        {
            anyhow::bail!("urgency must be either 'high' or 'low'");
        }

        // Validate status value
        if let Some(ref status) = params.status {
            let valid_statuses = ["triggered", "acknowledged", "resolved"];
            if !valid_statuses.contains(&status.as_str()) {
                anyhow::bail!("status must be one of: {}", valid_statuses.join(", "));
            }
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
    fn test_pagerduty_node_type() {
        let node = PagerDutyNode::new();
        assert_eq!(node.type_name(), "pagerduty");
        assert!(matches!(node.category(), NodeCategory::Action));
        assert!(matches!(node.subcategory(), NodeSubcategory::General));
        assert_eq!(node.required_credential_type(), Some("pagerduty_api"));
    }

    #[test]
    fn test_pagerduty_parameter_schema() {
        let node = PagerDutyNode::new();
        let schema = node.parameter_schema();

        assert_eq!(schema["type"], "object");
        assert!(schema["properties"]["credentials_name"].is_object());
        assert!(schema["properties"]["operation"].is_object());
        assert!(schema["properties"]["incident_id"].is_object());
    }

    #[tokio::test]
    async fn test_pagerduty_create_incident() {
        let node = PagerDutyNode::new();
        let context = NodeContext::new(Uuid::new_v4().to_string(), "test-workflow".to_string());

        let params = json!({
            "credentials_name": "my_pagerduty_creds",
            "operation": "create_incident",
            "title": "Test Incident",
            "description": "This is a test incident",
            "service_id": "PXXXXXX",
            "urgency": "high"
        });

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
    }

    #[tokio::test]
    async fn test_pagerduty_resolve_incident() {
        let node = PagerDutyNode::new();
        let context = NodeContext::new(Uuid::new_v4().to_string(), "test-workflow".to_string());

        let params = json!({
            "credentials_name": "my_pagerduty_creds",
            "operation": "resolve_incident",
            "incident_id": "PXXXXXX"
        });

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
    }

    #[test]
    fn test_pagerduty_validation() {
        let node = PagerDutyNode::new();

        // Valid create_incident
        let valid_params = json!({
            "credentials_name": "my_pagerduty_creds",
            "operation": "create_incident",
            "title": "Test Incident",
            "service_id": "PXXXXXX"
        });
        assert!(node.validate_parameters(&valid_params).is_ok());

        // Missing title for create_incident
        let invalid_params = json!({
            "credentials_name": "my_pagerduty_creds",
            "operation": "create_incident",
            "service_id": "PXXXXXX"
        });
        assert!(node.validate_parameters(&invalid_params).is_err());

        // Missing incident_id for resolve_incident
        let invalid_params = json!({
            "credentials_name": "my_pagerduty_creds",
            "operation": "resolve_incident"
        });
        assert!(node.validate_parameters(&invalid_params).is_err());

        // Invalid urgency
        let invalid_params = json!({
            "credentials_name": "my_pagerduty_creds",
            "operation": "create_incident",
            "title": "Test",
            "service_id": "PXXXXXX",
            "urgency": "medium"
        });
        assert!(node.validate_parameters(&invalid_params).is_err());
    }
}
