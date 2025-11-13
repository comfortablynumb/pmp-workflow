use crate::models::{Node, NodeCategory, NodeContext, NodeOutput, NodeSubcategory, NodeType};
use async_trait::async_trait;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PerformanceDashboardParams {
    /// Performance dashboard operation to perform
    pub operation: String,
    /// Workflow ID being monitored
    pub workflow_id: Option<String>,
    /// Execution time in milliseconds
    pub execution_time_ms: Option<u64>,
    /// Number of successful executions
    pub success_count: Option<u32>,
    /// Number of failed executions
    pub error_count: Option<u32>,
    /// Time window for statistics (e.g., "1h", "24h", "7d")
    pub time_window: Option<String>,
    /// Threshold value for alerts
    pub threshold: Option<f64>,
}

/// Performance Dashboard node - collects and visualizes workflow performance metrics
pub struct PerformanceDashboardNode;

impl PerformanceDashboardNode {
    pub fn new() -> Self {
        Self
    }
}

impl Default for PerformanceDashboardNode {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeType for PerformanceDashboardNode {
    fn type_name(&self) -> &str {
        "performance_dashboard"
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
                    "description": "Performance dashboard operation to perform",
                    "enum": [
                        "record_execution_time",
                        "record_throughput",
                        "record_error_rate",
                        "get_statistics",
                        "generate_report",
                        "create_alert"
                    ]
                },
                "workflow_id": {
                    "type": "string",
                    "description": "Workflow ID being monitored"
                },
                "execution_time_ms": {
                    "type": "integer",
                    "description": "Execution time in milliseconds",
                    "minimum": 0
                },
                "success_count": {
                    "type": "integer",
                    "description": "Number of successful executions",
                    "minimum": 0
                },
                "error_count": {
                    "type": "integer",
                    "description": "Number of failed executions",
                    "minimum": 0
                },
                "time_window": {
                    "type": "string",
                    "description": "Time window for statistics (e.g., '1h', '24h', '7d', '30d')",
                    "default": "24h"
                },
                "threshold": {
                    "type": "number",
                    "description": "Threshold value for alerts (e.g., error_rate > 0.05 or execution_time > 5000ms)",
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
impl Node for PerformanceDashboardNode {
    async fn execute(
        &self,
        context: &NodeContext,
        parameters: &serde_json::Value,
    ) -> anyhow::Result<NodeOutput> {
        let params: PerformanceDashboardParams = serde_json::from_value(parameters.clone())?;

        // TODO: In a real implementation:
        // 1. For record_execution_time: Record workflow execution time
        // 2. For record_throughput: Record number of executions per time period
        // 3. For record_error_rate: Record and calculate error rates
        // 4. For get_statistics: Retrieve performance statistics (p50, p95, p99, avg, min, max)
        // 5. For generate_report: Generate comprehensive performance report with charts
        // 6. For create_alert: Create alerts when metrics exceed thresholds
        // 7. Store metrics in time-series database (InfluxDB, Prometheus, etc.)
        // 8. Calculate rolling averages and trends
        // 9. Support multiple time windows for analysis
        // 10. Generate visual dashboards with Grafana or similar tools

        // For now, we'll return a placeholder response
        let result = serde_json::json!({
            "message": "Performance dashboard operation executed (placeholder implementation)",
            "operation": &params.operation,
            "workflow_id": params.workflow_id,
            "execution_time_ms": params.execution_time_ms,
            "success_count": params.success_count,
            "error_count": params.error_count,
            "time_window": params.time_window.unwrap_or_else(|| "24h".to_string()),
            "threshold": params.threshold,
            "context_execution_id": &context.execution_id,
            "performance_metrics": {
                "avg_execution_time_ms": 1234,
                "p95_execution_time_ms": 2500,
                "p99_execution_time_ms": 3200,
                "total_executions": 1000,
                "success_rate": 0.98,
                "error_rate": 0.02
            },
            "success": true
        });

        Ok(NodeOutput::success(result))
    }

    fn validate_parameters(&self, parameters: &serde_json::Value) -> anyhow::Result<()> {
        let params: PerformanceDashboardParams = serde_json::from_value(parameters.clone())
            .map_err(|e| anyhow::anyhow!("Invalid parameters: {}", e))?;

        // Validate operation
        let valid_operations = [
            "record_execution_time",
            "record_throughput",
            "record_error_rate",
            "get_statistics",
            "generate_report",
            "create_alert",
        ];

        if !valid_operations.contains(&params.operation.as_str()) {
            anyhow::bail!(
                "Invalid operation: {}. Must be one of: {}",
                params.operation,
                valid_operations.join(", ")
            );
        }

        // record_execution_time requires workflow_id and execution_time_ms
        if params.operation == "record_execution_time" {
            if params.workflow_id.is_none() {
                anyhow::bail!("record_execution_time operation requires 'workflow_id' parameter");
            }
            if params.execution_time_ms.is_none() {
                anyhow::bail!(
                    "record_execution_time operation requires 'execution_time_ms' parameter"
                );
            }
        }

        // record_throughput requires workflow_id and success_count
        if params.operation == "record_throughput" {
            if params.workflow_id.is_none() {
                anyhow::bail!("record_throughput operation requires 'workflow_id' parameter");
            }
            if params.success_count.is_none() {
                anyhow::bail!("record_throughput operation requires 'success_count' parameter");
            }
        }

        // record_error_rate requires workflow_id, success_count, and error_count
        if params.operation == "record_error_rate" {
            if params.workflow_id.is_none() {
                anyhow::bail!("record_error_rate operation requires 'workflow_id' parameter");
            }
            if params.success_count.is_none() || params.error_count.is_none() {
                anyhow::bail!(
                    "record_error_rate operation requires both 'success_count' and 'error_count' parameters"
                );
            }
        }

        // create_alert requires workflow_id and threshold
        if params.operation == "create_alert" {
            if params.workflow_id.is_none() {
                anyhow::bail!("create_alert operation requires 'workflow_id' parameter");
            }
            if params.threshold.is_none() {
                anyhow::bail!("create_alert operation requires 'threshold' parameter");
            }
        }

        // Validate time_window format if provided
        if let Some(ref time_window) = params.time_window {
            let valid_windows = ["1h", "6h", "12h", "24h", "7d", "30d"];
            if !valid_windows.contains(&time_window.as_str()) {
                anyhow::bail!(
                    "Invalid time_window: {}. Must be one of: {}",
                    time_window,
                    valid_windows.join(", ")
                );
            }
        }

        // Validate threshold is non-negative if provided
        if let Some(threshold) = params.threshold
            && threshold < 0.0
        {
            anyhow::bail!("threshold must be non-negative");
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
    fn test_performance_dashboard_node_type() {
        let node = PerformanceDashboardNode::new();
        assert_eq!(node.type_name(), "performance_dashboard");
        assert!(matches!(node.category(), NodeCategory::Action));
        assert!(matches!(node.subcategory(), NodeSubcategory::General));
        assert_eq!(node.required_credential_type(), None);
    }

    #[test]
    fn test_performance_dashboard_parameter_schema() {
        let node = PerformanceDashboardNode::new();
        let schema = node.parameter_schema();

        assert_eq!(schema["type"], "object");
        assert!(schema["properties"]["operation"].is_object());
        assert!(schema["properties"]["workflow_id"].is_object());
        assert!(schema["properties"]["execution_time_ms"].is_object());
    }

    #[tokio::test]
    async fn test_performance_dashboard_record_execution_time() {
        let node = PerformanceDashboardNode::new();
        let context = NodeContext::new(Uuid::new_v4().to_string(), "test-workflow".to_string());

        let params = json!({
            "operation": "record_execution_time",
            "workflow_id": "wf-123",
            "execution_time_ms": 2500
        });

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
    }

    #[tokio::test]
    async fn test_performance_dashboard_get_statistics() {
        let node = PerformanceDashboardNode::new();
        let context = NodeContext::new(Uuid::new_v4().to_string(), "test-workflow".to_string());

        let params = json!({
            "operation": "get_statistics",
            "workflow_id": "wf-123",
            "time_window": "7d"
        });

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
    }

    #[test]
    fn test_performance_dashboard_validation() {
        let node = PerformanceDashboardNode::new();

        // Valid record_execution_time
        let valid_params = json!({
            "operation": "record_execution_time",
            "workflow_id": "wf-123",
            "execution_time_ms": 1500
        });
        assert!(node.validate_parameters(&valid_params).is_ok());

        // Invalid operation
        let invalid_params = json!({
            "operation": "invalid_operation"
        });
        assert!(node.validate_parameters(&invalid_params).is_err());

        // Missing workflow_id for record_execution_time
        let invalid_params = json!({
            "operation": "record_execution_time",
            "execution_time_ms": 1500
        });
        assert!(node.validate_parameters(&invalid_params).is_err());

        // Missing execution_time_ms for record_execution_time
        let invalid_params = json!({
            "operation": "record_execution_time",
            "workflow_id": "wf-123"
        });
        assert!(node.validate_parameters(&invalid_params).is_err());

        // Invalid time_window
        let invalid_params = json!({
            "operation": "get_statistics",
            "time_window": "invalid_window"
        });
        assert!(node.validate_parameters(&invalid_params).is_err());

        // Negative threshold
        let invalid_params = json!({
            "operation": "create_alert",
            "workflow_id": "wf-123",
            "threshold": -1.0
        });
        assert!(node.validate_parameters(&invalid_params).is_err());
    }
}
