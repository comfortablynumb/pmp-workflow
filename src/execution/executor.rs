use crate::db;
use crate::execution::WorkflowEngine;
use crate::models::{NodeRegistry, WorkflowExecution};
use anyhow::Result;
use sqlx::PgPool;
use uuid::Uuid;

/// High-level executor for running workflows
pub struct WorkflowExecutor {
    engine: WorkflowEngine,
    pool: PgPool,
}

impl WorkflowExecutor {
    pub fn new(pool: PgPool, registry: NodeRegistry) -> Self {
        let engine = WorkflowEngine::new(pool.clone(), registry);
        Self { engine, pool }
    }

    /// Execute a workflow by ID
    pub async fn execute_by_id(
        &self,
        workflow_id: Uuid,
        input_data: Option<serde_json::Value>,
    ) -> Result<WorkflowExecution> {
        // Fetch workflow from database
        let workflow = db::get_workflow(&self.pool, workflow_id).await?;

        // Convert to definition
        let definition = workflow.to_definition()?;

        // Execute
        self.engine
            .execute_workflow(&definition, workflow_id, input_data)
            .await
    }

    /// Execute a workflow by name
    pub async fn execute_by_name(
        &self,
        workflow_name: &str,
        input_data: Option<serde_json::Value>,
    ) -> Result<WorkflowExecution> {
        // Fetch workflow from database
        let workflow = db::get_workflow_by_name(&self.pool, workflow_name).await?;

        // Convert to definition
        let definition = workflow.to_definition()?;

        // Execute
        self.engine
            .execute_workflow(&definition, workflow.id, input_data)
            .await
    }

    /// Get execution result
    pub async fn get_execution(&self, execution_id: Uuid) -> Result<WorkflowExecution> {
        db::get_workflow_execution(&self.pool, execution_id).await
    }
}
