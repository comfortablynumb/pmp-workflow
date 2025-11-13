use crate::models::{Node, NodeCategory, NodeContext, NodeOutput, NodeSubcategory, NodeType};
use async_trait::async_trait;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AssertionParams {
    /// Assertion operation to perform
    pub operation: String,
    /// Actual value to check
    pub actual_value: Option<serde_json::Value>,
    /// Expected value to compare against
    pub expected_value: Option<serde_json::Value>,
    /// JSON path to field to check (e.g., "user.email", "items[0].id")
    pub field_path: Option<String>,
    /// Custom error message if assertion fails
    pub error_message: Option<String>,
    /// Continue workflow execution even if assertion fails
    pub continue_on_failure: Option<bool>,
}

/// Assertion node - validates data and conditions within workflows
pub struct AssertionNode;

impl AssertionNode {
    pub fn new() -> Self {
        Self
    }
}

impl Default for AssertionNode {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeType for AssertionNode {
    fn type_name(&self) -> &str {
        "assertion"
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
                    "description": "Assertion operation to perform",
                    "enum": [
                        "assert_equals",
                        "assert_not_equals",
                        "assert_contains",
                        "assert_greater_than",
                        "assert_less_than",
                        "assert_exists",
                        "assert_type",
                        "assert_regex_match"
                    ]
                },
                "actual_value": {
                    "description": "Actual value to check (can be any JSON type)"
                },
                "expected_value": {
                    "description": "Expected value to compare against (can be any JSON type)"
                },
                "field_path": {
                    "type": "string",
                    "description": "JSON path to field to check (e.g., 'user.email', 'items[0].id')"
                },
                "error_message": {
                    "type": "string",
                    "description": "Custom error message if assertion fails"
                },
                "continue_on_failure": {
                    "type": "boolean",
                    "description": "Continue workflow execution even if assertion fails",
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
impl Node for AssertionNode {
    async fn execute(
        &self,
        context: &NodeContext,
        parameters: &serde_json::Value,
    ) -> anyhow::Result<NodeOutput> {
        let params: AssertionParams = serde_json::from_value(parameters.clone())?;

        // TODO: In a real implementation:
        // 1. For assert_equals: Check if actual_value equals expected_value
        // 2. For assert_not_equals: Check if actual_value does not equal expected_value
        // 3. For assert_contains: Check if actual_value (string/array) contains expected_value
        // 4. For assert_greater_than: Check if actual_value > expected_value (numeric)
        // 5. For assert_less_than: Check if actual_value < expected_value (numeric)
        // 6. For assert_exists: Check if field_path exists in actual_value
        // 7. For assert_type: Check if actual_value is of expected type
        // 8. For assert_regex_match: Check if actual_value matches regex pattern in expected_value
        // 9. If field_path is provided, extract value from nested structure
        // 10. If assertion fails and continue_on_failure is false, return error
        // 11. If assertion fails and continue_on_failure is true, return success with warning

        // For now, we'll return a placeholder response
        let result = serde_json::json!({
            "message": "Assertion executed (placeholder implementation)",
            "operation": &params.operation,
            "actual_value": params.actual_value,
            "expected_value": params.expected_value,
            "field_path": params.field_path,
            "continue_on_failure": params.continue_on_failure.unwrap_or(false),
            "context_execution_id": &context.execution_id,
            "assertion_result": {
                "passed": true,
                "details": "Assertion placeholder - actual validation not yet implemented"
            },
            "success": true
        });

        Ok(NodeOutput::success(result))
    }

    fn validate_parameters(&self, parameters: &serde_json::Value) -> anyhow::Result<()> {
        let params: AssertionParams = serde_json::from_value(parameters.clone())
            .map_err(|e| anyhow::anyhow!("Invalid parameters: {}", e))?;

        // Validate operation
        let valid_operations = [
            "assert_equals",
            "assert_not_equals",
            "assert_contains",
            "assert_greater_than",
            "assert_less_than",
            "assert_exists",
            "assert_type",
            "assert_regex_match",
        ];

        if !valid_operations.contains(&params.operation.as_str()) {
            anyhow::bail!(
                "Invalid operation: {}. Must be one of: {}",
                params.operation,
                valid_operations.join(", ")
            );
        }

        // Most operations require actual_value
        let requires_actual = [
            "assert_equals",
            "assert_not_equals",
            "assert_contains",
            "assert_greater_than",
            "assert_less_than",
            "assert_type",
            "assert_regex_match",
        ];
        if requires_actual.contains(&params.operation.as_str()) && params.actual_value.is_none() {
            anyhow::bail!(
                "{} operation requires 'actual_value' parameter",
                params.operation
            );
        }

        // Comparison operations require expected_value
        let requires_expected = [
            "assert_equals",
            "assert_not_equals",
            "assert_contains",
            "assert_greater_than",
            "assert_less_than",
            "assert_type",
            "assert_regex_match",
        ];
        if requires_expected.contains(&params.operation.as_str()) && params.expected_value.is_none()
        {
            anyhow::bail!(
                "{} operation requires 'expected_value' parameter",
                params.operation
            );
        }

        // assert_exists can use either actual_value or field_path
        if params.operation == "assert_exists"
            && params.actual_value.is_none()
            && params.field_path.is_none()
        {
            anyhow::bail!(
                "assert_exists operation requires either 'actual_value' or 'field_path' parameter"
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
    fn test_assertion_node_type() {
        let node = AssertionNode::new();
        assert_eq!(node.type_name(), "assertion");
        assert!(matches!(node.category(), NodeCategory::Action));
        assert!(matches!(node.subcategory(), NodeSubcategory::General));
        assert_eq!(node.required_credential_type(), None);
    }

    #[test]
    fn test_assertion_parameter_schema() {
        let node = AssertionNode::new();
        let schema = node.parameter_schema();

        assert_eq!(schema["type"], "object");
        assert!(schema["properties"]["operation"].is_object());
        assert!(schema["properties"]["actual_value"].is_object());
        assert!(schema["properties"]["expected_value"].is_object());
    }

    #[tokio::test]
    async fn test_assertion_assert_equals() {
        let node = AssertionNode::new();
        let context = NodeContext::new(Uuid::new_v4().to_string(), "test-workflow".to_string());

        let params = json!({
            "operation": "assert_equals",
            "actual_value": 42,
            "expected_value": 42
        });

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
    }

    #[tokio::test]
    async fn test_assertion_with_field_path() {
        let node = AssertionNode::new();
        let context = NodeContext::new(Uuid::new_v4().to_string(), "test-workflow".to_string());

        let params = json!({
            "operation": "assert_contains",
            "actual_value": "hello world",
            "expected_value": "world",
            "field_path": "message.text"
        });

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
    }

    #[test]
    fn test_assertion_validation() {
        let node = AssertionNode::new();

        // Valid parameters
        let valid_params = json!({
            "operation": "assert_equals",
            "actual_value": 10,
            "expected_value": 10
        });
        assert!(node.validate_parameters(&valid_params).is_ok());

        // Invalid operation
        let invalid_params = json!({
            "operation": "invalid_operation",
            "actual_value": 10
        });
        assert!(node.validate_parameters(&invalid_params).is_err());

        // Missing actual_value
        let invalid_params = json!({
            "operation": "assert_equals",
            "expected_value": 10
        });
        assert!(node.validate_parameters(&invalid_params).is_err());

        // Missing expected_value
        let invalid_params = json!({
            "operation": "assert_greater_than",
            "actual_value": 10
        });
        assert!(node.validate_parameters(&invalid_params).is_err());

        // assert_exists without actual_value or field_path
        let invalid_params = json!({
            "operation": "assert_exists"
        });
        assert!(node.validate_parameters(&invalid_params).is_err());
    }
}
