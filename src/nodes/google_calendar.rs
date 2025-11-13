use crate::models::{Node, NodeCategory, NodeContext, NodeOutput, NodeSubcategory, NodeType};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

/// Google Calendar node for managing calendar events
#[derive(Clone)]
pub struct GoogleCalendarNode {}

#[derive(Debug, Serialize, Deserialize)]
struct GoogleCalendarParams {
    credentials_name: String,
    operation: String,
    // Common parameters
    calendar_id: Option<String>,
    // Event parameters
    event_id: Option<String>,
    summary: Option<String>,
    description: Option<String>,
    location: Option<String>,
    start_time: Option<String>,
    end_time: Option<String>,
    attendees: Option<Vec<String>>,
    // List parameters
    time_min: Option<String>,
    time_max: Option<String>,
    max_results: Option<i32>,
    // Search parameters
    query: Option<String>,
    // Reminder parameters
    reminders: Option<Value>,
}

impl GoogleCalendarNode {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for GoogleCalendarNode {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeType for GoogleCalendarNode {
    fn type_name(&self) -> &str {
        "google_calendar"
    }

    fn category(&self) -> NodeCategory {
        NodeCategory::Action
    }

    fn subcategory(&self) -> NodeSubcategory {
        NodeSubcategory::General
    }

    fn required_credential_type(&self) -> Option<&str> {
        Some("google_oauth")
    }

    fn parameter_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "credentials_name": {
                    "type": "string",
                    "description": "Name of the Google OAuth credentials"
                },
                "operation": {
                    "type": "string",
                    "enum": [
                        "create_event",
                        "get_event",
                        "update_event",
                        "delete_event",
                        "list_events",
                        "search_events",
                        "get_free_busy"
                    ],
                    "description": "Calendar operation to perform"
                },
                "calendar_id": {
                    "type": "string",
                    "description": "Calendar ID (defaults to 'primary')"
                },
                "event_id": {
                    "type": "string",
                    "description": "Event ID for get/update/delete operations"
                },
                "summary": {
                    "type": "string",
                    "description": "Event title"
                },
                "description": {
                    "type": "string",
                    "description": "Event description"
                },
                "location": {
                    "type": "string",
                    "description": "Event location"
                },
                "start_time": {
                    "type": "string",
                    "description": "Event start time (ISO 8601 format)"
                },
                "end_time": {
                    "type": "string",
                    "description": "Event end time (ISO 8601 format)"
                },
                "attendees": {
                    "type": "array",
                    "items": {"type": "string"},
                    "description": "List of attendee email addresses"
                },
                "time_min": {
                    "type": "string",
                    "description": "Lower bound for event start time (ISO 8601)"
                },
                "time_max": {
                    "type": "string",
                    "description": "Upper bound for event start time (ISO 8601)"
                },
                "max_results": {
                    "type": "integer",
                    "description": "Maximum number of events to return (1-2500)",
                    "minimum": 1,
                    "maximum": 2500
                },
                "query": {
                    "type": "string",
                    "description": "Search query text"
                },
                "reminders": {
                    "type": "object",
                    "description": "Event reminder settings"
                }
            },
            "required": ["credentials_name", "operation"]
        })
    }
}

#[async_trait]
impl Node for GoogleCalendarNode {
    async fn execute(
        &self,
        context: &NodeContext,
        parameters: &serde_json::Value,
    ) -> anyhow::Result<NodeOutput> {
        let params: GoogleCalendarParams = serde_json::from_value(parameters.clone())?;

        // In a real implementation, this would use the Google Calendar API
        // For now, we'll simulate the operations

        let result = match params.operation.as_str() {
            "create_event" => {
                json!({
                    "event_id": "evt_12345",
                    "summary": params.summary.unwrap_or_default(),
                    "start": params.start_time.unwrap_or_default(),
                    "end": params.end_time.unwrap_or_default(),
                    "status": "confirmed",
                    "html_link": "https://calendar.google.com/event?eid=evt_12345"
                })
            }
            "get_event" => {
                json!({
                    "event_id": params.event_id.unwrap_or_default(),
                    "summary": "Team Meeting",
                    "status": "confirmed",
                    "start": "2024-03-25T14:00:00Z",
                    "end": "2024-03-25T15:00:00Z"
                })
            }
            "update_event" => {
                json!({
                    "event_id": params.event_id.unwrap_or_default(),
                    "summary": params.summary.unwrap_or_default(),
                    "updated": true
                })
            }
            "delete_event" => {
                json!({
                    "event_id": params.event_id.unwrap_or_default(),
                    "deleted": true
                })
            }
            "list_events" => {
                json!({
                    "events": [
                        {
                            "id": "evt_1",
                            "summary": "Morning Standup",
                            "start": "2024-03-25T09:00:00Z"
                        },
                        {
                            "id": "evt_2",
                            "summary": "Client Call",
                            "start": "2024-03-25T14:00:00Z"
                        }
                    ],
                    "count": 2
                })
            }
            "search_events" => {
                json!({
                    "events": [
                        {
                            "id": "evt_3",
                            "summary": "Project Review",
                            "start": "2024-03-26T10:00:00Z"
                        }
                    ],
                    "query": params.query.unwrap_or_default(),
                    "count": 1
                })
            }
            "get_free_busy" => {
                json!({
                    "calendars": {
                        "primary": {
                            "busy": [
                                {
                                    "start": "2024-03-25T09:00:00Z",
                                    "end": "2024-03-25T10:00:00Z"
                                },
                                {
                                    "start": "2024-03-25T14:00:00Z",
                                    "end": "2024-03-25T15:00:00Z"
                                }
                            ]
                        }
                    }
                })
            }
            _ => {
                anyhow::bail!("Unknown operation: {}", params.operation);
            }
        };

        let output = json!({
            "result": result,
            "credentials_name": params.credentials_name,
            "operation": params.operation,
            "context_execution_id": context.execution_id,
            "success": true
        });

        Ok(NodeOutput::success(output))
    }

    fn validate_parameters(&self, parameters: &Value) -> anyhow::Result<()> {
        let params: GoogleCalendarParams = serde_json::from_value(parameters.clone())
            .map_err(|e| anyhow::anyhow!("Invalid parameters: {}", e))?;

        if params.credentials_name.trim().is_empty() {
            anyhow::bail!("credentials_name cannot be empty");
        }

        // Validate operations
        let valid_operations = [
            "create_event",
            "get_event",
            "update_event",
            "delete_event",
            "list_events",
            "search_events",
            "get_free_busy",
        ];

        if !valid_operations.contains(&params.operation.as_str()) {
            anyhow::bail!(
                "Invalid operation: {}. Must be one of: {}",
                params.operation,
                valid_operations.join(", ")
            );
        }

        // Validate create_event requires summary and times
        if params.operation == "create_event" {
            if params.summary.is_none() {
                anyhow::bail!("create_event operation requires 'summary' parameter");
            }
            if params.start_time.is_none() {
                anyhow::bail!("create_event operation requires 'start_time' parameter");
            }
            if params.end_time.is_none() {
                anyhow::bail!("create_event operation requires 'end_time' parameter");
            }
        }

        // Validate event operations require event_id
        if ["get_event", "update_event", "delete_event"].contains(&params.operation.as_str())
            && params.event_id.is_none()
        {
            anyhow::bail!(
                "{} operation requires 'event_id' parameter",
                params.operation
            );
        }

        // Validate search_events requires query
        if params.operation == "search_events" && params.query.is_none() {
            anyhow::bail!("search_events operation requires 'query' parameter");
        }

        // Validate max_results range
        if let Some(max_results) = params.max_results
            && !(1..=2500).contains(&max_results)
        {
            anyhow::bail!("max_results must be between 1 and 2500");
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_google_calendar_node_type() {
        let node = GoogleCalendarNode::new();
        assert_eq!(node.type_name(), "google_calendar");
        assert_eq!(node.category(), NodeCategory::Action);
        assert_eq!(node.subcategory(), NodeSubcategory::General);
    }

    #[test]
    fn test_google_calendar_required_credential_type() {
        let node = GoogleCalendarNode::new();
        let cred_type = node.required_credential_type();
        assert!(cred_type.is_some());
        assert_eq!(cred_type.unwrap(), "google_oauth");
    }

    #[tokio::test]
    async fn test_google_calendar_create_event() {
        let node = GoogleCalendarNode::new();
        let context = NodeContext::new("exec-123".to_string(), "node-1".to_string());

        let params = json!({
            "credentials_name": "my_google",
            "operation": "create_event",
            "summary": "Team Meeting",
            "start_time": "2024-03-25T14:00:00Z",
            "end_time": "2024-03-25T15:00:00Z"
        });

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
    }

    #[test]
    fn test_google_calendar_validation() {
        let node = GoogleCalendarNode::new();

        // Valid create_event
        let params = json!({
            "credentials_name": "my_google",
            "operation": "create_event",
            "summary": "Meeting",
            "start_time": "2024-03-25T14:00:00Z",
            "end_time": "2024-03-25T15:00:00Z"
        });
        assert!(node.validate_parameters(&params).is_ok());

        // Missing summary
        let params = json!({
            "credentials_name": "my_google",
            "operation": "create_event",
            "start_time": "2024-03-25T14:00:00Z",
            "end_time": "2024-03-25T15:00:00Z"
        });
        assert!(node.validate_parameters(&params).is_err());

        // Missing event_id for get_event
        let params = json!({
            "credentials_name": "my_google",
            "operation": "get_event"
        });
        assert!(node.validate_parameters(&params).is_err());
    }
}
