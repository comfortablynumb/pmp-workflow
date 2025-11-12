use crate::db;
use crate::execution::WorkflowEngine;
use crate::models::{Node, NodeContext, NodeOutput, NodeRegistry};
use async_trait::async_trait;
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
struct ExecuteWorkflowParams {
    /// Workflow ID to execute
    workflow_id: Option<String>,
    /// Workflow name to execute (alternative to workflow_id)
    workflow_name: Option<String>,
    /// Input data to pass to the sub-workflow
    input: Option<serde_json::Value>,
    /// Whether to wait for the workflow to complete (default: true)
    #[serde(default = "default_wait")]
    wait: bool,
}

fn default_wait() -> bool {
    true
}

/// Execute Workflow node - executes another workflow as a sub-workflow
pub struct ExecuteWorkflowNode {
    pool: PgPool,
}

impl ExecuteWorkflowNode {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl Node for ExecuteWorkflowNode {
    fn node_type(&self) -> &str {
        "execute_workflow"
    }

    async fn execute(
        &self,
        context: &NodeContext,
        parameters: &serde_json::Value,
    ) -> anyhow::Result<NodeOutput> {
        let params: ExecuteWorkflowParams = serde_json::from_value(parameters.clone())?;

        // Determine input data: use provided input or pass through context input
        let input_data = if let Some(input) = params.input {
            Some(input)
        } else {
            context.get_main_input().cloned()
        };

        // Load the workflow by ID or name
        let workflow = if let Some(workflow_id_str) = params.workflow_id {
            let workflow_id = Uuid::parse_str(&workflow_id_str)
                .map_err(|e| anyhow::anyhow!("Invalid workflow ID: {}", e))?;
            db::get_workflow(&self.pool, workflow_id).await?
        } else if let Some(workflow_name) = params.workflow_name {
            db::get_workflow_by_name(&self.pool, &workflow_name).await?
        } else {
            anyhow::bail!("Either workflow_id or workflow_name must be specified");
        };

        // Check if workflow is active
        if !workflow.active {
            anyhow::bail!("Cannot execute inactive workflow: {}", workflow.name);
        }

        // Convert to workflow definition
        let workflow_def = workflow.to_definition()?;

        // Create execution engine with a full registry
        let mut registry = NodeRegistry::new();
        crate::nodes::register_builtin_nodes(&mut registry, &self.pool);
        let engine = WorkflowEngine::new(self.pool.clone(), registry);

        if params.wait {
            // Execute the workflow and wait for completion
            tracing::info!(
                "Executing sub-workflow '{}' (ID: {})",
                workflow.name,
                workflow.id
            );

            let execution = engine
                .execute_workflow(&workflow_def, workflow.id, input_data)
                .await?;

            // Check if execution was successful
            if execution.status == crate::models::ExecutionStatus::Success {
                Ok(NodeOutput::success(serde_json::json!({
                    "execution_id": execution.id,
                    "workflow_id": workflow.id,
                    "workflow_name": workflow.name,
                    "status": "success",
                    "output": execution.output_data
                })))
            } else {
                Ok(NodeOutput::error(format!(
                    "Sub-workflow '{}' failed: {}",
                    workflow.name,
                    execution
                        .error
                        .unwrap_or_else(|| "Unknown error".to_string())
                )))
            }
        } else {
            // Fire and forget - start the workflow but don't wait
            tokio::spawn(async move {
                let _ = engine
                    .execute_workflow(&workflow_def, workflow.id, input_data)
                    .await;
            });

            Ok(NodeOutput::success(serde_json::json!({
                "workflow_id": workflow.id,
                "workflow_name": workflow.name,
                "status": "started",
                "wait": false
            })))
        }
    }

    fn validate_parameters(&self, parameters: &serde_json::Value) -> anyhow::Result<()> {
        let params: ExecuteWorkflowParams = serde_json::from_value(parameters.clone())?;

        // Either workflow_id or workflow_name must be specified
        if params.workflow_id.is_none() && params.workflow_name.is_none() {
            anyhow::bail!("Either 'workflow_id' or 'workflow_name' must be specified");
        }

        // Both cannot be specified at the same time
        if params.workflow_id.is_some() && params.workflow_name.is_some() {
            anyhow::bail!("Cannot specify both 'workflow_id' and 'workflow_name'");
        }

        // Validate workflow_id format if provided
        if let Some(ref workflow_id_str) = params.workflow_id {
            Uuid::parse_str(workflow_id_str)
                .map_err(|e| anyhow::anyhow!("Invalid workflow ID format: {}", e))?;
        }

        Ok(())
    }

    fn parameter_schema(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "workflow_id": {
                    "type": "string",
                    "description": "UUID of the workflow to execute",
                    "format": "uuid"
                },
                "workflow_name": {
                    "type": "string",
                    "description": "Name of the workflow to execute (alternative to workflow_id)"
                },
                "input": {
                    "description": "Input data to pass to the sub-workflow. If not specified, passes through the current node's input."
                },
                "wait": {
                    "type": "boolean",
                    "description": "Whether to wait for the sub-workflow to complete",
                    "default": true
                }
            },
            "oneOf": [
                {
                    "required": ["workflow_id"]
                },
                {
                    "required": ["workflow_name"]
                }
            ],
            "additionalProperties": false
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to create a mock pool for testing
    fn create_test_node() -> ExecuteWorkflowNode {
        // Use a dummy pool URL for validation tests (no actual connection needed)
        let pool = PgPool::connect_lazy("postgres://localhost/test").unwrap();
        ExecuteWorkflowNode::new(pool)
    }

    #[tokio::test]
    async fn test_validate_parameters_success() {
        let node = create_test_node();

        // Test with workflow_id
        let params = serde_json::json!({
            "workflow_id": "550e8400-e29b-41d4-a716-446655440000"
        });
        assert!(node.validate_parameters(&params).is_ok());

        // Test with workflow_name
        let params = serde_json::json!({
            "workflow_name": "My Workflow"
        });
        assert!(node.validate_parameters(&params).is_ok());
    }

    #[tokio::test]
    async fn test_validate_parameters_missing_both() {
        let node = create_test_node();

        let params = serde_json::json!({});
        assert!(node.validate_parameters(&params).is_err());
    }

    #[tokio::test]
    async fn test_validate_parameters_both_specified() {
        let node = create_test_node();

        let params = serde_json::json!({
            "workflow_id": "550e8400-e29b-41d4-a716-446655440000",
            "workflow_name": "My Workflow"
        });
        assert!(node.validate_parameters(&params).is_err());
    }

    #[tokio::test]
    async fn test_validate_parameters_invalid_uuid() {
        let node = create_test_node();

        let params = serde_json::json!({
            "workflow_id": "invalid-uuid"
        });
        assert!(node.validate_parameters(&params).is_err());
    }
}
