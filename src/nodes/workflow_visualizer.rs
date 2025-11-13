use crate::models::{Node, NodeCategory, NodeContext, NodeOutput, NodeSubcategory, NodeType};
use async_trait::async_trait;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct WorkflowVisualizerParams {
    /// Visualizer operation to perform
    pub operation: String,
    /// Workflow YAML content to visualize
    pub workflow_yaml: Option<String>,
    /// Path to workflow file
    pub workflow_path: Option<String>,
    /// Output path for the generated diagram
    pub output_path: Option<String>,
    /// Output format (png, svg, pdf, etc.)
    pub format: Option<String>,
    /// Include detailed node information in diagram
    pub include_details: Option<bool>,
    /// Diagram orientation (TB for top-bottom, LR for left-right)
    pub orientation: Option<String>,
    /// Theme for the diagram (default, dark, forest, neutral)
    pub theme: Option<String>,
}

/// Workflow Visualizer node - generates visual diagrams from workflow YAML in various formats
pub struct WorkflowVisualizerNode;

impl WorkflowVisualizerNode {
    pub fn new() -> Self {
        Self
    }
}

impl Default for WorkflowVisualizerNode {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeType for WorkflowVisualizerNode {
    fn type_name(&self) -> &str {
        "workflow_visualizer"
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
                    "description": "Visualizer operation to perform",
                    "enum": [
                        "generate_mermaid",
                        "generate_dot",
                        "generate_plantuml",
                        "generate_ascii",
                        "export_diagram",
                        "analyze_complexity"
                    ]
                },
                "workflow_yaml": {
                    "type": "string",
                    "description": "Workflow YAML content to visualize"
                },
                "workflow_path": {
                    "type": "string",
                    "description": "Path to workflow YAML file"
                },
                "output_path": {
                    "type": "string",
                    "description": "Output path for the generated diagram file"
                },
                "format": {
                    "type": "string",
                    "description": "Output format for exported diagrams",
                    "enum": ["png", "svg", "pdf", "html"]
                },
                "include_details": {
                    "type": "boolean",
                    "description": "Include detailed node information in the diagram",
                    "default": false
                },
                "orientation": {
                    "type": "string",
                    "description": "Diagram orientation",
                    "enum": ["TB", "LR"],
                    "default": "TB"
                },
                "theme": {
                    "type": "string",
                    "description": "Theme for the diagram",
                    "enum": ["default", "dark", "forest", "neutral"],
                    "default": "default"
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
impl Node for WorkflowVisualizerNode {
    async fn execute(
        &self,
        context: &NodeContext,
        parameters: &serde_json::Value,
    ) -> anyhow::Result<NodeOutput> {
        let params: WorkflowVisualizerParams = serde_json::from_value(parameters.clone())?;

        // TODO: In a real implementation:
        // 1. Load the workflow YAML from workflow_yaml or workflow_path
        // 2. Parse the workflow structure
        // 3. Based on operation, generate the appropriate diagram format:
        //    - generate_mermaid: Generate Mermaid.js diagram syntax
        //    - generate_dot: Generate Graphviz DOT format
        //    - generate_plantuml: Generate PlantUML diagram
        //    - generate_ascii: Generate ASCII art representation
        //    - export_diagram: Export to image file (png, svg, pdf)
        //    - analyze_complexity: Analyze workflow complexity metrics
        // 4. Apply orientation (TB/LR) and theme settings
        // 5. If include_details is true, add node parameters and metadata
        // 6. If output_path is provided, save the diagram to file
        // 7. Return the diagram content and/or file path

        // For now, we'll return a placeholder response
        let result = serde_json::json!({
            "message": "Workflow visualizer operation executed (placeholder implementation)",
            "operation": &params.operation,
            "workflow_yaml_provided": params.workflow_yaml.is_some(),
            "workflow_path": params.workflow_path,
            "output_path": params.output_path,
            "format": params.format,
            "include_details": params.include_details.unwrap_or(false),
            "orientation": params.orientation.as_deref().unwrap_or("TB"),
            "theme": params.theme.as_deref().unwrap_or("default"),
            "context_execution_id": &context.execution_id,
            "success": true
        });

        Ok(NodeOutput::success(result))
    }

    fn validate_parameters(&self, parameters: &serde_json::Value) -> anyhow::Result<()> {
        let params: WorkflowVisualizerParams = serde_json::from_value(parameters.clone())
            .map_err(|e| anyhow::anyhow!("Invalid parameters: {}", e))?;

        // Validate operation
        let valid_operations = [
            "generate_mermaid",
            "generate_dot",
            "generate_plantuml",
            "generate_ascii",
            "export_diagram",
            "analyze_complexity",
        ];

        if !valid_operations.contains(&params.operation.as_str()) {
            anyhow::bail!(
                "Invalid operation: {}. Must be one of: {}",
                params.operation,
                valid_operations.join(", ")
            );
        }

        // Validate that workflow source is provided
        if params.workflow_yaml.is_none() && params.workflow_path.is_none() {
            anyhow::bail!("Either 'workflow_yaml' or 'workflow_path' parameter is required");
        }

        // Validate orientation
        if let Some(orientation) = &params.orientation {
            let valid_orientations = ["TB", "LR"];
            if !valid_orientations.contains(&orientation.as_str()) {
                anyhow::bail!(
                    "Invalid orientation: {}. Must be one of: {}",
                    orientation,
                    valid_orientations.join(", ")
                );
            }
        }

        // Validate theme
        if let Some(theme) = &params.theme {
            let valid_themes = ["default", "dark", "forest", "neutral"];
            if !valid_themes.contains(&theme.as_str()) {
                anyhow::bail!(
                    "Invalid theme: {}. Must be one of: {}",
                    theme,
                    valid_themes.join(", ")
                );
            }
        }

        // Validate format for export_diagram
        if params.operation == "export_diagram" {
            if params.format.is_none() {
                anyhow::bail!("export_diagram operation requires 'format' parameter");
            }
            if params.output_path.is_none() {
                anyhow::bail!("export_diagram operation requires 'output_path' parameter");
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
    fn test_workflow_visualizer_node_type() {
        let node = WorkflowVisualizerNode::new();
        assert_eq!(node.type_name(), "workflow_visualizer");
        assert!(matches!(node.category(), NodeCategory::Action));
        assert!(matches!(node.subcategory(), NodeSubcategory::General));
        assert_eq!(node.required_credential_type(), None);
    }

    #[test]
    fn test_workflow_visualizer_parameter_schema() {
        let node = WorkflowVisualizerNode::new();
        let schema = node.parameter_schema();

        assert_eq!(schema["type"], "object");
        assert!(schema["properties"]["operation"].is_object());
        assert!(schema["properties"]["workflow_yaml"].is_object());
        assert!(schema["properties"]["workflow_path"].is_object());
        assert!(schema["properties"]["format"].is_object());
    }

    #[tokio::test]
    async fn test_workflow_visualizer_generate_mermaid() {
        let node = WorkflowVisualizerNode::new();
        let context = NodeContext::new(Uuid::new_v4().to_string(), "test-workflow".to_string());

        let params = json!({
            "operation": "generate_mermaid",
            "workflow_yaml": "name: test\nnodes: []",
            "orientation": "TB",
            "theme": "default"
        });

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
    }

    #[tokio::test]
    async fn test_workflow_visualizer_export_diagram() {
        let node = WorkflowVisualizerNode::new();
        let context = NodeContext::new(Uuid::new_v4().to_string(), "test-workflow".to_string());

        let params = json!({
            "operation": "export_diagram",
            "workflow_yaml": "name: test\nnodes: []",
            "output_path": "/tmp/diagram.png",
            "format": "png",
            "include_details": true
        });

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
    }

    #[tokio::test]
    async fn test_workflow_visualizer_analyze_complexity() {
        let node = WorkflowVisualizerNode::new();
        let context = NodeContext::new(Uuid::new_v4().to_string(), "test-workflow".to_string());

        let params = json!({
            "operation": "analyze_complexity",
            "workflow_path": "/path/to/workflow.yaml"
        });

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
    }

    #[test]
    fn test_workflow_visualizer_validation() {
        let node = WorkflowVisualizerNode::new();

        // Valid generate_mermaid
        let valid_params = json!({
            "operation": "generate_mermaid",
            "workflow_yaml": "name: test\nnodes: []"
        });
        assert!(node.validate_parameters(&valid_params).is_ok());

        // Valid export_diagram
        let valid_params = json!({
            "operation": "export_diagram",
            "workflow_yaml": "name: test\nnodes: []",
            "output_path": "/tmp/diagram.png",
            "format": "png"
        });
        assert!(node.validate_parameters(&valid_params).is_ok());

        // Missing workflow source
        let invalid_params = json!({
            "operation": "generate_mermaid"
        });
        assert!(node.validate_parameters(&invalid_params).is_err());

        // export_diagram without format
        let invalid_params = json!({
            "operation": "export_diagram",
            "workflow_yaml": "name: test",
            "output_path": "/tmp/diagram.png"
        });
        assert!(node.validate_parameters(&invalid_params).is_err());

        // Invalid operation
        let invalid_params = json!({
            "operation": "invalid_operation",
            "workflow_yaml": "name: test"
        });
        assert!(node.validate_parameters(&invalid_params).is_err());

        // Invalid orientation
        let invalid_params = json!({
            "operation": "generate_mermaid",
            "workflow_yaml": "name: test",
            "orientation": "INVALID"
        });
        assert!(node.validate_parameters(&invalid_params).is_err());
    }
}
