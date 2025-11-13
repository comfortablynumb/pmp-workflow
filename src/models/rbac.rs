use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::collections::HashSet;
use uuid::Uuid;

/// Permission types for workflow operations
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, sqlx::Type)]
#[sqlx(type_name = "permission", rename_all = "snake_case")]
pub enum Permission {
    /// View workflow definitions
    ViewWorkflow,
    /// Create new workflows
    CreateWorkflow,
    /// Edit workflow definitions
    EditWorkflow,
    /// Delete workflows
    DeleteWorkflow,
    /// Execute workflows
    ExecuteWorkflow,
    /// Cancel running executions
    CancelExecution,
    /// View execution results
    ViewExecution,
    /// Manage workflow credentials
    ManageCredentials,
    /// Manage user roles
    ManageRoles,
    /// View audit logs
    ViewAudit,
    /// Manage system settings
    ManageSystem,
}

impl std::fmt::Display for Permission {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Permission::ViewWorkflow => write!(f, "view_workflow"),
            Permission::CreateWorkflow => write!(f, "create_workflow"),
            Permission::EditWorkflow => write!(f, "edit_workflow"),
            Permission::DeleteWorkflow => write!(f, "delete_workflow"),
            Permission::ExecuteWorkflow => write!(f, "execute_workflow"),
            Permission::CancelExecution => write!(f, "cancel_execution"),
            Permission::ViewExecution => write!(f, "view_execution"),
            Permission::ManageCredentials => write!(f, "manage_credentials"),
            Permission::ManageRoles => write!(f, "manage_roles"),
            Permission::ViewAudit => write!(f, "view_audit"),
            Permission::ManageSystem => write!(f, "manage_system"),
        }
    }
}

/// Role definition with associated permissions
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Role {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    #[sqlx(json)]
    pub permissions: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Role {
    pub fn new(name: String, description: Option<String>, permissions: Vec<Permission>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            name,
            description,
            permissions: permissions.iter().map(|p| p.to_string()).collect(),
            created_at: now,
            updated_at: now,
        }
    }

    /// Check if role has a specific permission
    pub fn has_permission(&self, permission: &Permission) -> bool {
        self.permissions.contains(&permission.to_string())
    }

    /// Get all permissions as Permission enum
    pub fn get_permissions(&self) -> Vec<Permission> {
        self.permissions
            .iter()
            .filter_map(|p| match p.as_str() {
                "view_workflow" => Some(Permission::ViewWorkflow),
                "create_workflow" => Some(Permission::CreateWorkflow),
                "edit_workflow" => Some(Permission::EditWorkflow),
                "delete_workflow" => Some(Permission::DeleteWorkflow),
                "execute_workflow" => Some(Permission::ExecuteWorkflow),
                "cancel_execution" => Some(Permission::CancelExecution),
                "view_execution" => Some(Permission::ViewExecution),
                "manage_credentials" => Some(Permission::ManageCredentials),
                "manage_roles" => Some(Permission::ManageRoles),
                "view_audit" => Some(Permission::ViewAudit),
                "manage_system" => Some(Permission::ManageSystem),
                _ => None,
            })
            .collect()
    }
}

/// User role assignment
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserRole {
    pub id: Uuid,
    pub user_id: String,
    pub role_id: Uuid,
    pub granted_by: String,
    pub granted_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
}

impl UserRole {
    pub fn new(
        user_id: String,
        role_id: Uuid,
        granted_by: String,
        expires_at: Option<DateTime<Utc>>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            role_id,
            granted_by,
            granted_at: Utc::now(),
            expires_at,
        }
    }

    /// Check if the role assignment is still valid
    pub fn is_valid(&self) -> bool {
        if let Some(expires_at) = self.expires_at {
            expires_at > Utc::now()
        } else {
            true
        }
    }
}

/// Workflow access control entry
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct WorkflowAcl {
    pub id: Uuid,
    pub workflow_id: Uuid,
    pub user_id: Option<String>,
    pub role_id: Option<Uuid>,
    #[sqlx(json)]
    pub permissions: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub created_by: String,
}

impl WorkflowAcl {
    pub fn new(
        workflow_id: Uuid,
        user_id: Option<String>,
        role_id: Option<Uuid>,
        permissions: Vec<Permission>,
        created_by: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            workflow_id,
            user_id,
            role_id,
            permissions: permissions.iter().map(|p| p.to_string()).collect(),
            created_at: Utc::now(),
            created_by,
        }
    }

    /// Check if ACL grants a specific permission
    pub fn grants_permission(&self, permission: &Permission) -> bool {
        self.permissions.contains(&permission.to_string())
    }
}

/// RBAC context for authorization checks
#[derive(Debug, Clone)]
pub struct RbacContext {
    pub user_id: String,
    pub roles: Vec<Role>,
    pub workflow_acls: Vec<WorkflowAcl>,
}

impl RbacContext {
    pub fn new(user_id: String) -> Self {
        Self {
            user_id,
            roles: Vec::new(),
            workflow_acls: Vec::new(),
        }
    }

    pub fn with_roles(mut self, roles: Vec<Role>) -> Self {
        self.roles = roles;
        self
    }

    pub fn with_workflow_acls(mut self, acls: Vec<WorkflowAcl>) -> Self {
        self.workflow_acls = acls;
        self
    }

    /// Check if user has a specific permission globally
    pub fn has_permission(&self, permission: &Permission) -> bool {
        self.roles.iter().any(|role| role.has_permission(permission))
    }

    /// Check if user has permission for a specific workflow
    pub fn has_workflow_permission(&self, workflow_id: Uuid, permission: &Permission) -> bool {
        // Check global permissions first
        if self.has_permission(permission) {
            return true;
        }

        // Check workflow-specific ACLs
        self.workflow_acls.iter().any(|acl| {
            acl.workflow_id == workflow_id
                && (acl.user_id.as_ref() == Some(&self.user_id)
                    || acl
                        .role_id
                        .map(|rid| self.roles.iter().any(|r| r.id == rid))
                        .unwrap_or(false))
                && acl.grants_permission(permission)
        })
    }

    /// Get all unique permissions across all roles
    pub fn get_all_permissions(&self) -> HashSet<Permission> {
        self.roles
            .iter()
            .flat_map(|role| role.get_permissions())
            .collect()
    }
}

/// Pre-defined role templates
pub struct RoleTemplates;

impl RoleTemplates {
    /// Admin role with all permissions
    pub fn admin() -> Role {
        Role::new(
            "admin".to_string(),
            Some("Full system administrator".to_string()),
            vec![
                Permission::ViewWorkflow,
                Permission::CreateWorkflow,
                Permission::EditWorkflow,
                Permission::DeleteWorkflow,
                Permission::ExecuteWorkflow,
                Permission::CancelExecution,
                Permission::ViewExecution,
                Permission::ManageCredentials,
                Permission::ManageRoles,
                Permission::ViewAudit,
                Permission::ManageSystem,
            ],
        )
    }

    /// Developer role with workflow management permissions
    pub fn developer() -> Role {
        Role::new(
            "developer".to_string(),
            Some("Workflow developer".to_string()),
            vec![
                Permission::ViewWorkflow,
                Permission::CreateWorkflow,
                Permission::EditWorkflow,
                Permission::ExecuteWorkflow,
                Permission::ViewExecution,
            ],
        )
    }

    /// Operator role with execution permissions
    pub fn operator() -> Role {
        Role::new(
            "operator".to_string(),
            Some("Workflow operator".to_string()),
            vec![
                Permission::ViewWorkflow,
                Permission::ExecuteWorkflow,
                Permission::CancelExecution,
                Permission::ViewExecution,
            ],
        )
    }

    /// Viewer role with read-only access
    pub fn viewer() -> Role {
        Role::new(
            "viewer".to_string(),
            Some("Read-only viewer".to_string()),
            vec![Permission::ViewWorkflow, Permission::ViewExecution],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permission_display() {
        assert_eq!(Permission::ViewWorkflow.to_string(), "view_workflow");
        assert_eq!(Permission::CreateWorkflow.to_string(), "create_workflow");
        assert_eq!(Permission::ManageRoles.to_string(), "manage_roles");
    }

    #[test]
    fn test_role_creation() {
        let role = Role::new(
            "test-role".to_string(),
            Some("Test role".to_string()),
            vec![Permission::ViewWorkflow, Permission::ExecuteWorkflow],
        );

        assert_eq!(role.name, "test-role");
        assert_eq!(role.description, Some("Test role".to_string()));
        assert_eq!(role.permissions.len(), 2);
        assert!(role.has_permission(&Permission::ViewWorkflow));
        assert!(role.has_permission(&Permission::ExecuteWorkflow));
        assert!(!role.has_permission(&Permission::DeleteWorkflow));
    }

    #[test]
    fn test_user_role_validity() {
        let future = Utc::now() + chrono::Duration::days(7);
        let past = Utc::now() - chrono::Duration::days(1);

        let valid_role = UserRole::new(
            "user1".to_string(),
            Uuid::new_v4(),
            "admin".to_string(),
            Some(future),
        );
        assert!(valid_role.is_valid());

        let expired_role = UserRole::new(
            "user1".to_string(),
            Uuid::new_v4(),
            "admin".to_string(),
            Some(past),
        );
        assert!(!expired_role.is_valid());

        let permanent_role = UserRole::new(
            "user1".to_string(),
            Uuid::new_v4(),
            "admin".to_string(),
            None,
        );
        assert!(permanent_role.is_valid());
    }

    #[test]
    fn test_rbac_context_global_permissions() {
        let admin_role = RoleTemplates::admin();
        let viewer_role = RoleTemplates::viewer();

        let admin_context = RbacContext::new("admin-user".to_string())
            .with_roles(vec![admin_role]);

        let viewer_context = RbacContext::new("viewer-user".to_string())
            .with_roles(vec![viewer_role]);

        assert!(admin_context.has_permission(&Permission::DeleteWorkflow));
        assert!(!viewer_context.has_permission(&Permission::DeleteWorkflow));
        assert!(viewer_context.has_permission(&Permission::ViewWorkflow));
    }

    #[test]
    fn test_rbac_context_workflow_permissions() {
        let workflow_id = Uuid::new_v4();
        let user_id = "test-user".to_string();

        let acl = WorkflowAcl::new(
            workflow_id,
            Some(user_id.clone()),
            None,
            vec![Permission::ExecuteWorkflow],
            "admin".to_string(),
        );

        let context = RbacContext::new(user_id.clone())
            .with_roles(vec![RoleTemplates::viewer()])
            .with_workflow_acls(vec![acl]);

        // Should have workflow-specific permission
        assert!(context.has_workflow_permission(workflow_id, &Permission::ExecuteWorkflow));

        // Should have global viewer permission
        assert!(context.has_workflow_permission(workflow_id, &Permission::ViewWorkflow));

        // Should not have delete permission
        assert!(!context.has_workflow_permission(workflow_id, &Permission::DeleteWorkflow));
    }

    #[test]
    fn test_workflow_acl_creation() {
        let workflow_id = Uuid::new_v4();
        let acl = WorkflowAcl::new(
            workflow_id,
            Some("user1".to_string()),
            None,
            vec![Permission::ViewWorkflow, Permission::ExecuteWorkflow],
            "admin".to_string(),
        );

        assert_eq!(acl.workflow_id, workflow_id);
        assert_eq!(acl.user_id, Some("user1".to_string()));
        assert!(acl.grants_permission(&Permission::ViewWorkflow));
        assert!(acl.grants_permission(&Permission::ExecuteWorkflow));
        assert!(!acl.grants_permission(&Permission::DeleteWorkflow));
    }

    #[test]
    fn test_role_templates() {
        let admin = RoleTemplates::admin();
        assert!(admin.has_permission(&Permission::ManageSystem));

        let developer = RoleTemplates::developer();
        assert!(developer.has_permission(&Permission::CreateWorkflow));
        assert!(!developer.has_permission(&Permission::ManageSystem));

        let operator = RoleTemplates::operator();
        assert!(operator.has_permission(&Permission::ExecuteWorkflow));
        assert!(!operator.has_permission(&Permission::EditWorkflow));

        let viewer = RoleTemplates::viewer();
        assert!(viewer.has_permission(&Permission::ViewWorkflow));
        assert!(!viewer.has_permission(&Permission::ExecuteWorkflow));
    }

    #[test]
    fn test_rbac_context_all_permissions() {
        let admin = RoleTemplates::admin();
        let developer = RoleTemplates::developer();

        let context = RbacContext::new("user".to_string())
            .with_roles(vec![admin, developer]);

        let all_perms = context.get_all_permissions();
        assert!(all_perms.contains(&Permission::ManageSystem));
        assert!(all_perms.contains(&Permission::CreateWorkflow));
    }
}
