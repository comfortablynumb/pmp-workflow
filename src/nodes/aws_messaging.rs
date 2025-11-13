use crate::models::{Node, NodeCategory, NodeContext, NodeOutput, NodeSubcategory, NodeType};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::json;

/// AWS Messaging node for SQS and SNS operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AwsMessagingNode {
    #[serde(skip)]
    _private: (),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AwsMessagingParams {
    /// The operation to perform
    pub operation: String,

    /// Service type (sqs or sns)
    pub service: String,

    // Queue operations (SQS)
    /// Queue URL for SQS operations
    pub queue_url: Option<String>,
    /// Queue name for SQS operations
    pub queue_name: Option<String>,

    // Topic operations (SNS)
    /// Topic ARN for SNS operations
    pub topic_arn: Option<String>,
    /// Topic name for SNS operations
    pub topic_name: Option<String>,

    // Message operations
    /// Message body to send
    pub message_body: Option<String>,
    /// Message attributes (key-value pairs)
    pub message_attributes: Option<serde_json::Value>,

    // SQS-specific parameters
    /// Delay seconds for message delivery (0-900)
    pub delay_seconds: Option<u32>,
    /// Visibility timeout for received messages (0-43200)
    pub visibility_timeout: Option<u32>,
    /// Maximum number of messages to receive (1-10)
    pub max_messages: Option<u32>,
    /// Wait time for long polling (0-20)
    pub wait_time_seconds: Option<u32>,
    /// Receipt handle for message deletion
    pub receipt_handle: Option<String>,

    // SNS-specific parameters
    /// Subject for email notifications
    pub subject: Option<String>,
    /// Phone number for SMS
    pub phone_number: Option<String>,
    /// Endpoint for subscriptions
    pub endpoint: Option<String>,
    /// Protocol for subscriptions (http, https, email, sms, sqs, application, lambda)
    pub protocol: Option<String>,
}

impl AwsMessagingNode {
    pub fn new() -> Self {
        Self { _private: () }
    }
}

impl Default for AwsMessagingNode {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl NodeType for AwsMessagingNode {
    fn type_name(&self) -> &str {
        "aws_messaging"
    }

    fn category(&self) -> NodeCategory {
        NodeCategory::Action
    }

    fn subcategory(&self) -> NodeSubcategory {
        NodeSubcategory::General
    }

    fn required_credential_type(&self) -> Option<&str> {
        Some("aws")
    }

    fn parameter_schema(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "required": ["operation", "service"],
            "properties": {
                "operation": {
                    "type": "string",
                    "enum": [
                        "send_message",
                        "receive_messages",
                        "delete_message",
                        "delete_messages",
                        "purge_queue",
                        "get_queue_attributes",
                        "set_queue_attributes",
                        "create_queue",
                        "delete_queue",
                        "list_queues",
                        "publish",
                        "create_topic",
                        "delete_topic",
                        "subscribe",
                        "unsubscribe",
                        "list_topics",
                        "list_subscriptions",
                        "get_topic_attributes"
                    ],
                    "description": "The AWS messaging operation to perform"
                },
                "service": {
                    "type": "string",
                    "enum": ["sqs", "sns"],
                    "description": "The AWS service to use (SQS or SNS)"
                },
                "queue_url": {
                    "type": "string",
                    "description": "Queue URL for SQS operations"
                },
                "queue_name": {
                    "type": "string",
                    "description": "Queue name for SQS operations"
                },
                "topic_arn": {
                    "type": "string",
                    "description": "Topic ARN for SNS operations"
                },
                "topic_name": {
                    "type": "string",
                    "description": "Topic name for SNS operations"
                },
                "message_body": {
                    "type": "string",
                    "description": "Message body to send"
                },
                "message_attributes": {
                    "description": "Message attributes (key-value pairs)"
                },
                "delay_seconds": {
                    "type": "integer",
                    "minimum": 0,
                    "maximum": 900,
                    "description": "Delay seconds for message delivery"
                },
                "visibility_timeout": {
                    "type": "integer",
                    "minimum": 0,
                    "maximum": 43200,
                    "description": "Visibility timeout for received messages"
                },
                "max_messages": {
                    "type": "integer",
                    "minimum": 1,
                    "maximum": 10,
                    "description": "Maximum number of messages to receive"
                },
                "wait_time_seconds": {
                    "type": "integer",
                    "minimum": 0,
                    "maximum": 20,
                    "description": "Wait time for long polling"
                },
                "receipt_handle": {
                    "type": "string",
                    "description": "Receipt handle for message deletion"
                },
                "subject": {
                    "type": "string",
                    "description": "Subject for email notifications"
                },
                "phone_number": {
                    "type": "string",
                    "description": "Phone number for SMS"
                },
                "endpoint": {
                    "type": "string",
                    "description": "Endpoint for subscriptions"
                },
                "protocol": {
                    "type": "string",
                    "enum": ["http", "https", "email", "email-json", "sms", "sqs", "application", "lambda"],
                    "description": "Protocol for subscriptions"
                }
            }
        })
    }
}

#[async_trait]
impl Node for AwsMessagingNode {
    async fn execute(
        &self,
        context: &NodeContext,
        parameters: &serde_json::Value,
    ) -> anyhow::Result<NodeOutput> {
        let params: AwsMessagingParams = serde_json::from_value(parameters.clone())?;

        // Mock implementation - in a real implementation, this would use AWS SDK
        let result = match params.service.as_str() {
            "sqs" => {
                json!({
                    "success": true,
                    "service": "sqs",
                    "operation": params.operation,
                    "message": format!("SQS {} operation would be executed here", params.operation),
                    "queue_url": params.queue_url,
                    "queue_name": params.queue_name,
                    "execution_id": &context.execution_id
                })
            }
            "sns" => {
                json!({
                    "success": true,
                    "service": "sns",
                    "operation": params.operation,
                    "message": format!("SNS {} operation would be executed here", params.operation),
                    "topic_arn": params.topic_arn,
                    "topic_name": params.topic_name,
                    "execution_id": &context.execution_id
                })
            }
            _ => {
                return Err(anyhow::anyhow!("Invalid service: {}", params.service));
            }
        };

        Ok(NodeOutput::success(result))
    }

    fn validate_parameters(&self, parameters: &serde_json::Value) -> anyhow::Result<()> {
        let params: AwsMessagingParams = serde_json::from_value(parameters.clone())?;

        // Validate service
        if params.service != "sqs" && params.service != "sns" {
            anyhow::bail!(
                "Invalid service: {}. Must be 'sqs' or 'sns'",
                params.service
            );
        }

        // Validate SQS operations
        if params.service == "sqs" {
            match params.operation.as_str() {
                "send_message" => {
                    if params.queue_url.is_none() && params.queue_name.is_none() {
                        anyhow::bail!(
                            "send_message operation requires 'queue_url' or 'queue_name' parameter"
                        );
                    }
                    if params.message_body.is_none() {
                        anyhow::bail!("send_message operation requires 'message_body' parameter");
                    }
                    // Validate delay_seconds if provided
                    if let Some(delay) = params.delay_seconds {
                        if delay > 900 {
                            anyhow::bail!("delay_seconds must be between 0 and 900");
                        }
                    }
                }
                "receive_messages" => {
                    if params.queue_url.is_none() && params.queue_name.is_none() {
                        anyhow::bail!(
                            "receive_messages operation requires 'queue_url' or 'queue_name' parameter"
                        );
                    }
                    // Validate max_messages if provided
                    if let Some(max) = params.max_messages {
                        if max < 1 || max > 10 {
                            anyhow::bail!("max_messages must be between 1 and 10");
                        }
                    }
                    // Validate wait_time_seconds if provided
                    if let Some(wait) = params.wait_time_seconds {
                        if wait > 20 {
                            anyhow::bail!("wait_time_seconds must be between 0 and 20");
                        }
                    }
                }
                "delete_message" => {
                    if params.queue_url.is_none() && params.queue_name.is_none() {
                        anyhow::bail!(
                            "delete_message operation requires 'queue_url' or 'queue_name' parameter"
                        );
                    }
                    if params.receipt_handle.is_none() {
                        anyhow::bail!(
                            "delete_message operation requires 'receipt_handle' parameter"
                        );
                    }
                }
                "delete_messages" => {
                    if params.queue_url.is_none() && params.queue_name.is_none() {
                        anyhow::bail!(
                            "delete_messages operation requires 'queue_url' or 'queue_name' parameter"
                        );
                    }
                }
                "purge_queue"
                | "get_queue_attributes"
                | "set_queue_attributes"
                | "delete_queue" => {
                    if params.queue_url.is_none() && params.queue_name.is_none() {
                        anyhow::bail!(
                            "{} operation requires 'queue_url' or 'queue_name' parameter",
                            params.operation
                        );
                    }
                }
                "create_queue" => {
                    if params.queue_name.is_none() {
                        anyhow::bail!("create_queue operation requires 'queue_name' parameter");
                    }
                }
                "list_queues" => {
                    // No required parameters
                }
                _ => {
                    anyhow::bail!(
                        "Unknown SQS operation: {}. Valid operations: send_message, receive_messages, delete_message, delete_messages, purge_queue, get_queue_attributes, set_queue_attributes, create_queue, delete_queue, list_queues",
                        params.operation
                    );
                }
            }
        }

        // Validate SNS operations
        if params.service == "sns" {
            match params.operation.as_str() {
                "publish" => {
                    if params.topic_arn.is_none()
                        && params.topic_name.is_none()
                        && params.phone_number.is_none()
                    {
                        anyhow::bail!(
                            "publish operation requires 'topic_arn', 'topic_name', or 'phone_number' parameter"
                        );
                    }
                    if params.message_body.is_none() {
                        anyhow::bail!("publish operation requires 'message_body' parameter");
                    }
                }
                "create_topic" => {
                    if params.topic_name.is_none() {
                        anyhow::bail!("create_topic operation requires 'topic_name' parameter");
                    }
                }
                "delete_topic" | "get_topic_attributes" => {
                    if params.topic_arn.is_none() && params.topic_name.is_none() {
                        anyhow::bail!(
                            "{} operation requires 'topic_arn' or 'topic_name' parameter",
                            params.operation
                        );
                    }
                }
                "subscribe" => {
                    if params.topic_arn.is_none() && params.topic_name.is_none() {
                        anyhow::bail!(
                            "subscribe operation requires 'topic_arn' or 'topic_name' parameter"
                        );
                    }
                    if params.protocol.is_none() {
                        anyhow::bail!("subscribe operation requires 'protocol' parameter");
                    }
                    if params.endpoint.is_none() {
                        anyhow::bail!("subscribe operation requires 'endpoint' parameter");
                    }
                    // Validate protocol
                    let valid_protocols = [
                        "http",
                        "https",
                        "email",
                        "email-json",
                        "sms",
                        "sqs",
                        "application",
                        "lambda",
                    ];
                    if let Some(protocol) = &params.protocol {
                        if !valid_protocols.contains(&protocol.as_str()) {
                            anyhow::bail!(
                                "Invalid protocol: {}. Must be one of: {}",
                                protocol,
                                valid_protocols.join(", ")
                            );
                        }
                    }
                }
                "unsubscribe" => {
                    // Would require subscription_arn, but we'll accept it as a string in message_body for now
                    if params.message_body.is_none() && params.endpoint.is_none() {
                        anyhow::bail!(
                            "unsubscribe operation requires 'message_body' (subscription_arn) or 'endpoint' parameter"
                        );
                    }
                }
                "list_topics" | "list_subscriptions" => {
                    // No required parameters
                }
                _ => {
                    anyhow::bail!(
                        "Unknown SNS operation: {}. Valid operations: publish, create_topic, delete_topic, subscribe, unsubscribe, list_topics, list_subscriptions, get_topic_attributes",
                        params.operation
                    );
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aws_messaging_node_creation() {
        let node = AwsMessagingNode::new();
        assert_eq!(node.type_name(), "aws_messaging");
        assert!(matches!(node.category(), NodeCategory::Action));
        assert!(matches!(node.subcategory(), NodeSubcategory::General));
    }

    #[test]
    fn test_aws_messaging_required_credential_type() {
        let node = AwsMessagingNode::new();
        assert_eq!(node.required_credential_type(), Some("aws"));
    }

    #[test]
    fn test_sqs_send_message_validation() {
        let node = AwsMessagingNode::new();

        // Valid send_message with queue_url
        let valid_params = json!({
            "operation": "send_message",
            "service": "sqs",
            "queue_url": "https://sqs.us-east-1.amazonaws.com/123456789012/MyQueue",
            "message_body": "Hello, World!"
        });
        assert!(node.validate_parameters(&valid_params).is_ok());

        // Valid send_message with queue_name
        let valid_params_name = json!({
            "operation": "send_message",
            "service": "sqs",
            "queue_name": "MyQueue",
            "message_body": "Hello, World!",
            "delay_seconds": 30
        });
        assert!(node.validate_parameters(&valid_params_name).is_ok());

        // Invalid: missing message_body
        let invalid_params = json!({
            "operation": "send_message",
            "service": "sqs",
            "queue_url": "https://sqs.us-east-1.amazonaws.com/123456789012/MyQueue"
        });
        assert!(node.validate_parameters(&invalid_params).is_err());

        // Invalid: missing queue_url and queue_name
        let invalid_params_no_queue = json!({
            "operation": "send_message",
            "service": "sqs",
            "message_body": "Hello, World!"
        });
        assert!(node.validate_parameters(&invalid_params_no_queue).is_err());

        // Invalid: delay_seconds out of range
        let invalid_delay = json!({
            "operation": "send_message",
            "service": "sqs",
            "queue_url": "https://sqs.us-east-1.amazonaws.com/123456789012/MyQueue",
            "message_body": "Hello, World!",
            "delay_seconds": 1000
        });
        assert!(node.validate_parameters(&invalid_delay).is_err());
    }

    #[test]
    fn test_sqs_receive_messages_validation() {
        let node = AwsMessagingNode::new();

        // Valid receive_messages
        let valid_params = json!({
            "operation": "receive_messages",
            "service": "sqs",
            "queue_url": "https://sqs.us-east-1.amazonaws.com/123456789012/MyQueue",
            "max_messages": 5,
            "wait_time_seconds": 10
        });
        assert!(node.validate_parameters(&valid_params).is_ok());

        // Invalid: max_messages out of range (too high)
        let invalid_max = json!({
            "operation": "receive_messages",
            "service": "sqs",
            "queue_url": "https://sqs.us-east-1.amazonaws.com/123456789012/MyQueue",
            "max_messages": 15
        });
        assert!(node.validate_parameters(&invalid_max).is_err());

        // Invalid: max_messages out of range (too low)
        let invalid_max_low = json!({
            "operation": "receive_messages",
            "service": "sqs",
            "queue_url": "https://sqs.us-east-1.amazonaws.com/123456789012/MyQueue",
            "max_messages": 0
        });
        assert!(node.validate_parameters(&invalid_max_low).is_err());

        // Invalid: wait_time_seconds out of range
        let invalid_wait = json!({
            "operation": "receive_messages",
            "service": "sqs",
            "queue_url": "https://sqs.us-east-1.amazonaws.com/123456789012/MyQueue",
            "wait_time_seconds": 25
        });
        assert!(node.validate_parameters(&invalid_wait).is_err());
    }

    #[test]
    fn test_sqs_delete_message_validation() {
        let node = AwsMessagingNode::new();

        // Valid delete_message
        let valid_params = json!({
            "operation": "delete_message",
            "service": "sqs",
            "queue_url": "https://sqs.us-east-1.amazonaws.com/123456789012/MyQueue",
            "receipt_handle": "AQEBwJnKyrHigUMZj6rYigCgxlaS3SLy0a..."
        });
        assert!(node.validate_parameters(&valid_params).is_ok());

        // Invalid: missing receipt_handle
        let invalid_params = json!({
            "operation": "delete_message",
            "service": "sqs",
            "queue_url": "https://sqs.us-east-1.amazonaws.com/123456789012/MyQueue"
        });
        assert!(node.validate_parameters(&invalid_params).is_err());
    }

    #[test]
    fn test_sqs_create_queue_validation() {
        let node = AwsMessagingNode::new();

        // Valid create_queue
        let valid_params = json!({
            "operation": "create_queue",
            "service": "sqs",
            "queue_name": "MyNewQueue"
        });
        assert!(node.validate_parameters(&valid_params).is_ok());

        // Invalid: missing queue_name
        let invalid_params = json!({
            "operation": "create_queue",
            "service": "sqs"
        });
        assert!(node.validate_parameters(&invalid_params).is_err());
    }

    #[test]
    fn test_sqs_list_queues_validation() {
        let node = AwsMessagingNode::new();

        // Valid list_queues (no required parameters)
        let valid_params = json!({
            "operation": "list_queues",
            "service": "sqs"
        });
        assert!(node.validate_parameters(&valid_params).is_ok());
    }

    #[test]
    fn test_sns_publish_validation() {
        let node = AwsMessagingNode::new();

        // Valid publish with topic_arn
        let valid_params = json!({
            "operation": "publish",
            "service": "sns",
            "topic_arn": "arn:aws:sns:us-east-1:123456789012:MyTopic",
            "message_body": "Hello from SNS!",
            "subject": "Test Message"
        });
        assert!(node.validate_parameters(&valid_params).is_ok());

        // Valid publish with phone_number (SMS)
        let valid_sms = json!({
            "operation": "publish",
            "service": "sns",
            "phone_number": "+1234567890",
            "message_body": "SMS message"
        });
        assert!(node.validate_parameters(&valid_sms).is_ok());

        // Invalid: missing message_body
        let invalid_params = json!({
            "operation": "publish",
            "service": "sns",
            "topic_arn": "arn:aws:sns:us-east-1:123456789012:MyTopic"
        });
        assert!(node.validate_parameters(&invalid_params).is_err());

        // Invalid: missing topic_arn, topic_name, and phone_number
        let invalid_no_target = json!({
            "operation": "publish",
            "service": "sns",
            "message_body": "Hello!"
        });
        assert!(node.validate_parameters(&invalid_no_target).is_err());
    }

    #[test]
    fn test_sns_create_topic_validation() {
        let node = AwsMessagingNode::new();

        // Valid create_topic
        let valid_params = json!({
            "operation": "create_topic",
            "service": "sns",
            "topic_name": "MyNewTopic"
        });
        assert!(node.validate_parameters(&valid_params).is_ok());

        // Invalid: missing topic_name
        let invalid_params = json!({
            "operation": "create_topic",
            "service": "sns"
        });
        assert!(node.validate_parameters(&invalid_params).is_err());
    }

    #[test]
    fn test_sns_subscribe_validation() {
        let node = AwsMessagingNode::new();

        // Valid subscribe with email protocol
        let valid_params = json!({
            "operation": "subscribe",
            "service": "sns",
            "topic_arn": "arn:aws:sns:us-east-1:123456789012:MyTopic",
            "protocol": "email",
            "endpoint": "user@example.com"
        });
        assert!(node.validate_parameters(&valid_params).is_ok());

        // Valid subscribe with https protocol
        let valid_https = json!({
            "operation": "subscribe",
            "service": "sns",
            "topic_arn": "arn:aws:sns:us-east-1:123456789012:MyTopic",
            "protocol": "https",
            "endpoint": "https://example.com/sns-endpoint"
        });
        assert!(node.validate_parameters(&valid_https).is_ok());

        // Invalid: missing protocol
        let invalid_no_protocol = json!({
            "operation": "subscribe",
            "service": "sns",
            "topic_arn": "arn:aws:sns:us-east-1:123456789012:MyTopic",
            "endpoint": "user@example.com"
        });
        assert!(node.validate_parameters(&invalid_no_protocol).is_err());

        // Invalid: missing endpoint
        let invalid_no_endpoint = json!({
            "operation": "subscribe",
            "service": "sns",
            "topic_arn": "arn:aws:sns:us-east-1:123456789012:MyTopic",
            "protocol": "email"
        });
        assert!(node.validate_parameters(&invalid_no_endpoint).is_err());

        // Invalid: invalid protocol
        let invalid_protocol = json!({
            "operation": "subscribe",
            "service": "sns",
            "topic_arn": "arn:aws:sns:us-east-1:123456789012:MyTopic",
            "protocol": "invalid_protocol",
            "endpoint": "user@example.com"
        });
        assert!(node.validate_parameters(&invalid_protocol).is_err());
    }

    #[test]
    fn test_sns_list_topics_validation() {
        let node = AwsMessagingNode::new();

        // Valid list_topics (no required parameters)
        let valid_params = json!({
            "operation": "list_topics",
            "service": "sns"
        });
        assert!(node.validate_parameters(&valid_params).is_ok());
    }

    #[test]
    fn test_invalid_service_validation() {
        let node = AwsMessagingNode::new();

        // Invalid: unknown service
        let invalid_service = json!({
            "operation": "send_message",
            "service": "kinesis",
            "queue_url": "test",
            "message_body": "test"
        });
        assert!(node.validate_parameters(&invalid_service).is_err());
    }

    #[test]
    fn test_invalid_operation_for_service() {
        let node = AwsMessagingNode::new();

        // Invalid: SQS operation with SNS service
        let invalid_op = json!({
            "operation": "invalid_operation",
            "service": "sqs",
            "queue_url": "test"
        });
        assert!(node.validate_parameters(&invalid_op).is_err());

        // Invalid: SNS operation
        let invalid_sns_op = json!({
            "operation": "invalid_sns_op",
            "service": "sns",
            "topic_arn": "test"
        });
        assert!(node.validate_parameters(&invalid_sns_op).is_err());
    }

    #[tokio::test]
    async fn test_sqs_execute() {
        let node = AwsMessagingNode::new();
        let params = json!({
            "operation": "send_message",
            "service": "sqs",
            "queue_url": "https://sqs.us-east-1.amazonaws.com/123456789012/MyQueue",
            "message_body": "Test message"
        });
        let context = NodeContext::new("test-exec".to_string(), "test-node".to_string());

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
        assert_eq!(output.data["service"], "sqs");
        assert_eq!(output.data["operation"], "send_message");
    }

    #[tokio::test]
    async fn test_sns_execute() {
        let node = AwsMessagingNode::new();
        let params = json!({
            "operation": "publish",
            "service": "sns",
            "topic_arn": "arn:aws:sns:us-east-1:123456789012:MyTopic",
            "message_body": "Test message"
        });
        let context = NodeContext::new("test-exec".to_string(), "test-node".to_string());

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
        assert_eq!(output.data["service"], "sns");
        assert_eq!(output.data["operation"], "publish");
    }

    #[test]
    fn test_parameter_schema() {
        let node = AwsMessagingNode::new();
        let schema = node.parameter_schema();

        assert_eq!(schema["type"], "object");
        assert!(schema["properties"]["operation"].is_object());
        assert!(schema["properties"]["service"].is_object());
        assert!(schema["properties"]["operation"]["enum"].is_array());
        assert!(schema["properties"]["service"]["enum"].is_array());
        assert_eq!(schema["required"].as_array().unwrap().len(), 2);
    }
}
