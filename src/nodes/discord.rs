use crate::models::{Node, NodeCategory, NodeContext, NodeOutput, NodeSubcategory, NodeType};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::json;

/// Discord node for messaging and server management operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscordNode {
    #[serde(skip)]
    _private: (),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscordNodeParams {
    /// The operation to perform
    pub operation: String,

    // Message operations
    /// Channel ID
    pub channel_id: Option<String>,
    /// Message content
    pub content: Option<String>,
    /// Message ID
    pub message_id: Option<String>,
    /// Username for webhook
    pub username: Option<String>,
    /// Avatar URL for webhook
    pub avatar_url: Option<String>,

    // Embed parameters
    /// Embeds array
    pub embeds: Option<Vec<serde_json::Value>>,
    /// Embed title
    pub embed_title: Option<String>,
    /// Embed description
    pub embed_description: Option<String>,
    /// Embed color (decimal)
    pub embed_color: Option<i32>,
    /// Embed URL
    pub embed_url: Option<String>,
    /// Embed thumbnail URL
    pub embed_thumbnail: Option<String>,
    /// Embed image URL
    pub embed_image: Option<String>,
    /// Embed footer text
    pub embed_footer: Option<String>,
    /// Embed fields
    pub embed_fields: Option<Vec<serde_json::Value>>,

    // File attachments
    /// File URL or path
    pub file_url: Option<String>,
    /// File content (base64 encoded)
    pub file_content: Option<String>,
    /// Filename
    pub filename: Option<String>,

    // Server/Guild operations
    /// Guild/Server ID
    pub guild_id: Option<String>,
    /// User ID
    pub user_id: Option<String>,
    /// Role ID
    pub role_id: Option<String>,
    /// Role name
    pub role_name: Option<String>,
    /// Channel name
    pub channel_name: Option<String>,
    /// Channel type (text, voice, category, etc.)
    pub channel_type: Option<i32>,

    // Reaction operations
    /// Emoji for reaction (Unicode or custom emoji ID)
    pub emoji: Option<String>,

    // Thread operations
    /// Thread name
    pub thread_name: Option<String>,
    /// Auto archive duration in minutes
    pub auto_archive_duration: Option<i32>,

    // Webhook URL (alternative to bot token)
    pub webhook_url: Option<String>,

    // Advanced options
    /// Text-to-speech
    pub tts: Option<bool>,
    /// Message flags
    pub flags: Option<i32>,
}

impl DiscordNode {
    pub fn new() -> Self {
        Self { _private: () }
    }
}

impl Default for DiscordNode {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl NodeType for DiscordNode {
    fn type_name(&self) -> &str {
        "discord"
    }

    fn category(&self) -> NodeCategory {
        NodeCategory::Action
    }

    fn subcategory(&self) -> NodeSubcategory {
        NodeSubcategory::Communication
    }

    fn required_credential_type(&self) -> Option<&str> {
        Some("discord_bot_token")
    }

    fn parameter_schema(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "required": ["operation"],
            "properties": {
                "operation": {
                    "type": "string",
                    "enum": [
                        "send_message",
                        "send_webhook_message",
                        "edit_message",
                        "delete_message",
                        "get_message",
                        "get_channel_messages",
                        "create_channel",
                        "delete_channel",
                        "get_channel",
                        "update_channel",
                        "add_reaction",
                        "remove_reaction",
                        "create_thread",
                        "join_thread",
                        "leave_thread",
                        "add_thread_member",
                        "remove_thread_member",
                        "get_guild",
                        "list_guild_members",
                        "get_guild_member",
                        "add_guild_member_role",
                        "remove_guild_member_role",
                        "kick_guild_member",
                        "ban_guild_member",
                        "unban_guild_member",
                        "create_role",
                        "delete_role",
                        "update_role",
                        "list_roles"
                    ],
                    "description": "The operation to perform"
                },
                "channel_id": {
                    "type": "string",
                    "description": "Discord channel ID"
                },
                "content": {
                    "type": "string",
                    "description": "Message content"
                },
                "message_id": {
                    "type": "string",
                    "description": "Message ID"
                },
                "username": {
                    "type": "string",
                    "description": "Username for webhook message"
                },
                "avatar_url": {
                    "type": "string",
                    "description": "Avatar URL for webhook message"
                },
                "embeds": {
                    "type": "array",
                    "description": "Array of embed objects"
                },
                "embed_title": {
                    "type": "string",
                    "description": "Embed title"
                },
                "embed_description": {
                    "type": "string",
                    "description": "Embed description"
                },
                "embed_color": {
                    "type": "integer",
                    "description": "Embed color (decimal)"
                },
                "embed_url": {
                    "type": "string",
                    "description": "Embed URL"
                },
                "embed_thumbnail": {
                    "type": "string",
                    "description": "Embed thumbnail URL"
                },
                "embed_image": {
                    "type": "string",
                    "description": "Embed image URL"
                },
                "embed_footer": {
                    "type": "string",
                    "description": "Embed footer text"
                },
                "embed_fields": {
                    "type": "array",
                    "description": "Embed fields array"
                },
                "file_url": {
                    "type": "string",
                    "description": "File URL or path"
                },
                "file_content": {
                    "type": "string",
                    "description": "File content (base64 encoded)"
                },
                "filename": {
                    "type": "string",
                    "description": "Filename for attachment"
                },
                "guild_id": {
                    "type": "string",
                    "description": "Guild/Server ID"
                },
                "user_id": {
                    "type": "string",
                    "description": "User ID"
                },
                "role_id": {
                    "type": "string",
                    "description": "Role ID"
                },
                "role_name": {
                    "type": "string",
                    "description": "Role name"
                },
                "channel_name": {
                    "type": "string",
                    "description": "Channel name"
                },
                "channel_type": {
                    "type": "integer",
                    "description": "Channel type (0=text, 2=voice, 4=category, etc.)"
                },
                "emoji": {
                    "type": "string",
                    "description": "Emoji for reaction"
                },
                "thread_name": {
                    "type": "string",
                    "description": "Thread name"
                },
                "auto_archive_duration": {
                    "type": "integer",
                    "description": "Auto archive duration in minutes"
                },
                "webhook_url": {
                    "type": "string",
                    "description": "Webhook URL (alternative to bot token)"
                },
                "tts": {
                    "type": "boolean",
                    "description": "Text-to-speech"
                },
                "flags": {
                    "type": "integer",
                    "description": "Message flags"
                }
            }
        })
    }
}

#[async_trait]
impl Node for DiscordNode {
    async fn execute(
        &self,
        context: &NodeContext,
        parameters: &serde_json::Value,
    ) -> anyhow::Result<NodeOutput> {
        let _params: DiscordNodeParams = serde_json::from_value(parameters.clone())?;

        // Mock implementation - in a real implementation, this would make HTTP requests to Discord API
        let result = json!({
            "success": true,
            "message": "Discord operation would be executed here",
            "execution_id": &context.execution_id
        });

        Ok(NodeOutput::success(result))
    }

    fn validate_parameters(&self, parameters: &serde_json::Value) -> anyhow::Result<()> {
        let params: DiscordNodeParams = serde_json::from_value(parameters.clone())?;

        // Validate based on operation
        match params.operation.as_str() {
            "send_message" => {
                if params.channel_id.is_none() {
                    anyhow::bail!("send_message operation requires 'channel_id' parameter");
                }
                if params.content.is_none() && params.embeds.is_none() {
                    anyhow::bail!(
                        "send_message operation requires 'content' or 'embeds' parameter"
                    );
                }
            }
            "send_webhook_message" => {
                if params.webhook_url.is_none() {
                    anyhow::bail!(
                        "send_webhook_message operation requires 'webhook_url' parameter"
                    );
                }
                if params.content.is_none() && params.embeds.is_none() {
                    anyhow::bail!(
                        "send_webhook_message operation requires 'content' or 'embeds' parameter"
                    );
                }
            }
            "edit_message" | "delete_message" | "get_message" => {
                if params.channel_id.is_none() {
                    anyhow::bail!(
                        "{} operation requires 'channel_id' parameter",
                        params.operation
                    );
                }
                if params.message_id.is_none() {
                    anyhow::bail!(
                        "{} operation requires 'message_id' parameter",
                        params.operation
                    );
                }
            }
            "get_channel_messages" | "get_channel" | "update_channel" | "delete_channel" => {
                if params.channel_id.is_none() {
                    anyhow::bail!(
                        "{} operation requires 'channel_id' parameter",
                        params.operation
                    );
                }
            }
            "create_channel" => {
                if params.guild_id.is_none() {
                    anyhow::bail!("create_channel operation requires 'guild_id' parameter");
                }
                if params.channel_name.is_none() {
                    anyhow::bail!("create_channel operation requires 'channel_name' parameter");
                }
            }
            "add_reaction" | "remove_reaction" => {
                if params.channel_id.is_none() {
                    anyhow::bail!(
                        "{} operation requires 'channel_id' parameter",
                        params.operation
                    );
                }
                if params.message_id.is_none() {
                    anyhow::bail!(
                        "{} operation requires 'message_id' parameter",
                        params.operation
                    );
                }
                if params.emoji.is_none() {
                    anyhow::bail!("{} operation requires 'emoji' parameter", params.operation);
                }
            }
            "create_thread" => {
                if params.channel_id.is_none() {
                    anyhow::bail!("create_thread operation requires 'channel_id' parameter");
                }
                if params.thread_name.is_none() {
                    anyhow::bail!("create_thread operation requires 'thread_name' parameter");
                }
            }
            "join_thread" | "leave_thread" => {
                if params.channel_id.is_none() {
                    anyhow::bail!(
                        "{} operation requires 'channel_id' parameter (thread ID)",
                        params.operation
                    );
                }
            }
            "add_thread_member" | "remove_thread_member" => {
                if params.channel_id.is_none() {
                    anyhow::bail!(
                        "{} operation requires 'channel_id' parameter (thread ID)",
                        params.operation
                    );
                }
                if params.user_id.is_none() {
                    anyhow::bail!(
                        "{} operation requires 'user_id' parameter",
                        params.operation
                    );
                }
            }
            "get_guild" | "list_guild_members" | "list_roles" => {
                if params.guild_id.is_none() {
                    anyhow::bail!(
                        "{} operation requires 'guild_id' parameter",
                        params.operation
                    );
                }
            }
            "get_guild_member" | "kick_guild_member" | "ban_guild_member"
            | "unban_guild_member" => {
                if params.guild_id.is_none() {
                    anyhow::bail!(
                        "{} operation requires 'guild_id' parameter",
                        params.operation
                    );
                }
                if params.user_id.is_none() {
                    anyhow::bail!(
                        "{} operation requires 'user_id' parameter",
                        params.operation
                    );
                }
            }
            "add_guild_member_role" | "remove_guild_member_role" => {
                if params.guild_id.is_none() {
                    anyhow::bail!(
                        "{} operation requires 'guild_id' parameter",
                        params.operation
                    );
                }
                if params.user_id.is_none() {
                    anyhow::bail!(
                        "{} operation requires 'user_id' parameter",
                        params.operation
                    );
                }
                if params.role_id.is_none() {
                    anyhow::bail!(
                        "{} operation requires 'role_id' parameter",
                        params.operation
                    );
                }
            }
            "create_role" => {
                if params.guild_id.is_none() {
                    anyhow::bail!("create_role operation requires 'guild_id' parameter");
                }
                if params.role_name.is_none() {
                    anyhow::bail!("create_role operation requires 'role_name' parameter");
                }
            }
            "delete_role" | "update_role" => {
                if params.guild_id.is_none() {
                    anyhow::bail!(
                        "{} operation requires 'guild_id' parameter",
                        params.operation
                    );
                }
                if params.role_id.is_none() {
                    anyhow::bail!(
                        "{} operation requires 'role_id' parameter",
                        params.operation
                    );
                }
            }
            _ => {
                anyhow::bail!("Unknown operation: {}", params.operation);
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_discord_node_creation() {
        let node = DiscordNode::new();
        assert_eq!(node.type_name(), "discord");
        assert!(matches!(node.category(), NodeCategory::Action));
        assert!(matches!(node.subcategory(), NodeSubcategory::Communication));
    }

    #[test]
    fn test_discord_required_credential_type() {
        let node = DiscordNode::new();
        assert_eq!(node.required_credential_type(), Some("discord_bot_token"));
    }

    #[test]
    fn test_send_message_validation() {
        let node = DiscordNode::new();

        // Valid send_message
        let valid_params = json!({
            "operation": "send_message",
            "channel_id": "123456789",
            "content": "Hello, Discord!"
        });
        assert!(node.validate_parameters(&valid_params).is_ok());

        // Missing channel_id
        let invalid_params = json!({
            "operation": "send_message",
            "content": "Hello"
        });
        assert!(node.validate_parameters(&invalid_params).is_err());
    }

    #[test]
    fn test_send_webhook_validation() {
        let node = DiscordNode::new();

        // Valid webhook
        let valid_params = json!({
            "operation": "send_webhook_message",
            "webhook_url": "https://discord.com/api/webhooks/...",
            "content": "Webhook message"
        });
        assert!(node.validate_parameters(&valid_params).is_ok());
    }

    #[test]
    fn test_create_thread_validation() {
        let node = DiscordNode::new();

        // Valid create_thread
        let valid_params = json!({
            "operation": "create_thread",
            "channel_id": "123456789",
            "thread_name": "Discussion Thread"
        });
        assert!(node.validate_parameters(&valid_params).is_ok());
    }

    #[tokio::test]
    async fn test_discord_execute() {
        let node = DiscordNode::new();
        let params = json!({
            "operation": "send_message",
            "channel_id": "123456789",
            "content": "Test message"
        });
        let context = NodeContext::new("test-exec".to_string(), "test-node".to_string());

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
    }
}
