use crate::models::{Node, NodeCategory, NodeContext, NodeOutput, NodeSubcategory, NodeType};
use async_trait::async_trait;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GitHubParams {
    /// Credentials name to use for GitHub API
    pub credentials_name: String,
    /// GitHub operation to perform
    pub operation: String,
    /// Repository owner (username or organization)
    pub owner: Option<String>,
    /// Repository name
    pub repo: Option<String>,
    /// Issue or PR number
    pub issue_number: Option<u32>,
    /// PR number
    pub pull_number: Option<u32>,
    /// Release ID or tag name
    pub release_id: Option<String>,
    /// Title for issues, PRs, releases
    pub title: Option<String>,
    /// Body content (markdown supported)
    pub body: Option<String>,
    /// Labels (comma-separated)
    pub labels: Option<Vec<String>>,
    /// Assignees (comma-separated usernames)
    pub assignees: Option<Vec<String>>,
    /// Milestone number
    pub milestone: Option<u32>,
    /// State: "open", "closed", "all"
    pub state: Option<String>,
    /// Base branch for PR
    pub base: Option<String>,
    /// Head branch for PR
    pub head: Option<String>,
    /// Tag name for releases
    pub tag_name: Option<String>,
    /// Target commitish for releases
    pub target_commitish: Option<String>,
    /// Is draft PR/release
    pub draft: Option<bool>,
    /// Is prerelease
    pub prerelease: Option<bool>,
    /// Comment body
    pub comment_body: Option<String>,
    /// File path in repository
    pub path: Option<String>,
    /// File content (base64 for binary files)
    pub content: Option<String>,
    /// Commit message
    pub message: Option<String>,
    /// Branch name
    pub branch: Option<String>,
    /// Commit SHA
    pub sha: Option<String>,
    /// Search query
    pub query: Option<String>,
    /// Results per page (1-100)
    pub per_page: Option<u32>,
    /// Page number for pagination
    pub page: Option<u32>,
    /// Additional parameters
    #[serde(default)]
    pub additional_params: serde_json::Value,
}

/// GitHub node - performs GitHub API operations
pub struct GitHubNode;

impl GitHubNode {
    pub fn new() -> Self {
        Self
    }
}

impl Default for GitHubNode {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeType for GitHubNode {
    fn type_name(&self) -> &str {
        "github"
    }

    fn category(&self) -> NodeCategory {
        NodeCategory::Action
    }

    fn subcategory(&self) -> NodeSubcategory {
        NodeSubcategory::General
    }

    fn required_credential_type(&self) -> Option<&str> {
        Some("github_token")
    }

    fn parameter_schema(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "credentials_name": {
                    "type": "string",
                    "description": "Name of the GitHub credentials to use",
                    "minLength": 1
                },
                "operation": {
                    "type": "string",
                    "description": "GitHub operation to perform",
                    "enum": [
                        "get_repo",
                        "list_repos",
                        "create_repo",
                        "update_repo",
                        "delete_repo",
                        "list_issues",
                        "get_issue",
                        "create_issue",
                        "update_issue",
                        "close_issue",
                        "list_pull_requests",
                        "get_pull_request",
                        "create_pull_request",
                        "update_pull_request",
                        "merge_pull_request",
                        "close_pull_request",
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
                        "get_file_content",
                        "create_or_update_file",
                        "delete_file",
                        "search_repositories",
                        "search_issues",
                        "search_code",
                        "list_collaborators",
                        "add_collaborator",
                        "remove_collaborator"
                    ]
                },
                "owner": {
                    "type": "string",
                    "description": "Repository owner (username or organization)"
                },
                "repo": {
                    "type": "string",
                    "description": "Repository name"
                },
                "issue_number": {
                    "type": "integer",
                    "description": "Issue number",
                    "minimum": 1
                },
                "pull_number": {
                    "type": "integer",
                    "description": "Pull request number",
                    "minimum": 1
                },
                "title": {
                    "type": "string",
                    "description": "Title for issues, PRs, or releases"
                },
                "body": {
                    "type": "string",
                    "description": "Body content (markdown supported)"
                },
                "labels": {
                    "type": "array",
                    "description": "Labels to apply",
                    "items": {
                        "type": "string"
                    }
                },
                "assignees": {
                    "type": "array",
                    "description": "Usernames to assign",
                    "items": {
                        "type": "string"
                    }
                },
                "state": {
                    "type": "string",
                    "description": "State filter",
                    "enum": ["open", "closed", "all"]
                },
                "base": {
                    "type": "string",
                    "description": "Base branch for pull request"
                },
                "head": {
                    "type": "string",
                    "description": "Head branch for pull request"
                },
                "tag_name": {
                    "type": "string",
                    "description": "Tag name for release"
                },
                "draft": {
                    "type": "boolean",
                    "description": "Is draft"
                },
                "prerelease": {
                    "type": "boolean",
                    "description": "Is prerelease"
                },
                "path": {
                    "type": "string",
                    "description": "File path in repository"
                },
                "content": {
                    "type": "string",
                    "description": "File content (base64 encoded for binary)"
                },
                "message": {
                    "type": "string",
                    "description": "Commit message"
                },
                "branch": {
                    "type": "string",
                    "description": "Branch name"
                },
                "sha": {
                    "type": "string",
                    "description": "Commit SHA"
                },
                "query": {
                    "type": "string",
                    "description": "Search query"
                },
                "per_page": {
                    "type": "integer",
                    "description": "Results per page (1-100)",
                    "minimum": 1,
                    "maximum": 100,
                    "default": 30
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
impl Node for GitHubNode {
    async fn execute(
        &self,
        context: &NodeContext,
        parameters: &serde_json::Value,
    ) -> anyhow::Result<NodeOutput> {
        let params: GitHubParams = serde_json::from_value(parameters.clone())?;

        // TODO: In a real implementation:
        // 1. Look up credentials from the credentials table using params.credentials_name
        // 2. Verify the credential type matches required_credential_type() (github_token)
        // 3. Decrypt the credentials data to get the GitHub token
        // 4. Create GitHub API client using reqwest with authentication
        // 5. Execute the operation based on params.operation:
        //
        //    API Base: https://api.github.com
        //    Headers: Authorization: Bearer {token}, Accept: application/vnd.github+json
        //
        //    get_repo: GET /repos/{owner}/{repo}
        //    list_repos: GET /user/repos or /orgs/{org}/repos
        //    create_repo: POST /user/repos or /orgs/{org}/repos
        //    list_issues: GET /repos/{owner}/{repo}/issues
        //    create_issue: POST /repos/{owner}/{repo}/issues
        //    create_pull_request: POST /repos/{owner}/{repo}/pulls
        //    merge_pull_request: PUT /repos/{owner}/{repo}/pulls/{pull_number}/merge
        //    create_release: POST /repos/{owner}/{repo}/releases
        //    get_file_content: GET /repos/{owner}/{repo}/contents/{path}
        //    create_or_update_file: PUT /repos/{owner}/{repo}/contents/{path}
        //    search_repositories: GET /search/repositories?q={query}
        //    etc.
        //
        // 6. Handle pagination with Link headers
        // 7. Handle rate limiting (check X-RateLimit-* headers)
        // 8. Parse the response and return results

        // For now, we'll return a placeholder response
        let result = serde_json::json!({
            "message": "GitHub operation executed (placeholder implementation)",
            "credentials_name": &params.credentials_name,
            "operation": &params.operation,
            "owner": params.owner,
            "repo": params.repo,
            "context_execution_id": &context.execution_id,
            "success": true
        });

        Ok(NodeOutput::success(result))
    }

    fn validate_parameters(&self, parameters: &serde_json::Value) -> anyhow::Result<()> {
        let params: GitHubParams = serde_json::from_value(parameters.clone())
            .map_err(|e| anyhow::anyhow!("Invalid parameters: {}", e))?;

        if params.credentials_name.trim().is_empty() {
            anyhow::bail!("credentials_name cannot be empty");
        }

        // Validate operation
        let valid_operations = [
            "get_repo",
            "list_repos",
            "create_repo",
            "update_repo",
            "delete_repo",
            "list_issues",
            "get_issue",
            "create_issue",
            "update_issue",
            "close_issue",
            "list_pull_requests",
            "get_pull_request",
            "create_pull_request",
            "update_pull_request",
            "merge_pull_request",
            "close_pull_request",
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
            "get_file_content",
            "create_or_update_file",
            "delete_file",
            "search_repositories",
            "search_issues",
            "search_code",
            "list_collaborators",
            "add_collaborator",
            "remove_collaborator",
        ];

        if !valid_operations.contains(&params.operation.as_str()) {
            anyhow::bail!(
                "Invalid operation: {}. Must be one of: {}",
                params.operation,
                valid_operations.join(", ")
            );
        }

        // Validate repository operations have owner and repo
        let repo_ops = [
            "get_repo",
            "update_repo",
            "delete_repo",
            "list_issues",
            "get_issue",
            "create_issue",
            "update_issue",
            "close_issue",
            "list_pull_requests",
            "get_pull_request",
            "create_pull_request",
            "update_pull_request",
            "merge_pull_request",
            "close_pull_request",
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
            "get_file_content",
            "create_or_update_file",
            "delete_file",
            "list_collaborators",
            "add_collaborator",
            "remove_collaborator",
        ];

        if repo_ops.contains(&params.operation.as_str()) {
            if params.owner.is_none() {
                anyhow::bail!("{} operation requires 'owner' parameter", params.operation);
            }
            if params.repo.is_none() {
                anyhow::bail!("{} operation requires 'repo' parameter", params.operation);
            }
        }

        // Validate issue operations
        if ["get_issue", "update_issue", "close_issue"].contains(&params.operation.as_str())
            && params.issue_number.is_none()
        {
            anyhow::bail!(
                "{} operation requires 'issue_number' parameter",
                params.operation
            );
        }

        // Validate PR operations
        if [
            "get_pull_request",
            "update_pull_request",
            "merge_pull_request",
            "close_pull_request",
        ]
        .contains(&params.operation.as_str())
            && params.pull_number.is_none()
        {
            anyhow::bail!(
                "{} operation requires 'pull_number' parameter",
                params.operation
            );
        }

        // Validate create operations
        if params.operation == "create_issue" && params.title.is_none() {
            anyhow::bail!("create_issue operation requires 'title' parameter");
        }

        if params.operation == "create_pull_request" {
            if params.title.is_none() {
                anyhow::bail!("create_pull_request operation requires 'title' parameter");
            }
            if params.head.is_none() {
                anyhow::bail!("create_pull_request operation requires 'head' parameter");
            }
            if params.base.is_none() {
                anyhow::bail!("create_pull_request operation requires 'base' parameter");
            }
        }

        if params.operation == "create_release" && params.tag_name.is_none() {
            anyhow::bail!("create_release operation requires 'tag_name' parameter");
        }

        // Validate file operations
        if ["get_file_content", "delete_file"].contains(&params.operation.as_str())
            && params.path.is_none()
        {
            anyhow::bail!("{} operation requires 'path' parameter", params.operation);
        }

        if params.operation == "create_or_update_file" {
            if params.path.is_none() {
                anyhow::bail!("create_or_update_file operation requires 'path' parameter");
            }
            if params.message.is_none() {
                anyhow::bail!("create_or_update_file operation requires 'message' parameter");
            }
            if params.content.is_none() {
                anyhow::bail!("create_or_update_file operation requires 'content' parameter");
            }
        }

        // Validate search operations
        if ["search_repositories", "search_issues", "search_code"]
            .contains(&params.operation.as_str())
            && params.query.is_none()
        {
            anyhow::bail!("{} operation requires 'query' parameter", params.operation);
        }

        // Validate per_page range
        if let Some(per_page) = params.per_page
            && !(1..=100).contains(&per_page)
        {
            anyhow::bail!("per_page must be between 1 and 100");
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_github_create_issue() {
        let node = GitHubNode::new();
        let context = NodeContext::new("exec-123".to_string(), "node-1".to_string());

        let params = serde_json::json!({
            "credentials_name": "my_github",
            "operation": "create_issue",
            "owner": "myorg",
            "repo": "myrepo",
            "title": "Bug: Application crashes on startup",
            "body": "## Description\nThe application crashes when starting...",
            "labels": ["bug", "high-priority"]
        });

        let result = node.execute(&context, &params).await.unwrap();
        assert!(result.success);
        assert_eq!(result.data["operation"], "create_issue");
        assert_eq!(result.data["owner"], "myorg");
        assert_eq!(result.data["repo"], "myrepo");
    }

    #[tokio::test]
    async fn test_github_create_pull_request() {
        let node = GitHubNode::new();
        let context = NodeContext::new("exec-123".to_string(), "node-1".to_string());

        let params = serde_json::json!({
            "credentials_name": "my_github",
            "operation": "create_pull_request",
            "owner": "myorg",
            "repo": "myrepo",
            "title": "feat: Add new feature",
            "body": "This PR adds a new feature",
            "head": "feature-branch",
            "base": "main"
        });

        let result = node.execute(&context, &params).await.unwrap();
        assert!(result.success);
        assert_eq!(result.data["operation"], "create_pull_request");
    }

    #[tokio::test]
    async fn test_github_create_release() {
        let node = GitHubNode::new();
        let context = NodeContext::new("exec-123".to_string(), "node-1".to_string());

        let params = serde_json::json!({
            "credentials_name": "my_github",
            "operation": "create_release",
            "owner": "myorg",
            "repo": "myrepo",
            "tag_name": "v1.0.0",
            "title": "Release v1.0.0",
            "body": "## Changes\n- Feature A\n- Bug fix B",
            "draft": false,
            "prerelease": false
        });

        let result = node.execute(&context, &params).await.unwrap();
        assert!(result.success);
        assert_eq!(result.data["operation"], "create_release");
    }

    #[tokio::test]
    async fn test_github_search_repositories() {
        let node = GitHubNode::new();
        let context = NodeContext::new("exec-123".to_string(), "node-1".to_string());

        let params = serde_json::json!({
            "credentials_name": "my_github",
            "operation": "search_repositories",
            "query": "language:rust stars:>1000",
            "per_page": 50
        });

        let result = node.execute(&context, &params).await.unwrap();
        assert!(result.success);
        assert_eq!(result.data["operation"], "search_repositories");
    }

    #[test]
    fn test_github_validation() {
        let node = GitHubNode::new();

        // Valid create_issue
        let valid_create_issue = serde_json::json!({
            "credentials_name": "my_github",
            "operation": "create_issue",
            "owner": "myorg",
            "repo": "myrepo",
            "title": "Test issue"
        });
        assert!(node.validate_parameters(&valid_create_issue).is_ok());

        // Invalid: create_issue without owner
        let invalid_no_owner = serde_json::json!({
            "credentials_name": "my_github",
            "operation": "create_issue",
            "repo": "myrepo",
            "title": "Test"
        });
        assert!(node.validate_parameters(&invalid_no_owner).is_err());

        // Invalid: create_issue without title
        let invalid_no_title = serde_json::json!({
            "credentials_name": "my_github",
            "operation": "create_issue",
            "owner": "myorg",
            "repo": "myrepo"
        });
        assert!(node.validate_parameters(&invalid_no_title).is_err());

        // Invalid: create_pull_request without base
        let invalid_pr = serde_json::json!({
            "credentials_name": "my_github",
            "operation": "create_pull_request",
            "owner": "myorg",
            "repo": "myrepo",
            "title": "Test PR",
            "head": "feature"
        });
        assert!(node.validate_parameters(&invalid_pr).is_err());

        // Invalid: search without query
        let invalid_search = serde_json::json!({
            "credentials_name": "my_github",
            "operation": "search_repositories"
        });
        assert!(node.validate_parameters(&invalid_search).is_err());

        // Invalid: unknown operation
        let invalid_op = serde_json::json!({
            "credentials_name": "my_github",
            "operation": "invalid_op"
        });
        assert!(node.validate_parameters(&invalid_op).is_err());
    }

    #[test]
    fn test_github_node_type() {
        let node = GitHubNode::new();
        assert_eq!(node.type_name(), "github");
        assert_eq!(node.category(), NodeCategory::Action);
        assert_eq!(node.subcategory(), NodeSubcategory::General);
        assert_eq!(node.required_credential_type(), Some("github_token"));
    }

    #[test]
    fn test_github_parameter_schema() {
        let node = GitHubNode::new();
        let schema = node.parameter_schema();

        assert_eq!(schema["type"], "object");
        assert!(schema["properties"]["credentials_name"].is_object());
        assert!(schema["properties"]["operation"].is_object());
        assert!(schema["properties"]["operation"]["enum"].is_array());
        assert_eq!(schema["required"].as_array().unwrap().len(), 2);
    }
}
