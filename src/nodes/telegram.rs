use crate::models::{Node, NodeCategory, NodeContext, NodeOutput, NodeSubcategory, NodeType};
use async_trait::async_trait;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TelegramParams {
    /// Credentials name to use for Telegram Bot API
    pub credentials_name: String,
    /// Telegram operation to perform
    pub operation: String,
    /// Chat ID (can be user ID, group ID, or channel username)
    pub chat_id: Option<String>,
    /// Message text
    pub text: Option<String>,
    /// Message ID
    pub message_id: Option<i64>,
    /// Parse mode: "Markdown", "MarkdownV2", "HTML"
    pub parse_mode: Option<String>,
    /// Disable web page preview
    pub disable_web_page_preview: Option<bool>,
    /// Disable notification
    pub disable_notification: Option<bool>,
    /// Reply to message ID
    pub reply_to_message_id: Option<i64>,
    /// Inline keyboard markup
    pub reply_markup: Option<serde_json::Value>,
    /// Photo file path or URL
    pub photo: Option<String>,
    /// Document file path
    pub document: Option<String>,
    /// Audio file path
    pub audio: Option<String>,
    /// Video file path
    pub video: Option<String>,
    /// Caption for media
    pub caption: Option<String>,
    /// Sticker file path
    pub sticker: Option<String>,
    /// User ID for actions
    pub user_id: Option<i64>,
    /// Callback query ID
    pub callback_query_id: Option<String>,
    /// Answer text for callback
    pub answer_text: Option<String>,
    /// Show alert for callback
    pub show_alert: Option<bool>,
    /// Poll question
    pub question: Option<String>,
    /// Poll options
    pub options: Option<Vec<String>>,
    /// Additional parameters
    #[serde(default)]
    pub additional_params: serde_json::Value,
}

/// Telegram node - performs Telegram Bot API operations
pub struct TelegramNode;

impl TelegramNode {
    pub fn new() -> Self {
        Self
    }
}

impl Default for TelegramNode {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeType for TelegramNode {
    fn type_name(&self) -> &str {
        "telegram"
    }

    fn category(&self) -> NodeCategory {
        NodeCategory::Action
    }

    fn subcategory(&self) -> NodeSubcategory {
        NodeSubcategory::Communication
    }

    fn required_credential_type(&self) -> Option<&str> {
        Some("telegram_bot_token")
    }

    fn parameter_schema(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "credentials_name": {
                    "type": "string",
                    "description": "Name of the Telegram bot credentials to use",
                    "minLength": 1
                },
                "operation": {
                    "type": "string",
                    "description": "Telegram operation to perform",
                    "enum": [
                        "send_message",
                        "send_photo",
                        "send_document",
                        "send_audio",
                        "send_video",
                        "send_sticker",
                        "send_poll",
                        "edit_message",
                        "delete_message",
                        "forward_message",
                        "get_updates",
                        "get_me",
                        "get_chat",
                        "get_chat_member",
                        "get_chat_administrators",
                        "get_chat_member_count",
                        "ban_chat_member",
                        "unban_chat_member",
                        "restrict_chat_member",
                        "promote_chat_member",
                        "set_chat_title",
                        "set_chat_description",
                        "pin_chat_message",
                        "unpin_chat_message",
                        "leave_chat",
                        "answer_callback_query"
                    ]
                },
                "chat_id": {
                    "type": "string",
                    "description": "Chat ID (user ID, group ID, or @channel_username)"
                },
                "text": {
                    "type": "string",
                    "description": "Message text"
                },
                "message_id": {
                    "type": "integer",
                    "description": "Message ID"
                },
                "parse_mode": {
                    "type": "string",
                    "description": "Parse mode for formatting",
                    "enum": ["Markdown", "MarkdownV2", "HTML"]
                },
                "disable_web_page_preview": {
                    "type": "boolean",
                    "description": "Disable web page preview",
                    "default": false
                },
                "disable_notification": {
                    "type": "boolean",
                    "description": "Send silently",
                    "default": false
                },
                "reply_to_message_id": {
                    "type": "integer",
                    "description": "Reply to message ID"
                },
                "reply_markup": {
                    "type": "object",
                    "description": "Inline keyboard markup"
                },
                "photo": {
                    "type": "string",
                    "description": "Photo file path or URL"
                },
                "document": {
                    "type": "string",
                    "description": "Document file path"
                },
                "audio": {
                    "type": "string",
                    "description": "Audio file path"
                },
                "video": {
                    "type": "string",
                    "description": "Video file path"
                },
                "caption": {
                    "type": "string",
                    "description": "Caption for media"
                },
                "sticker": {
                    "type": "string",
                    "description": "Sticker file path"
                },
                "user_id": {
                    "type": "integer",
                    "description": "User ID for member actions"
                },
                "callback_query_id": {
                    "type": "string",
                    "description": "Callback query ID"
                },
                "answer_text": {
                    "type": "string",
                    "description": "Answer text for callback query"
                },
                "show_alert": {
                    "type": "boolean",
                    "description": "Show alert instead of notification",
                    "default": false
                },
                "question": {
                    "type": "string",
                    "description": "Poll question"
                },
                "options": {
                    "type": "array",
                    "description": "Poll options",
                    "items": {
                        "type": "string"
                    }
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
impl Node for TelegramNode {
    async fn execute(
        &self,
        context: &NodeContext,
        parameters: &serde_json::Value,
    ) -> anyhow::Result<NodeOutput> {
        let params: TelegramParams = serde_json::from_value(parameters.clone())?;

        // TODO: In a real implementation:
        // 1. Look up credentials from credentials table
        // 2. Decrypt bot token
        // 3. Make API requests to Telegram Bot API:
        //
        //    API Base: https://api.telegram.org/bot{token}/
        //
        //    send_message: POST /sendMessage
        //    send_photo: POST /sendPhoto (multipart)
        //    send_document: POST /sendDocument (multipart)
        //    edit_message: POST /editMessageText
        //    delete_message: POST /deleteMessage
        //    get_updates: POST /getUpdates
        //    get_me: POST /getMe
        //    ban_chat_member: POST /banChatMember
        //    answer_callback_query: POST /answerCallbackQuery
        //
        // 4. Handle file uploads for media
        // 5. Handle inline keyboards
        // 6. Return results

        let result = serde_json::json!({
            "message": "Telegram operation executed (placeholder)",
            "credentials_name": &params.credentials_name,
            "operation": &params.operation,
            "chat_id": params.chat_id,
            "context_execution_id": &context.execution_id,
            "success": true
        });

        Ok(NodeOutput::success(result))
    }

    fn validate_parameters(&self, parameters: &serde_json::Value) -> anyhow::Result<()> {
        let params: TelegramParams = serde_json::from_value(parameters.clone())
            .map_err(|e| anyhow::anyhow!("Invalid parameters: {}", e))?;

        if params.credentials_name.trim().is_empty() {
            anyhow::bail!("credentials_name cannot be empty");
        }

        let valid_operations = [
            "send_message",
            "send_photo",
            "send_document",
            "send_audio",
            "send_video",
            "send_sticker",
            "send_poll",
            "edit_message",
            "delete_message",
            "forward_message",
            "get_updates",
            "get_me",
            "get_chat",
            "get_chat_member",
            "get_chat_administrators",
            "get_chat_member_count",
            "ban_chat_member",
            "unban_chat_member",
            "restrict_chat_member",
            "promote_chat_member",
            "set_chat_title",
            "set_chat_description",
            "pin_chat_message",
            "unpin_chat_message",
            "leave_chat",
            "answer_callback_query",
        ];

        if !valid_operations.contains(&params.operation.as_str()) {
            anyhow::bail!(
                "Invalid operation: {}. Must be one of: {}",
                params.operation,
                valid_operations.join(", ")
            );
        }

        // Validate chat-based operations require chat_id
        let chat_ops = [
            "send_message",
            "send_photo",
            "send_document",
            "send_audio",
            "send_video",
            "send_sticker",
            "send_poll",
            "get_chat",
            "get_chat_administrators",
            "get_chat_member_count",
            "set_chat_title",
            "set_chat_description",
            "leave_chat",
        ];

        if chat_ops.contains(&params.operation.as_str()) && params.chat_id.is_none() {
            anyhow::bail!(
                "{} operation requires 'chat_id' parameter",
                params.operation
            );
        }

        // Validate send_message requires text
        if params.operation == "send_message" && params.text.is_none() {
            anyhow::bail!("send_message operation requires 'text' parameter");
        }

        // Validate media operations
        if params.operation == "send_photo" && params.photo.is_none() {
            anyhow::bail!("send_photo operation requires 'photo' parameter");
        }

        if params.operation == "send_document" && params.document.is_none() {
            anyhow::bail!("send_document operation requires 'document' parameter");
        }

        // Validate poll requires question and options
        if params.operation == "send_poll" {
            if params.question.is_none() {
                anyhow::bail!("send_poll operation requires 'question' parameter");
            }
            if params.options.is_none() {
                anyhow::bail!("send_poll operation requires 'options' parameter");
            }
        }

        // Validate message operations require message_id
        if [
            "edit_message",
            "delete_message",
            "pin_chat_message",
            "unpin_chat_message",
        ]
        .contains(&params.operation.as_str())
            && params.message_id.is_none()
        {
            anyhow::bail!(
                "{} operation requires 'message_id' parameter",
                params.operation
            );
        }

        // Validate member operations require user_id
        if [
            "get_chat_member",
            "ban_chat_member",
            "unban_chat_member",
            "restrict_chat_member",
            "promote_chat_member",
        ]
        .contains(&params.operation.as_str())
            && params.user_id.is_none()
        {
            anyhow::bail!(
                "{} operation requires 'user_id' parameter",
                params.operation
            );
        }

        // Validate callback query operations
        if params.operation == "answer_callback_query" && params.callback_query_id.is_none() {
            anyhow::bail!("answer_callback_query operation requires 'callback_query_id' parameter");
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_telegram_send_message() {
        let node = TelegramNode::new();
        let context = NodeContext::new("exec-123".to_string(), "node-1".to_string());

        let params = serde_json::json!({
            "credentials_name": "my_telegram",
            "operation": "send_message",
            "chat_id": "123456789",
            "text": "Hello from workflow!",
            "parse_mode": "Markdown"
        });

        let result = node.execute(&context, &params).await.unwrap();
        assert!(result.success);
        assert_eq!(result.data["operation"], "send_message");
    }

    #[tokio::test]
    async fn test_telegram_send_poll() {
        let node = TelegramNode::new();
        let context = NodeContext::new("exec-123".to_string(), "node-1".to_string());

        let params = serde_json::json!({
            "credentials_name": "my_telegram",
            "operation": "send_poll",
            "chat_id": "@my_channel",
            "question": "What's your favorite language?",
            "options": ["Rust", "Python", "JavaScript", "Go"]
        });

        let result = node.execute(&context, &params).await.unwrap();
        assert!(result.success);
        assert_eq!(result.data["operation"], "send_poll");
    }

    #[test]
    fn test_telegram_validation() {
        let node = TelegramNode::new();

        // Valid send_message
        let valid = serde_json::json!({
            "credentials_name": "my_telegram",
            "operation": "send_message",
            "chat_id": "123",
            "text": "Hello"
        });
        assert!(node.validate_parameters(&valid).is_ok());

        // Invalid: send_message without text
        let invalid = serde_json::json!({
            "credentials_name": "my_telegram",
            "operation": "send_message",
            "chat_id": "123"
        });
        assert!(node.validate_parameters(&invalid).is_err());

        // Invalid: send_poll without options
        let invalid2 = serde_json::json!({
            "credentials_name": "my_telegram",
            "operation": "send_poll",
            "chat_id": "123",
            "question": "Test?"
        });
        assert!(node.validate_parameters(&invalid2).is_err());
    }

    #[test]
    fn test_telegram_node_type() {
        let node = TelegramNode::new();
        assert_eq!(node.type_name(), "telegram");
        assert_eq!(node.category(), NodeCategory::Action);
        assert_eq!(node.subcategory(), NodeSubcategory::Communication);
        assert_eq!(node.required_credential_type(), Some("telegram_bot_token"));
    }
}
