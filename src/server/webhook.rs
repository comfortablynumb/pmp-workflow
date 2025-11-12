use crate::{create_node_registry, db, WorkflowExecutor};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Json, Response},
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::sync::Arc;
use tower_http::{
    cors::CorsLayer,
    trace::{DefaultMakeSpan, TraceLayer},
};
use uuid::Uuid;

/// Application state shared across handlers
#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
}

/// Request body for webhook trigger
#[derive(Debug, Deserialize)]
pub struct WebhookTriggerRequest {
    /// Optional input data for the workflow
    #[serde(default)]
    pub data: serde_json::Value,
}

/// Response for webhook trigger
#[derive(Debug, Serialize)]
pub struct WebhookTriggerResponse {
    pub success: bool,
    pub execution_id: Uuid,
    pub workflow_id: Uuid,
    pub message: String,
}

/// Error response
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub details: Option<String>,
}

/// Custom error type for API errors
pub enum ApiError {
    NotFound(String),
    BadRequest(String),
    InternalError(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error_message, details) = match self {
            ApiError::NotFound(msg) => (StatusCode::NOT_FOUND, "Not Found", Some(msg)),
            ApiError::BadRequest(msg) => (StatusCode::BAD_REQUEST, "Bad Request", Some(msg)),
            ApiError::InternalError(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Error",
                Some(msg),
            ),
        };

        let body = Json(ErrorResponse {
            error: error_message.to_string(),
            details,
        });

        (status, body).into_response()
    }
}

/// Health check endpoint
async fn health_check() -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "healthy",
        "service": "pmp-workflow",
        "version": env!("CARGO_PKG_VERSION")
    }))
}

/// Webhook trigger endpoint
async fn trigger_webhook(
    State(state): State<Arc<AppState>>,
    Path((workflow_id, trigger_node_id)): Path<(Uuid, String)>,
    Json(payload): Json<WebhookTriggerRequest>,
) -> Result<Json<WebhookTriggerResponse>, ApiError> {
    tracing::info!(
        "Webhook trigger received for workflow {} and trigger node {}",
        workflow_id,
        trigger_node_id
    );

    // Fetch workflow from database
    let workflow = db::get_workflow(&state.pool, workflow_id)
        .await
        .map_err(|e| ApiError::NotFound(format!("Workflow not found: {}", e)))?;

    // Verify that the workflow is active
    if !workflow.active {
        return Err(ApiError::BadRequest("Workflow is not active".to_string()));
    }

    // Verify that the trigger node exists in the workflow
    let workflow_def = workflow
        .to_definition()
        .map_err(|e| ApiError::InternalError(format!("Failed to parse workflow: {}", e)))?;

    let trigger_node = workflow_def
        .nodes
        .iter()
        .find(|n| n.id == trigger_node_id)
        .ok_or_else(|| {
            ApiError::NotFound(format!(
                "Trigger node '{}' not found in workflow",
                trigger_node_id
            ))
        })?;

    // Verify that the node is a webhook trigger
    if trigger_node.node_type != "webhook_trigger" {
        return Err(ApiError::BadRequest(format!(
            "Node '{}' is not a webhook trigger (type: {})",
            trigger_node_id, trigger_node.node_type
        )));
    }

    // Execute the workflow
    let registry = create_node_registry();
    let executor = WorkflowExecutor::new(state.pool.clone(), registry);

    let input_data = if payload.data.is_null() {
        None
    } else {
        Some(payload.data)
    };

    let execution = executor
        .execute_by_id(workflow_id, input_data)
        .await
        .map_err(|e| ApiError::InternalError(format!("Workflow execution failed: {}", e)))?;

    tracing::info!(
        "Workflow {} execution started with ID {}",
        workflow_id,
        execution.id
    );

    Ok(Json(WebhookTriggerResponse {
        success: true,
        execution_id: execution.id,
        workflow_id,
        message: "Workflow execution started successfully".to_string(),
    }))
}

/// Create the webhook server router
pub fn create_router(pool: PgPool) -> Router {
    let state = Arc::new(AppState { pool });

    Router::new()
        .route("/health", get(health_check))
        .route(
            "/api/v1/webhook/:workflow_id/trigger/:trigger_node_id",
            post(trigger_webhook),
        )
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::default().include_headers(true)),
        )
        .layer(CorsLayer::permissive())
        .with_state(state)
}

/// Start the webhook server
pub async fn start_server(pool: PgPool, host: &str, port: u16) -> anyhow::Result<()> {
    let app = create_router(pool);

    let addr = format!("{}:{}", host, port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    tracing::info!("Webhook server listening on http://{}", addr);
    tracing::info!(
        "Webhook endpoint: POST http://{}/api/v1/webhook/{{workflow-id}}/trigger/{{trigger-node-id}}",
        addr
    );

    axum::serve(listener, app).await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use tower::ServiceExt;

    #[tokio::test]
    #[ignore] // Requires a database connection
    async fn test_health_check() {
        // This test requires a real database connection
        let database_url =
            std::env::var("DATABASE_URL").expect("DATABASE_URL must be set for integration tests");

        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(1)
            .connect(&database_url)
            .await
            .expect("Failed to connect to database");

        let app = create_router(pool);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}
