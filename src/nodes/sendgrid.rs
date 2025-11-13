use crate::models::{Node, NodeCategory, NodeContext, NodeOutput, NodeSubcategory, NodeType};
use async_trait::async_trait;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SendGridParams {
    /// Credentials name to use for SendGrid API
    pub credentials_name: String,
    /// SendGrid operation to perform
    pub operation: String,
    /// To email address(es)
    pub to: Option<Vec<String>>,
    /// From email address
    pub from: Option<String>,
    /// Email subject
    pub subject: Option<String>,
    /// Email content (plain text or HTML)
    pub content: Option<String>,
    /// Template ID for template email
    pub template_id: Option<String>,
    /// List ID for list operations
    pub list_id: Option<String>,
    /// Recipients for list operations
    pub recipients: Option<Vec<serde_json::Value>>,
}

/// SendGrid node - performs SendGrid email operations
pub struct SendGridNode;

impl SendGridNode {
    pub fn new() -> Self {
        Self
    }
}

impl Default for SendGridNode {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeType for SendGridNode {
    fn type_name(&self) -> &str {
        "sendgrid"
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
                    "description": "Name of the SendGrid API credentials to use",
                    "minLength": 1
                },
                "operation": {
                    "type": "string",
                    "description": "SendGrid operation to perform",
                    "enum": [
                        "send_email",
                        "send_template_email",
                        "add_recipient",
                        "create_list",
                        "delete_list"
                    ]
                },
                "to": {
                    "type": "array",
                    "description": "Array of recipient email addresses",
                    "items": {
                        "type": "string",
                        "format": "email"
                    }
                },
                "from": {
                    "type": "string",
                    "description": "Sender email address",
                    "format": "email"
                },
                "subject": {
                    "type": "string",
                    "description": "Email subject line"
                },
                "content": {
                    "type": "string",
                    "description": "Email content (plain text or HTML)"
                },
                "template_id": {
                    "type": "string",
                    "description": "SendGrid template ID for template emails"
                },
                "list_id": {
                    "type": "string",
                    "description": "List ID for list operations"
                },
                "recipients": {
                    "type": "array",
                    "description": "Array of recipient objects for list operations",
                    "items": {
                        "type": "object"
                    }
                }
            },
            "required": ["credentials_name", "operation"],
            "additionalProperties": false
        })
    }

    fn required_credential_type(&self) -> Option<&str> {
        Some("sendgrid_api")
    }
}

#[async_trait]
impl Node for SendGridNode {
    async fn execute(
        &self,
        context: &NodeContext,
        parameters: &serde_json::Value,
    ) -> anyhow::Result<NodeOutput> {
        let params: SendGridParams = serde_json::from_value(parameters.clone())?;

        // TODO: In a real implementation:
        // 1. Look up credentials from the credentials table using params.credentials_name
        // 2. Decrypt the credentials data to get the SendGrid API key
        // 3. Create a SendGrid client using the API key
        // 4. Execute the operation based on params.operation:
        //    - send_email: Send a simple email
        //    - send_template_email: Send an email using a template
        //    - add_recipient: Add a recipient to a list
        //    - create_list: Create a new mailing list
        //    - delete_list: Delete a mailing list
        // 5. Return the API response

        // For now, we'll return a placeholder response
        let result = serde_json::json!({
            "message": "SendGrid operation executed (placeholder implementation)",
            "credentials_name": &params.credentials_name,
            "operation": &params.operation,
            "to": params.to,
            "from": params.from,
            "subject": params.subject,
            "context_execution_id": &context.execution_id,
            "success": true
        });

        Ok(NodeOutput::success(result))
    }

    fn validate_parameters(&self, parameters: &serde_json::Value) -> anyhow::Result<()> {
        let params: SendGridParams = serde_json::from_value(parameters.clone())
            .map_err(|e| anyhow::anyhow!("Invalid parameters: {}", e))?;

        if params.credentials_name.trim().is_empty() {
            anyhow::bail!("credentials_name cannot be empty");
        }

        // Validate operation
        let valid_operations = [
            "send_email",
            "send_template_email",
            "add_recipient",
            "create_list",
            "delete_list",
        ];

        if !valid_operations.contains(&params.operation.as_str()) {
            anyhow::bail!(
                "Invalid operation: {}. Must be one of: {}",
                params.operation,
                valid_operations.join(", ")
            );
        }

        // Validate email operations
        let email_ops = ["send_email", "send_template_email"];
        if email_ops.contains(&params.operation.as_str()) {
            if params.to.is_none() || params.to.as_ref().unwrap().is_empty() {
                anyhow::bail!("{} operation requires 'to' parameter", params.operation);
            }
            if params.from.is_none() {
                anyhow::bail!("{} operation requires 'from' parameter", params.operation);
            }
        }

        // Validate send_email specific fields
        if params.operation == "send_email" {
            if params.subject.is_none() {
                anyhow::bail!("send_email operation requires 'subject' parameter");
            }
            if params.content.is_none() {
                anyhow::bail!("send_email operation requires 'content' parameter");
            }
        }

        // Validate send_template_email specific fields
        if params.operation == "send_template_email" && params.template_id.is_none() {
            anyhow::bail!("send_template_email operation requires 'template_id' parameter");
        }

        // Validate list operations
        let list_ops = ["add_recipient", "delete_list"];
        if list_ops.contains(&params.operation.as_str()) && params.list_id.is_none() {
            anyhow::bail!(
                "{} operation requires 'list_id' parameter",
                params.operation
            );
        }

        // Validate add_recipient
        if params.operation == "add_recipient" && params.recipients.is_none() {
            anyhow::bail!("add_recipient operation requires 'recipients' parameter");
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
    fn test_sendgrid_node_type() {
        let node = SendGridNode::new();
        assert_eq!(node.type_name(), "sendgrid");
        assert!(matches!(node.category(), NodeCategory::Action));
        assert!(matches!(node.subcategory(), NodeSubcategory::Communication));
        assert_eq!(node.required_credential_type(), Some("sendgrid_api"));
    }

    #[test]
    fn test_sendgrid_parameter_schema() {
        let node = SendGridNode::new();
        let schema = node.parameter_schema();

        assert_eq!(schema["type"], "object");
        assert!(schema["properties"]["credentials_name"].is_object());
        assert!(schema["properties"]["operation"].is_object());
        assert!(schema["properties"]["to"].is_object());
        assert!(schema["properties"]["from"].is_object());
    }

    #[tokio::test]
    async fn test_sendgrid_send_email() {
        let node = SendGridNode::new();
        let context = NodeContext::new(Uuid::new_v4().to_string(), "test-workflow".to_string());

        let params = json!({
            "credentials_name": "my_sendgrid_creds",
            "operation": "send_email",
            "to": ["recipient@example.com"],
            "from": "sender@example.com",
            "subject": "Test Email",
            "content": "This is a test email"
        });

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
    }

    #[tokio::test]
    async fn test_sendgrid_send_template_email() {
        let node = SendGridNode::new();
        let context = NodeContext::new(Uuid::new_v4().to_string(), "test-workflow".to_string());

        let params = json!({
            "credentials_name": "my_sendgrid_creds",
            "operation": "send_template_email",
            "to": ["recipient@example.com"],
            "from": "sender@example.com",
            "template_id": "d-1234567890abcdef"
        });

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
    }

    #[test]
    fn test_sendgrid_validation() {
        let node = SendGridNode::new();

        // Valid send_email
        let valid_params = json!({
            "credentials_name": "my_sendgrid_creds",
            "operation": "send_email",
            "to": ["test@example.com"],
            "from": "sender@example.com",
            "subject": "Test",
            "content": "Test content"
        });
        assert!(node.validate_parameters(&valid_params).is_ok());

        // Missing 'to' for send_email
        let invalid_params = json!({
            "credentials_name": "my_sendgrid_creds",
            "operation": "send_email",
            "from": "sender@example.com",
            "subject": "Test",
            "content": "Test"
        });
        assert!(node.validate_parameters(&invalid_params).is_err());

        // Missing template_id for send_template_email
        let invalid_params = json!({
            "credentials_name": "my_sendgrid_creds",
            "operation": "send_template_email",
            "to": ["test@example.com"],
            "from": "sender@example.com"
        });
        assert!(node.validate_parameters(&invalid_params).is_err());

        // Invalid operation
        let invalid_params = json!({
            "credentials_name": "my_sendgrid_creds",
            "operation": "invalid_operation"
        });
        assert!(node.validate_parameters(&invalid_params).is_err());
    }
}
