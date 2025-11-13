use crate::models::{Node, NodeCategory, NodeContext, NodeOutput, NodeSubcategory, NodeType};
use async_trait::async_trait;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SlackParams {
    /// Credentials name to use for Slack API
    pub credentials_name: String,
    /// Slack operation to perform
    pub operation: String,
    /// Channel ID or name (with # prefix)
    pub channel: Option<String>,
    /// User ID
    pub user: Option<String>,
    /// Message text
    pub text: Option<String>,
    /// Message timestamp (for updates/deletes)
    pub ts: Option<String>,
    /// Blocks for rich message formatting
    pub blocks: Option<Vec<serde_json::Value>>,
    /// Attachments for message
    pub attachments: Option<Vec<serde_json::Value>>,
    /// Thread timestamp to reply in thread
    pub thread_ts: Option<String>,
    /// File path for file uploads
    pub file_path: Option<String>,
    /// File content (base64 encoded or text)
    pub file_content: Option<String>,
    /// Filename for uploads
    pub filename: Option<String>,
    /// File title
    pub title: Option<String>,
    /// Initial comment for file
    pub initial_comment: Option<String>,
    /// Channel name for creation
    pub name: Option<String>,
    /// Channel description
    pub description: Option<String>,
    /// Channel topic
    pub topic: Option<String>,
    /// Channel purpose
    pub purpose: Option<String>,
    /// Is private channel
    pub is_private: Option<bool>,
    /// Emoji name for reactions
    pub emoji: Option<String>,
    /// Invite users (comma-separated user IDs)
    pub users: Option<String>,
    /// Additional parameters
    #[serde(default)]
    pub additional_params: serde_json::Value,
}

/// Slack node - performs Slack API operations
pub struct SlackNode;

impl SlackNode {
    pub fn new() -> Self {
        Self
    }
}

impl Default for SlackNode {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeType for SlackNode {
    fn type_name(&self) -> &str {
        "slack"
    }

    fn category(&self) -> NodeCategory {
        NodeCategory::Action
    }

    fn subcategory(&self) -> NodeSubcategory {
        NodeSubcategory::Communication
    }

    fn parameter_schema(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "credentials_name": {
                    "type": "string",
                    "description": "Name of the Slack credentials to use",
                    "minLength": 1
                },
                "operation": {
                    "type": "string",
                    "description": "Slack operation to perform",
                    "enum": [
                        "send_message",
                        "update_message",
                        "delete_message",
                        "post_file",
                        "list_channels",
                        "create_channel",
                        "archive_channel",
                        "invite_to_channel",
                        "kick_from_channel",
                        "get_channel_info",
                        "set_channel_topic",
                        "set_channel_purpose",
                        "get_user_info",
                        "list_users",
                        "add_reaction",
                        "remove_reaction",
                        "get_message_history",
                        "search_messages"
                    ]
                },
                "channel": {
                    "type": "string",
                    "description": "Channel ID or name (with # prefix)"
                },
                "user": {
                    "type": "string",
                    "description": "User ID"
                },
                "text": {
                    "type": "string",
                    "description": "Message text (supports Slack markdown)"
                },
                "ts": {
                    "type": "string",
                    "description": "Message timestamp (for updates/deletes)"
                },
                "blocks": {
                    "type": "array",
                    "description": "Blocks for rich message formatting",
                    "items": {
                        "type": "object"
                    }
                },
                "attachments": {
                    "type": "array",
                    "description": "Message attachments",
                    "items": {
                        "type": "object"
                    }
                },
                "thread_ts": {
                    "type": "string",
                    "description": "Thread timestamp to reply in thread"
                },
                "file_path": {
                    "type": "string",
                    "description": "Local file path for uploads"
                },
                "file_content": {
                    "type": "string",
                    "description": "File content (base64 encoded or text)"
                },
                "filename": {
                    "type": "string",
                    "description": "Filename for uploads"
                },
                "title": {
                    "type": "string",
                    "description": "File title"
                },
                "initial_comment": {
                    "type": "string",
                    "description": "Initial comment for file upload"
                },
                "name": {
                    "type": "string",
                    "description": "Channel name (for creation)"
                },
                "description": {
                    "type": "string",
                    "description": "Channel description"
                },
                "topic": {
                    "type": "string",
                    "description": "Channel topic"
                },
                "purpose": {
                    "type": "string",
                    "description": "Channel purpose"
                },
                "is_private": {
                    "type": "boolean",
                    "description": "Create private channel (default: false)"
                },
                "emoji": {
                    "type": "string",
                    "description": "Emoji name for reactions (without colons)"
                },
                "users": {
                    "type": "string",
                    "description": "Comma-separated user IDs"
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
impl Node for SlackNode {
    async fn execute(
        &self,
        context: &NodeContext,
        parameters: &serde_json::Value,
    ) -> anyhow::Result<NodeOutput> {
        let params: SlackParams = serde_json::from_value(parameters.clone())?;

        // TODO: In a real implementation:
        // 1. Look up credentials from the credentials table using params.credentials_name
        // 2. Decrypt the credentials data to get the Slack bot token
        // 3. Create a Slack client using reqwest with the bot token
        // 4. Execute the operation based on params.operation:
        //
        //    send_message:
        //    POST https://slack.com/api/chat.postMessage
        //    { "channel": "C1234567890", "text": "Hello", "blocks": [...] }
        //
        //    update_message:
        //    POST https://slack.com/api/chat.update
        //    { "channel": "C1234567890", "ts": "1234567890.123456", "text": "Updated" }
        //
        //    delete_message:
        //    POST https://slack.com/api/chat.delete
        //    { "channel": "C1234567890", "ts": "1234567890.123456" }
        //
        //    post_file:
        //    POST https://slack.com/api/files.upload
        //    { "channels": "C1234567890", "file": <binary>, "filename": "file.txt" }
        //
        //    list_channels:
        //    GET https://slack.com/api/conversations.list
        //    { "types": "public_channel,private_channel" }
        //
        //    create_channel:
        //    POST https://slack.com/api/conversations.create
        //    { "name": "new-channel", "is_private": false }
        //
        //    archive_channel:
        //    POST https://slack.com/api/conversations.archive
        //    { "channel": "C1234567890" }
        //
        //    invite_to_channel:
        //    POST https://slack.com/api/conversations.invite
        //    { "channel": "C1234567890", "users": "U1234567890,U0987654321" }
        //
        //    kick_from_channel:
        //    POST https://slack.com/api/conversations.kick
        //    { "channel": "C1234567890", "user": "U1234567890" }
        //
        //    get_channel_info:
        //    GET https://slack.com/api/conversations.info
        //    { "channel": "C1234567890" }
        //
        //    set_channel_topic:
        //    POST https://slack.com/api/conversations.setTopic
        //    { "channel": "C1234567890", "topic": "New topic" }
        //
        //    set_channel_purpose:
        //    POST https://slack.com/api/conversations.setPurpose
        //    { "channel": "C1234567890", "purpose": "New purpose" }
        //
        //    get_user_info:
        //    GET https://slack.com/api/users.info
        //    { "user": "U1234567890" }
        //
        //    list_users:
        //    GET https://slack.com/api/users.list
        //
        //    add_reaction:
        //    POST https://slack.com/api/reactions.add
        //    { "channel": "C1234567890", "timestamp": "1234567890.123456", "name": "thumbsup" }
        //
        //    remove_reaction:
        //    POST https://slack.com/api/reactions.remove
        //    { "channel": "C1234567890", "timestamp": "1234567890.123456", "name": "thumbsup" }
        //
        //    get_message_history:
        //    GET https://slack.com/api/conversations.history
        //    { "channel": "C1234567890", "limit": 100 }
        //
        //    search_messages:
        //    GET https://slack.com/api/search.messages
        //    { "query": "search term" }
        //
        // 5. Handle authentication headers: Authorization: Bearer <token>
        // 6. Parse the response and check response.ok field
        // 7. Return the results

        // For now, we'll return a placeholder response
        let result = serde_json::json!({
            "message": "Slack operation executed (placeholder implementation)",
            "credentials_name": &params.credentials_name,
            "operation": &params.operation,
            "channel": params.channel,
            "text": params.text,
            "context_execution_id": &context.execution_id,
            "success": true
        });

        Ok(NodeOutput::success(result))
    }

    fn validate_parameters(&self, parameters: &serde_json::Value) -> anyhow::Result<()> {
        let params: SlackParams = serde_json::from_value(parameters.clone())
            .map_err(|e| anyhow::anyhow!("Invalid parameters: {}", e))?;

        if params.credentials_name.trim().is_empty() {
            anyhow::bail!("credentials_name cannot be empty");
        }

        // Validate operation
        let valid_operations = [
            "send_message",
            "update_message",
            "delete_message",
            "post_file",
            "list_channels",
            "create_channel",
            "archive_channel",
            "invite_to_channel",
            "kick_from_channel",
            "get_channel_info",
            "set_channel_topic",
            "set_channel_purpose",
            "get_user_info",
            "list_users",
            "add_reaction",
            "remove_reaction",
            "get_message_history",
            "search_messages",
        ];

        if !valid_operations.contains(&params.operation.as_str()) {
            anyhow::bail!(
                "Invalid operation: {}. Must be one of: {}",
                params.operation,
                valid_operations.join(", ")
            );
        }

        // Validate that message operations have required parameters
        if ["send_message", "update_message"].contains(&params.operation.as_str()) {
            if params.channel.is_none() {
                anyhow::bail!("{} operation requires 'channel' parameter", params.operation);
            }
            if params.text.is_none() && params.blocks.is_none() {
                anyhow::bail!(
                    "{} operation requires either 'text' or 'blocks' parameter",
                    params.operation
                );
            }
        }

        // Validate that update_message and delete_message have ts
        if ["update_message", "delete_message"].contains(&params.operation.as_str())
            && params.ts.is_none()
        {
            anyhow::bail!("{} operation requires 'ts' parameter", params.operation);
        }

        // Validate that post_file has file_path or file_content
        if params.operation == "post_file" {
            if params.file_path.is_none() && params.file_content.is_none() {
                anyhow::bail!("post_file operation requires either 'file_path' or 'file_content' parameter");
            }
            if params.channel.is_none() {
                anyhow::bail!("post_file operation requires 'channel' parameter");
            }
        }

        // Validate that create_channel has name
        if params.operation == "create_channel" && params.name.is_none() {
            anyhow::bail!("create_channel operation requires 'name' parameter");
        }

        // Validate that channel operations have channel
        if [
            "archive_channel",
            "get_channel_info",
            "set_channel_topic",
            "set_channel_purpose",
            "invite_to_channel",
            "kick_from_channel",
            "get_message_history",
        ]
        .contains(&params.operation.as_str())
            && params.channel.is_none()
        {
            anyhow::bail!("{} operation requires 'channel' parameter", params.operation);
        }

        // Validate that invite_to_channel has users
        if params.operation == "invite_to_channel" && params.users.is_none() {
            anyhow::bail!("invite_to_channel operation requires 'users' parameter");
        }

        // Validate that kick_from_channel has user
        if params.operation == "kick_from_channel" && params.user.is_none() {
            anyhow::bail!("kick_from_channel operation requires 'user' parameter");
        }

        // Validate that set_channel_topic has topic
        if params.operation == "set_channel_topic" && params.topic.is_none() {
            anyhow::bail!("set_channel_topic operation requires 'topic' parameter");
        }

        // Validate that set_channel_purpose has purpose
        if params.operation == "set_channel_purpose" && params.purpose.is_none() {
            anyhow::bail!("set_channel_purpose operation requires 'purpose' parameter");
        }

        // Validate that get_user_info has user
        if params.operation == "get_user_info" && params.user.is_none() {
            anyhow::bail!("get_user_info operation requires 'user' parameter");
        }

        // Validate that reaction operations have required parameters
        if ["add_reaction", "remove_reaction"].contains(&params.operation.as_str()) {
            if params.channel.is_none() {
                anyhow::bail!("{} operation requires 'channel' parameter", params.operation);
            }
            if params.ts.is_none() {
                anyhow::bail!("{} operation requires 'ts' parameter", params.operation);
            }
            if params.emoji.is_none() {
                anyhow::bail!("{} operation requires 'emoji' parameter", params.operation);
            }
        }

        // Validate that search_messages has text
        if params.operation == "search_messages" && params.text.is_none() {
            anyhow::bail!("search_messages operation requires 'text' parameter");
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_slack_send_message() {
        let node = SlackNode::new();
        let context = NodeContext::new("exec-123".to_string(), "node-1".to_string());

        let params = serde_json::json!({
            "credentials_name": "my_slack",
            "operation": "send_message",
            "channel": "#general",
            "text": "Hello, Slack!"
        });

        let result = node.execute(&context, &params).await.unwrap();
        assert!(result.success);
        assert_eq!(result.data["operation"], "send_message");
        assert_eq!(result.data["channel"], "#general");
    }

    #[tokio::test]
    async fn test_slack_send_message_with_blocks() {
        let node = SlackNode::new();
        let context = NodeContext::new("exec-123".to_string(), "node-1".to_string());

        let params = serde_json::json!({
            "credentials_name": "my_slack",
            "operation": "send_message",
            "channel": "C1234567890",
            "blocks": [
                {
                    "type": "section",
                    "text": {
                        "type": "mrkdwn",
                        "text": "*Hello* from blocks!"
                    }
                }
            ]
        });

        let result = node.execute(&context, &params).await.unwrap();
        assert!(result.success);
        assert_eq!(result.data["operation"], "send_message");
    }

    #[tokio::test]
    async fn test_slack_post_file() {
        let node = SlackNode::new();
        let context = NodeContext::new("exec-123".to_string(), "node-1".to_string());

        let params = serde_json::json!({
            "credentials_name": "my_slack",
            "operation": "post_file",
            "channel": "#general",
            "file_content": "SGVsbG8gV29ybGQh",
            "filename": "hello.txt",
            "title": "Test File",
            "initial_comment": "Here's a test file"
        });

        let result = node.execute(&context, &params).await.unwrap();
        assert!(result.success);
        assert_eq!(result.data["operation"], "post_file");
    }

    #[tokio::test]
    async fn test_slack_create_channel() {
        let node = SlackNode::new();
        let context = NodeContext::new("exec-123".to_string(), "node-1".to_string());

        let params = serde_json::json!({
            "credentials_name": "my_slack",
            "operation": "create_channel",
            "name": "new-project",
            "description": "Project discussion channel",
            "is_private": false
        });

        let result = node.execute(&context, &params).await.unwrap();
        assert!(result.success);
        assert_eq!(result.data["operation"], "create_channel");
    }

    #[tokio::test]
    async fn test_slack_add_reaction() {
        let node = SlackNode::new();
        let context = NodeContext::new("exec-123".to_string(), "node-1".to_string());

        let params = serde_json::json!({
            "credentials_name": "my_slack",
            "operation": "add_reaction",
            "channel": "C1234567890",
            "ts": "1234567890.123456",
            "emoji": "thumbsup"
        });

        let result = node.execute(&context, &params).await.unwrap();
        assert!(result.success);
        assert_eq!(result.data["operation"], "add_reaction");
    }

    #[tokio::test]
    async fn test_slack_list_channels() {
        let node = SlackNode::new();
        let context = NodeContext::new("exec-123".to_string(), "node-1".to_string());

        let params = serde_json::json!({
            "credentials_name": "my_slack",
            "operation": "list_channels"
        });

        let result = node.execute(&context, &params).await.unwrap();
        assert!(result.success);
        assert_eq!(result.data["operation"], "list_channels");
    }

    #[test]
    fn test_slack_validation() {
        let node = SlackNode::new();

        // Valid send_message
        let valid_send = serde_json::json!({
            "credentials_name": "my_slack",
            "operation": "send_message",
            "channel": "#general",
            "text": "Hello"
        });
        assert!(node.validate_parameters(&valid_send).is_ok());

        // Invalid: send_message without channel
        let invalid_send = serde_json::json!({
            "credentials_name": "my_slack",
            "operation": "send_message",
            "text": "Hello"
        });
        assert!(node.validate_parameters(&invalid_send).is_err());

        // Invalid: send_message without text or blocks
        let invalid_send2 = serde_json::json!({
            "credentials_name": "my_slack",
            "operation": "send_message",
            "channel": "#general"
        });
        assert!(node.validate_parameters(&invalid_send2).is_err());

        // Invalid: update_message without ts
        let invalid_update = serde_json::json!({
            "credentials_name": "my_slack",
            "operation": "update_message",
            "channel": "#general",
            "text": "Updated"
        });
        assert!(node.validate_parameters(&invalid_update).is_err());

        // Invalid: post_file without file_path or file_content
        let invalid_file = serde_json::json!({
            "credentials_name": "my_slack",
            "operation": "post_file",
            "channel": "#general"
        });
        assert!(node.validate_parameters(&invalid_file).is_err());

        // Invalid: create_channel without name
        let invalid_create = serde_json::json!({
            "credentials_name": "my_slack",
            "operation": "create_channel"
        });
        assert!(node.validate_parameters(&invalid_create).is_err());

        // Invalid: add_reaction without emoji
        let invalid_reaction = serde_json::json!({
            "credentials_name": "my_slack",
            "operation": "add_reaction",
            "channel": "C1234567890",
            "ts": "1234567890.123456"
        });
        assert!(node.validate_parameters(&invalid_reaction).is_err());

        // Invalid: unknown operation
        let invalid_op = serde_json::json!({
            "credentials_name": "my_slack",
            "operation": "invalid_op"
        });
        assert!(node.validate_parameters(&invalid_op).is_err());
    }

    #[test]
    fn test_slack_node_type() {
        let node = SlackNode::new();
        assert_eq!(node.type_name(), "slack");
        assert_eq!(node.category(), NodeCategory::Action);
        assert_eq!(node.subcategory(), NodeSubcategory::Communication);
    }

    #[test]
    fn test_slack_parameter_schema() {
        let node = SlackNode::new();
        let schema = node.parameter_schema();

        assert_eq!(schema["type"], "object");
        assert!(schema["properties"]["credentials_name"].is_object());
        assert!(schema["properties"]["operation"].is_object());
        assert!(schema["properties"]["operation"]["enum"].is_array());
        assert_eq!(schema["required"].as_array().unwrap().len(), 2);
    }
}
