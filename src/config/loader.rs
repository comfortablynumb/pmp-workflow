use crate::models::{NodeCategory, NodeRegistry, WorkflowDefinition};
use anyhow::{Context, Result};
use std::path::Path;

/// Load a workflow definition from a YAML file
pub async fn load_workflow_from_file(path: &Path) -> Result<WorkflowDefinition> {
    let content = tokio::fs::read_to_string(path)
        .await
        .with_context(|| format!("Failed to read workflow file: {}", path.display()))?;

    let workflow: WorkflowDefinition = serde_yaml::from_str(&content)
        .with_context(|| format!("Failed to parse workflow YAML: {}", path.display()))?;

    validate_workflow(&workflow, None)?;

    Ok(workflow)
}

/// Load a workflow definition from a YAML string
pub fn load_workflow_from_str(yaml: &str) -> Result<WorkflowDefinition> {
    let workflow: WorkflowDefinition =
        serde_yaml::from_str(yaml).context("Failed to parse workflow YAML")?;

    validate_workflow(&workflow, None)?;

    Ok(workflow)
}

/// Load and validate a workflow with a registry to check node categories
pub fn load_workflow_with_registry(
    yaml: &str,
    registry: &NodeRegistry,
) -> Result<WorkflowDefinition> {
    let workflow: WorkflowDefinition =
        serde_yaml::from_str(yaml).context("Failed to parse workflow YAML")?;

    validate_workflow(&workflow, Some(registry))?;

    Ok(workflow)
}

/// Validate a workflow definition
fn validate_workflow(workflow: &WorkflowDefinition, registry: Option<&NodeRegistry>) -> Result<()> {
    // Check that workflow has at least one node
    if workflow.nodes.is_empty() {
        anyhow::bail!("Workflow must have at least one node");
    }

    // Check that all node IDs are unique
    let mut seen_ids = std::collections::HashSet::new();
    for node in &workflow.nodes {
        if !seen_ids.insert(&node.id) {
            anyhow::bail!("Duplicate node ID: {}", node.id);
        }
    }

    // Check that all edges reference valid nodes
    for edge in &workflow.edges {
        if !seen_ids.contains(&edge.from) {
            anyhow::bail!("Edge references unknown source node: {}", edge.from);
        }
        if !seen_ids.contains(&edge.to) {
            anyhow::bail!("Edge references unknown target node: {}", edge.to);
        }
    }

    // Check for cycles (basic check - we'll do a more thorough check during execution)
    if has_cycle(workflow) {
        anyhow::bail!("Workflow contains a cycle");
    }

    // If registry is provided, validate that first nodes are trigger nodes
    if let Some(registry) = registry {
        validate_trigger_nodes(workflow, registry)?;
    }

    Ok(())
}

/// Validate that all starting nodes (nodes with no incoming edges) are trigger nodes
fn validate_trigger_nodes(workflow: &WorkflowDefinition, registry: &NodeRegistry) -> Result<()> {
    use std::collections::HashSet;

    // Find all nodes that have incoming edges
    let mut nodes_with_incoming: HashSet<&str> = HashSet::new();
    for edge in &workflow.edges {
        nodes_with_incoming.insert(&edge.to);
    }

    // Find starting nodes (nodes with no incoming edges)
    let starting_nodes: Vec<_> = workflow
        .nodes
        .iter()
        .filter(|node| !nodes_with_incoming.contains(node.id.as_str()))
        .collect();

    if starting_nodes.is_empty() {
        anyhow::bail!(
            "Workflow must have at least one starting node (node with no incoming edges)"
        );
    }

    // Validate that all starting nodes are trigger nodes
    for node in starting_nodes {
        // Create node instance to check category
        let node_instance = registry
            .create(&node.node_type)
            .with_context(|| format!("Unknown node type: {}", node.node_type))?;

        if node_instance.category() != NodeCategory::Trigger {
            anyhow::bail!(
                "Starting node '{}' (type: '{}') must be a trigger node. Only trigger nodes can be used as the first node in a workflow.",
                node.id,
                node.node_type
            );
        }
    }

    Ok(())
}

/// Check if the workflow has a cycle using DFS
fn has_cycle(workflow: &WorkflowDefinition) -> bool {
    use std::collections::{HashMap, HashSet};

    // Build adjacency list
    let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();
    for node in &workflow.nodes {
        graph.insert(&node.id, Vec::new());
    }
    for edge in &workflow.edges {
        graph.get_mut(edge.from.as_str()).unwrap().push(&edge.to);
    }

    // DFS to detect cycles
    let mut visited = HashSet::new();
    let mut rec_stack = HashSet::new();

    fn dfs<'a>(
        node: &'a str,
        graph: &HashMap<&'a str, Vec<&'a str>>,
        visited: &mut HashSet<&'a str>,
        rec_stack: &mut HashSet<&'a str>,
    ) -> bool {
        visited.insert(node);
        rec_stack.insert(node);

        if let Some(neighbors) = graph.get(node) {
            for &neighbor in neighbors {
                if !visited.contains(neighbor) {
                    if dfs(neighbor, graph, visited, rec_stack) {
                        return true;
                    }
                } else if rec_stack.contains(neighbor) {
                    return true;
                }
            }
        }

        rec_stack.remove(node);
        false
    }

    for node in workflow.nodes.iter() {
        if !visited.contains(node.id.as_str())
            && dfs(&node.id, &graph, &mut visited, &mut rec_stack)
        {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_valid_workflow() {
        let yaml = r#"
name: Test Workflow
description: A test workflow
nodes:
  - id: node1
    node_type: start
    name: Start Node
    parameters: {}
  - id: node2
    node_type: end
    name: End Node
    parameters: {}
edges:
  - from: node1
    to: node2
"#;

        let result = load_workflow_from_str(yaml);
        assert!(result.is_ok());
    }

    #[test]
    fn test_detect_cycle() {
        let yaml = r#"
name: Cyclic Workflow
nodes:
  - id: node1
    node_type: start
    name: Node 1
    parameters: {}
  - id: node2
    node_type: middle
    name: Node 2
    parameters: {}
edges:
  - from: node1
    to: node2
  - from: node2
    to: node1
"#;

        let result = load_workflow_from_str(yaml);
        assert!(result.is_err());
    }

    #[test]
    fn test_trigger_node_validation_with_trigger_start() {
        use crate::nodes::{HttpRequestNode, ManualTriggerNode};

        let mut registry = NodeRegistry::new();
        registry.register("manual_trigger", || Box::new(ManualTriggerNode));
        registry.register("http_request", || Box::new(HttpRequestNode));

        let yaml = r#"
name: Valid Trigger Workflow
nodes:
  - id: trigger1
    node_type: manual_trigger
    name: Manual Trigger
    parameters: {}
  - id: action1
    node_type: http_request
    name: HTTP Request
    parameters:
      url: "https://example.com"
edges:
  - from: trigger1
    to: action1
"#;

        let result = load_workflow_with_registry(yaml, &registry);
        assert!(
            result.is_ok(),
            "Workflow with trigger start should be valid"
        );
    }

    #[test]
    fn test_trigger_node_validation_with_action_start() {
        use crate::nodes::{HttpRequestNode, ManualTriggerNode};

        let mut registry = NodeRegistry::new();
        registry.register("manual_trigger", || Box::new(ManualTriggerNode));
        registry.register("http_request", || Box::new(HttpRequestNode));

        let yaml = r#"
name: Invalid Action Start Workflow
nodes:
  - id: action1
    node_type: http_request
    name: HTTP Request
    parameters:
      url: "https://example.com"
  - id: trigger1
    node_type: manual_trigger
    name: Manual Trigger
    parameters: {}
edges:
  - from: action1
    to: trigger1
"#;

        let result = load_workflow_with_registry(yaml, &registry);
        assert!(
            result.is_err(),
            "Workflow starting with action node should be invalid"
        );
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("must be a trigger node")
        );
    }

    #[test]
    fn test_trigger_node_validation_multiple_start_nodes() {
        use crate::nodes::{HttpRequestNode, ManualTriggerNode, WebhookTriggerNode};

        let mut registry = NodeRegistry::new();
        registry.register("manual_trigger", || Box::new(ManualTriggerNode));
        registry.register("webhook_trigger", || Box::new(WebhookTriggerNode));
        registry.register("http_request", || Box::new(HttpRequestNode));

        let yaml = r#"
name: Multiple Triggers Workflow
nodes:
  - id: trigger1
    node_type: manual_trigger
    name: Manual Trigger
    parameters: {}
  - id: trigger2
    node_type: webhook_trigger
    name: Webhook Trigger
    parameters: {}
  - id: action1
    node_type: http_request
    name: HTTP Request
    parameters:
      url: "https://example.com"
edges:
  - from: trigger1
    to: action1
  - from: trigger2
    to: action1
"#;

        let result = load_workflow_with_registry(yaml, &registry);
        assert!(
            result.is_ok(),
            "Workflow with multiple trigger starts should be valid"
        );
    }

    #[test]
    fn test_trigger_node_validation_mixed_start_nodes() {
        use crate::nodes::{HttpRequestNode, ManualTriggerNode};

        let mut registry = NodeRegistry::new();
        registry.register("manual_trigger", || Box::new(ManualTriggerNode));
        registry.register("http_request", || Box::new(HttpRequestNode));

        let yaml = r#"
name: Mixed Start Nodes Workflow
nodes:
  - id: trigger1
    node_type: manual_trigger
    name: Manual Trigger
    parameters: {}
  - id: action1
    node_type: http_request
    name: HTTP Request 1
    parameters:
      url: "https://example.com"
  - id: action2
    node_type: http_request
    name: HTTP Request 2
    parameters:
      url: "https://example.com"
edges:
  - from: trigger1
    to: action2
"#;

        let result = load_workflow_with_registry(yaml, &registry);
        assert!(
            result.is_err(),
            "Workflow with action node as a starting node should be invalid"
        );
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("must be a trigger node")
        );
    }
}
