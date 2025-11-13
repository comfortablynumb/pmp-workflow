use crate::models::{Node, NodeCategory, NodeContext, NodeOutput, NodeSubcategory, NodeType};
use async_trait::async_trait;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TwilioParams {
    /// Credentials name to use for Twilio API
    pub credentials_name: String,
    /// Twilio operation to perform
    pub operation: String,
    /// From phone number (Twilio number)
    pub from: Option<String>,
    /// To phone number
    pub to: Option<String>,
    /// Message body (for SMS)
    pub body: Option<String>,
    /// Message SID
    pub message_sid: Option<String>,
    /// Call SID
    pub call_sid: Option<String>,
    /// TwiML URL for voice calls
    pub url: Option<String>,
    /// TwiML content
    pub twiml: Option<String>,
    /// Status callback URL
    pub status_callback: Option<String>,
    /// Media URL (for MMS)
    pub media_url: Option<Vec<String>>,
    /// Voice (for calls): "man", "woman", "alice"
    pub voice: Option<String>,
    /// Language for voice
    pub language: Option<String>,
    /// Recording SID
    pub recording_sid: Option<String>,
    /// Date filter (for listing)
    pub date_sent: Option<String>,
    /// Page size for pagination
    pub page_size: Option<u32>,
    /// Additional parameters
    #[serde(default)]
    pub additional_params: serde_json::Value,
}

/// Twilio node - performs Twilio API operations
pub struct TwilioNode;

impl TwilioNode {
    pub fn new() -> Self {
        Self
    }
}

impl Default for TwilioNode {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeType for TwilioNode {
    fn type_name(&self) -> &str {
        "twilio"
    }

    fn category(&self) -> NodeCategory {
        NodeCategory::Action
    }

    fn subcategory(&self) -> NodeSubcategory {
        NodeSubcategory::Communication
    }

    fn required_credential_type(&self) -> Option<&str> {
        Some("twilio_api_key")
    }

    fn parameter_schema(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "credentials_name": {
                    "type": "string",
                    "description": "Name of the Twilio credentials to use",
                    "minLength": 1
                },
                "operation": {
                    "type": "string",
                    "description": "Twilio operation to perform",
                    "enum": [
                        "send_sms",
                        "send_mms",
                        "make_call",
                        "get_message",
                        "list_messages",
                        "delete_message",
                        "get_call",
                        "list_calls",
                        "update_call",
                        "delete_call",
                        "get_recording",
                        "list_recordings",
                        "delete_recording",
                        "lookup_phone",
                        "get_account",
                        "list_available_numbers"
                    ]
                },
                "from": {
                    "type": "string",
                    "description": "From phone number (must be a Twilio number)",
                    "pattern": "^\\+?[1-9]\\d{1,14}$"
                },
                "to": {
                    "type": "string",
                    "description": "To phone number (E.164 format)",
                    "pattern": "^\\+?[1-9]\\d{1,14}$"
                },
                "body": {
                    "type": "string",
                    "description": "Message body (up to 1600 characters)"
                },
                "message_sid": {
                    "type": "string",
                    "description": "Message SID"
                },
                "call_sid": {
                    "type": "string",
                    "description": "Call SID"
                },
                "url": {
                    "type": "string",
                    "description": "TwiML URL for voice calls",
                    "format": "uri"
                },
                "twiml": {
                    "type": "string",
                    "description": "TwiML content for voice calls"
                },
                "status_callback": {
                    "type": "string",
                    "description": "Status callback URL",
                    "format": "uri"
                },
                "media_url": {
                    "type": "array",
                    "description": "Media URLs for MMS (up to 10)",
                    "items": {
                        "type": "string",
                        "format": "uri"
                    },
                    "maxItems": 10
                },
                "voice": {
                    "type": "string",
                    "description": "Voice for calls",
                    "enum": ["man", "woman", "alice"]
                },
                "language": {
                    "type": "string",
                    "description": "Language code (e.g., en-US, es-ES)"
                },
                "recording_sid": {
                    "type": "string",
                    "description": "Recording SID"
                },
                "date_sent": {
                    "type": "string",
                    "description": "Date filter (YYYY-MM-DD)"
                },
                "page_size": {
                    "type": "integer",
                    "description": "Page size for pagination (1-1000)",
                    "minimum": 1,
                    "maximum": 1000,
                    "default": 50
                },
                "additional_params": {
                    "type": "object",
                    "description": "Additional parameters to pass to the API"
                }
            },
            "required": ["credentials_name", "operation"],
            "additionalProperties": false
        })
    }
}

#[async_trait]
impl Node for TwilioNode {
    async fn execute(
        &self,
        context: &NodeContext,
        parameters: &serde_json::Value,
    ) -> anyhow::Result<NodeOutput> {
        let params: TwilioParams = serde_json::from_value(parameters.clone())?;

        // TODO: In a real implementation:
        // 1. Look up credentials from credentials table
        // 2. Decrypt account_sid and auth_token
        // 3. Make API requests to Twilio:
        //
        //    API Base: https://api.twilio.com/2010-04-01/Accounts/{AccountSid}/
        //    Auth: Basic Auth with AccountSid:AuthToken
        //
        //    send_sms: POST /Messages.json
        //      Body: From, To, Body
        //    send_mms: POST /Messages.json
        //      Body: From, To, Body, MediaUrl
        //    make_call: POST /Calls.json
        //      Body: From, To, Url (TwiML)
        //    get_message: GET /Messages/{MessageSid}.json
        //    list_messages: GET /Messages.json
        //    get_call: GET /Calls/{CallSid}.json
        //    update_call: POST /Calls/{CallSid}.json
        //    get_recording: GET /Recordings/{RecordingSid}.json
        //    lookup_phone: GET https://lookups.twilio.com/v1/PhoneNumbers/{phone}
        //
        // 4. Handle pagination
        // 5. Return results

        let result = serde_json::json!({
            "message": "Twilio operation executed (placeholder)",
            "credentials_name": &params.credentials_name,
            "operation": &params.operation,
            "from": params.from,
            "to": params.to,
            "context_execution_id": &context.execution_id,
            "success": true
        });

        Ok(NodeOutput::success(result))
    }

    fn validate_parameters(&self, parameters: &serde_json::Value) -> anyhow::Result<()> {
        let params: TwilioParams = serde_json::from_value(parameters.clone())
            .map_err(|e| anyhow::anyhow!("Invalid parameters: {}", e))?;

        if params.credentials_name.trim().is_empty() {
            anyhow::bail!("credentials_name cannot be empty");
        }

        let valid_operations = [
            "send_sms",
            "send_mms",
            "make_call",
            "get_message",
            "list_messages",
            "delete_message",
            "get_call",
            "list_calls",
            "update_call",
            "delete_call",
            "get_recording",
            "list_recordings",
            "delete_recording",
            "lookup_phone",
            "get_account",
            "list_available_numbers",
        ];

        if !valid_operations.contains(&params.operation.as_str()) {
            anyhow::bail!(
                "Invalid operation: {}. Must be one of: {}",
                params.operation,
                valid_operations.join(", ")
            );
        }

        // Validate SMS operations
        if ["send_sms", "send_mms"].contains(&params.operation.as_str()) {
            if params.from.is_none() {
                anyhow::bail!("{} operation requires 'from' parameter", params.operation);
            }
            if params.to.is_none() {
                anyhow::bail!("{} operation requires 'to' parameter", params.operation);
            }
            if params.body.is_none() {
                anyhow::bail!("{} operation requires 'body' parameter", params.operation);
            }
        }

        // Validate MMS requires media_url
        if params.operation == "send_mms" && params.media_url.is_none() {
            anyhow::bail!("send_mms operation requires 'media_url' parameter");
        }

        // Validate call operations
        if params.operation == "make_call" {
            if params.from.is_none() {
                anyhow::bail!("make_call operation requires 'from' parameter");
            }
            if params.to.is_none() {
                anyhow::bail!("make_call operation requires 'to' parameter");
            }
            if params.url.is_none() && params.twiml.is_none() {
                anyhow::bail!("make_call operation requires 'url' or 'twiml' parameter");
            }
        }

        // Validate message operations require message_sid
        if ["get_message", "delete_message"].contains(&params.operation.as_str())
            && params.message_sid.is_none()
        {
            anyhow::bail!("{} operation requires 'message_sid' parameter", params.operation);
        }

        // Validate call operations require call_sid
        if ["get_call", "update_call", "delete_call"].contains(&params.operation.as_str())
            && params.call_sid.is_none()
        {
            anyhow::bail!("{} operation requires 'call_sid' parameter", params.operation);
        }

        // Validate recording operations require recording_sid
        if ["get_recording", "delete_recording"].contains(&params.operation.as_str())
            && params.recording_sid.is_none()
        {
            anyhow::bail!("{} operation requires 'recording_sid' parameter", params.operation);
        }

        // Validate lookup_phone requires to
        if params.operation == "lookup_phone" && params.to.is_none() {
            anyhow::bail!("lookup_phone operation requires 'to' parameter");
        }

        // Validate page_size range
        if let Some(page_size) = params.page_size
            && !(1..=1000).contains(&page_size)
        {
            anyhow::bail!("page_size must be between 1 and 1000");
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_twilio_send_sms() {
        let node = TwilioNode::new();
        let context = NodeContext::new("exec-123".to_string(), "node-1".to_string());

        let params = serde_json::json!({
            "credentials_name": "my_twilio",
            "operation": "send_sms",
            "from": "+15551234567",
            "to": "+15559876543",
            "body": "Hello from workflow automation!"
        });

        let result = node.execute(&context, &params).await.unwrap();
        assert!(result.success);
        assert_eq!(result.data["operation"], "send_sms");
    }

    #[tokio::test]
    async fn test_twilio_make_call() {
        let node = TwilioNode::new();
        let context = NodeContext::new("exec-123".to_string(), "node-1".to_string());

        let params = serde_json::json!({
            "credentials_name": "my_twilio",
            "operation": "make_call",
            "from": "+15551234567",
            "to": "+15559876543",
            "url": "https://example.com/twiml.xml"
        });

        let result = node.execute(&context, &params).await.unwrap();
        assert!(result.success);
        assert_eq!(result.data["operation"], "make_call");
    }

    #[test]
    fn test_twilio_validation() {
        let node = TwilioNode::new();

        // Valid SMS
        let valid = serde_json::json!({
            "credentials_name": "my_twilio",
            "operation": "send_sms",
            "from": "+15551234567",
            "to": "+15559876543",
            "body": "Test"
        });
        assert!(node.validate_parameters(&valid).is_ok());

        // Invalid: SMS without body
        let invalid = serde_json::json!({
            "credentials_name": "my_twilio",
            "operation": "send_sms",
            "from": "+15551234567",
            "to": "+15559876543"
        });
        assert!(node.validate_parameters(&invalid).is_err());

        // Invalid: make_call without url or twiml
        let invalid2 = serde_json::json!({
            "credentials_name": "my_twilio",
            "operation": "make_call",
            "from": "+15551234567",
            "to": "+15559876543"
        });
        assert!(node.validate_parameters(&invalid2).is_err());
    }

    #[test]
    fn test_twilio_node_type() {
        let node = TwilioNode::new();
        assert_eq!(node.type_name(), "twilio");
        assert_eq!(node.category(), NodeCategory::Action);
        assert_eq!(node.subcategory(), NodeSubcategory::Communication);
        assert_eq!(node.required_credential_type(), Some("twilio_api_key"));
    }
}
