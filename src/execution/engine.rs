use crate::db;
use crate::models::{
    ExecutionMode, ExecutionStatus, NodeContext, NodeExecution, NodeRegistry, WorkflowDefinition,
    WorkflowExecution,
};
use anyhow::{Context, Result};
use sqlx::PgPool;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tokio::time::timeout;
use uuid::Uuid;

/// Workflow execution engine
pub struct WorkflowEngine {
    pool: PgPool,
    registry: Arc<NodeRegistry>,
}

impl WorkflowEngine {
    pub fn new(pool: PgPool, registry: NodeRegistry) -> Self {
        Self {
            pool,
            registry: Arc::new(registry),
        }
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

    /// Run the workflow nodes
    async fn run_workflow(
        &self,
        workflow: &WorkflowDefinition,
        execution: &WorkflowExecution,
        input_data: Option<serde_json::Value>,
    ) -> Result<serde_json::Value> {
        match workflow.execution_mode {
            ExecutionMode::Sequential => {
                self.run_workflow_sequential(workflow, execution, input_data)
                    .await
            }
            ExecutionMode::Parallel => {
                self.run_workflow_parallel(workflow, execution, input_data)
                    .await
            }
        }
    }

    /// Run the workflow nodes sequentially
    async fn run_workflow_sequential(
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
            let mut input_data_json = serde_json::Map::new();
            for edge in &workflow.edges {
                if edge.to == *node_id {
                    if let Some(input) = node_outputs.get(&edge.from) {
                        let input_key = if edge.to_input.is_empty() {
                            edge.from.clone()
                        } else {
                            edge.to_input.clone()
                        };
                        context.add_input(input_key.clone(), input.clone());
                        input_data_json.insert(input_key, input.clone());
                    }
                }
            }

            // Store input data
            let input_data_value = if input_data_json.is_empty() {
                None
            } else {
                Some(serde_json::Value::Object(input_data_json))
            };

            if let Some(ref input_data) = input_data_value {
                node_execution.input_data = Some(input_data.clone());
            }

            // Determine timeout duration
            let timeout_duration = node_def
                .timeout_seconds
                .or(workflow.timeout_seconds)
                .map(Duration::from_secs);

            // Execute the node with timeout
            let node = self.registry.create(&node_def.node_type)?;
            let execute_future = node.execute(&context, &node_def.parameters);

            let execution_result = if let Some(duration) = timeout_duration {
                match timeout(duration, execute_future).await {
                    Ok(result) => result,
                    Err(_) => Err(anyhow::anyhow!(
                        "Node execution timed out after {} seconds",
                        duration.as_secs()
                    )),
                }
            } else {
                execute_future.await
            };

            match execution_result {
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

    /// Run the workflow nodes in parallel where possible
    async fn run_workflow_parallel(
        &self,
        workflow: &WorkflowDefinition,
        execution: &WorkflowExecution,
        input_data: Option<serde_json::Value>,
    ) -> Result<serde_json::Value> {
        tracing::info!("Running workflow in parallel mode");

        // Group nodes by dependency level
        let levels = self.compute_node_levels(workflow)?;

        // Track node outputs (shared across parallel tasks)
        let node_outputs = Arc::new(RwLock::new(HashMap::<String, serde_json::Value>::new()));
        let workflow_variables = Arc::new(RwLock::new(HashMap::<String, serde_json::Value>::new()));

        // Initialize with input data if provided
        if let Some(input) = input_data {
            workflow_variables
                .write()
                .await
                .insert("input".to_string(), input.clone());

            // If there's a start node, initialize it with input
            for node_def in &workflow.nodes {
                if node_def.node_type == "start" {
                    node_outputs
                        .write()
                        .await
                        .insert(node_def.id.clone(), input.clone());
                    break;
                }
            }
        }

        // Execute nodes level by level
        let mut last_node_id = String::new();
        for (level_num, level_nodes) in levels.iter().enumerate() {
            tracing::info!(
                "Executing level {} with {} nodes",
                level_num,
                level_nodes.len()
            );

            // Execute all nodes in this level in parallel
            let mut tasks = Vec::new();

            for node_id in level_nodes {
                let node_def = workflow
                    .nodes
                    .iter()
                    .find(|n| &n.id == node_id)
                    .ok_or_else(|| anyhow::anyhow!("Node not found: {}", node_id))?
                    .clone();

                let pool = self.pool.clone();
                let registry = Arc::clone(&self.registry);
                let execution_id = execution.id;
                let workflow_edges = workflow.edges.clone();
                let node_outputs_clone = Arc::clone(&node_outputs);
                let workflow_variables_clone = Arc::clone(&workflow_variables);
                let timeout_duration = node_def
                    .timeout_seconds
                    .or(workflow.timeout_seconds)
                    .map(Duration::from_secs);

                // Spawn a task for each node
                let task = tokio::spawn(async move {
                    Self::execute_node(
                        pool,
                        registry,
                        execution_id,
                        node_def,
                        workflow_edges,
                        node_outputs_clone,
                        workflow_variables_clone,
                        timeout_duration,
                    )
                    .await
                });

                tasks.push((node_id.clone(), task));
            }

            // Wait for all tasks in this level to complete
            for (node_id, task) in tasks {
                match task.await {
                    Ok(result) => {
                        result?; // Propagate any execution errors
                        last_node_id = node_id;
                    }
                    Err(e) => {
                        anyhow::bail!("Task for node {} panicked: {}", node_id, e);
                    }
                }
            }
        }

        // Return the output of the last node
        let final_output = node_outputs
            .read()
            .await
            .get(&last_node_id)
            .cloned()
            .unwrap_or(serde_json::json!({}));

        Ok(final_output)
    }

    /// Execute a single node (helper for parallel execution)
    #[allow(clippy::too_many_arguments)]
    async fn execute_node(
        pool: PgPool,
        registry: Arc<NodeRegistry>,
        execution_id: Uuid,
        node_def: crate::models::NodeDefinition,
        workflow_edges: Vec<crate::models::EdgeDefinition>,
        node_outputs: Arc<RwLock<HashMap<String, serde_json::Value>>>,
        workflow_variables: Arc<RwLock<HashMap<String, serde_json::Value>>>,
        timeout_duration: Option<Duration>,
    ) -> Result<()> {
        let node_id = node_def.id.clone();

        // Create node execution record
        let mut node_execution = NodeExecution::new(execution_id, node_id.clone());
        node_execution = db::create_node_execution(&pool, &node_execution).await?;

        tracing::info!("Executing node: {} ({})", node_def.name, node_def.node_type);

        // Prepare node context
        let mut context = NodeContext::new(execution_id.to_string(), node_id.clone());
        context.variables = workflow_variables.read().await.clone();

        // Collect inputs from predecessor nodes
        let mut input_data_json = serde_json::Map::new();
        {
            let outputs = node_outputs.read().await;
            for edge in &workflow_edges {
                if edge.to == node_id {
                    if let Some(input) = outputs.get(&edge.from) {
                        let input_key = if edge.to_input.is_empty() {
                            edge.from.clone()
                        } else {
                            edge.to_input.clone()
                        };
                        context.add_input(input_key.clone(), input.clone());
                        input_data_json.insert(input_key, input.clone());
                    }
                }
            }
        }

        // Store input data
        let input_data_value = if input_data_json.is_empty() {
            None
        } else {
            Some(serde_json::Value::Object(input_data_json))
        };

        if let Some(ref input_data) = input_data_value {
            node_execution.input_data = Some(input_data.clone());
        }

        // Execute the node with timeout
        let node = registry.create(&node_def.node_type)?;
        let execute_future = node.execute(&context, &node_def.parameters);

        let execution_result = if let Some(duration) = timeout_duration {
            match timeout(duration, execute_future).await {
                Ok(result) => result,
                Err(_) => Err(anyhow::anyhow!(
                    "Node execution timed out after {} seconds",
                    duration.as_secs()
                )),
            }
        } else {
            execute_future.await
        };

        match execution_result {
            Ok(output) => {
                if output.success {
                    tracing::info!("Node {} completed successfully", node_id);

                    // Store output for downstream nodes
                    node_outputs
                        .write()
                        .await
                        .insert(node_id.clone(), output.data.clone());

                    // Update node execution as successful
                    db::update_node_execution_status(
                        &pool,
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
                        &pool,
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
                    &pool,
                    node_execution.id,
                    ExecutionStatus::Failed,
                    None,
                    Some(e.to_string()),
                )
                .await?;

                return Err(e);
            }
        }

        Ok(())
    }

    /// Compute node levels for parallel execution
    /// Returns a vector of levels, where each level is a vector of node IDs that can be executed in parallel
    fn compute_node_levels(&self, workflow: &WorkflowDefinition) -> Result<Vec<Vec<String>>> {
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

        // Group nodes by level
        let mut levels: Vec<Vec<String>> = Vec::new();
        let mut current_level: Vec<String> = in_degree
            .iter()
            .filter(|(_, &degree)| degree == 0)
            .map(|(id, _)| id.clone())
            .collect();

        let mut processed = 0;

        while !current_level.is_empty() {
            levels.push(current_level.clone());
            processed += current_level.len();

            let mut next_level = Vec::new();

            for node_id in &current_level {
                // Reduce in-degree for neighbors
                if let Some(neighbors) = adj_list.get(node_id) {
                    for neighbor in neighbors {
                        let degree = in_degree.get_mut(neighbor).unwrap();
                        *degree -= 1;
                        if *degree == 0 {
                            next_level.push(neighbor.clone());
                        }
                    }
                }
            }

            current_level = next_level;
        }

        // Check if all nodes were processed (cycle detection)
        if processed != workflow.nodes.len() {
            anyhow::bail!("Workflow contains a cycle");
        }

        Ok(levels)
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
