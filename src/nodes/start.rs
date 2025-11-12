use crate::models::{Node, NodeContext, NodeOutput};
use async_trait::async_trait;

/// Start node - the entry point of a workflow
pub struct StartNode;

#[async_trait]
impl Node for StartNode {
    fn node_type(&self) -> &str {
        "start"
    }

    async fn execute(
        &self,
        context: &NodeContext,
        _parameters: &serde_json::Value,
    ) -> anyhow::Result<NodeOutput> {
        // Start node simply passes through the input or returns empty data
        let data = context
            .get_main_input()
            .cloned()
            .unwrap_or(serde_json::json!({}));

        Ok(NodeOutput::success(data))
    }

    fn parameter_schema(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {},
            "description": "Start node does not require any parameters. It passes through input data to downstream nodes.",
            "additionalProperties": false
        })
    }
}
