use crate::models::{Node, NodeCategory, NodeContext, NodeOutput, NodeSubcategory, NodeType};
use async_trait::async_trait;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct WorkflowValidatorParams {
    /// Workflow validation operation to perform
    pub operation: String,
    /// YAML content to validate (alternative to workflow_path)
    pub workflow_yaml: Option<String>,
    /// Path to workflow YAML file
    pub workflow_path: Option<String>,
    /// Enable strict validation mode
    pub strict_mode: Option<bool>,
    /// Check that all referenced credentials exist
    pub check_credentials: Option<bool>,
}

/// Workflow Validator node - validates workflow YAML structure and integrity before execution
pub struct WorkflowValidatorNode;

impl WorkflowValidatorNode {
    pub fn new() -> Self {
        Self
    }
}

impl Default for WorkflowValidatorNode {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeType for WorkflowValidatorNode {
    fn type_name(&self) -> &str {
        "workflow_validator"
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
                    "description": "Workflow validation operation to perform",
                    "enum": [
                        "validate_yaml",
                        "validate_structure",
                        "validate_nodes",
                        "validate_connections",
                        "validate_credentials",
                        "full_validation"
                    ]
                },
                "workflow_yaml": {
                    "type": "string",
                    "description": "YAML content to validate (alternative to workflow_path)"
                },
                "workflow_path": {
                    "type": "string",
                    "description": "Path to workflow YAML file to validate"
                },
                "strict_mode": {
                    "type": "boolean",
                    "description": "Enable strict validation mode (fails on warnings)",
                    "default": false
                },
                "check_credentials": {
                    "type": "boolean",
                    "description": "Check that all referenced credentials exist",
                    "default": true
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
impl Node for WorkflowValidatorNode {
    async fn execute(
        &self,
        context: &NodeContext,
        parameters: &serde_json::Value,
    ) -> anyhow::Result<NodeOutput> {
        let params: WorkflowValidatorParams = serde_json::from_value(parameters.clone())?;

        // TODO: In a real implementation:
        // 1. For validate_yaml: Parse YAML and check for syntax errors
        // 2. For validate_structure: Verify workflow has required fields (name, nodes, connections)
        // 3. For validate_nodes: Check all node types are valid and have required parameters
        // 4. For validate_connections: Verify all connections reference existing nodes
        // 5. For validate_credentials: Check all credential references exist in database
        // 6. For full_validation: Run all validation checks
        // 7. Return detailed validation report with errors, warnings, and suggestions
        // 8. If strict_mode is enabled, treat warnings as errors

        // For now, we'll return a placeholder response
        let result = serde_json::json!({
            "message": "Workflow validation executed (placeholder implementation)",
            "operation": &params.operation,
            "strict_mode": params.strict_mode.unwrap_or(false),
            "check_credentials": params.check_credentials.unwrap_or(true),
            "has_workflow_yaml": params.workflow_yaml.is_some(),
            "has_workflow_path": params.workflow_path.is_some(),
            "context_execution_id": &context.execution_id,
            "validation_result": {
                "valid": true,
                "errors": [],
                "warnings": [],
                "info": ["Validation placeholder - actual validation not yet implemented"]
            },
            "success": true
        });

        Ok(NodeOutput::success(result))
    }

    fn validate_parameters(&self, parameters: &serde_json::Value) -> anyhow::Result<()> {
        let params: WorkflowValidatorParams = serde_json::from_value(parameters.clone())
            .map_err(|e| anyhow::anyhow!("Invalid parameters: {}", e))?;

        // Validate operation
        let valid_operations = [
            "validate_yaml",
            "validate_structure",
            "validate_nodes",
            "validate_connections",
            "validate_credentials",
            "full_validation",
        ];

        if !valid_operations.contains(&params.operation.as_str()) {
            anyhow::bail!(
                "Invalid operation: {}. Must be one of: {}",
                params.operation,
                valid_operations.join(", ")
            );
        }

        // Require either workflow_yaml or workflow_path
        if params.workflow_yaml.is_none() && params.workflow_path.is_none() {
            anyhow::bail!("Either 'workflow_yaml' or 'workflow_path' must be provided");
        }

        // Don't allow both
        if params.workflow_yaml.is_some() && params.workflow_path.is_some() {
            anyhow::bail!("Cannot specify both 'workflow_yaml' and 'workflow_path'");
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
    fn test_workflow_validator_node_type() {
        let node = WorkflowValidatorNode::new();
        assert_eq!(node.type_name(), "workflow_validator");
        assert!(matches!(node.category(), NodeCategory::Action));
        assert!(matches!(node.subcategory(), NodeSubcategory::General));
        assert_eq!(node.required_credential_type(), None);
    }

    #[test]
    fn test_workflow_validator_parameter_schema() {
        let node = WorkflowValidatorNode::new();
        let schema = node.parameter_schema();

        assert_eq!(schema["type"], "object");
        assert!(schema["properties"]["operation"].is_object());
        assert!(schema["properties"]["workflow_yaml"].is_object());
        assert!(schema["properties"]["strict_mode"].is_object());
    }

    #[tokio::test]
    async fn test_workflow_validator_validate_yaml() {
        let node = WorkflowValidatorNode::new();
        let context = NodeContext::new(Uuid::new_v4().to_string(), "test-workflow".to_string());

        let params = json!({
            "operation": "validate_yaml",
            "workflow_yaml": "name: test\nnodes: []",
            "strict_mode": true
        });

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
    }

    #[tokio::test]
    async fn test_workflow_validator_full_validation() {
        let node = WorkflowValidatorNode::new();
        let context = NodeContext::new(Uuid::new_v4().to_string(), "test-workflow".to_string());

        let params = json!({
            "operation": "full_validation",
            "workflow_path": "/path/to/workflow.yaml",
            "check_credentials": true
        });

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
    }

    #[test]
    fn test_workflow_validator_validation() {
        let node = WorkflowValidatorNode::new();

        // Valid parameters with workflow_yaml
        let valid_params = json!({
            "operation": "validate_structure",
            "workflow_yaml": "name: test"
        });
        assert!(node.validate_parameters(&valid_params).is_ok());

        // Valid parameters with workflow_path
        let valid_params = json!({
            "operation": "validate_nodes",
            "workflow_path": "/path/to/workflow.yaml"
        });
        assert!(node.validate_parameters(&valid_params).is_ok());

        // Invalid operation
        let invalid_params = json!({
            "operation": "invalid_operation",
            "workflow_yaml": "test"
        });
        assert!(node.validate_parameters(&invalid_params).is_err());

        // Missing both workflow_yaml and workflow_path
        let invalid_params = json!({
            "operation": "validate_yaml"
        });
        assert!(node.validate_parameters(&invalid_params).is_err());

        // Both workflow_yaml and workflow_path specified
        let invalid_params = json!({
            "operation": "validate_yaml",
            "workflow_yaml": "test",
            "workflow_path": "/path/to/workflow.yaml"
        });
        assert!(node.validate_parameters(&invalid_params).is_err());
    }
}
