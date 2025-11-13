use crate::models::{Node, NodeCategory, NodeContext, NodeOutput, NodeSubcategory, NodeType};
use async_trait::async_trait;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TestDataGeneratorParams {
    /// Test data generation operation to perform
    pub operation: String,
    /// Type of data to generate (person, company, address, etc.)
    pub data_type: Option<String>,
    /// Number of items to generate
    pub count: Option<u32>,
    /// Locale for data generation (e.g., "en_US", "de_DE")
    pub locale: Option<String>,
    /// Custom schema for generate_custom operation
    #[serde(default)]
    pub custom_schema: serde_json::Value,
    /// Seed for reproducible random generation
    pub seed: Option<u64>,
}

/// Test Data Generator node - generates realistic test data for various data types
pub struct TestDataGeneratorNode;

impl TestDataGeneratorNode {
    pub fn new() -> Self {
        Self
    }
}

impl Default for TestDataGeneratorNode {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeType for TestDataGeneratorNode {
    fn type_name(&self) -> &str {
        "test_data_generator"
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
                    "description": "Test data generation operation to perform",
                    "enum": [
                        "generate_person",
                        "generate_company",
                        "generate_address",
                        "generate_email",
                        "generate_phone",
                        "generate_credit_card",
                        "generate_custom",
                        "generate_array"
                    ]
                },
                "data_type": {
                    "type": "string",
                    "description": "Type of data to generate (for generate_array)"
                },
                "count": {
                    "type": "integer",
                    "description": "Number of items to generate",
                    "minimum": 1,
                    "maximum": 10000,
                    "default": 1
                },
                "locale": {
                    "type": "string",
                    "description": "Locale for data generation (e.g., 'en_US', 'de_DE', 'fr_FR')",
                    "default": "en_US"
                },
                "custom_schema": {
                    "type": "object",
                    "description": "Custom schema for generate_custom operation (JSON schema)"
                },
                "seed": {
                    "type": "integer",
                    "description": "Seed for reproducible random generation",
                    "minimum": 0
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
impl Node for TestDataGeneratorNode {
    async fn execute(
        &self,
        context: &NodeContext,
        parameters: &serde_json::Value,
    ) -> anyhow::Result<NodeOutput> {
        let params: TestDataGeneratorParams = serde_json::from_value(parameters.clone())?;

        // TODO: In a real implementation:
        // 1. Use a faker library (e.g., fake-rs) to generate realistic test data
        // 2. Support multiple locales for international data generation
        // 3. For generate_person: Generate name, email, phone, address, birthdate, etc.
        // 4. For generate_company: Generate company name, industry, website, etc.
        // 5. For generate_address: Generate street, city, state, zip, country
        // 6. For generate_email: Generate realistic email addresses
        // 7. For generate_phone: Generate phone numbers with proper formatting
        // 8. For generate_credit_card: Generate valid test credit card numbers
        // 9. For generate_custom: Use provided schema to generate structured data
        // 10. For generate_array: Generate multiple items of specified data_type
        // 11. If seed is provided, use it for reproducible generation

        // For now, we'll return a placeholder response
        let result = serde_json::json!({
            "message": "Test data generation executed (placeholder implementation)",
            "operation": &params.operation,
            "count": params.count.unwrap_or(1),
            "locale": params.locale.unwrap_or_else(|| "en_US".to_string()),
            "seed": params.seed,
            "context_execution_id": &context.execution_id,
            "generated_data": {
                "sample": "Generated test data would appear here"
            },
            "success": true
        });

        Ok(NodeOutput::success(result))
    }

    fn validate_parameters(&self, parameters: &serde_json::Value) -> anyhow::Result<()> {
        let params: TestDataGeneratorParams = serde_json::from_value(parameters.clone())
            .map_err(|e| anyhow::anyhow!("Invalid parameters: {}", e))?;

        // Validate operation
        let valid_operations = [
            "generate_person",
            "generate_company",
            "generate_address",
            "generate_email",
            "generate_phone",
            "generate_credit_card",
            "generate_custom",
            "generate_array",
        ];

        if !valid_operations.contains(&params.operation.as_str()) {
            anyhow::bail!(
                "Invalid operation: {}. Must be one of: {}",
                params.operation,
                valid_operations.join(", ")
            );
        }

        // Validate count range
        if let Some(count) = params.count
            && !(1..=10000).contains(&count)
        {
            anyhow::bail!("count must be between 1 and 10000");
        }

        // Validate that generate_custom has custom_schema
        if params.operation == "generate_custom" && params.custom_schema.is_null() {
            anyhow::bail!("generate_custom operation requires 'custom_schema' parameter");
        }

        // Validate that generate_array has data_type
        if params.operation == "generate_array" && params.data_type.is_none() {
            anyhow::bail!("generate_array operation requires 'data_type' parameter");
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
    fn test_test_data_generator_node_type() {
        let node = TestDataGeneratorNode::new();
        assert_eq!(node.type_name(), "test_data_generator");
        assert!(matches!(node.category(), NodeCategory::Action));
        assert!(matches!(node.subcategory(), NodeSubcategory::General));
        assert_eq!(node.required_credential_type(), None);
    }

    #[test]
    fn test_test_data_generator_parameter_schema() {
        let node = TestDataGeneratorNode::new();
        let schema = node.parameter_schema();

        assert_eq!(schema["type"], "object");
        assert!(schema["properties"]["operation"].is_object());
        assert!(schema["properties"]["count"].is_object());
        assert!(schema["properties"]["locale"].is_object());
    }

    #[tokio::test]
    async fn test_test_data_generator_generate_person() {
        let node = TestDataGeneratorNode::new();
        let context = NodeContext::new(Uuid::new_v4().to_string(), "test-workflow".to_string());

        let params = json!({
            "operation": "generate_person",
            "count": 5,
            "locale": "en_US"
        });

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
    }

    #[tokio::test]
    async fn test_test_data_generator_with_seed() {
        let node = TestDataGeneratorNode::new();
        let context = NodeContext::new(Uuid::new_v4().to_string(), "test-workflow".to_string());

        let params = json!({
            "operation": "generate_email",
            "count": 10,
            "seed": 12345
        });

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
    }

    #[test]
    fn test_test_data_generator_validation() {
        let node = TestDataGeneratorNode::new();

        // Valid parameters
        let valid_params = json!({
            "operation": "generate_company",
            "count": 3
        });
        assert!(node.validate_parameters(&valid_params).is_ok());

        // Invalid operation
        let invalid_params = json!({
            "operation": "invalid_operation"
        });
        assert!(node.validate_parameters(&invalid_params).is_err());

        // Invalid count (too high)
        let invalid_params = json!({
            "operation": "generate_person",
            "count": 20000
        });
        assert!(node.validate_parameters(&invalid_params).is_err());

        // Missing custom_schema for generate_custom
        let invalid_params = json!({
            "operation": "generate_custom"
        });
        assert!(node.validate_parameters(&invalid_params).is_err());

        // Missing data_type for generate_array
        let invalid_params = json!({
            "operation": "generate_array",
            "count": 5
        });
        assert!(node.validate_parameters(&invalid_params).is_err());
    }
}
