use crate::models::{Node, NodeCategory, NodeContext, NodeOutput, NodeSubcategory, NodeType};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

/// AWS CloudWatch node for monitoring, logging, and alarms
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AwsCloudWatchNode {
    #[serde(skip)]
    _private: (),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AwsCloudWatchParams {
    /// Operation to perform
    pub operation: String,

    // Metrics operations
    /// CloudWatch namespace (e.g., "AWS/EC2", "Custom/MyApp")
    pub namespace: Option<String>,
    /// Metric name
    pub metric_name: Option<String>,
    /// Metric dimensions (as JSON object)
    pub dimensions: Option<Value>,
    /// Metric value
    pub value: Option<f64>,
    /// Metric unit (Seconds, Microseconds, Milliseconds, Bytes, Kilobytes, etc.)
    pub unit: Option<String>,
    /// Timestamp for the metric (ISO 8601 format)
    pub timestamp: Option<String>,

    // Get metric statistics operations
    /// Statistic type (Average, Sum, SampleCount, Maximum, Minimum)
    pub statistic: Option<String>,
    /// Multiple statistics
    pub statistics: Option<Vec<String>>,
    /// Period in seconds (must be multiple of 60)
    pub period: Option<u32>,
    /// Start time for metric query (ISO 8601 format)
    pub start_time: Option<String>,
    /// End time for metric query (ISO 8601 format)
    pub end_time: Option<String>,

    // Logs operations
    /// Log group name
    pub log_group_name: Option<String>,
    /// Log stream name
    pub log_stream_name: Option<String>,
    /// Log events (array of log messages)
    pub log_events: Option<Value>,
    /// Filter pattern for log queries
    pub filter_pattern: Option<String>,

    // Alarms operations
    /// Alarm name
    pub alarm_name: Option<String>,
    /// Comparison operator (GreaterThanThreshold, LessThanThreshold, etc.)
    pub comparison_operator: Option<String>,
    /// Number of periods over which to evaluate the alarm
    pub evaluation_periods: Option<u32>,
    /// Threshold value for the alarm
    pub threshold: Option<f64>,
}

impl AwsCloudWatchNode {
    pub fn new() -> Self {
        Self { _private: () }
    }
}

impl Default for AwsCloudWatchNode {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeType for AwsCloudWatchNode {
    fn type_name(&self) -> &str {
        "aws_cloudwatch"
    }

    fn category(&self) -> NodeCategory {
        NodeCategory::Action
    }

    fn subcategory(&self) -> NodeSubcategory {
        NodeSubcategory::General
    }

    fn parameter_schema(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "required": ["operation"],
            "properties": {
                "operation": {
                    "type": "string",
                    "enum": [
                        "put_metric_data",
                        "get_metric_statistics",
                        "list_metrics",
                        "create_log_group",
                        "create_log_stream",
                        "put_log_events",
                        "filter_log_events",
                        "describe_log_streams",
                        "put_metric_alarm",
                        "describe_alarms",
                        "delete_alarms",
                        "set_alarm_state"
                    ],
                    "description": "The CloudWatch operation to perform"
                },
                "namespace": {
                    "type": "string",
                    "description": "CloudWatch namespace (e.g., 'AWS/EC2', 'Custom/MyApp')"
                },
                "metric_name": {
                    "type": "string",
                    "description": "Metric name"
                },
                "dimensions": {
                    "description": "Metric dimensions as JSON object"
                },
                "value": {
                    "type": "number",
                    "description": "Metric value"
                },
                "unit": {
                    "type": "string",
                    "description": "Metric unit (Seconds, Microseconds, Milliseconds, Bytes, etc.)"
                },
                "timestamp": {
                    "type": "string",
                    "description": "Timestamp in ISO 8601 format"
                },
                "statistic": {
                    "type": "string",
                    "enum": ["Average", "Sum", "SampleCount", "Maximum", "Minimum"],
                    "description": "Statistic type"
                },
                "statistics": {
                    "type": "array",
                    "items": {
                        "type": "string"
                    },
                    "description": "Multiple statistics to retrieve"
                },
                "period": {
                    "type": "integer",
                    "description": "Period in seconds (must be multiple of 60)"
                },
                "start_time": {
                    "type": "string",
                    "description": "Start time in ISO 8601 format"
                },
                "end_time": {
                    "type": "string",
                    "description": "End time in ISO 8601 format"
                },
                "log_group_name": {
                    "type": "string",
                    "description": "Log group name"
                },
                "log_stream_name": {
                    "type": "string",
                    "description": "Log stream name"
                },
                "log_events": {
                    "description": "Array of log events to publish"
                },
                "filter_pattern": {
                    "type": "string",
                    "description": "Filter pattern for log queries"
                },
                "alarm_name": {
                    "type": "string",
                    "description": "Alarm name"
                },
                "comparison_operator": {
                    "type": "string",
                    "enum": [
                        "GreaterThanThreshold",
                        "GreaterThanOrEqualToThreshold",
                        "LessThanThreshold",
                        "LessThanOrEqualToThreshold"
                    ],
                    "description": "Comparison operator for alarm"
                },
                "evaluation_periods": {
                    "type": "integer",
                    "description": "Number of periods to evaluate"
                },
                "threshold": {
                    "type": "number",
                    "description": "Threshold value for alarm"
                }
            }
        })
    }

    fn required_credential_type(&self) -> Option<&str> {
        Some("aws")
    }
}

#[async_trait]
impl Node for AwsCloudWatchNode {
    async fn execute(
        &self,
        _context: &NodeContext,
        parameters: &serde_json::Value,
    ) -> anyhow::Result<NodeOutput> {
        let params: AwsCloudWatchParams = serde_json::from_value(parameters.clone())?;

        // TODO: In a real implementation:
        // 1. Look up AWS credentials from the credentials table
        // 2. Create CloudWatch SDK client with credentials
        // 3. Execute the requested operation
        // 4. Return the API response

        // Mock implementation for now
        let result = match params.operation.as_str() {
            "put_metric_data" => {
                json!({
                    "success": true,
                    "operation": "put_metric_data",
                    "namespace": params.namespace,
                    "metric_name": params.metric_name,
                    "value": params.value,
                    "unit": params.unit,
                    "dimensions": params.dimensions,
                    "message": "Metric data published successfully"
                })
            }
            "get_metric_statistics" => {
                json!({
                    "success": true,
                    "operation": "get_metric_statistics",
                    "namespace": params.namespace,
                    "metric_name": params.metric_name,
                    "statistics": params.statistics,
                    "period": params.period,
                    "start_time": params.start_time,
                    "end_time": params.end_time,
                    "datapoints": [
                        {
                            "timestamp": "2025-01-15T10:00:00Z",
                            "average": 45.5,
                            "sum": 910.0,
                            "sample_count": 20.0,
                            "unit": "Percent"
                        },
                        {
                            "timestamp": "2025-01-15T10:05:00Z",
                            "average": 52.3,
                            "sum": 1046.0,
                            "sample_count": 20.0,
                            "unit": "Percent"
                        }
                    ],
                    "label": params.metric_name
                })
            }
            "list_metrics" => {
                json!({
                    "success": true,
                    "operation": "list_metrics",
                    "namespace": params.namespace,
                    "metrics": [
                        {
                            "namespace": params.namespace.as_ref().unwrap_or(&"AWS/EC2".to_string()),
                            "metric_name": "CPUUtilization",
                            "dimensions": [
                                {"name": "InstanceId", "value": "i-1234567890abcdef0"}
                            ]
                        },
                        {
                            "namespace": params.namespace.as_ref().unwrap_or(&"AWS/EC2".to_string()),
                            "metric_name": "NetworkIn",
                            "dimensions": [
                                {"name": "InstanceId", "value": "i-1234567890abcdef0"}
                            ]
                        }
                    ]
                })
            }
            "create_log_group" => {
                json!({
                    "success": true,
                    "operation": "create_log_group",
                    "log_group_name": params.log_group_name,
                    "message": "Log group created successfully"
                })
            }
            "create_log_stream" => {
                json!({
                    "success": true,
                    "operation": "create_log_stream",
                    "log_group_name": params.log_group_name,
                    "log_stream_name": params.log_stream_name,
                    "message": "Log stream created successfully"
                })
            }
            "put_log_events" => {
                json!({
                    "success": true,
                    "operation": "put_log_events",
                    "log_group_name": params.log_group_name,
                    "log_stream_name": params.log_stream_name,
                    "next_sequence_token": "49600567071483824704371468346334102967283043685367021698",
                    "message": "Log events published successfully"
                })
            }
            "filter_log_events" => {
                json!({
                    "success": true,
                    "operation": "filter_log_events",
                    "log_group_name": params.log_group_name,
                    "filter_pattern": params.filter_pattern,
                    "events": [
                        {
                            "log_stream_name": "app-stream-1",
                            "timestamp": 1705315200000i64,
                            "message": "ERROR: Failed to connect to database",
                            "ingestion_time": 1705315201000i64
                        },
                        {
                            "log_stream_name": "app-stream-1",
                            "timestamp": 1705315260000i64,
                            "message": "ERROR: Timeout while processing request",
                            "ingestion_time": 1705315261000i64
                        }
                    ],
                    "searched_log_streams": [
                        {"log_stream_name": "app-stream-1", "searched_completely": true}
                    ]
                })
            }
            "describe_log_streams" => {
                json!({
                    "success": true,
                    "operation": "describe_log_streams",
                    "log_group_name": params.log_group_name,
                    "log_streams": [
                        {
                            "log_stream_name": "app-stream-1",
                            "creation_time": 1705315000000i64,
                            "first_event_timestamp": 1705315100000i64,
                            "last_event_timestamp": 1705315300000i64,
                            "last_ingestion_time": 1705315301000i64,
                            "stored_bytes": 1024
                        },
                        {
                            "log_stream_name": "app-stream-2",
                            "creation_time": 1705315050000i64,
                            "first_event_timestamp": 1705315150000i64,
                            "last_event_timestamp": 1705315350000i64,
                            "last_ingestion_time": 1705315351000i64,
                            "stored_bytes": 2048
                        }
                    ]
                })
            }
            "put_metric_alarm" => {
                json!({
                    "success": true,
                    "operation": "put_metric_alarm",
                    "alarm_name": params.alarm_name,
                    "namespace": params.namespace,
                    "metric_name": params.metric_name,
                    "comparison_operator": params.comparison_operator,
                    "threshold": params.threshold,
                    "evaluation_periods": params.evaluation_periods,
                    "message": "Alarm created successfully"
                })
            }
            "describe_alarms" => {
                json!({
                    "success": true,
                    "operation": "describe_alarms",
                    "metric_alarms": [
                        {
                            "alarm_name": "HighCPUUtilization",
                            "alarm_arn": "arn:aws:cloudwatch:us-east-1:123456789012:alarm:HighCPUUtilization",
                            "alarm_description": "Alert when CPU exceeds 80%",
                            "state_value": "OK",
                            "state_reason": "Threshold Crossed: 1 datapoint [45.5] was not greater than the threshold (80.0).",
                            "metric_name": "CPUUtilization",
                            "namespace": "AWS/EC2",
                            "statistic": "Average",
                            "period": 300,
                            "evaluation_periods": 2,
                            "threshold": 80.0,
                            "comparison_operator": "GreaterThanThreshold"
                        }
                    ]
                })
            }
            "delete_alarms" => {
                json!({
                    "success": true,
                    "operation": "delete_alarms",
                    "alarm_name": params.alarm_name,
                    "message": "Alarm deleted successfully"
                })
            }
            "set_alarm_state" => {
                json!({
                    "success": true,
                    "operation": "set_alarm_state",
                    "alarm_name": params.alarm_name,
                    "message": "Alarm state updated successfully"
                })
            }
            _ => {
                anyhow::bail!("Unsupported operation: {}", params.operation);
            }
        };

        Ok(NodeOutput::success(result))
    }

    fn validate_parameters(&self, parameters: &serde_json::Value) -> anyhow::Result<()> {
        let params: AwsCloudWatchParams = serde_json::from_value(parameters.clone())
            .map_err(|e| anyhow::anyhow!("Invalid parameters: {}", e))?;

        // Validate operation
        let valid_operations = [
            "put_metric_data",
            "get_metric_statistics",
            "list_metrics",
            "create_log_group",
            "create_log_stream",
            "put_log_events",
            "filter_log_events",
            "describe_log_streams",
            "put_metric_alarm",
            "describe_alarms",
            "delete_alarms",
            "set_alarm_state",
        ];

        if !valid_operations.contains(&params.operation.as_str()) {
            anyhow::bail!(
                "Invalid operation: {}. Must be one of: {}",
                params.operation,
                valid_operations.join(", ")
            );
        }

        // Validate put_metric_data
        if params.operation == "put_metric_data" {
            if params.namespace.is_none() {
                anyhow::bail!("put_metric_data operation requires 'namespace' parameter");
            }
            if params.metric_name.is_none() {
                anyhow::bail!("put_metric_data operation requires 'metric_name' parameter");
            }
            if params.value.is_none() {
                anyhow::bail!("put_metric_data operation requires 'value' parameter");
            }
        }

        // Validate get_metric_statistics
        if params.operation == "get_metric_statistics" {
            if params.namespace.is_none() {
                anyhow::bail!("get_metric_statistics operation requires 'namespace' parameter");
            }
            if params.metric_name.is_none() {
                anyhow::bail!("get_metric_statistics operation requires 'metric_name' parameter");
            }
            if params.period.is_none() {
                anyhow::bail!("get_metric_statistics operation requires 'period' parameter");
            }
            if params.start_time.is_none() {
                anyhow::bail!("get_metric_statistics operation requires 'start_time' parameter");
            }
            if params.end_time.is_none() {
                anyhow::bail!("get_metric_statistics operation requires 'end_time' parameter");
            }
            if params.statistics.is_none() && params.statistic.is_none() {
                anyhow::bail!(
                    "get_metric_statistics operation requires 'statistics' or 'statistic' parameter"
                );
            }
            // Validate period is multiple of 60
            if let Some(period) = params.period
                && period % 60 != 0
            {
                anyhow::bail!("period must be a multiple of 60 seconds");
            }
        }

        // Validate create_log_group
        if params.operation == "create_log_group" && params.log_group_name.is_none() {
            anyhow::bail!("create_log_group operation requires 'log_group_name' parameter");
        }

        // Validate create_log_stream
        if params.operation == "create_log_stream" {
            if params.log_group_name.is_none() {
                anyhow::bail!("create_log_stream operation requires 'log_group_name' parameter");
            }
            if params.log_stream_name.is_none() {
                anyhow::bail!("create_log_stream operation requires 'log_stream_name' parameter");
            }
        }

        // Validate put_log_events
        if params.operation == "put_log_events" {
            if params.log_group_name.is_none() {
                anyhow::bail!("put_log_events operation requires 'log_group_name' parameter");
            }
            if params.log_stream_name.is_none() {
                anyhow::bail!("put_log_events operation requires 'log_stream_name' parameter");
            }
            if params.log_events.is_none() {
                anyhow::bail!("put_log_events operation requires 'log_events' parameter");
            }
        }

        // Validate filter_log_events
        if params.operation == "filter_log_events" && params.log_group_name.is_none() {
            anyhow::bail!("filter_log_events operation requires 'log_group_name' parameter");
        }

        // Validate describe_log_streams
        if params.operation == "describe_log_streams" && params.log_group_name.is_none() {
            anyhow::bail!("describe_log_streams operation requires 'log_group_name' parameter");
        }

        // Validate put_metric_alarm
        if params.operation == "put_metric_alarm" {
            if params.alarm_name.is_none() {
                anyhow::bail!("put_metric_alarm operation requires 'alarm_name' parameter");
            }
            if params.namespace.is_none() {
                anyhow::bail!("put_metric_alarm operation requires 'namespace' parameter");
            }
            if params.metric_name.is_none() {
                anyhow::bail!("put_metric_alarm operation requires 'metric_name' parameter");
            }
            if params.comparison_operator.is_none() {
                anyhow::bail!(
                    "put_metric_alarm operation requires 'comparison_operator' parameter"
                );
            }
            if params.evaluation_periods.is_none() {
                anyhow::bail!("put_metric_alarm operation requires 'evaluation_periods' parameter");
            }
            if params.threshold.is_none() {
                anyhow::bail!("put_metric_alarm operation requires 'threshold' parameter");
            }
        }

        // Validate delete_alarms
        if params.operation == "delete_alarms" && params.alarm_name.is_none() {
            anyhow::bail!("delete_alarms operation requires 'alarm_name' parameter");
        }

        // Validate set_alarm_state
        if params.operation == "set_alarm_state" && params.alarm_name.is_none() {
            anyhow::bail!("set_alarm_state operation requires 'alarm_name' parameter");
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
    fn test_aws_cloudwatch_node_type() {
        let node = AwsCloudWatchNode::new();
        assert_eq!(node.type_name(), "aws_cloudwatch");
        assert!(matches!(node.category(), NodeCategory::Action));
        assert!(matches!(node.subcategory(), NodeSubcategory::General));
        assert_eq!(node.required_credential_type(), Some("aws"));
    }

    #[test]
    fn test_aws_cloudwatch_parameter_schema() {
        let node = AwsCloudWatchNode::new();
        let schema = node.parameter_schema();

        assert_eq!(schema["type"], "object");
        assert!(schema["properties"]["operation"].is_object());
        assert!(schema["properties"]["namespace"].is_object());
        assert!(schema["properties"]["metric_name"].is_object());
        assert!(schema["properties"]["log_group_name"].is_object());
        assert!(schema["properties"]["alarm_name"].is_object());
    }

    #[tokio::test]
    async fn test_cloudwatch_put_metric_data() {
        let node = AwsCloudWatchNode::new();
        let context = NodeContext::new(Uuid::new_v4().to_string(), "test-workflow".to_string());

        let params = json!({
            "operation": "put_metric_data",
            "namespace": "Custom/MyApp",
            "metric_name": "RequestCount",
            "value": 42.0,
            "unit": "Count",
            "dimensions": {
                "Environment": "Production",
                "Service": "API"
            }
        });

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
        assert_eq!(output.data["operation"], "put_metric_data");
        assert_eq!(output.data["namespace"], "Custom/MyApp");
        assert_eq!(output.data["metric_name"], "RequestCount");
    }

    #[tokio::test]
    async fn test_cloudwatch_get_metric_statistics() {
        let node = AwsCloudWatchNode::new();
        let context = NodeContext::new(Uuid::new_v4().to_string(), "test-workflow".to_string());

        let params = json!({
            "operation": "get_metric_statistics",
            "namespace": "AWS/EC2",
            "metric_name": "CPUUtilization",
            "statistics": ["Average", "Maximum"],
            "period": 300,
            "start_time": "2025-01-15T10:00:00Z",
            "end_time": "2025-01-15T11:00:00Z"
        });

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
        assert_eq!(output.data["operation"], "get_metric_statistics");
        assert!(output.data["datapoints"].is_array());
    }

    #[tokio::test]
    async fn test_cloudwatch_create_log_group() {
        let node = AwsCloudWatchNode::new();
        let context = NodeContext::new(Uuid::new_v4().to_string(), "test-workflow".to_string());

        let params = json!({
            "operation": "create_log_group",
            "log_group_name": "/aws/lambda/my-function"
        });

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
        assert_eq!(output.data["operation"], "create_log_group");
        assert_eq!(output.data["log_group_name"], "/aws/lambda/my-function");
    }

    #[tokio::test]
    async fn test_cloudwatch_put_log_events() {
        let node = AwsCloudWatchNode::new();
        let context = NodeContext::new(Uuid::new_v4().to_string(), "test-workflow".to_string());

        let params = json!({
            "operation": "put_log_events",
            "log_group_name": "/aws/lambda/my-function",
            "log_stream_name": "2025/01/15/[$LATEST]abcdef123456",
            "log_events": [
                {
                    "timestamp": 1705315200000i64,
                    "message": "START RequestId: 12345678"
                },
                {
                    "timestamp": 1705315201000i64,
                    "message": "END RequestId: 12345678"
                }
            ]
        });

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
        assert_eq!(output.data["operation"], "put_log_events");
        assert!(output.data["next_sequence_token"].is_string());
    }

    #[tokio::test]
    async fn test_cloudwatch_put_metric_alarm() {
        let node = AwsCloudWatchNode::new();
        let context = NodeContext::new(Uuid::new_v4().to_string(), "test-workflow".to_string());

        let params = json!({
            "operation": "put_metric_alarm",
            "alarm_name": "HighCPUUtilization",
            "namespace": "AWS/EC2",
            "metric_name": "CPUUtilization",
            "comparison_operator": "GreaterThanThreshold",
            "threshold": 80.0,
            "evaluation_periods": 2,
            "period": 300,
            "statistic": "Average"
        });

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
        assert_eq!(output.data["operation"], "put_metric_alarm");
        assert_eq!(output.data["alarm_name"], "HighCPUUtilization");
    }

    #[test]
    fn test_cloudwatch_validation_put_metric_data() {
        let node = AwsCloudWatchNode::new();

        // Valid put_metric_data
        let valid_params = json!({
            "operation": "put_metric_data",
            "namespace": "Custom/MyApp",
            "metric_name": "RequestCount",
            "value": 100.0
        });
        assert!(node.validate_parameters(&valid_params).is_ok());

        // Missing namespace
        let invalid_params = json!({
            "operation": "put_metric_data",
            "metric_name": "RequestCount",
            "value": 100.0
        });
        assert!(node.validate_parameters(&invalid_params).is_err());

        // Missing metric_name
        let invalid_params = json!({
            "operation": "put_metric_data",
            "namespace": "Custom/MyApp",
            "value": 100.0
        });
        assert!(node.validate_parameters(&invalid_params).is_err());

        // Missing value
        let invalid_params = json!({
            "operation": "put_metric_data",
            "namespace": "Custom/MyApp",
            "metric_name": "RequestCount"
        });
        assert!(node.validate_parameters(&invalid_params).is_err());
    }

    #[test]
    fn test_cloudwatch_validation_get_metric_statistics() {
        let node = AwsCloudWatchNode::new();

        // Valid get_metric_statistics
        let valid_params = json!({
            "operation": "get_metric_statistics",
            "namespace": "AWS/EC2",
            "metric_name": "CPUUtilization",
            "statistics": ["Average"],
            "period": 300,
            "start_time": "2025-01-15T10:00:00Z",
            "end_time": "2025-01-15T11:00:00Z"
        });
        assert!(node.validate_parameters(&valid_params).is_ok());

        // Invalid period (not multiple of 60)
        let invalid_params = json!({
            "operation": "get_metric_statistics",
            "namespace": "AWS/EC2",
            "metric_name": "CPUUtilization",
            "statistics": ["Average"],
            "period": 45,
            "start_time": "2025-01-15T10:00:00Z",
            "end_time": "2025-01-15T11:00:00Z"
        });
        assert!(node.validate_parameters(&invalid_params).is_err());

        // Missing statistics
        let invalid_params = json!({
            "operation": "get_metric_statistics",
            "namespace": "AWS/EC2",
            "metric_name": "CPUUtilization",
            "period": 300,
            "start_time": "2025-01-15T10:00:00Z",
            "end_time": "2025-01-15T11:00:00Z"
        });
        assert!(node.validate_parameters(&invalid_params).is_err());
    }

    #[test]
    fn test_cloudwatch_validation_log_operations() {
        let node = AwsCloudWatchNode::new();

        // Valid create_log_stream
        let valid_params = json!({
            "operation": "create_log_stream",
            "log_group_name": "/aws/lambda/my-function",
            "log_stream_name": "stream-1"
        });
        assert!(node.validate_parameters(&valid_params).is_ok());

        // Missing log_group_name
        let invalid_params = json!({
            "operation": "create_log_stream",
            "log_stream_name": "stream-1"
        });
        assert!(node.validate_parameters(&invalid_params).is_err());

        // Valid put_log_events
        let valid_params = json!({
            "operation": "put_log_events",
            "log_group_name": "/aws/lambda/my-function",
            "log_stream_name": "stream-1",
            "log_events": [{"timestamp": 1705315200000i64, "message": "test"}]
        });
        assert!(node.validate_parameters(&valid_params).is_ok());
    }

    #[test]
    fn test_cloudwatch_validation_alarm_operations() {
        let node = AwsCloudWatchNode::new();

        // Valid put_metric_alarm
        let valid_params = json!({
            "operation": "put_metric_alarm",
            "alarm_name": "HighCPU",
            "namespace": "AWS/EC2",
            "metric_name": "CPUUtilization",
            "comparison_operator": "GreaterThanThreshold",
            "evaluation_periods": 2,
            "threshold": 80.0
        });
        assert!(node.validate_parameters(&valid_params).is_ok());

        // Missing threshold
        let invalid_params = json!({
            "operation": "put_metric_alarm",
            "alarm_name": "HighCPU",
            "namespace": "AWS/EC2",
            "metric_name": "CPUUtilization",
            "comparison_operator": "GreaterThanThreshold",
            "evaluation_periods": 2
        });
        assert!(node.validate_parameters(&invalid_params).is_err());

        // Valid delete_alarms
        let valid_params = json!({
            "operation": "delete_alarms",
            "alarm_name": "HighCPU"
        });
        assert!(node.validate_parameters(&valid_params).is_ok());
    }

    #[test]
    fn test_cloudwatch_validation_invalid_operation() {
        let node = AwsCloudWatchNode::new();

        let invalid_params = json!({
            "operation": "invalid_operation"
        });
        assert!(node.validate_parameters(&invalid_params).is_err());
    }
}
