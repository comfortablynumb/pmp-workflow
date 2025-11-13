use crate::models::{Node, NodeCategory, NodeContext, NodeOutput, NodeSubcategory, NodeType};
use async_trait::async_trait;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct IntegrationTestRunnerParams {
    /// Integration test operation to perform
    pub operation: String,
    /// Path to test suite directory or file
    pub test_suite_path: Option<String>,
    /// Name of specific test to run
    pub test_name: Option<String>,
    /// Timeout for test execution in seconds
    pub timeout_seconds: Option<u32>,
    /// Enable parallel test execution
    pub parallel_execution: Option<bool>,
    /// Maximum number of retries for failed tests
    pub max_retries: Option<u32>,
}

/// Integration Test Runner node - executes end-to-end workflow integration tests
pub struct IntegrationTestRunnerNode;

impl IntegrationTestRunnerNode {
    pub fn new() -> Self {
        Self
    }
}

impl Default for IntegrationTestRunnerNode {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeType for IntegrationTestRunnerNode {
    fn type_name(&self) -> &str {
        "integration_test_runner"
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
                    "description": "Integration test operation to perform",
                    "enum": [
                        "run_test_suite",
                        "run_single_test",
                        "run_smoke_tests",
                        "run_regression_tests",
                        "generate_report"
                    ]
                },
                "test_suite_path": {
                    "type": "string",
                    "description": "Path to test suite directory or file"
                },
                "test_name": {
                    "type": "string",
                    "description": "Name of specific test to run (for run_single_test)"
                },
                "timeout_seconds": {
                    "type": "integer",
                    "description": "Timeout for test execution in seconds",
                    "minimum": 1,
                    "maximum": 3600,
                    "default": 300
                },
                "parallel_execution": {
                    "type": "boolean",
                    "description": "Enable parallel test execution",
                    "default": false
                },
                "max_retries": {
                    "type": "integer",
                    "description": "Maximum number of retries for failed tests",
                    "minimum": 0,
                    "maximum": 10,
                    "default": 0
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
impl Node for IntegrationTestRunnerNode {
    async fn execute(
        &self,
        context: &NodeContext,
        parameters: &serde_json::Value,
    ) -> anyhow::Result<NodeOutput> {
        let params: IntegrationTestRunnerParams = serde_json::from_value(parameters.clone())?;

        // TODO: In a real implementation:
        // 1. For run_test_suite: Execute all tests in the specified suite
        // 2. For run_single_test: Execute a specific test by name
        // 3. For run_smoke_tests: Execute quick smoke tests to verify basic functionality
        // 4. For run_regression_tests: Execute comprehensive regression test suite
        // 5. For generate_report: Generate detailed test report with results, coverage, etc.
        // 6. Support parallel execution when enabled
        // 7. Implement retry logic for flaky tests
        // 8. Enforce timeout limits
        // 9. Capture test output, logs, and screenshots
        // 10. Return detailed test results including passed, failed, skipped counts

        // For now, we'll return a placeholder response
        let result = serde_json::json!({
            "message": "Integration test execution completed (placeholder implementation)",
            "operation": &params.operation,
            "test_suite_path": params.test_suite_path,
            "test_name": params.test_name,
            "timeout_seconds": params.timeout_seconds.unwrap_or(300),
            "parallel_execution": params.parallel_execution.unwrap_or(false),
            "max_retries": params.max_retries.unwrap_or(0),
            "context_execution_id": &context.execution_id,
            "test_results": {
                "total": 10,
                "passed": 8,
                "failed": 1,
                "skipped": 1,
                "duration_ms": 5432
            },
            "success": true
        });

        Ok(NodeOutput::success(result))
    }

    fn validate_parameters(&self, parameters: &serde_json::Value) -> anyhow::Result<()> {
        let params: IntegrationTestRunnerParams = serde_json::from_value(parameters.clone())
            .map_err(|e| anyhow::anyhow!("Invalid parameters: {}", e))?;

        // Validate operation
        let valid_operations = [
            "run_test_suite",
            "run_single_test",
            "run_smoke_tests",
            "run_regression_tests",
            "generate_report",
        ];

        if !valid_operations.contains(&params.operation.as_str()) {
            anyhow::bail!(
                "Invalid operation: {}. Must be one of: {}",
                params.operation,
                valid_operations.join(", ")
            );
        }

        // Validate timeout range
        if let Some(timeout) = params.timeout_seconds
            && !(1..=3600).contains(&timeout)
        {
            anyhow::bail!("timeout_seconds must be between 1 and 3600");
        }

        // Validate max_retries range
        if let Some(max_retries) = params.max_retries
            && !(0..=10).contains(&max_retries)
        {
            anyhow::bail!("max_retries must be between 0 and 10");
        }

        // run_single_test requires test_name
        if params.operation == "run_single_test" && params.test_name.is_none() {
            anyhow::bail!("run_single_test operation requires 'test_name' parameter");
        }

        // Operations other than generate_report require test_suite_path
        if params.operation != "generate_report" && params.test_suite_path.is_none() {
            anyhow::bail!(
                "{} operation requires 'test_suite_path' parameter",
                params.operation
            );
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
    fn test_integration_test_runner_node_type() {
        let node = IntegrationTestRunnerNode::new();
        assert_eq!(node.type_name(), "integration_test_runner");
        assert!(matches!(node.category(), NodeCategory::Action));
        assert!(matches!(node.subcategory(), NodeSubcategory::General));
        assert_eq!(node.required_credential_type(), None);
    }

    #[test]
    fn test_integration_test_runner_parameter_schema() {
        let node = IntegrationTestRunnerNode::new();
        let schema = node.parameter_schema();

        assert_eq!(schema["type"], "object");
        assert!(schema["properties"]["operation"].is_object());
        assert!(schema["properties"]["test_suite_path"].is_object());
        assert!(schema["properties"]["parallel_execution"].is_object());
    }

    #[tokio::test]
    async fn test_integration_test_runner_run_suite() {
        let node = IntegrationTestRunnerNode::new();
        let context = NodeContext::new(Uuid::new_v4().to_string(), "test-workflow".to_string());

        let params = json!({
            "operation": "run_test_suite",
            "test_suite_path": "/tests/integration",
            "parallel_execution": true,
            "timeout_seconds": 600
        });

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
    }

    #[tokio::test]
    async fn test_integration_test_runner_single_test() {
        let node = IntegrationTestRunnerNode::new();
        let context = NodeContext::new(Uuid::new_v4().to_string(), "test-workflow".to_string());

        let params = json!({
            "operation": "run_single_test",
            "test_suite_path": "/tests/integration",
            "test_name": "test_workflow_execution",
            "max_retries": 3
        });

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
    }

    #[test]
    fn test_integration_test_runner_validation() {
        let node = IntegrationTestRunnerNode::new();

        // Valid parameters
        let valid_params = json!({
            "operation": "run_smoke_tests",
            "test_suite_path": "/tests/smoke"
        });
        assert!(node.validate_parameters(&valid_params).is_ok());

        // Invalid operation
        let invalid_params = json!({
            "operation": "invalid_operation",
            "test_suite_path": "/tests"
        });
        assert!(node.validate_parameters(&invalid_params).is_err());

        // Missing test_name for run_single_test
        let invalid_params = json!({
            "operation": "run_single_test",
            "test_suite_path": "/tests"
        });
        assert!(node.validate_parameters(&invalid_params).is_err());

        // Missing test_suite_path
        let invalid_params = json!({
            "operation": "run_test_suite"
        });
        assert!(node.validate_parameters(&invalid_params).is_err());

        // Invalid timeout
        let invalid_params = json!({
            "operation": "run_test_suite",
            "test_suite_path": "/tests",
            "timeout_seconds": 5000
        });
        assert!(node.validate_parameters(&invalid_params).is_err());
    }
}
