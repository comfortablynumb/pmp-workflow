use crate::models::{Node, NodeCategory, NodeContext, NodeOutput, NodeSubcategory, NodeType};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::json;

/// Jira node for project management and issue tracking operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JiraNode {
    #[serde(skip)]
    _private: (),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JiraNodeParams {
    /// The operation to perform
    pub operation: String,

    // Common parameters
    /// Issue key (e.g., "PROJ-123")
    pub issue_key: Option<String>,
    /// Project key or ID
    pub project_key: Option<String>,

    // Issue operations
    /// Issue summary/title
    pub summary: Option<String>,
    /// Issue description
    pub description: Option<String>,
    /// Issue type (Bug, Task, Story, etc.)
    pub issue_type: Option<String>,
    /// Priority (Highest, High, Medium, Low, Lowest)
    pub priority: Option<String>,
    /// Assignee username
    pub assignee: Option<String>,
    /// Labels
    pub labels: Option<Vec<String>>,
    /// Custom fields (JSON object)
    pub fields: Option<serde_json::Value>,

    // Search operations
    /// JQL query string
    pub jql: Option<String>,
    /// Maximum results to return
    pub max_results: Option<i32>,
    /// Start at index (for pagination)
    pub start_at: Option<i32>,

    // Comment operations
    /// Comment body text
    pub comment: Option<String>,
    /// Comment ID
    pub comment_id: Option<String>,

    // Transition operations
    /// Transition ID or name
    pub transition: Option<String>,

    // Sprint operations
    /// Sprint ID
    pub sprint_id: Option<String>,
    /// Sprint name
    pub sprint_name: Option<String>,
    /// Board ID
    pub board_id: Option<String>,

    // Attachment operations
    /// File path or URL
    pub file_path: Option<String>,
    /// File content (base64 encoded)
    pub file_content: Option<String>,
    /// Filename
    pub filename: Option<String>,

    // Link operations
    /// Link type (Blocks, Clones, Duplicates, Relates to, etc.)
    pub link_type: Option<String>,
    /// Inward issue key
    pub inward_issue: Option<String>,
    /// Outward issue key
    pub outward_issue: Option<String>,
}

impl JiraNode {
    pub fn new() -> Self {
        Self { _private: () }
    }
}

impl Default for JiraNode {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl NodeType for JiraNode {
    fn type_name(&self) -> &str {
        "jira"
    }

    fn category(&self) -> NodeCategory {
        NodeCategory::Action
    }

    fn subcategory(&self) -> NodeSubcategory {
        NodeSubcategory::General
    }

    fn required_credential_type(&self) -> Option<&str> {
        Some("jira_api_token")
    }

    fn parameter_schema(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "required": ["operation"],
            "properties": {
                "operation": {
                    "type": "string",
                    "enum": [
                        "get_issue",
                        "create_issue",
                        "update_issue",
                        "delete_issue",
                        "search_issues",
                        "get_issue_transitions",
                        "transition_issue",
                        "add_comment",
                        "get_comments",
                        "update_comment",
                        "delete_comment",
                        "add_attachment",
                        "get_attachments",
                        "create_issue_link",
                        "delete_issue_link",
                        "get_project",
                        "list_projects",
                        "create_project",
                        "get_board",
                        "list_boards",
                        "get_sprint",
                        "list_sprints",
                        "create_sprint",
                        "start_sprint",
                        "close_sprint",
                        "add_issues_to_sprint",
                        "get_issue_worklogs",
                        "add_worklog",
                        "get_user",
                        "search_users"
                    ],
                    "description": "The operation to perform"
                },
                "issue_key": {
                    "type": "string",
                    "description": "Issue key (e.g., PROJ-123)"
                },
                "project_key": {
                    "type": "string",
                    "description": "Project key or ID"
                },
                "summary": {
                    "type": "string",
                    "description": "Issue summary/title"
                },
                "description": {
                    "type": "string",
                    "description": "Issue description"
                },
                "issue_type": {
                    "type": "string",
                    "description": "Issue type (Bug, Task, Story, etc.)"
                },
                "priority": {
                    "type": "string",
                    "description": "Priority (Highest, High, Medium, Low, Lowest)"
                },
                "assignee": {
                    "type": "string",
                    "description": "Assignee username"
                },
                "labels": {
                    "type": "array",
                    "items": {"type": "string"},
                    "description": "Issue labels"
                },
                "fields": {
                    "type": "object",
                    "description": "Custom fields as JSON object"
                },
                "jql": {
                    "type": "string",
                    "description": "JQL query string"
                },
                "max_results": {
                    "type": "integer",
                    "description": "Maximum results to return"
                },
                "start_at": {
                    "type": "integer",
                    "description": "Start at index for pagination"
                },
                "comment": {
                    "type": "string",
                    "description": "Comment body text"
                },
                "comment_id": {
                    "type": "string",
                    "description": "Comment ID"
                },
                "transition": {
                    "type": "string",
                    "description": "Transition ID or name"
                },
                "sprint_id": {
                    "type": "string",
                    "description": "Sprint ID"
                },
                "sprint_name": {
                    "type": "string",
                    "description": "Sprint name"
                },
                "board_id": {
                    "type": "string",
                    "description": "Board ID"
                },
                "file_path": {
                    "type": "string",
                    "description": "File path for attachment"
                },
                "file_content": {
                    "type": "string",
                    "description": "File content (base64 encoded)"
                },
                "filename": {
                    "type": "string",
                    "description": "Filename for attachment"
                },
                "link_type": {
                    "type": "string",
                    "description": "Link type (Blocks, Clones, Duplicates, etc.)"
                },
                "inward_issue": {
                    "type": "string",
                    "description": "Inward issue key for link"
                },
                "outward_issue": {
                    "type": "string",
                    "description": "Outward issue key for link"
                }
            }
        })
    }

}

#[async_trait]
impl Node for JiraNode {
    async fn execute(
        &self,
        context: &NodeContext,
        parameters: &serde_json::Value,
    ) -> anyhow::Result<NodeOutput> {
        let _params: JiraNodeParams = serde_json::from_value(parameters.clone())?;

        // Mock implementation - in a real implementation, this would make HTTP requests to Jira API
        let result = json!({
            "success": true,
            "message": "Jira operation would be executed here",
            "execution_id": &context.execution_id
        });

        Ok(NodeOutput::success(result))
    }

    fn validate_parameters(&self, parameters: &serde_json::Value) -> anyhow::Result<()> {
        let params: JiraNodeParams = serde_json::from_value(parameters.clone())?;

        // Validate based on operation
        match params.operation.as_str() {
            "get_issue" | "delete_issue" | "get_issue_transitions" | "get_comments"
            | "get_attachments" | "get_issue_worklogs" => {
                if params.issue_key.is_none() {
                    anyhow::bail!("{} operation requires 'issue_key' parameter", params.operation);
                }
            }
            "create_issue" => {
                if params.project_key.is_none() {
                    anyhow::bail!("create_issue operation requires 'project_key' parameter");
                }
                if params.summary.is_none() {
                    anyhow::bail!("create_issue operation requires 'summary' parameter");
                }
                if params.issue_type.is_none() {
                    anyhow::bail!("create_issue operation requires 'issue_type' parameter");
                }
            }
            "update_issue" => {
                if params.issue_key.is_none() {
                    anyhow::bail!("update_issue operation requires 'issue_key' parameter");
                }
            }
            "search_issues" => {
                if params.jql.is_none() {
                    anyhow::bail!("search_issues operation requires 'jql' parameter");
                }
            }
            "transition_issue" => {
                if params.issue_key.is_none() {
                    anyhow::bail!("transition_issue operation requires 'issue_key' parameter");
                }
                if params.transition.is_none() {
                    anyhow::bail!("transition_issue operation requires 'transition' parameter");
                }
            }
            "add_comment" => {
                if params.issue_key.is_none() {
                    anyhow::bail!("add_comment operation requires 'issue_key' parameter");
                }
                if params.comment.is_none() {
                    anyhow::bail!("add_comment operation requires 'comment' parameter");
                }
            }
            "update_comment" | "delete_comment" => {
                if params.issue_key.is_none() {
                    anyhow::bail!("{} operation requires 'issue_key' parameter", params.operation);
                }
                if params.comment_id.is_none() {
                    anyhow::bail!("{} operation requires 'comment_id' parameter", params.operation);
                }
            }
            "add_attachment" => {
                if params.issue_key.is_none() {
                    anyhow::bail!("add_attachment operation requires 'issue_key' parameter");
                }
                if params.file_path.is_none() && params.file_content.is_none() {
                    anyhow::bail!("add_attachment operation requires 'file_path' or 'file_content' parameter");
                }
            }
            "create_issue_link" => {
                if params.link_type.is_none() {
                    anyhow::bail!("create_issue_link operation requires 'link_type' parameter");
                }
                if params.inward_issue.is_none() || params.outward_issue.is_none() {
                    anyhow::bail!("create_issue_link operation requires 'inward_issue' and 'outward_issue' parameters");
                }
            }
            "get_project" | "create_project" => {
                if params.project_key.is_none() {
                    anyhow::bail!("{} operation requires 'project_key' parameter", params.operation);
                }
            }
            "get_board" => {
                if params.board_id.is_none() {
                    anyhow::bail!("get_board operation requires 'board_id' parameter");
                }
            }
            "get_sprint" | "start_sprint" | "close_sprint" => {
                if params.sprint_id.is_none() {
                    anyhow::bail!("{} operation requires 'sprint_id' parameter", params.operation);
                }
            }
            "list_sprints" => {
                if params.board_id.is_none() {
                    anyhow::bail!("list_sprints operation requires 'board_id' parameter");
                }
            }
            "create_sprint" => {
                if params.board_id.is_none() {
                    anyhow::bail!("create_sprint operation requires 'board_id' parameter");
                }
                if params.sprint_name.is_none() {
                    anyhow::bail!("create_sprint operation requires 'sprint_name' parameter");
                }
            }
            "add_issues_to_sprint" => {
                if params.sprint_id.is_none() {
                    anyhow::bail!("add_issues_to_sprint operation requires 'sprint_id' parameter");
                }
            }
            "add_worklog" => {
                if params.issue_key.is_none() {
                    anyhow::bail!("add_worklog operation requires 'issue_key' parameter");
                }
            }
            "search_users" => {
                // No required parameters
            }
            "get_user" => {
                // accountId or username can be used
            }
            "list_projects" | "list_boards" | "delete_issue_link" => {
                // No required parameters for these operations
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
    fn test_jira_node_creation() {
        let node = JiraNode::new();
        assert_eq!(node.type_name(), "jira");
        assert!(matches!(node.category(), NodeCategory::Action));
    }

    #[test]
    fn test_jira_required_credential_type() {
        let node = JiraNode::new();
        assert_eq!(node.required_credential_type(), Some("jira_api_token"));
    }

    #[test]
    fn test_create_issue_validation() {
        let node = JiraNode::new();

        // Valid create_issue
        let valid_params = json!({
            "operation": "create_issue",
            "project_key": "PROJ",
            "summary": "Test issue",
            "issue_type": "Task"
        });
        assert!(node.validate_parameters(&valid_params).is_ok());

        // Missing project_key
        let invalid_params = json!({
            "operation": "create_issue",
            "summary": "Test issue",
            "issue_type": "Task"
        });
        assert!(node.validate_parameters(&invalid_params).is_err());
    }

    #[test]
    fn test_search_issues_validation() {
        let node = JiraNode::new();

        // Valid search
        let valid_params = json!({
            "operation": "search_issues",
            "jql": "project = PROJ AND status = Open"
        });
        assert!(node.validate_parameters(&valid_params).is_ok());

        // Missing jql
        let invalid_params = json!({
            "operation": "search_issues"
        });
        assert!(node.validate_parameters(&invalid_params).is_err());
    }

    #[test]
    fn test_add_comment_validation() {
        let node = JiraNode::new();

        // Valid add_comment
        let valid_params = json!({
            "operation": "add_comment",
            "issue_key": "PROJ-123",
            "comment": "Test comment"
        });
        assert!(node.validate_parameters(&valid_params).is_ok());

        // Missing comment
        let invalid_params = json!({
            "operation": "add_comment",
            "issue_key": "PROJ-123"
        });
        assert!(node.validate_parameters(&invalid_params).is_err());
    }

    #[test]
    fn test_transition_issue_validation() {
        let node = JiraNode::new();

        // Valid transition
        let valid_params = json!({
            "operation": "transition_issue",
            "issue_key": "PROJ-123",
            "transition": "Done"
        });
        assert!(node.validate_parameters(&valid_params).is_ok());
    }

    #[tokio::test]
    async fn test_jira_execute() {
        let node = JiraNode::new();
        let params = json!({
            "operation": "get_issue",
            "issue_key": "PROJ-123"
        });
        let context = NodeContext::new("test-exec".to_string(), "test-node".to_string());

        let result = node.execute(&context, &params).await;
        assert!(result.is_ok());
    }
}
