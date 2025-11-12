use crate::models::{Node, NodeCategory, NodeContext, NodeOutput, NodeSubcategory, NodeType};
use async_trait::async_trait;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GitLabParams {
    /// Credentials name to use for GitLab API
    pub credentials_name: String,
    /// GitLab operation to perform
    pub operation: String,
    /// Project ID or path (e.g., "group/project")
    pub project: Option<String>,
    /// Issue IID (internal ID)
    pub issue_iid: Option<u32>,
    /// Merge request IID
    pub merge_request_iid: Option<u32>,
    /// Pipeline ID
    pub pipeline_id: Option<u32>,
    /// Job ID
    pub job_id: Option<u32>,
    /// Title for issues, MRs
    pub title: Option<String>,
    /// Description content (markdown supported)
    pub description: Option<String>,
    /// Labels (comma-separated)
    pub labels: Option<Vec<String>>,
    /// Assignee IDs
    pub assignee_ids: Option<Vec<u32>>,
    /// Milestone ID
    pub milestone_id: Option<u32>,
    /// State: "opened", "closed", "merged", "all"
    pub state: Option<String>,
    /// Source branch for MR
    pub source_branch: Option<String>,
    /// Target branch for MR
    pub target_branch: Option<String>,
    /// Tag name
    pub tag_name: Option<String>,
    /// Release description
    pub release_description: Option<String>,
    /// File path in repository
    pub file_path: Option<String>,
    /// File content
    pub content: Option<String>,
    /// Commit message
    pub commit_message: Option<String>,
    /// Branch name
    pub branch: Option<String>,
    /// Ref (branch/tag/commit)
    pub ref_: Option<String>,
    /// Comment/note body
    pub note_body: Option<String>,
    /// Search query
    pub search: Option<String>,
    /// Scope for search
    pub scope: Option<String>,
    /// Results per page (1-100)
    pub per_page: Option<u32>,
    /// Page number for pagination
    pub page: Option<u32>,
    /// Additional parameters
    #[serde(default)]
    pub additional_params: serde_json::Value,
}

/// GitLab node - performs GitLab API operations
pub struct GitLabNode;

impl GitLabNode {
    pub fn new() -> Self {
        Self
    }
}

impl Default for GitLabNode {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeType for GitLabNode {
    fn type_name(&self) -> &str {
        "gitlab"
    }

    fn category(&self) -> NodeCategory {
        NodeCategory::Action
    }

    fn subcategory(&self) -> NodeSubcategory {
        NodeSubcategory::General
    }

    fn required_credential_type(&self) -> Option<&str> {
        Some("gitlab_token")
    }

    fn parameter_schema(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "credentials_name": {
                    "type": "string",
                    "description": "Name of the GitLab credentials to use",
                    "minLength": 1
                },
                "operation": {
                    "type": "string",
                    "description": "GitLab operation to perform",
                    "enum": [
                        "get_project",
                        "list_projects",
                        "create_project",
                        "update_project",
                        "delete_project",
                        "list_issues",
                        "get_issue",
                        "create_issue",
                        "update_issue",
                        "close_issue",
                        "list_merge_requests",
                        "get_merge_request",
                        "create_merge_request",
                        "update_merge_request",
                        "merge_merge_request",
                        "close_merge_request",
                        "list_pipelines",
                        "get_pipeline",
                        "create_pipeline",
                        "cancel_pipeline",
                        "retry_pipeline",
                        "list_jobs",
                        "get_job",
                        "retry_job",
                        "cancel_job",
                        "list_releases",
                        "get_release",
                        "create_release",
                        "update_release",
                        "delete_release",
                        "list_commits",
                        "get_commit",
                        "create_commit_comment",
                        "list_branches",
                        "get_branch",
                        "create_branch",
                        "delete_branch",
                        "get_file",
                        "create_file",
                        "update_file",
                        "delete_file",
                        "search_projects",
                        "search_issues",
                        "list_members",
                        "add_member",
                        "remove_member"
                    ]
                },
                "project": {
                    "type": "string",
                    "description": "Project ID or path (e.g., 'group/project' or '123')"
                },
                "issue_iid": {
                    "type": "integer",
                    "description": "Issue IID (internal ID)",
                    "minimum": 1
                },
                "merge_request_iid": {
                    "type": "integer",
                    "description": "Merge request IID",
                    "minimum": 1
                },
                "title": {
                    "type": "string",
                    "description": "Title for issues, merge requests, or releases"
                },
                "description": {
                    "type": "string",
                    "description": "Description content (markdown supported)"
                },
                "labels": {
                    "type": "array",
                    "description": "Labels to apply",
                    "items": {
                        "type": "string"
                    }
                },
                "assignee_ids": {
                    "type": "array",
                    "description": "User IDs to assign",
                    "items": {
                        "type": "integer"
                    }
                },
                "state": {
                    "type": "string",
                    "description": "State filter",
                    "enum": ["opened", "closed", "merged", "all"]
                },
                "source_branch": {
                    "type": "string",
                    "description": "Source branch for merge request"
                },
                "target_branch": {
                    "type": "string",
                    "description": "Target branch for merge request"
                },
                "tag_name": {
                    "type": "string",
                    "description": "Tag name for release"
                },
                "file_path": {
                    "type": "string",
                    "description": "File path in repository"
                },
                "content": {
                    "type": "string",
                    "description": "File content"
                },
                "commit_message": {
                    "type": "string",
                    "description": "Commit message"
                },
                "branch": {
                    "type": "string",
                    "description": "Branch name"
                },
                "ref_": {
                    "type": "string",
                    "description": "Git ref (branch, tag, or commit SHA)"
                },
                "search": {
                    "type": "string",
                    "description": "Search query"
                },
                "per_page": {
                    "type": "integer",
                    "description": "Results per page (1-100)",
                    "minimum": 1,
                    "maximum": 100,
                    "default": 20
                },
                "page": {
                    "type": "integer",
                    "description": "Page number",
                    "minimum": 1,
                    "default": 1
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
impl Node for GitLabNode {
    async fn execute(
        &self,
        context: &NodeContext,
        parameters: &serde_json::Value,
    ) -> anyhow::Result<NodeOutput> {
        let params: GitLabParams = serde_json::from_value(parameters.clone())?;

        // TODO: In a real implementation:
        // 1. Look up credentials from the credentials table using params.credentials_name
        // 2. Verify the credential type matches required_credential_type() (gitlab_token)
        // 3. Decrypt the credentials data to get the GitLab token and base_url
        // 4. Create GitLab API client using reqwest with authentication
        // 5. Execute the operation based on params.operation:
        //
        //    API Base: {base_url}/api/v4 (default: https://gitlab.com/api/v4)
        //    Headers: PRIVATE-TOKEN: {token} or Authorization: Bearer {token}
        //
        //    get_project: GET /projects/{id}
        //    list_projects: GET /projects
        //    list_issues: GET /projects/{id}/issues
        //    create_issue: POST /projects/{id}/issues
        //    create_merge_request: POST /projects/{id}/merge_requests
        //    merge_merge_request: PUT /projects/{id}/merge_requests/{iid}/merge
        //    list_pipelines: GET /projects/{id}/pipelines
        //    create_pipeline: POST /projects/{id}/pipeline
        //    create_release: POST /projects/{id}/releases
        //    get_file: GET /projects/{id}/repository/files/{file_path}
        //    create_file: POST /projects/{id}/repository/files/{file_path}
        //    search_projects: GET /search?scope=projects&search={query}
        //    etc.
        //
        // 6. Handle pagination with Link headers and per_page/page params
        // 7. Handle rate limiting (check RateLimit-* headers)
        // 8. Parse the response and return results

        // For now, we'll return a placeholder response
        let result = serde_json::json!({
            "message": "GitLab operation executed (placeholder implementation)",
            "credentials_name": &params.credentials_name,
            "operation": &params.operation,
            "project": params.project,
            "context_execution_id": &context.execution_id,
            "success": true
        });

        Ok(NodeOutput::success(result))
    }

    fn validate_parameters(&self, parameters: &serde_json::Value) -> anyhow::Result<()> {
        let params: GitLabParams = serde_json::from_value(parameters.clone())
            .map_err(|e| anyhow::anyhow!("Invalid parameters: {}", e))?;

        if params.credentials_name.trim().is_empty() {
            anyhow::bail!("credentials_name cannot be empty");
        }

        // Validate operation
        let valid_operations = [
            "get_project",
            "list_projects",
            "create_project",
            "update_project",
            "delete_project",
            "list_issues",
            "get_issue",
            "create_issue",
            "update_issue",
            "close_issue",
            "list_merge_requests",
            "get_merge_request",
            "create_merge_request",
            "update_merge_request",
            "merge_merge_request",
            "close_merge_request",
            "list_pipelines",
            "get_pipeline",
            "create_pipeline",
            "cancel_pipeline",
            "retry_pipeline",
            "list_jobs",
            "get_job",
            "retry_job",
            "cancel_job",
            "list_releases",
            "get_release",
            "create_release",
            "update_release",
            "delete_release",
            "list_commits",
            "get_commit",
            "create_commit_comment",
            "list_branches",
            "get_branch",
            "create_branch",
            "delete_branch",
            "get_file",
            "create_file",
            "update_file",
            "delete_file",
            "search_projects",
            "search_issues",
            "list_members",
            "add_member",
            "remove_member",
        ];

        if !valid_operations.contains(&params.operation.as_str()) {
            anyhow::bail!(
                "Invalid operation: {}. Must be one of: {}",
                params.operation,
                valid_operations.join(", ")
            );
        }

        // Validate project-scoped operations
        let project_ops = [
            "get_project",
            "update_project",
            "delete_project",
            "list_issues",
            "get_issue",
            "create_issue",
            "update_issue",
            "close_issue",
            "list_merge_requests",
            "get_merge_request",
            "create_merge_request",
            "update_merge_request",
            "merge_merge_request",
            "close_merge_request",
            "list_pipelines",
            "get_pipeline",
            "create_pipeline",
            "cancel_pipeline",
            "retry_pipeline",
            "list_jobs",
            "get_job",
            "retry_job",
            "cancel_job",
            "list_releases",
            "get_release",
            "create_release",
            "update_release",
            "delete_release",
            "list_commits",
            "get_commit",
            "create_commit_comment",
            "list_branches",
            "get_branch",
            "create_branch",
            "delete_branch",
            "get_file",
            "create_file",
            "update_file",
            "delete_file",
            "list_members",
            "add_member",
            "remove_member",
        ];

        if project_ops.contains(&params.operation.as_str()) && params.project.is_none() {
            anyhow::bail!("{} operation requires 'project' parameter", params.operation);
        }

        // Validate issue operations
        if ["get_issue", "update_issue", "close_issue"].contains(&params.operation.as_str())
            && params.issue_iid.is_none()
        {
            anyhow::bail!("{} operation requires 'issue_iid' parameter", params.operation);
        }

        // Validate MR operations
        if [
            "get_merge_request",
            "update_merge_request",
            "merge_merge_request",
            "close_merge_request",
        ]
        .contains(&params.operation.as_str())
            && params.merge_request_iid.is_none()
        {
            anyhow::bail!(
                "{} operation requires 'merge_request_iid' parameter",
                params.operation
            );
        }

        // Validate create operations
        if params.operation == "create_issue" && params.title.is_none() {
            anyhow::bail!("create_issue operation requires 'title' parameter");
        }

        if params.operation == "create_merge_request" {
            if params.title.is_none() {
                anyhow::bail!("create_merge_request operation requires 'title' parameter");
            }
            if params.source_branch.is_none() {
                anyhow::bail!("create_merge_request operation requires 'source_branch' parameter");
            }
            if params.target_branch.is_none() {
                anyhow::bail!("create_merge_request operation requires 'target_branch' parameter");
            }
        }

        if params.operation == "create_release" && params.tag_name.is_none() {
            anyhow::bail!("create_release operation requires 'tag_name' parameter");
        }

        // Validate file operations
        if ["get_file", "delete_file"].contains(&params.operation.as_str())
            && params.file_path.is_none()
        {
            anyhow::bail!("{} operation requires 'file_path' parameter", params.operation);
        }

        if ["create_file", "update_file"].contains(&params.operation.as_str()) {
            if params.file_path.is_none() {
                anyhow::bail!("{} operation requires 'file_path' parameter", params.operation);
            }
            if params.content.is_none() {
                anyhow::bail!("{} operation requires 'content' parameter", params.operation);
            }
            if params.commit_message.is_none() {
                anyhow::bail!("{} operation requires 'commit_message' parameter", params.operation);
            }
        }

        // Validate search operations
        if ["search_projects", "search_issues"].contains(&params.operation.as_str())
            && params.search.is_none()
        {
            anyhow::bail!("{} operation requires 'search' parameter", params.operation);
        }

        // Validate per_page range
        if let Some(per_page) = params.per_page {
            if !(1..=100).contains(&per_page) {
                anyhow::bail!("per_page must be between 1 and 100");
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_gitlab_create_issue() {
        let node = GitLabNode::new();
        let context = NodeContext::new("exec-123".to_string(), "node-1".to_string());

        let params = serde_json::json!({
            "credentials_name": "my_gitlab",
            "operation": "create_issue",
            "project": "mygroup/myproject",
            "title": "Bug: Application crashes",
            "description": "## Description\nThe app crashes...",
            "labels": ["bug", "critical"]
        });

        let result = node.execute(&context, &params).await.unwrap();
        assert!(result.success);
        assert_eq!(result.data["operation"], "create_issue");
        assert_eq!(result.data["project"], "mygroup/myproject");
    }

    #[tokio::test]
    async fn test_gitlab_create_merge_request() {
        let node = GitLabNode::new();
        let context = NodeContext::new("exec-123".to_string(), "node-1".to_string());

        let params = serde_json::json!({
            "credentials_name": "my_gitlab",
            "operation": "create_merge_request",
            "project": "123",
            "title": "feat: Add new feature",
            "description": "This MR adds a new feature",
            "source_branch": "feature-branch",
            "target_branch": "main"
        });

        let result = node.execute(&context, &params).await.unwrap();
        assert!(result.success);
        assert_eq!(result.data["operation"], "create_merge_request");
    }

    #[tokio::test]
    async fn test_gitlab_create_pipeline() {
        let node = GitLabNode::new();
        let context = NodeContext::new("exec-123".to_string(), "node-1".to_string());

        let params = serde_json::json!({
            "credentials_name": "my_gitlab",
            "operation": "create_pipeline",
            "project": "mygroup/myproject",
            "ref_": "main"
        });

        let result = node.execute(&context, &params).await.unwrap();
        assert!(result.success);
        assert_eq!(result.data["operation"], "create_pipeline");
    }

    #[tokio::test]
    async fn test_gitlab_search_projects() {
        let node = GitLabNode::new();
        let context = NodeContext::new("exec-123".to_string(), "node-1".to_string());

        let params = serde_json::json!({
            "credentials_name": "my_gitlab",
            "operation": "search_projects",
            "search": "workflow automation",
            "per_page": 50
        });

        let result = node.execute(&context, &params).await.unwrap();
        assert!(result.success);
        assert_eq!(result.data["operation"], "search_projects");
    }

    #[test]
    fn test_gitlab_validation() {
        let node = GitLabNode::new();

        // Valid create_issue
        let valid_create_issue = serde_json::json!({
            "credentials_name": "my_gitlab",
            "operation": "create_issue",
            "project": "mygroup/myproject",
            "title": "Test issue"
        });
        assert!(node.validate_parameters(&valid_create_issue).is_ok());

        // Invalid: create_issue without project
        let invalid_no_project = serde_json::json!({
            "credentials_name": "my_gitlab",
            "operation": "create_issue",
            "title": "Test"
        });
        assert!(node.validate_parameters(&invalid_no_project).is_err());

        // Invalid: create_issue without title
        let invalid_no_title = serde_json::json!({
            "credentials_name": "my_gitlab",
            "operation": "create_issue",
            "project": "123"
        });
        assert!(node.validate_parameters(&invalid_no_title).is_err());

        // Invalid: create_merge_request without target_branch
        let invalid_mr = serde_json::json!({
            "credentials_name": "my_gitlab",
            "operation": "create_merge_request",
            "project": "123",
            "title": "Test MR",
            "source_branch": "feature"
        });
        assert!(node.validate_parameters(&invalid_mr).is_err());

        // Invalid: search without search parameter
        let invalid_search = serde_json::json!({
            "credentials_name": "my_gitlab",
            "operation": "search_projects"
        });
        assert!(node.validate_parameters(&invalid_search).is_err());

        // Invalid: unknown operation
        let invalid_op = serde_json::json!({
            "credentials_name": "my_gitlab",
            "operation": "invalid_op"
        });
        assert!(node.validate_parameters(&invalid_op).is_err());
    }

    #[test]
    fn test_gitlab_node_type() {
        let node = GitLabNode::new();
        assert_eq!(node.type_name(), "gitlab");
        assert_eq!(node.category(), NodeCategory::Action);
        assert_eq!(node.subcategory(), NodeSubcategory::General);
        assert_eq!(node.required_credential_type(), Some("gitlab_token"));
    }

    #[test]
    fn test_gitlab_parameter_schema() {
        let node = GitLabNode::new();
        let schema = node.parameter_schema();

        assert_eq!(schema["type"], "object");
        assert!(schema["properties"]["credentials_name"].is_object());
        assert!(schema["properties"]["operation"].is_object());
        assert!(schema["properties"]["operation"]["enum"].is_array());
        assert_eq!(schema["required"].as_array().unwrap().len(), 2);
    }
}
