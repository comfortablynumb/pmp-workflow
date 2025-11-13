use crate::models::{Node, NodeCategory, NodeContext, NodeOutput, NodeSubcategory, NodeType};
use async_trait::async_trait;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GmailParams {
    /// Credentials name to use for Gmail API
    pub credentials_name: String,
    /// Gmail operation to perform
    pub operation: String,
    /// Message ID
    pub message_id: Option<String>,
    /// Thread ID
    pub thread_id: Option<String>,
    /// Email recipient(s) - comma-separated for multiple
    pub to: Option<String>,
    /// Email CC recipient(s)
    pub cc: Option<String>,
    /// Email BCC recipient(s)
    pub bcc: Option<String>,
    /// Email subject
    pub subject: Option<String>,
    /// Email body (plain text or HTML)
    pub body: Option<String>,
    /// Body type: "text" or "html"
    pub body_type: Option<String>,
    /// Attachments (array of {filename, content_base64, mime_type})
    pub attachments: Option<Vec<serde_json::Value>>,
    /// Label IDs to apply/remove
    pub label_ids: Option<Vec<String>>,
    /// Labels to add
    pub add_labels: Option<Vec<String>>,
    /// Labels to remove
    pub remove_labels: Option<Vec<String>>,
    /// Search query (Gmail search syntax)
    pub query: Option<String>,
    /// Maximum results to return
    pub max_results: Option<u32>,
    /// Page token for pagination
    pub page_token: Option<String>,
    /// Include spam and trash in results
    pub include_spam_trash: Option<bool>,
    /// Draft ID
    pub draft_id: Option<String>,
    /// Label name for creation
    pub label_name: Option<String>,
    /// Label visibility: "labelShow", "labelShowIfUnread", "labelHide"
    pub label_list_visibility: Option<String>,
    /// Message visibility: "show", "hide"
    pub message_list_visibility: Option<String>,
    /// Format for message retrieval: "full", "metadata", "minimal", "raw"
    pub format: Option<String>,
    /// Metadata headers to include
    pub metadata_headers: Option<Vec<String>>,
    /// Additional parameters
    #[serde(default)]
    pub additional_params: serde_json::Value,
}

/// Gmail node - performs Gmail API operations
pub struct GmailNode;

impl GmailNode {
    pub fn new() -> Self {
        Self
    }
}

impl Default for GmailNode {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeType for GmailNode {
    fn type_name(&self) -> &str {
        "gmail"
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
                    "description": "Name of the Gmail API credentials to use (OAuth2)",
                    "minLength": 1
                },
                "operation": {
                    "type": "string",
                    "description": "Gmail operation to perform",
                    "enum": [
                        "send_email",
                        "get_message",
                        "list_messages",
                        "delete_message",
                        "trash_message",
                        "untrash_message",
                        "modify_labels",
                        "create_draft",
                        "update_draft",
                        "send_draft",
                        "delete_draft",
                        "get_draft",
                        "list_drafts",
                        "search_messages",
                        "get_thread",
                        "list_threads",
                        "modify_thread",
                        "delete_thread",
                        "trash_thread",
                        "untrash_thread",
                        "create_label",
                        "update_label",
                        "delete_label",
                        "list_labels",
                        "get_attachment"
                    ]
                },
                "message_id": {
                    "type": "string",
                    "description": "Gmail message ID"
                },
                "thread_id": {
                    "type": "string",
                    "description": "Gmail thread ID"
                },
                "to": {
                    "type": "string",
                    "description": "Email recipient(s), comma-separated"
                },
                "cc": {
                    "type": "string",
                    "description": "Email CC recipient(s), comma-separated"
                },
                "bcc": {
                    "type": "string",
                    "description": "Email BCC recipient(s), comma-separated"
                },
                "subject": {
                    "type": "string",
                    "description": "Email subject line"
                },
                "body": {
                    "type": "string",
                    "description": "Email body content"
                },
                "body_type": {
                    "type": "string",
                    "description": "Body content type",
                    "enum": ["text", "html"],
                    "default": "text"
                },
                "attachments": {
                    "type": "array",
                    "description": "Email attachments",
                    "items": {
                        "type": "object",
                        "properties": {
                            "filename": {
                                "type": "string"
                            },
                            "content_base64": {
                                "type": "string",
                                "description": "Base64 encoded file content"
                            },
                            "mime_type": {
                                "type": "string"
                            }
                        },
                        "required": ["filename", "content_base64"]
                    }
                },
                "label_ids": {
                    "type": "array",
                    "description": "Label IDs to apply",
                    "items": {
                        "type": "string"
                    }
                },
                "add_labels": {
                    "type": "array",
                    "description": "Label IDs to add",
                    "items": {
                        "type": "string"
                    }
                },
                "remove_labels": {
                    "type": "array",
                    "description": "Label IDs to remove",
                    "items": {
                        "type": "string"
                    }
                },
                "query": {
                    "type": "string",
                    "description": "Search query using Gmail search syntax"
                },
                "max_results": {
                    "type": "integer",
                    "description": "Maximum number of results to return",
                    "minimum": 1,
                    "maximum": 500,
                    "default": 100
                },
                "page_token": {
                    "type": "string",
                    "description": "Page token for pagination"
                },
                "include_spam_trash": {
                    "type": "boolean",
                    "description": "Include spam and trash in results",
                    "default": false
                },
                "draft_id": {
                    "type": "string",
                    "description": "Draft ID"
                },
                "label_name": {
                    "type": "string",
                    "description": "Label name for creation/update"
                },
                "label_list_visibility": {
                    "type": "string",
                    "description": "Label visibility in label list",
                    "enum": ["labelShow", "labelShowIfUnread", "labelHide"]
                },
                "message_list_visibility": {
                    "type": "string",
                    "description": "Message visibility in message list",
                    "enum": ["show", "hide"]
                },
                "format": {
                    "type": "string",
                    "description": "Format for message retrieval",
                    "enum": ["full", "metadata", "minimal", "raw"],
                    "default": "full"
                },
                "metadata_headers": {
                    "type": "array",
                    "description": "Metadata headers to include",
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
impl Node for GmailNode {
    async fn execute(
        &self,
        context: &NodeContext,
        parameters: &serde_json::Value,
    ) -> anyhow::Result<NodeOutput> {
        let params: GmailParams = serde_json::from_value(parameters.clone())?;

        // TODO: In a real implementation:
        // 1. Look up credentials from the credentials table using params.credentials_name
        // 2. Decrypt the credentials data to get OAuth2 tokens (access_token, refresh_token)
        // 3. Create a Gmail API client using reqwest with OAuth2 authentication
        // 4. Execute the operation based on params.operation:
        //
        //    send_email:
        //    POST https://gmail.googleapis.com/gmail/v1/users/me/messages/send
        //    Create RFC 2822 formatted email with MIME encoding
        //    { "raw": "<base64url encoded email>" }
        //
        //    get_message:
        //    GET https://gmail.googleapis.com/gmail/v1/users/me/messages/{messageId}
        //    ?format=full&metadataHeaders=From,To,Subject
        //
        //    list_messages:
        //    GET https://gmail.googleapis.com/gmail/v1/users/me/messages
        //    ?q=query&maxResults=100&labelIds=INBOX
        //
        //    delete_message:
        //    DELETE https://gmail.googleapis.com/gmail/v1/users/me/messages/{messageId}
        //
        //    trash_message:
        //    POST https://gmail.googleapis.com/gmail/v1/users/me/messages/{messageId}/trash
        //
        //    untrash_message:
        //    POST https://gmail.googleapis.com/gmail/v1/users/me/messages/{messageId}/untrash
        //
        //    modify_labels:
        //    POST https://gmail.googleapis.com/gmail/v1/users/me/messages/{messageId}/modify
        //    { "addLabelIds": ["Label_1"], "removeLabelIds": ["Label_2"] }
        //
        //    create_draft:
        //    POST https://gmail.googleapis.com/gmail/v1/users/me/drafts
        //    { "message": { "raw": "<base64url encoded email>" } }
        //
        //    send_draft:
        //    POST https://gmail.googleapis.com/gmail/v1/users/me/drafts/send
        //    { "id": "draftId" }
        //
        //    list_labels:
        //    GET https://gmail.googleapis.com/gmail/v1/users/me/labels
        //
        //    create_label:
        //    POST https://gmail.googleapis.com/gmail/v1/users/me/labels
        //    { "name": "MyLabel", "labelListVisibility": "labelShow" }
        //
        //    get_thread:
        //    GET https://gmail.googleapis.com/gmail/v1/users/me/threads/{threadId}
        //
        //    list_threads:
        //    GET https://gmail.googleapis.com/gmail/v1/users/me/threads
        //    ?q=query&maxResults=100
        //
        //    get_attachment:
        //    GET https://gmail.googleapis.com/gmail/v1/users/me/messages/{messageId}/attachments/{attachmentId}
        //
        // 5. Handle OAuth2 token refresh if access token expires
        // 6. Parse the response and handle errors
        // 7. Return the results

        // For now, we'll return a placeholder response
        let result = serde_json::json!({
            "message": "Gmail operation executed (placeholder implementation)",
            "credentials_name": &params.credentials_name,
            "operation": &params.operation,
            "to": params.to,
            "subject": params.subject,
            "context_execution_id": &context.execution_id,
            "success": true
        });

        Ok(NodeOutput::success(result))
    }

    fn validate_parameters(&self, parameters: &serde_json::Value) -> anyhow::Result<()> {
        let params: GmailParams = serde_json::from_value(parameters.clone())
            .map_err(|e| anyhow::anyhow!("Invalid parameters: {}", e))?;

        if params.credentials_name.trim().is_empty() {
            anyhow::bail!("credentials_name cannot be empty");
        }

        // Validate operation
        let valid_operations = [
            "send_email",
            "get_message",
            "list_messages",
            "delete_message",
            "trash_message",
            "untrash_message",
            "modify_labels",
            "create_draft",
            "update_draft",
            "send_draft",
            "delete_draft",
            "get_draft",
            "list_drafts",
            "search_messages",
            "get_thread",
            "list_threads",
            "modify_thread",
            "delete_thread",
            "trash_thread",
            "untrash_thread",
            "create_label",
            "update_label",
            "delete_label",
            "list_labels",
            "get_attachment",
        ];

        if !valid_operations.contains(&params.operation.as_str()) {
            anyhow::bail!(
                "Invalid operation: {}. Must be one of: {}",
                params.operation,
                valid_operations.join(", ")
            );
        }

        // Validate that send_email and create_draft have required parameters
        if ["send_email", "create_draft"].contains(&params.operation.as_str()) {
            if params.to.is_none() {
                anyhow::bail!("{} operation requires 'to' parameter", params.operation);
            }
            if params.subject.is_none() {
                anyhow::bail!("{} operation requires 'subject' parameter", params.operation);
            }
            if params.body.is_none() {
                anyhow::bail!("{} operation requires 'body' parameter", params.operation);
            }
        }

        // Validate that message operations have message_id
        if [
            "get_message",
            "delete_message",
            "trash_message",
            "untrash_message",
            "modify_labels",
        ]
        .contains(&params.operation.as_str())
            && params.message_id.is_none()
        {
            anyhow::bail!("{} operation requires 'message_id' parameter", params.operation);
        }

        // Validate that thread operations have thread_id
        if [
            "get_thread",
            "modify_thread",
            "delete_thread",
            "trash_thread",
            "untrash_thread",
        ]
        .contains(&params.operation.as_str())
            && params.thread_id.is_none()
        {
            anyhow::bail!("{} operation requires 'thread_id' parameter", params.operation);
        }

        // Validate that draft operations have draft_id
        if ["update_draft", "send_draft", "delete_draft", "get_draft"]
            .contains(&params.operation.as_str())
            && params.draft_id.is_none()
        {
            anyhow::bail!("{} operation requires 'draft_id' parameter", params.operation);
        }

        // Validate that modify_labels has add_labels or remove_labels
        if params.operation == "modify_labels"
            && params.add_labels.is_none()
            && params.remove_labels.is_none()
        {
            anyhow::bail!(
                "modify_labels operation requires either 'add_labels' or 'remove_labels' parameter"
            );
        }

        // Validate that search_messages has query
        if params.operation == "search_messages" && params.query.is_none() {
            anyhow::bail!("search_messages operation requires 'query' parameter");
        }

        // Validate that create_label has label_name
        if params.operation == "create_label" && params.label_name.is_none() {
            anyhow::bail!("create_label operation requires 'label_name' parameter");
        }

        // Validate max_results range
        if let Some(max_results) = params.max_results
            && !(1..=500).contains(&max_results)
        {
            anyhow::bail!("max_results must be between 1 and 500");
        }

        // Validate body_type
        if let Some(ref body_type) = params.body_type
            && !["text", "html"].contains(&body_type.as_str())
        {
            anyhow::bail!("body_type must be 'text' or 'html'");
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_gmail_send_email() {
        let node = GmailNode::new();
        let context = NodeContext::new("exec-123".to_string(), "node-1".to_string());

        let params = serde_json::json!({
            "credentials_name": "my_gmail",
            "operation": "send_email",
            "to": "user@example.com",
            "subject": "Test Email",
            "body": "This is a test email",
            "body_type": "text"
        });

        let result = node.execute(&context, &params).await.unwrap();
        assert!(result.success);
        assert_eq!(result.data["operation"], "send_email");
        assert_eq!(result.data["to"], "user@example.com");
    }

    #[tokio::test]
    async fn test_gmail_send_email_with_attachments() {
        let node = GmailNode::new();
        let context = NodeContext::new("exec-123".to_string(), "node-1".to_string());

        let params = serde_json::json!({
            "credentials_name": "my_gmail",
            "operation": "send_email",
            "to": "user@example.com",
            "cc": "cc@example.com",
            "subject": "Test with Attachment",
            "body": "<h1>Hello</h1>",
            "body_type": "html",
            "attachments": [
                {
                    "filename": "document.pdf",
                    "content_base64": "JVBERi0xLjQ...",
                    "mime_type": "application/pdf"
                }
            ]
        });

        let result = node.execute(&context, &params).await.unwrap();
        assert!(result.success);
        assert_eq!(result.data["operation"], "send_email");
    }

    #[tokio::test]
    async fn test_gmail_list_messages() {
        let node = GmailNode::new();
        let context = NodeContext::new("exec-123".to_string(), "node-1".to_string());

        let params = serde_json::json!({
            "credentials_name": "my_gmail",
            "operation": "list_messages",
            "max_results": 50
        });

        let result = node.execute(&context, &params).await.unwrap();
        assert!(result.success);
        assert_eq!(result.data["operation"], "list_messages");
    }

    #[tokio::test]
    async fn test_gmail_search_messages() {
        let node = GmailNode::new();
        let context = NodeContext::new("exec-123".to_string(), "node-1".to_string());

        let params = serde_json::json!({
            "credentials_name": "my_gmail",
            "operation": "search_messages",
            "query": "from:important@example.com subject:urgent",
            "max_results": 25
        });

        let result = node.execute(&context, &params).await.unwrap();
        assert!(result.success);
        assert_eq!(result.data["operation"], "search_messages");
    }

    #[tokio::test]
    async fn test_gmail_modify_labels() {
        let node = GmailNode::new();
        let context = NodeContext::new("exec-123".to_string(), "node-1".to_string());

        let params = serde_json::json!({
            "credentials_name": "my_gmail",
            "operation": "modify_labels",
            "message_id": "12345678",
            "add_labels": ["IMPORTANT", "STARRED"],
            "remove_labels": ["UNREAD"]
        });

        let result = node.execute(&context, &params).await.unwrap();
        assert!(result.success);
        assert_eq!(result.data["operation"], "modify_labels");
    }

    #[tokio::test]
    async fn test_gmail_create_draft() {
        let node = GmailNode::new();
        let context = NodeContext::new("exec-123".to_string(), "node-1".to_string());

        let params = serde_json::json!({
            "credentials_name": "my_gmail",
            "operation": "create_draft",
            "to": "user@example.com",
            "subject": "Draft Email",
            "body": "This is a draft"
        });

        let result = node.execute(&context, &params).await.unwrap();
        assert!(result.success);
        assert_eq!(result.data["operation"], "create_draft");
    }

    #[tokio::test]
    async fn test_gmail_create_label() {
        let node = GmailNode::new();
        let context = NodeContext::new("exec-123".to_string(), "node-1".to_string());

        let params = serde_json::json!({
            "credentials_name": "my_gmail",
            "operation": "create_label",
            "label_name": "Important Projects",
            "label_list_visibility": "labelShow",
            "message_list_visibility": "show"
        });

        let result = node.execute(&context, &params).await.unwrap();
        assert!(result.success);
        assert_eq!(result.data["operation"], "create_label");
    }

    #[test]
    fn test_gmail_validation() {
        let node = GmailNode::new();

        // Valid send_email
        let valid_send = serde_json::json!({
            "credentials_name": "my_gmail",
            "operation": "send_email",
            "to": "user@example.com",
            "subject": "Test",
            "body": "Hello"
        });
        assert!(node.validate_parameters(&valid_send).is_ok());

        // Invalid: send_email without to
        let invalid_send = serde_json::json!({
            "credentials_name": "my_gmail",
            "operation": "send_email",
            "subject": "Test",
            "body": "Hello"
        });
        assert!(node.validate_parameters(&invalid_send).is_err());

        // Invalid: send_email without subject
        let invalid_send2 = serde_json::json!({
            "credentials_name": "my_gmail",
            "operation": "send_email",
            "to": "user@example.com",
            "body": "Hello"
        });
        assert!(node.validate_parameters(&invalid_send2).is_err());

        // Invalid: get_message without message_id
        let invalid_get = serde_json::json!({
            "credentials_name": "my_gmail",
            "operation": "get_message"
        });
        assert!(node.validate_parameters(&invalid_get).is_err());

        // Invalid: modify_labels without add_labels or remove_labels
        let invalid_modify = serde_json::json!({
            "credentials_name": "my_gmail",
            "operation": "modify_labels",
            "message_id": "12345"
        });
        assert!(node.validate_parameters(&invalid_modify).is_err());

        // Invalid: search_messages without query
        let invalid_search = serde_json::json!({
            "credentials_name": "my_gmail",
            "operation": "search_messages"
        });
        assert!(node.validate_parameters(&invalid_search).is_err());

        // Invalid: create_label without label_name
        let invalid_label = serde_json::json!({
            "credentials_name": "my_gmail",
            "operation": "create_label"
        });
        assert!(node.validate_parameters(&invalid_label).is_err());

        // Invalid: max_results out of range
        let invalid_max = serde_json::json!({
            "credentials_name": "my_gmail",
            "operation": "list_messages",
            "max_results": 1000
        });
        assert!(node.validate_parameters(&invalid_max).is_err());

        // Invalid: unknown operation
        let invalid_op = serde_json::json!({
            "credentials_name": "my_gmail",
            "operation": "invalid_op"
        });
        assert!(node.validate_parameters(&invalid_op).is_err());
    }

    #[test]
    fn test_gmail_node_type() {
        let node = GmailNode::new();
        assert_eq!(node.type_name(), "gmail");
        assert_eq!(node.category(), NodeCategory::Action);
        assert_eq!(node.subcategory(), NodeSubcategory::Communication);
    }

    #[test]
    fn test_gmail_parameter_schema() {
        let node = GmailNode::new();
        let schema = node.parameter_schema();

        assert_eq!(schema["type"], "object");
        assert!(schema["properties"]["credentials_name"].is_object());
        assert!(schema["properties"]["operation"].is_object());
        assert!(schema["properties"]["operation"]["enum"].is_array());
        assert_eq!(schema["required"].as_array().unwrap().len(), 2);
    }
}
