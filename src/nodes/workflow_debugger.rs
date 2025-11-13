use crate::models::{Node, NodeCategory, NodeContext, NodeOutput, NodeSubcategory, NodeType};
use async_trait::async_trait;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct WorkflowDebuggerParams {
    /// Debugger operation to perform
    pub operation: String,
    /// Workflow ID being debugged
    pub workflow_id: Option<String>,
    /// Execution ID of the running workflow
    pub execution_id: Option<String>,
    /// Node ID for breakpoint operations
    pub node_id: Option<String>,
    /// Condition for conditional breakpoints
    pub breakpoint_condition: Option<String>,
    /// Variable name to inspect
    pub variable_name: Option<String>,
    /// Debug session ID
    pub session_id: Option<String>,
}

/// Workflow Debugger node - debug workflows interactively with breakpoints and step execution
pub struct WorkflowDebuggerNode;

impl WorkflowDebuggerNode {
    pub fn new() -> Self {
        Self
    }
}

impl Default for WorkflowDebuggerNode {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeType for WorkflowDebuggerNode {
    fn type_name(&self) -> &str {
        "workflow_debugger"
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
                    "description": "Debugger operation to perform",
                    "enum": [
                        "start_debug_session",
                        "set_breakpoint",
                        "remove_breakpoint",
                        "step_over",
                        "step_into",
                        "continue_execution",
                        "inspect_variables",
                        "get_call_stack"
                    ]
                },
                "workflow_id": {
                    "type": "string",
                    "description": "Workflow ID to debug"
                },
                "execution_id": {
                    "type": "string",
                    "description": "Execution ID of the running workflow"
                },
                "node_id": {
                    "type": "string",
                    "description": "Node ID for breakpoint operations"
                },
                "breakpoint_condition": {
                    "type": "string",
                    "description": "Condition expression for conditional breakpoints"
                },
                "variable_name": {
                    "type": "string",
                    "description": "Variable name to inspect"
                },
                "session_id": {
                    "type": "string",
                    "description": "Debug session ID"
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
impl Node for WorkflowDebuggerNode {
    async fn execute(
        &self,
        context: &NodeContext,
        parameters: &serde_json::Value,
    ) -> anyhow::Result<NodeOutput> {
        let params: WorkflowDebuggerParams = serde_json::from_value(parameters.clone())?;

        // TODO: In a real implementation:
        // 1. Manage debug sessions with session state
        // 2. Based on operation, perform debugging actions:
        //    - start_debug_session: Create a new debug session for a workflow
        //    - set_breakpoint: Set a breakpoint at a specific node (with optional condition)
        //    - remove_breakpoint: Remove a breakpoint from a node
        //    - step_over: Execute current node and pause at next node
        //    - step_into: Step into nested workflow execution
        //    - continue_execution: Resume execution until next breakpoint
        //    - inspect_variables: Get current values of workflow variables
        //    - get_call_stack: Get the current execution call stack
        // 3. Maintain breakpoint state and workflow execution state
        // 4. Evaluate breakpoint conditions
        // 5. Return debug information including current node, variables, and call stack

        // For now, we'll return a placeholder response
        let result = serde_json::json!({
            "message": "Workflow debugger operation executed (placeholder implementation)",
            "operation": &params.operation,
            "workflow_id": params.workflow_id,
            "execution_id": params.execution_id,
            "node_id": params.node_id,
            "session_id": params.session_id,
            "breakpoint_condition": params.breakpoint_condition,
            "variable_name": params.variable_name,
            "context_execution_id": &context.execution_id,
            "success": true
        });

        Ok(NodeOutput::success(result))
    }

    fn validate_parameters(&self, parameters: &serde_json::Value) -> anyhow::Result<()> {
        let params: WorkflowDebuggerParams = serde_json::from_value(parameters.clone())
            .map_err(|e| anyhow::anyhow!("Invalid parameters: {}", e))?;

        // Validate operation
        let valid_operations = [
            "start_debug_session",
            "set_breakpoint",
            "remove_breakpoint",
            "step_over",
            "step_into",
            "continue_execution",
            "inspect_variables",
            "get_call_stack",
        ];

        if !valid_operations.contains(&params.operation.as_str()) {
            anyhow::bail!(
                "Invalid operation: {}. Must be one of: {}",
                params.operation,
                valid_operations.join(", ")
            );
        }

        // Validate required parameters based on operation
        match params.operation.as_str() {
            "start_debug_session" => {
                if params.workflow_id.is_none() {
                    anyhow::bail!("start_debug_session operation requires 'workflow_id' parameter");
                }
            }
            "set_breakpoint" | "remove_breakpoint" => {
                if params.session_id.is_none() {
                    anyhow::bail!(
                        "{} operation requires 'session_id' parameter",
                        params.operation
                    );
                }
                if params.node_id.is_none() {
                    anyhow::bail!(
                        "{} operation requires 'node_id' parameter",
                        params.operation
                    );
                }
            }
            "step_over" | "step_into" | "continue_execution" | "get_call_stack" => {
                if params.session_id.is_none() {
                    anyhow::bail!(
                        "{} operation requires 'session_id' parameter",
                        params.operation
                    );
                }
            }
            "inspect_variables" => {
                if params.session_id.is_none() {
                    anyhow::bail!("inspect_variables operation requires 'session_id' parameter");
                }
            }
            _ => {}
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
    fn test_workflow_debugger_node_type() {
        let node = WorkflowDebuggerNode::new();
        assert_eq!(node.type_name(), "workflow_debugger");
        assert!(matches!(node.category(), NodeCategory::Action));
        assert!(matches!(node.subcategory(), NodeSubcategory::General));
        assert_eq!(node.required_credential_type(), None);
    }

    #[test]
    fn test_workflow_debugger_parameter_schema() {
        let node = WorkflowDebuggerNode::new();
        let schema = node.parameter_schema();

        assert_eq!(schema["type"], "object");
        assert!(schema["properties"]["operation"].is_object());
        assert!(schema["properties"]["workflow_id"].is_object());
        assert!(schema["properties"]["session_id"].is_object());
        assert!(schema["properties"]["node_id"].is_object());
    }

    #[tokio::test]
    async fn test_workflow_debugger_start_session() {
        let node = WorkflowDebuggerNode::new();
        let context = NodeContext::new(Uuid::new_v4().to_string(), "test-workflow".to_string());

        let params = json!({
            "operation": "start_debug_session",
            "workflow_id": "workflow-123"
        });

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
    }

    #[tokio::test]
    async fn test_workflow_debugger_set_breakpoint() {
        let node = WorkflowDebuggerNode::new();
        let context = NodeContext::new(Uuid::new_v4().to_string(), "test-workflow".to_string());

        let params = json!({
            "operation": "set_breakpoint",
            "session_id": "session-123",
            "node_id": "node-456",
            "breakpoint_condition": "output.value > 100"
        });

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
    }

    #[tokio::test]
    async fn test_workflow_debugger_step_over() {
        let node = WorkflowDebuggerNode::new();
        let context = NodeContext::new(Uuid::new_v4().to_string(), "test-workflow".to_string());

        let params = json!({
            "operation": "step_over",
            "session_id": "session-123"
        });

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
    }

    #[tokio::test]
    async fn test_workflow_debugger_inspect_variables() {
        let node = WorkflowDebuggerNode::new();
        let context = NodeContext::new(Uuid::new_v4().to_string(), "test-workflow".to_string());

        let params = json!({
            "operation": "inspect_variables",
            "session_id": "session-123",
            "variable_name": "user_data"
        });

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
    }

    #[test]
    fn test_workflow_debugger_validation() {
        let node = WorkflowDebuggerNode::new();

        // Valid start_debug_session
        let valid_params = json!({
            "operation": "start_debug_session",
            "workflow_id": "workflow-123"
        });
        assert!(node.validate_parameters(&valid_params).is_ok());

        // Valid set_breakpoint
        let valid_params = json!({
            "operation": "set_breakpoint",
            "session_id": "session-123",
            "node_id": "node-456"
        });
        assert!(node.validate_parameters(&valid_params).is_ok());

        // Missing workflow_id for start_debug_session
        let invalid_params = json!({
            "operation": "start_debug_session"
        });
        assert!(node.validate_parameters(&invalid_params).is_err());

        // Missing session_id for set_breakpoint
        let invalid_params = json!({
            "operation": "set_breakpoint",
            "node_id": "node-456"
        });
        assert!(node.validate_parameters(&invalid_params).is_err());

        // Missing node_id for set_breakpoint
        let invalid_params = json!({
            "operation": "set_breakpoint",
            "session_id": "session-123"
        });
        assert!(node.validate_parameters(&invalid_params).is_err());

        // Invalid operation
        let invalid_params = json!({
            "operation": "invalid_operation"
        });
        assert!(node.validate_parameters(&invalid_params).is_err());
    }
}
