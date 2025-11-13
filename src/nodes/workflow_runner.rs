use crate::models::{Node, NodeCategory, NodeContext, NodeOutput, NodeSubcategory, NodeType};
use async_trait::async_trait;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct WorkflowRunnerParams {
    /// Workflow runner operation to perform
    pub operation: String,
    /// Workflow YAML content (for run_workflow, dry_run, validate_and_run)
    pub workflow_yaml: Option<String>,
    /// Path to workflow file (for run_workflow_file)
    pub workflow_path: Option<String>,
    /// Workflow ID to run (for run_workflow)
    pub workflow_id: Option<String>,
    /// Input parameters for the workflow execution
    #[serde(default)]
    pub inputs: serde_json::Value,
    /// Enable dry run mode (validate without executing)
    pub dry_run: Option<bool>,
    /// Timeout in seconds for workflow execution
    pub timeout_seconds: Option<u64>,
    /// Execute independent nodes in parallel
    pub parallel: Option<bool>,
    /// Continue execution even if a node fails
    pub continue_on_error: Option<bool>,
}

/// Workflow Runner node - executes workflows programmatically with various execution options
pub struct WorkflowRunnerNode;

impl WorkflowRunnerNode {
    pub fn new() -> Self {
        Self
    }
}

impl Default for WorkflowRunnerNode {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeType for WorkflowRunnerNode {
    fn type_name(&self) -> &str {
        "workflow_runner"
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
                    "description": "Workflow runner operation to perform",
                    "enum": [
                        "run_workflow",
                        "run_workflow_file",
                        "dry_run",
                        "validate_and_run",
                        "run_with_inputs"
                    ]
                },
                "workflow_yaml": {
                    "type": "string",
                    "description": "Workflow YAML content to execute"
                },
                "workflow_path": {
                    "type": "string",
                    "description": "Path to workflow YAML file"
                },
                "workflow_id": {
                    "type": "string",
                    "description": "Workflow ID to run from database"
                },
                "inputs": {
                    "type": "object",
                    "description": "Input parameters for workflow execution",
                    "default": {}
                },
                "dry_run": {
                    "type": "boolean",
                    "description": "Validate workflow without executing",
                    "default": false
                },
                "timeout_seconds": {
                    "type": "integer",
                    "description": "Timeout in seconds for workflow execution",
                    "minimum": 1,
                    "maximum": 3600
                },
                "parallel": {
                    "type": "boolean",
                    "description": "Execute independent nodes in parallel",
                    "default": false
                },
                "continue_on_error": {
                    "type": "boolean",
                    "description": "Continue execution even if a node fails",
                    "default": false
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
impl Node for WorkflowRunnerNode {
    async fn execute(
        &self,
        context: &NodeContext,
        parameters: &serde_json::Value,
    ) -> anyhow::Result<NodeOutput> {
        let params: WorkflowRunnerParams = serde_json::from_value(parameters.clone())?;

        // TODO: In a real implementation:
        // 1. Parse and validate the workflow YAML based on the operation:
        //    - run_workflow: Parse workflow_yaml and execute
        //    - run_workflow_file: Load workflow from workflow_path and execute
        //    - dry_run: Validate workflow without executing
        //    - validate_and_run: Validate then execute if valid
        //    - run_with_inputs: Execute workflow with provided inputs
        // 2. Create a new workflow execution context
        // 3. If parallel is true, identify independent nodes and execute in parallel
        // 4. Apply timeout if specified
        // 5. Handle errors based on continue_on_error flag
        // 6. Return execution results with node outputs, execution time, and status

        // For now, we'll return a placeholder response
        let result = serde_json::json!({
            "message": "Workflow runner operation executed (placeholder implementation)",
            "operation": &params.operation,
            "workflow_yaml_provided": params.workflow_yaml.is_some(),
            "workflow_path": params.workflow_path,
            "workflow_id": params.workflow_id,
            "dry_run": params.dry_run.unwrap_or(false),
            "timeout_seconds": params.timeout_seconds,
            "parallel": params.parallel.unwrap_or(false),
            "continue_on_error": params.continue_on_error.unwrap_or(false),
            "context_execution_id": &context.execution_id,
            "success": true
        });

        Ok(NodeOutput::success(result))
    }

    fn validate_parameters(&self, parameters: &serde_json::Value) -> anyhow::Result<()> {
        let params: WorkflowRunnerParams = serde_json::from_value(parameters.clone())
            .map_err(|e| anyhow::anyhow!("Invalid parameters: {}", e))?;

        // Validate operation
        let valid_operations = [
            "run_workflow",
            "run_workflow_file",
            "dry_run",
            "validate_and_run",
            "run_with_inputs",
        ];

        if !valid_operations.contains(&params.operation.as_str()) {
            anyhow::bail!(
                "Invalid operation: {}. Must be one of: {}",
                params.operation,
                valid_operations.join(", ")
            );
        }

        // Validate that appropriate workflow source is provided
        match params.operation.as_str() {
            "run_workflow" | "dry_run" | "validate_and_run" => {
                if params.workflow_yaml.is_none() && params.workflow_id.is_none() {
                    anyhow::bail!(
                        "{} operation requires either 'workflow_yaml' or 'workflow_id' parameter",
                        params.operation
                    );
                }
            }
            "run_workflow_file" => {
                if params.workflow_path.is_none() {
                    anyhow::bail!(
                        "{} operation requires 'workflow_path' parameter",
                        params.operation
                    );
                }
            }
            "run_with_inputs" => {
                if params.workflow_yaml.is_none()
                    && params.workflow_path.is_none()
                    && params.workflow_id.is_none()
                {
                    anyhow::bail!(
                        "{} operation requires 'workflow_yaml', 'workflow_path', or 'workflow_id' parameter",
                        params.operation
                    );
                }
            }
            _ => {}
        }

        // Validate timeout_seconds range
        if let Some(timeout) = params.timeout_seconds
            && !(1..=3600).contains(&timeout)
        {
            anyhow::bail!("timeout_seconds must be between 1 and 3600");
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
    fn test_workflow_runner_node_type() {
        let node = WorkflowRunnerNode::new();
        assert_eq!(node.type_name(), "workflow_runner");
        assert!(matches!(node.category(), NodeCategory::Action));
        assert!(matches!(node.subcategory(), NodeSubcategory::General));
        assert_eq!(node.required_credential_type(), None);
    }

    #[test]
    fn test_workflow_runner_parameter_schema() {
        let node = WorkflowRunnerNode::new();
        let schema = node.parameter_schema();

        assert_eq!(schema["type"], "object");
        assert!(schema["properties"]["operation"].is_object());
        assert!(schema["properties"]["workflow_yaml"].is_object());
        assert!(schema["properties"]["workflow_path"].is_object());
        assert!(schema["properties"]["inputs"].is_object());
    }

    #[tokio::test]
    async fn test_workflow_runner_run_workflow() {
        let node = WorkflowRunnerNode::new();
        let context = NodeContext::new(Uuid::new_v4().to_string(), "test-workflow".to_string());

        let params = json!({
            "operation": "run_workflow",
            "workflow_yaml": "name: test\nnodes: []",
            "inputs": {"key": "value"},
            "parallel": true,
            "timeout_seconds": 300
        });

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
    }

    #[tokio::test]
    async fn test_workflow_runner_dry_run() {
        let node = WorkflowRunnerNode::new();
        let context = NodeContext::new(Uuid::new_v4().to_string(), "test-workflow".to_string());

        let params = json!({
            "operation": "dry_run",
            "workflow_yaml": "name: test\nnodes: []",
            "dry_run": true
        });

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
    }

    #[test]
    fn test_workflow_runner_validation() {
        let node = WorkflowRunnerNode::new();

        // Valid run_workflow
        let valid_params = json!({
            "operation": "run_workflow",
            "workflow_yaml": "name: test\nnodes: []"
        });
        assert!(node.validate_parameters(&valid_params).is_ok());

        // Valid run_workflow_file
        let valid_params = json!({
            "operation": "run_workflow_file",
            "workflow_path": "/path/to/workflow.yaml"
        });
        assert!(node.validate_parameters(&valid_params).is_ok());

        // Missing workflow source for run_workflow
        let invalid_params = json!({
            "operation": "run_workflow"
        });
        assert!(node.validate_parameters(&invalid_params).is_err());

        // Missing workflow_path for run_workflow_file
        let invalid_params = json!({
            "operation": "run_workflow_file"
        });
        assert!(node.validate_parameters(&invalid_params).is_err());

        // Invalid operation
        let invalid_params = json!({
            "operation": "invalid_operation"
        });
        assert!(node.validate_parameters(&invalid_params).is_err());

        // Invalid timeout range
        let invalid_params = json!({
            "operation": "run_workflow",
            "workflow_yaml": "name: test",
            "timeout_seconds": 5000
        });
        assert!(node.validate_parameters(&invalid_params).is_err());
    }
}
