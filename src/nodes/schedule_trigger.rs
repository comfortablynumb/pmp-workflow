use crate::models::{Node, NodeContext, NodeOutput, NodeType};
use async_trait::async_trait;
use cron::Schedule;
use serde::Deserialize;
use std::str::FromStr;

#[derive(Debug, Deserialize)]
pub struct ScheduleTriggerParams {
    /// Cron expression (e.g., "0 0 0 * * *" for daily at midnight)
    /// Format: second minute hour day-of-month month day-of-week
    pub cron: String,
    /// Optional timezone (defaults to UTC)
    #[serde(default = "default_timezone")]
    pub timezone: String,
    /// Optional description
    pub description: Option<String>,
}

fn default_timezone() -> String {
    "UTC".to_string()
}

/// Schedule Trigger node - starts workflow based on cron schedule
/// Note: This node only validates and stores the schedule configuration.
/// Actual scheduling is handled externally.
pub struct ScheduleTriggerNode;

impl NodeType for ScheduleTriggerNode {
    fn type_name(&self) -> &str {
        "schedule_trigger"
    }

    fn parameter_schema(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "cron": {
                    "type": "string",
                    "description": "Cron expression in 6-field format: second minute hour day-of-month month day-of-week (e.g., '0 0 0 * * *' for daily at midnight)",
                    "pattern": "^[0-9*,/-]+ [0-9*,/-]+ [0-9*,/-]+ [0-9*,/-]+ [0-9*,/-]+ [0-9A-Z*,/-]+$"
                },
                "timezone": {
                    "type": "string",
                    "description": "Timezone for schedule execution",
                    "default": "UTC"
                },
                "description": {
                    "type": "string",
                    "description": "Description of this schedule trigger"
                }
            },
            "required": ["cron"],
            "additionalProperties": false
        })
    }
}

#[async_trait]
impl Node for ScheduleTriggerNode {
    async fn execute(
        &self,
        context: &NodeContext,
        _parameters: &serde_json::Value,
    ) -> anyhow::Result<NodeOutput> {
        // Schedule trigger passes through input data
        // In practice, this would include timestamp and other schedule metadata
        let data = context.get_main_input().cloned().unwrap_or_else(|| {
            serde_json::json!({
                "trigger_type": "schedule",
                "triggered_at": chrono::Utc::now().to_rfc3339()
            })
        });

        Ok(NodeOutput::success(data))
    }

    fn validate_parameters(&self, parameters: &serde_json::Value) -> anyhow::Result<()> {
        let params: ScheduleTriggerParams = serde_json::from_value(parameters.clone())?;

        // Validate cron expression
        Schedule::from_str(&params.cron)
            .map_err(|e| anyhow::anyhow!("Invalid cron expression '{}': {}", params.cron, e))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_schedule_trigger_execution() {
        let node = ScheduleTriggerNode;
        let context = NodeContext::new("exec-123".to_string(), "trigger-1".to_string());

        let params = serde_json::json!({
            "cron": "0 0 0 * * *",
            "timezone": "UTC",
            "description": "Daily at midnight"
        });

        let result = node.execute(&context, &params).await.unwrap();
        assert!(result.success);
        assert!(result.data.get("trigger_type").is_some());
    }

    #[test]
    fn test_schedule_trigger_validation() {
        let node = ScheduleTriggerNode;

        // Valid cron expressions (format: sec min hour day month day-of-week)
        let valid_daily = serde_json::json!({
            "cron": "0 0 0 * * *"  // Daily at midnight
        });
        assert!(node.validate_parameters(&valid_daily).is_ok());

        let valid_hourly = serde_json::json!({
            "cron": "0 0 * * * *"  // Every hour
        });
        assert!(node.validate_parameters(&valid_hourly).is_ok());

        let valid_every_5_min = serde_json::json!({
            "cron": "0 */5 * * * *"  // Every 5 minutes
        });
        assert!(node.validate_parameters(&valid_every_5_min).is_ok());

        // Invalid cron expression
        let invalid_params = serde_json::json!({
            "cron": "invalid cron"
        });
        assert!(node.validate_parameters(&invalid_params).is_err());
    }

    #[test]
    fn test_various_cron_patterns() {
        let node = ScheduleTriggerNode;

        let test_cases = vec![
            ("0 0 0 * * *", true),          // Daily at midnight
            ("0 0 */2 * * *", true),        // Every 2 hours
            ("0 0 9-17 * * MON-FRI", true), // Weekdays 9am-5pm
            ("0 0 0 1 * *", true),          // First day of month
            ("invalid", false),             // Invalid
            ("99 99 99 * * *", false),      // Invalid values
        ];

        for (cron_expr, should_be_valid) in test_cases {
            let params = serde_json::json!({
                "cron": cron_expr
            });
            let result = node.validate_parameters(&params);
            assert_eq!(
                result.is_ok(),
                should_be_valid,
                "Cron expression '{}' validation mismatch",
                cron_expr
            );
        }
    }
}
