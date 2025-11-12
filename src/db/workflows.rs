use crate::models::{Workflow, WorkflowDefinition};
use anyhow::{Context, Result};
use sqlx::PgPool;
use uuid::Uuid;

/// Create a new workflow in the database
pub async fn create_workflow(pool: &PgPool, workflow: &Workflow) -> Result<Workflow> {
    let result = sqlx::query_as::<_, Workflow>(
        r#"
        INSERT INTO workflows (id, name, description, active, nodes, edges, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING *
        "#,
    )
    .bind(workflow.id)
    .bind(&workflow.name)
    .bind(&workflow.description)
    .bind(workflow.active)
    .bind(&workflow.nodes)
    .bind(&workflow.edges)
    .bind(workflow.created_at)
    .bind(workflow.updated_at)
    .fetch_one(pool)
    .await
    .context("Failed to create workflow")?;

    Ok(result)
}

/// Get a workflow by ID
pub async fn get_workflow(pool: &PgPool, id: Uuid) -> Result<Workflow> {
    let workflow = sqlx::query_as::<_, Workflow>(
        r#"
        SELECT * FROM workflows WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_one(pool)
    .await
    .context("Failed to get workflow")?;

    Ok(workflow)
}

/// Get a workflow by name
pub async fn get_workflow_by_name(pool: &PgPool, name: &str) -> Result<Workflow> {
    let workflow = sqlx::query_as::<_, Workflow>(
        r#"
        SELECT * FROM workflows WHERE name = $1
        "#,
    )
    .bind(name)
    .fetch_one(pool)
    .await
    .context("Failed to get workflow by name")?;

    Ok(workflow)
}

/// List all workflows
pub async fn list_workflows(pool: &PgPool, active_only: bool) -> Result<Vec<Workflow>> {
    let query = if active_only {
        "SELECT * FROM workflows WHERE active = true ORDER BY created_at DESC"
    } else {
        "SELECT * FROM workflows ORDER BY created_at DESC"
    };

    let workflows = sqlx::query_as::<_, Workflow>(query)
        .fetch_all(pool)
        .await
        .context("Failed to list workflows")?;

    Ok(workflows)
}

/// Update a workflow
pub async fn update_workflow(pool: &PgPool, workflow: &Workflow) -> Result<Workflow> {
    let result = sqlx::query_as::<_, Workflow>(
        r#"
        UPDATE workflows
        SET name = $2, description = $3, active = $4, nodes = $5, edges = $6, updated_at = NOW()
        WHERE id = $1
        RETURNING *
        "#,
    )
    .bind(workflow.id)
    .bind(&workflow.name)
    .bind(&workflow.description)
    .bind(workflow.active)
    .bind(&workflow.nodes)
    .bind(&workflow.edges)
    .fetch_one(pool)
    .await
    .context("Failed to update workflow")?;

    Ok(result)
}

/// Delete a workflow
pub async fn delete_workflow(pool: &PgPool, id: Uuid) -> Result<()> {
    sqlx::query("DELETE FROM workflows WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await
        .context("Failed to delete workflow")?;

    Ok(())
}

/// Import a workflow from a definition
pub async fn import_workflow(pool: &PgPool, definition: &WorkflowDefinition) -> Result<Workflow> {
    let workflow = definition.to_workflow()?;
    create_workflow(pool, &workflow).await
}
