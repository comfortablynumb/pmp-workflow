use crate::models::{Node, NodeCategory, NodeContext, NodeOutput, NodeSubcategory, NodeType};
use async_trait::async_trait;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct WorkflowTemplateParams {
    /// Template operation to perform
    pub operation: String,
    /// Template name
    pub template_name: Option<String>,
    /// Path to template file
    pub template_path: Option<String>,
    /// Variables to substitute in the template
    #[serde(default)]
    pub template_variables: serde_json::Value,
    /// Output path for generated workflow
    pub output_path: Option<String>,
    /// Workflow YAML to create template from
    pub workflow_yaml: Option<String>,
    /// Description of the template
    pub description: Option<String>,
    /// Category for template organization
    pub category: Option<String>,
}

/// Workflow Template node - generate workflows from templates or create reusable templates
pub struct WorkflowTemplateNode;

impl WorkflowTemplateNode {
    pub fn new() -> Self {
        Self
    }
}

impl Default for WorkflowTemplateNode {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeType for WorkflowTemplateNode {
    fn type_name(&self) -> &str {
        "workflow_template"
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
                    "description": "Template operation to perform",
                    "enum": [
                        "generate_from_template",
                        "create_template",
                        "list_templates",
                        "validate_template",
                        "render_template",
                        "save_template"
                    ]
                },
                "template_name": {
                    "type": "string",
                    "description": "Name of the template"
                },
                "template_path": {
                    "type": "string",
                    "description": "Path to template file"
                },
                "template_variables": {
                    "type": "object",
                    "description": "Variables to substitute in the template",
                    "default": {}
                },
                "output_path": {
                    "type": "string",
                    "description": "Output path for generated workflow"
                },
                "workflow_yaml": {
                    "type": "string",
                    "description": "Workflow YAML content to create template from"
                },
                "description": {
                    "type": "string",
                    "description": "Description of the template"
                },
                "category": {
                    "type": "string",
                    "description": "Category for template organization"
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
impl Node for WorkflowTemplateNode {
    async fn execute(
        &self,
        context: &NodeContext,
        parameters: &serde_json::Value,
    ) -> anyhow::Result<NodeOutput> {
        let params: WorkflowTemplateParams = serde_json::from_value(parameters.clone())?;

        // TODO: In a real implementation:
        // 1. Based on operation, perform template operations:
        //    - generate_from_template: Load template and substitute variables to generate workflow
        //    - create_template: Convert a workflow into a reusable template with variables
        //    - list_templates: List all available templates with metadata
        //    - validate_template: Validate template syntax and required variables
        //    - render_template: Render template with variables without saving
        //    - save_template: Save a template to the template repository
        // 2. Support template variable substitution (e.g., {{ variable_name }})
        // 3. Validate that all required variables are provided
        // 4. Load templates from file system or database
        // 5. Store templates with metadata (name, description, category, required_variables)
        // 6. Return generated workflow or template information

        // For now, we'll return a placeholder response
        let result = serde_json::json!({
            "message": "Workflow template operation executed (placeholder implementation)",
            "operation": &params.operation,
            "template_name": params.template_name,
            "template_path": params.template_path,
            "output_path": params.output_path,
            "template_variables": params.template_variables,
            "description": params.description,
            "category": params.category,
            "context_execution_id": &context.execution_id,
            "success": true
        });

        Ok(NodeOutput::success(result))
    }

    fn validate_parameters(&self, parameters: &serde_json::Value) -> anyhow::Result<()> {
        let params: WorkflowTemplateParams = serde_json::from_value(parameters.clone())
            .map_err(|e| anyhow::anyhow!("Invalid parameters: {}", e))?;

        // Validate operation
        let valid_operations = [
            "generate_from_template",
            "create_template",
            "list_templates",
            "validate_template",
            "render_template",
            "save_template",
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
            "generate_from_template" | "render_template" => {
                if params.template_name.is_none() && params.template_path.is_none() {
                    anyhow::bail!(
                        "{} operation requires either 'template_name' or 'template_path' parameter",
                        params.operation
                    );
                }
            }
            "create_template" | "save_template" => {
                if params.workflow_yaml.is_none() {
                    anyhow::bail!(
                        "{} operation requires 'workflow_yaml' parameter",
                        params.operation
                    );
                }
                if params.template_name.is_none() {
                    anyhow::bail!(
                        "{} operation requires 'template_name' parameter",
                        params.operation
                    );
                }
            }
            "validate_template" => {
                if params.template_name.is_none() && params.template_path.is_none() {
                    anyhow::bail!(
                        "validate_template operation requires either 'template_name' or 'template_path' parameter"
                    );
                }
            }
            _ => {}
        }

        // Validate that output_path is provided for generate_from_template if needed
        if params.operation == "generate_from_template" && params.output_path.is_some() {
            // Output path is optional for generate_from_template
            // If provided, it will save the generated workflow
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
    fn test_workflow_template_node_type() {
        let node = WorkflowTemplateNode::new();
        assert_eq!(node.type_name(), "workflow_template");
        assert!(matches!(node.category(), NodeCategory::Action));
        assert!(matches!(node.subcategory(), NodeSubcategory::General));
        assert_eq!(node.required_credential_type(), None);
    }

    #[test]
    fn test_workflow_template_parameter_schema() {
        let node = WorkflowTemplateNode::new();
        let schema = node.parameter_schema();

        assert_eq!(schema["type"], "object");
        assert!(schema["properties"]["operation"].is_object());
        assert!(schema["properties"]["template_name"].is_object());
        assert!(schema["properties"]["template_path"].is_object());
        assert!(schema["properties"]["template_variables"].is_object());
    }

    #[tokio::test]
    async fn test_workflow_template_generate_from_template() {
        let node = WorkflowTemplateNode::new();
        let context = NodeContext::new(Uuid::new_v4().to_string(), "test-workflow".to_string());

        let params = json!({
            "operation": "generate_from_template",
            "template_name": "api_workflow",
            "template_variables": {
                "api_endpoint": "https://api.example.com",
                "method": "GET"
            },
            "output_path": "/tmp/generated_workflow.yaml"
        });

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
    }

    #[tokio::test]
    async fn test_workflow_template_create_template() {
        let node = WorkflowTemplateNode::new();
        let context = NodeContext::new(Uuid::new_v4().to_string(), "test-workflow".to_string());

        let params = json!({
            "operation": "create_template",
            "template_name": "my_template",
            "workflow_yaml": "name: test\nnodes: []",
            "description": "A test template",
            "category": "utilities"
        });

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
    }

    #[tokio::test]
    async fn test_workflow_template_list_templates() {
        let node = WorkflowTemplateNode::new();
        let context = NodeContext::new(Uuid::new_v4().to_string(), "test-workflow".to_string());

        let params = json!({
            "operation": "list_templates",
            "category": "api"
        });

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
    }

    #[tokio::test]
    async fn test_workflow_template_validate_template() {
        let node = WorkflowTemplateNode::new();
        let context = NodeContext::new(Uuid::new_v4().to_string(), "test-workflow".to_string());

        let params = json!({
            "operation": "validate_template",
            "template_path": "/path/to/template.yaml"
        });

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
    }

    #[test]
    fn test_workflow_template_validation() {
        let node = WorkflowTemplateNode::new();

        // Valid generate_from_template
        let valid_params = json!({
            "operation": "generate_from_template",
            "template_name": "api_workflow",
            "template_variables": {"key": "value"}
        });
        assert!(node.validate_parameters(&valid_params).is_ok());

        // Valid create_template
        let valid_params = json!({
            "operation": "create_template",
            "template_name": "my_template",
            "workflow_yaml": "name: test\nnodes: []"
        });
        assert!(node.validate_parameters(&valid_params).is_ok());

        // Valid list_templates
        let valid_params = json!({
            "operation": "list_templates"
        });
        assert!(node.validate_parameters(&valid_params).is_ok());

        // Missing template source for generate_from_template
        let invalid_params = json!({
            "operation": "generate_from_template"
        });
        assert!(node.validate_parameters(&invalid_params).is_err());

        // Missing workflow_yaml for create_template
        let invalid_params = json!({
            "operation": "create_template",
            "template_name": "my_template"
        });
        assert!(node.validate_parameters(&invalid_params).is_err());

        // Missing template_name for create_template
        let invalid_params = json!({
            "operation": "create_template",
            "workflow_yaml": "name: test"
        });
        assert!(node.validate_parameters(&invalid_params).is_err());

        // Invalid operation
        let invalid_params = json!({
            "operation": "invalid_operation"
        });
        assert!(node.validate_parameters(&invalid_params).is_err());
    }
}
