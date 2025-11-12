use crate::models::WorkflowDefinition;
use anyhow::{Context, Result};
use std::path::Path;

/// Load a workflow definition from a YAML file
pub async fn load_workflow_from_file(path: &Path) -> Result<WorkflowDefinition> {
    let content = tokio::fs::read_to_string(path)
        .await
        .with_context(|| format!("Failed to read workflow file: {}", path.display()))?;

    let workflow: WorkflowDefinition = serde_yaml::from_str(&content)
        .with_context(|| format!("Failed to parse workflow YAML: {}", path.display()))?;

    validate_workflow(&workflow)?;

    Ok(workflow)
}

/// Load a workflow definition from a YAML string
pub fn load_workflow_from_str(yaml: &str) -> Result<WorkflowDefinition> {
    let workflow: WorkflowDefinition = serde_yaml::from_str(yaml)
        .context("Failed to parse workflow YAML")?;

    validate_workflow(&workflow)?;

    Ok(workflow)
}

/// Validate a workflow definition
fn validate_workflow(workflow: &WorkflowDefinition) -> Result<()> {
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
        graph
            .get_mut(edge.from.as_str())
            .unwrap()
            .push(&edge.to);
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
        if !visited.contains(node.id.as_str()) {
            if dfs(&node.id, &graph, &mut visited, &mut rec_stack) {
                return true;
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{EdgeDefinition, NodeDefinition};

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
}
