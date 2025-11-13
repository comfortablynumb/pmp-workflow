use crate::models::{Node, NodeCategory, NodeContext, NodeOutput, NodeSubcategory, NodeType};
use async_trait::async_trait;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DatadogParams {
    /// Credentials name to use for Datadog API
    pub credentials_name: String,
    /// Datadog operation to perform
    pub operation: String,
    /// Metric name
    pub metric_name: Option<String>,
    /// Metric value
    pub value: Option<f64>,
    /// Tags for metrics/events
    pub tags: Option<Vec<String>>,
    /// Event title
    pub event_title: Option<String>,
    /// Event text/description
    pub event_text: Option<String>,
    /// Monitor ID
    pub monitor_id: Option<String>,
    /// Query for metrics or monitor
    pub query: Option<String>,
    /// Alert type (error, warning, info, success)
    pub alert_type: Option<String>,
}

/// Datadog node - performs Datadog monitoring and metrics operations
pub struct DatadogNode;

impl DatadogNode {
    pub fn new() -> Self {
        Self
    }
}

impl Default for DatadogNode {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeType for DatadogNode {
    fn type_name(&self) -> &str {
        "datadog"
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
                "credentials_name": {
                    "type": "string",
                    "description": "Name of the Datadog API credentials to use",
                    "minLength": 1
                },
                "operation": {
                    "type": "string",
                    "description": "Datadog operation to perform",
                    "enum": [
                        "send_metric",
                        "send_event",
                        "create_monitor",
                        "update_monitor",
                        "delete_monitor",
                        "query_metrics",
                        "get_monitor_status"
                    ]
                },
                "metric_name": {
                    "type": "string",
                    "description": "Name of the metric (e.g., 'system.cpu.usage')"
                },
                "value": {
                    "type": "number",
                    "description": "Metric value"
                },
                "tags": {
                    "type": "array",
                    "description": "Tags for metrics or events (e.g., ['env:prod', 'service:api'])",
                    "items": {
                        "type": "string"
                    }
                },
                "event_title": {
                    "type": "string",
                    "description": "Title for event"
                },
                "event_text": {
                    "type": "string",
                    "description": "Description/text for event"
                },
                "monitor_id": {
                    "type": "string",
                    "description": "Monitor ID for update, delete, or status operations"
                },
                "query": {
                    "type": "string",
                    "description": "Query string for metrics or monitor definition"
                },
                "alert_type": {
                    "type": "string",
                    "description": "Alert type for events",
                    "enum": ["error", "warning", "info", "success"]
                }
            },
            "required": ["credentials_name", "operation"],
            "additionalProperties": false
        })
    }

    fn required_credential_type(&self) -> Option<&str> {
        Some("datadog_api")
    }
}

#[async_trait]
impl Node for DatadogNode {
    async fn execute(
        &self,
        context: &NodeContext,
        parameters: &serde_json::Value,
    ) -> anyhow::Result<NodeOutput> {
        let params: DatadogParams = serde_json::from_value(parameters.clone())?;

        // TODO: In a real implementation:
        // 1. Look up credentials from the credentials table using params.credentials_name
        // 2. Decrypt the credentials data to get the Datadog API key and app key
        // 3. Create a Datadog client using the API credentials
        // 4. Execute the operation based on params.operation:
        //    - send_metric: Submit a metric data point
        //    - send_event: Create an event
        //    - create_monitor: Create a new monitor
        //    - update_monitor: Update an existing monitor
        //    - delete_monitor: Delete a monitor
        //    - query_metrics: Query metric time series
        //    - get_monitor_status: Get monitor status
        // 5. Return the API response

        // For now, we'll return a placeholder response
        let result = serde_json::json!({
            "message": "Datadog operation executed (placeholder implementation)",
            "credentials_name": &params.credentials_name,
            "operation": &params.operation,
            "metric_name": params.metric_name,
            "value": params.value,
            "tags": params.tags,
            "context_execution_id": &context.execution_id,
            "success": true
        });

        Ok(NodeOutput::success(result))
    }

    fn validate_parameters(&self, parameters: &serde_json::Value) -> anyhow::Result<()> {
        let params: DatadogParams = serde_json::from_value(parameters.clone())
            .map_err(|e| anyhow::anyhow!("Invalid parameters: {}", e))?;

        if params.credentials_name.trim().is_empty() {
            anyhow::bail!("credentials_name cannot be empty");
        }

        // Validate operation
        let valid_operations = [
            "send_metric",
            "send_event",
            "create_monitor",
            "update_monitor",
            "delete_monitor",
            "query_metrics",
            "get_monitor_status",
        ];

        if !valid_operations.contains(&params.operation.as_str()) {
            anyhow::bail!(
                "Invalid operation: {}. Must be one of: {}",
                params.operation,
                valid_operations.join(", ")
            );
        }

        // Validate send_metric
        if params.operation == "send_metric" {
            if params.metric_name.is_none() {
                anyhow::bail!("send_metric operation requires 'metric_name' parameter");
            }
            if params.value.is_none() {
                anyhow::bail!("send_metric operation requires 'value' parameter");
            }
        }

        // Validate send_event
        if params.operation == "send_event" {
            if params.event_title.is_none() {
                anyhow::bail!("send_event operation requires 'event_title' parameter");
            }
            if params.event_text.is_none() {
                anyhow::bail!("send_event operation requires 'event_text' parameter");
            }
        }

        // Validate create_monitor
        if params.operation == "create_monitor" && params.query.is_none() {
            anyhow::bail!("create_monitor operation requires 'query' parameter");
        }

        // Validate operations requiring monitor_id
        let monitor_id_ops = ["update_monitor", "delete_monitor", "get_monitor_status"];
        if monitor_id_ops.contains(&params.operation.as_str()) && params.monitor_id.is_none() {
            anyhow::bail!(
                "{} operation requires 'monitor_id' parameter",
                params.operation
            );
        }

        // Validate query_metrics
        if params.operation == "query_metrics" && params.query.is_none() {
            anyhow::bail!("query_metrics operation requires 'query' parameter");
        }

        // Validate alert_type
        if let Some(ref alert_type) = params.alert_type {
            let valid_types = ["error", "warning", "info", "success"];
            if !valid_types.contains(&alert_type.as_str()) {
                anyhow::bail!("alert_type must be one of: {}", valid_types.join(", "));
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
    fn test_datadog_node_type() {
        let node = DatadogNode::new();
        assert_eq!(node.type_name(), "datadog");
        assert!(matches!(node.category(), NodeCategory::Action));
        assert!(matches!(node.subcategory(), NodeSubcategory::General));
        assert_eq!(node.required_credential_type(), Some("datadog_api"));
    }

    #[test]
    fn test_datadog_parameter_schema() {
        let node = DatadogNode::new();
        let schema = node.parameter_schema();

        assert_eq!(schema["type"], "object");
        assert!(schema["properties"]["credentials_name"].is_object());
        assert!(schema["properties"]["operation"].is_object());
        assert!(schema["properties"]["metric_name"].is_object());
    }

    #[tokio::test]
    async fn test_datadog_send_metric() {
        let node = DatadogNode::new();
        let context = NodeContext::new(Uuid::new_v4().to_string(), "test-workflow".to_string());

        let params = json!({
            "credentials_name": "my_datadog_creds",
            "operation": "send_metric",
            "metric_name": "custom.metric",
            "value": 42.5,
            "tags": ["env:prod", "service:api"]
        });

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
    }

    #[tokio::test]
    async fn test_datadog_send_event() {
        let node = DatadogNode::new();
        let context = NodeContext::new(Uuid::new_v4().to_string(), "test-workflow".to_string());

        let params = json!({
            "credentials_name": "my_datadog_creds",
            "operation": "send_event",
            "event_title": "Deployment Complete",
            "event_text": "Application deployed to production",
            "alert_type": "success",
            "tags": ["env:prod"]
        });

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
    }

    #[test]
    fn test_datadog_validation() {
        let node = DatadogNode::new();

        // Valid send_metric
        let valid_params = json!({
            "credentials_name": "my_datadog_creds",
            "operation": "send_metric",
            "metric_name": "custom.metric",
            "value": 100.0
        });
        assert!(node.validate_parameters(&valid_params).is_ok());

        // Missing metric_name for send_metric
        let invalid_params = json!({
            "credentials_name": "my_datadog_creds",
            "operation": "send_metric",
            "value": 100.0
        });
        assert!(node.validate_parameters(&invalid_params).is_err());

        // Missing event_title for send_event
        let invalid_params = json!({
            "credentials_name": "my_datadog_creds",
            "operation": "send_event",
            "event_text": "Some text"
        });
        assert!(node.validate_parameters(&invalid_params).is_err());

        // Invalid alert_type
        let invalid_params = json!({
            "credentials_name": "my_datadog_creds",
            "operation": "send_event",
            "event_title": "Test",
            "event_text": "Test",
            "alert_type": "critical"
        });
        assert!(node.validate_parameters(&invalid_params).is_err());
    }
}
