use crate::db;
use crate::models::{
    ExecutionStatus, NodeContext, NodeExecution, NodeRegistry, WorkflowDefinition,
    WorkflowExecution,
};
use anyhow::{Context, Result};
use sqlx::PgPool;
use std::collections::HashMap;
use uuid::Uuid;

/// Workflow execution engine
pub struct WorkflowEngine {
    pool: PgPool,
    registry: NodeRegistry,
}

impl WorkflowEngine {
    pub fn new(pool: PgPool, registry: NodeRegistry) -> Self {
        Self { pool, registry }
    }

    /// Execute a workflow
    pub async fn execute_workflow(
        &self,
        workflow: &WorkflowDefinition,
        workflow_id: Uuid,
        input_data: Option<serde_json::Value>,
    ) -> Result<WorkflowExecution> {
        // Create workflow execution record
        let mut execution = WorkflowExecution::new(workflow_id, input_data.clone());
        execution = db::create_workflow_execution(&self.pool, &execution).await?;

        tracing::info!(
            "Starting workflow execution {} for workflow {}",
            execution.id,
            workflow_id
        );

        // Execute the workflow
        match self.run_workflow(workflow, &execution, input_data).await {
            Ok(output) => {
                // Update execution as successful
                execution = db::update_workflow_execution_status(
                    &self.pool,
                    execution.id,
                    ExecutionStatus::Success,
                    Some(output),
                    None,
                )
                .await?;

                tracing::info!("Workflow execution {} completed successfully", execution.id);
            }
            Err(e) => {
                // Update execution as failed
                execution = db::update_workflow_execution_status(
                    &self.pool,
                    execution.id,
                    ExecutionStatus::Failed,
                    None,
                    Some(e.to_string()),
                )
                .await?;

                tracing::error!("Workflow execution {} failed: {}", execution.id, e);
            }
        }

        Ok(execution)
    }

    /// Run the workflow nodes in topological order
    async fn run_workflow(
        &self,
        workflow: &WorkflowDefinition,
        execution: &WorkflowExecution,
        input_data: Option<serde_json::Value>,
    ) -> Result<serde_json::Value> {
        // Build execution order using topological sort
        let execution_order = self.topological_sort(workflow)?;

        // Track node outputs
        let mut node_outputs: HashMap<String, serde_json::Value> = HashMap::new();
        let mut workflow_variables: HashMap<String, serde_json::Value> = HashMap::new();

        // Initialize with input data if provided
        if let Some(input) = input_data {
            workflow_variables.insert("input".to_string(), input.clone());

            // If there's a start node, initialize it with input
            for node_def in &workflow.nodes {
                if node_def.node_type == "start" {
                    node_outputs.insert(node_def.id.clone(), input.clone());
                    break;
                }
            }
        }

        // Execute nodes in order
        for node_id in &execution_order {
            let node_def = workflow
                .nodes
                .iter()
                .find(|n| n.id == *node_id)
                .ok_or_else(|| anyhow::anyhow!("Node not found: {}", node_id))?;

            // Create node execution record
            let mut node_execution = NodeExecution::new(execution.id, node_id.clone());
            node_execution = db::create_node_execution(&self.pool, &node_execution).await?;

            tracing::info!("Executing node: {} ({})", node_def.name, node_def.node_type);

            // Prepare node context
            let mut context = NodeContext::new(execution.id.to_string(), node_id.clone());
            context.variables = workflow_variables.clone();

            // Collect inputs from predecessor nodes
            for edge in &workflow.edges {
                if edge.to == *node_id {
                    if let Some(input) = node_outputs.get(&edge.from) {
                        let input_key = if edge.to_input.is_empty() {
                            edge.from.clone()
                        } else {
                            edge.to_input.clone()
                        };
                        context.add_input(input_key, input.clone());
                    }
                }
            }

            // Execute the node
            let node = self.registry.create(&node_def.node_type)?;

            match node.execute(&context, &node_def.parameters).await {
                Ok(output) => {
                    if output.success {
                        tracing::info!("Node {} completed successfully", node_id);

                        // Store output for downstream nodes
                        node_outputs.insert(node_id.clone(), output.data.clone());

                        // Update node execution as successful
                        db::update_node_execution_status(
                            &self.pool,
                            node_execution.id,
                            ExecutionStatus::Success,
                            Some(output.data),
                            None,
                        )
                        .await?;
                    } else {
                        let error_msg = output.error.unwrap_or_else(|| "Unknown error".to_string());
                        tracing::error!("Node {} failed: {}", node_id, error_msg);

                        // Update node execution as failed
                        db::update_node_execution_status(
                            &self.pool,
                            node_execution.id,
                            ExecutionStatus::Failed,
                            None,
                            Some(error_msg.clone()),
                        )
                        .await?;

                        anyhow::bail!("Node {} failed: {}", node_id, error_msg);
                    }
                }
                Err(e) => {
                    tracing::error!("Node {} error: {}", node_id, e);

                    // Update node execution as failed
                    db::update_node_execution_status(
                        &self.pool,
                        node_execution.id,
                        ExecutionStatus::Failed,
                        None,
                        Some(e.to_string()),
                    )
                    .await?;

                    return Err(e);
                }
            }
        }

        // Return the output of the last node
        let last_node_id = execution_order.last().context("No nodes executed")?;
        let final_output = node_outputs
            .get(last_node_id)
            .cloned()
            .unwrap_or(serde_json::json!({}));

        Ok(final_output)
    }

    /// Perform topological sort to determine execution order
    fn topological_sort(&self, workflow: &WorkflowDefinition) -> Result<Vec<String>> {
        let mut in_degree: HashMap<String, usize> = HashMap::new();
        let mut adj_list: HashMap<String, Vec<String>> = HashMap::new();

        // Initialize
        for node in &workflow.nodes {
            in_degree.insert(node.id.clone(), 0);
            adj_list.insert(node.id.clone(), Vec::new());
        }

        // Build adjacency list and in-degree map
        for edge in &workflow.edges {
            adj_list.get_mut(&edge.from).unwrap().push(edge.to.clone());
            *in_degree.get_mut(&edge.to).unwrap() += 1;
        }

        // Find nodes with no incoming edges
        let mut queue: Vec<String> = in_degree
            .iter()
            .filter(|(_, &degree)| degree == 0)
            .map(|(id, _)| id.clone())
            .collect();

        let mut result = Vec::new();

        while let Some(node_id) = queue.pop() {
            result.push(node_id.clone());

            // Reduce in-degree for neighbors
            if let Some(neighbors) = adj_list.get(&node_id) {
                for neighbor in neighbors {
                    let degree = in_degree.get_mut(neighbor).unwrap();
                    *degree -= 1;
                    if *degree == 0 {
                        queue.push(neighbor.clone());
                    }
                }
            }
        }

        // Check if all nodes were processed (cycle detection)
        if result.len() != workflow.nodes.len() {
            anyhow::bail!("Workflow contains a cycle");
        }

        Ok(result)
    }
}
