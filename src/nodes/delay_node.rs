use crate::models::{Node, NodeCategory, NodeContext, NodeOutput, NodeSubcategory, NodeType};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use tokio::time::{Duration, sleep};

/// Delay node that waits for a specified time
#[derive(Clone)]
pub struct DelayNode {}

#[derive(Debug, Serialize, Deserialize)]
struct DelayParams {
    /// Duration in seconds
    duration_seconds: Option<u64>,
    /// Duration in milliseconds
    duration_milliseconds: Option<u64>,
    /// ISO 8601 duration string (e.g., "PT1H30M" for 1 hour 30 minutes)
    duration: Option<String>,
}

impl DelayNode {
    pub fn new() -> Self {
        Self {}
    }

    fn parse_duration(&self, params: &DelayParams) -> anyhow::Result<Duration> {
        if let Some(seconds) = params.duration_seconds {
            Ok(Duration::from_secs(seconds))
        } else if let Some(ms) = params.duration_milliseconds {
            Ok(Duration::from_millis(ms))
        } else if let Some(duration_str) = &params.duration {
            // Simple parser for ISO 8601 durations (simplified implementation)
            // In production, use a proper ISO 8601 duration parser
            if duration_str.starts_with("PT") {
                let duration_str = duration_str.trim_start_matches("PT");
                if let Some(hours) = duration_str.strip_suffix('H') {
                    let hours: u64 = hours.parse()?;
                    Ok(Duration::from_secs(hours * 3600))
                } else if let Some(minutes) = duration_str.strip_suffix('M') {
                    let minutes: u64 = minutes.parse()?;
                    Ok(Duration::from_secs(minutes * 60))
                } else if let Some(seconds) = duration_str.strip_suffix('S') {
                    let seconds: u64 = seconds.parse()?;
                    Ok(Duration::from_secs(seconds))
                } else {
                    anyhow::bail!("Invalid duration format: {}", duration_str);
                }
            } else {
                anyhow::bail!("Duration must start with 'PT'");
            }
        } else {
            anyhow::bail!("No duration specified");
        }
    }
}

impl Default for DelayNode {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeType for DelayNode {
    fn type_name(&self) -> &str {
        "delay"
    }

    fn category(&self) -> NodeCategory {
        NodeCategory::Control
    }

    fn subcategory(&self) -> NodeSubcategory {
        NodeSubcategory::General
    }

    fn parameter_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "duration_seconds": {
                    "type": "integer",
                    "description": "Duration in seconds",
                    "minimum": 0,
                    "maximum": 3600
                },
                "duration_milliseconds": {
                    "type": "integer",
                    "description": "Duration in milliseconds",
                    "minimum": 0
                },
                "duration": {
                    "type": "string",
                    "description": "ISO 8601 duration (e.g., 'PT5S' for 5 seconds)",
                    "pattern": "^PT[0-9]+[HMS]$"
                }
            },
            "oneOf": [
                {"required": ["duration_seconds"]},
                {"required": ["duration_milliseconds"]},
                {"required": ["duration"]}
            ]
        })
    }
}

#[async_trait]
impl Node for DelayNode {
    async fn execute(
        &self,
        _context: &NodeContext,
        parameters: &serde_json::Value,
    ) -> anyhow::Result<NodeOutput> {
        let params: DelayParams = serde_json::from_value(parameters.clone())?;

        let duration = self.parse_duration(&params)?;
        let start = std::time::Instant::now();

        // Actually wait for the specified duration
        sleep(duration).await;

        let elapsed = start.elapsed();

        let result = json!({
            "delayed_seconds": elapsed.as_secs_f64(),
            "requested_duration_ms": duration.as_millis(),
            "actual_duration_ms": elapsed.as_millis()
        });

        Ok(NodeOutput::success(result))
    }

    fn validate_parameters(&self, parameters: &Value) -> anyhow::Result<()> {
        let params: DelayParams = serde_json::from_value(parameters.clone())
            .map_err(|e| anyhow::anyhow!("Invalid parameters: {}", e))?;

        // Ensure at least one duration parameter is provided
        if params.duration_seconds.is_none()
            && params.duration_milliseconds.is_none()
            && params.duration.is_none()
        {
            anyhow::bail!(
                "Must specify one of: duration_seconds, duration_milliseconds, or duration"
            );
        }

        // Validate reasonable limits
        if let Some(seconds) = params.duration_seconds {
            if seconds > 3600 {
                anyhow::bail!("duration_seconds cannot exceed 3600 (1 hour)");
            }
        }

        if let Some(ms) = params.duration_milliseconds {
            if ms > 3_600_000 {
                anyhow::bail!("duration_milliseconds cannot exceed 3600000 (1 hour)");
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_delay_node_milliseconds() {
        let node = DelayNode::new();
        let context = NodeContext::new("exec-123".to_string(), "node-1".to_string());
        let params = json!({"duration_milliseconds": 100});

        let start = std::time::Instant::now();
        let result = node.execute(&context, &params).await;
        let elapsed = start.elapsed();

        assert!(result.is_ok());
        assert!(elapsed.as_millis() >= 100);
    }

    #[test]
    fn test_delay_node_validation() {
        let node = DelayNode::new();

        // Valid
        let params = json!({"duration_seconds": 5});
        assert!(node.validate_parameters(&params).is_ok());

        // Invalid - no duration specified
        let params = json!({});
        assert!(node.validate_parameters(&params).is_err());

        // Invalid - exceeds limit
        let params = json!({"duration_seconds": 4000});
        assert!(node.validate_parameters(&params).is_err());
    }
}
