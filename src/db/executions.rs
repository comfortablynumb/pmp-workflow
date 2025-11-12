use crate::models::{ExecutionStatus, NodeExecution, WorkflowExecution};
use anyhow::{Context, Result};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

/// Create a new workflow execution
pub async fn create_workflow_execution(
    pool: &PgPool,
    execution: &WorkflowExecution,
) -> Result<WorkflowExecution> {
    let result = sqlx::query_as::<_, WorkflowExecution>(
        r#"
        INSERT INTO workflow_executions (id, workflow_id, status, started_at, finished_at, input_data, output_data, error)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING *
        "#,
    )
    .bind(execution.id)
    .bind(execution.workflow_id)
    .bind(execution.status.to_string())
    .bind(execution.started_at)
    .bind(execution.finished_at)
    .bind(&execution.input_data)
    .bind(&execution.output_data)
    .bind(&execution.error)
    .fetch_one(pool)
    .await
    .context("Failed to create workflow execution")?;

    Ok(result)
}

/// Get a workflow execution by ID
pub async fn get_workflow_execution(pool: &PgPool, id: Uuid) -> Result<WorkflowExecution> {
    let execution = sqlx::query_as::<_, WorkflowExecution>(
        r#"
        SELECT * FROM workflow_executions WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_one(pool)
    .await
    .context("Failed to get workflow execution")?;

    Ok(execution)
}

/// List workflow executions for a workflow
pub async fn list_workflow_executions(
    pool: &PgPool,
    workflow_id: Uuid,
    limit: Option<i64>,
) -> Result<Vec<WorkflowExecution>> {
    let query = if let Some(limit) = limit {
        sqlx::query_as::<_, WorkflowExecution>(
            r#"
            SELECT * FROM workflow_executions
            WHERE workflow_id = $1
            ORDER BY started_at DESC
            LIMIT $2
            "#,
        )
        .bind(workflow_id)
        .bind(limit)
    } else {
        sqlx::query_as::<_, WorkflowExecution>(
            r#"
            SELECT * FROM workflow_executions
            WHERE workflow_id = $1
            ORDER BY started_at DESC
            "#,
        )
        .bind(workflow_id)
    };

    let executions = query
        .fetch_all(pool)
        .await
        .context("Failed to list workflow executions")?;

    Ok(executions)
}

/// Update workflow execution status
pub async fn update_workflow_execution_status(
    pool: &PgPool,
    id: Uuid,
    status: ExecutionStatus,
    output_data: Option<serde_json::Value>,
    error: Option<String>,
) -> Result<WorkflowExecution> {
    let result = sqlx::query_as::<_, WorkflowExecution>(
        r#"
        UPDATE workflow_executions
        SET status = $2, finished_at = $3, output_data = $4, error = $5
        WHERE id = $1
        RETURNING *
        "#,
    )
    .bind(id)
    .bind(status.to_string())
    .bind(Utc::now())
    .bind(output_data)
    .bind(error)
    .fetch_one(pool)
    .await
    .context("Failed to update workflow execution status")?;

    Ok(result)
}

/// Create a new node execution
pub async fn create_node_execution(
    pool: &PgPool,
    execution: &NodeExecution,
) -> Result<NodeExecution> {
    let result = sqlx::query_as::<_, NodeExecution>(
        r#"
        INSERT INTO node_executions (id, execution_id, node_id, status, started_at, finished_at, input_data, output_data, error)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        RETURNING *
        "#,
    )
    .bind(execution.id)
    .bind(execution.execution_id)
    .bind(&execution.node_id)
    .bind(execution.status.to_string())
    .bind(execution.started_at)
    .bind(execution.finished_at)
    .bind(&execution.input_data)
    .bind(&execution.output_data)
    .bind(&execution.error)
    .fetch_one(pool)
    .await
    .context("Failed to create node execution")?;

    Ok(result)
}

/// Update node execution status
pub async fn update_node_execution_status(
    pool: &PgPool,
    id: Uuid,
    status: ExecutionStatus,
    output_data: Option<serde_json::Value>,
    error: Option<String>,
) -> Result<NodeExecution> {
    let result = sqlx::query_as::<_, NodeExecution>(
        r#"
        UPDATE node_executions
        SET status = $2, finished_at = $3, output_data = $4, error = $5
        WHERE id = $1
        RETURNING *
        "#,
    )
    .bind(id)
    .bind(status.to_string())
    .bind(Utc::now())
    .bind(output_data)
    .bind(error)
    .fetch_one(pool)
    .await
    .context("Failed to update node execution status")?;

    Ok(result)
}

/// List node executions for a workflow execution
pub async fn list_node_executions(pool: &PgPool, execution_id: Uuid) -> Result<Vec<NodeExecution>> {
    let executions = sqlx::query_as::<_, NodeExecution>(
        r#"
        SELECT * FROM node_executions
        WHERE execution_id = $1
        ORDER BY started_at ASC
        "#,
    )
    .bind(execution_id)
    .fetch_all(pool)
    .await
    .context("Failed to list node executions")?;

    Ok(executions)
}
